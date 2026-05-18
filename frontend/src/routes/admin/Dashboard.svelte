<script lang="ts">
  import { onMount } from 'svelte';
  import { navigate } from '../../lib/router';
  import { authState, requireAuth } from '../../lib/auth';
  import { listGaleri } from '../../lib/api/galeri';
  import { listLowongan } from '../../lib/api/lowongan';
  import { listBerita } from '../../lib/api/berita';
  import { getRingkasan } from '../../lib/api/penduduk';
  import AdminShell from '../../lib/admin/AdminShell.svelte';
  import DonutChart from '../../lib/admin/charts/DonutChart.svelte';
  import BarChart from '../../lib/admin/charts/BarChart.svelte';
  import {
    ArrowRight,
    ImageIcon,
    Briefcase,
    Newspaper,
    Users
  } from '../../lib/components/icons';
  import type { Lowongan, BeritaRingkasan, PendudukRingkasan } from '../../lib/types';

  interface SummaryCard {
    key: 'galeri' | 'lowongan' | 'berita' | 'penduduk';
    title: string;
    description: string;
    href: string;
    icon: typeof ImageIcon;
    accent: string;
  }

  const cards: SummaryCard[] = [
    {
      key: 'galeri',
      title: 'Galeri',
      description: 'Foto wisata, budaya, agrowisata, dan kegiatan desa.',
      href: '/admin/galeri',
      icon: ImageIcon,
      accent: 'bg-menoreh-50 text-menoreh-700'
    },
    {
      key: 'lowongan',
      title: 'Lowongan',
      description: 'Lowongan kerja UMKM, formal, dan freelance.',
      href: '/admin/lowongan',
      icon: Briefcase,
      accent: 'bg-terakota-50 text-terakota-700'
    },
    {
      key: 'berita',
      title: 'Berita',
      description: 'Berita dan agenda kegiatan desa.',
      href: '/admin/berita',
      icon: Newspaper,
      accent: 'bg-tanah-50 text-tanah-700'
    },
    {
      key: 'penduduk',
      title: 'Data Penduduk',
      description: 'Ringkasan jumlah penduduk per pedukuhan.',
      href: '/admin/penduduk',
      icon: Users,
      accent: 'bg-krem-100 text-arang-700'
    }
  ];

  // null = loading, number = ready, undefined = fetch failed (silently degrades
  // to a dash placeholder). We deliberately reuse the public GET endpoints
  // because the backend exposes them without auth and they already return the
  // row sets we need to count.
  type CountState = number | null | undefined;
  let counts: Record<SummaryCard['key'], CountState> = {
    galeri: null,
    lowongan: null,
    berita: null,
    penduduk: null
  };

  // Raw datasets retained for chart derivation. Loading state is tracked
  // separately from the stat counts so we can skeleton the chart blocks
  // independently of the KPI strip. Galeri rows are only counted (no chart
  // consumes them yet) so we keep that one as a derived count above.
  let lowonganRows: Lowongan[] = [];
  let beritaRows: BeritaRingkasan[] = [];
  let pendudukRingkasan: PendudukRingkasan | null = null;
  let chartsLoading = true;

  async function loadDashboard(): Promise<void> {
    const results = await Promise.allSettled([
      listGaleri(),
      listLowongan(),
      listBerita(),
      getRingkasan()
    ]);

    if (results[1].status === 'fulfilled') lowonganRows = results[1].value;
    if (results[2].status === 'fulfilled') beritaRows = results[2].value;
    if (results[3].status === 'fulfilled') pendudukRingkasan = results[3].value;

    counts = {
      galeri: results[0].status === 'fulfilled' ? results[0].value.length : undefined,
      lowongan:
        results[1].status === 'fulfilled'
          ? results[1].value.filter((l) => l.status === 'aktif').length
          : undefined,
      berita: results[2].status === 'fulfilled' ? results[2].value.length : undefined,
      penduduk:
        results[3].status === 'fulfilled' ? results[3].value.total_penduduk : undefined
    };

    chartsLoading = false;
  }

  onMount(() => {
    if (!requireAuth()) return;
    loadDashboard();
  });

  function formatCount(value: CountState): string {
    if (value === null) return '...';
    if (value === undefined) return '-';
    return value.toLocaleString('id-ID');
  }

  function unitFor(key: SummaryCard['key']): string {
    if (key === 'lowongan') return 'aktif';
    if (key === 'penduduk') return 'jiwa';
    return 'entri';
  }

  function goTo(href: string): void {
    navigate(href);
  }

  // Long-form Indonesian date for the welcome strip. Recomputed at mount only;
  // refreshing on a timer would be overkill for a dashboard that the operator
  // reloads anyway when starting a session.
  const todayLong = new Date().toLocaleDateString('id-ID', {
    weekday: 'long',
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  });

  // ----- Chart derivations -----
  // Demografi donut: 2-segment laki vs perempuan. menoreh-600 / terakota-600
  // pulled from tailwind.config.js so the values stay in lockstep with the
  // rest of the palette.
  $: demografiData = pendudukRingkasan
    ? [
        { label: 'Laki-laki', value: pendudukRingkasan.total_laki, color: '#244226' },
        { label: 'Perempuan', value: pendudukRingkasan.total_perempuan, color: '#9a4a2c' }
      ]
    : [];

  // Lowongan by category. We count every job (any status) since the bar chart
  // visualises distribution across categories rather than active-only metrics
  // (the KPI strip already surfaces the "aktif" count). Stable category order
  // keeps the bar layout deterministic between renders.
  const LOWONGAN_KATEGORIS: Array<{ key: string; label: string; color: string }> = [
    { key: 'umkm', label: 'UMKM', color: '#9a4a2c' },
    { key: 'formal', label: 'Formal', color: '#244226' },
    { key: 'freelance', label: 'Freelance', color: '#856642' }
  ];
  $: lowonganByKategori = LOWONGAN_KATEGORIS.map(({ key, label, color }) => ({
    label,
    color,
    value: lowonganRows.filter((l) => l.kategori === key).length
  }));

  // Berita per month: 6-month rolling window ending on the current month.
  // We bucket by `published_at` truncated to YYYY-MM, then map back to a
  // short Indonesian month label for the x-axis.
  $: beritaPerBulan = (() => {
    const buckets: { key: string; label: string }[] = [];
    const now = new Date();
    for (let i = 5; i >= 0; i--) {
      const d = new Date(now.getFullYear(), now.getMonth() - i, 1);
      const key = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}`;
      const label = d.toLocaleDateString('id-ID', { month: 'short' });
      buckets.push({ key, label });
    }

    const counts = new Map<string, number>();
    for (const b of beritaRows) {
      if (!b.published_at) continue;
      const d = new Date(b.published_at);
      if (Number.isNaN(d.getTime())) continue;
      const key = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}`;
      counts.set(key, (counts.get(key) ?? 0) + 1);
    }

    return buckets.map((b) => ({
      label: b.label,
      value: counts.get(b.key) ?? 0,
      color: '#3e7242'
    }));
  })();
</script>

<AdminShell title="Dashboard">
  <div class="flex flex-col gap-6 md:gap-8 max-w-6xl mx-auto">
    <!-- Welcome strip. Greeting uses the cached username from the auth store
         so it renders instantly on mount without waiting for any network
         round-trip. The "Kelola sebagai" pill on the right gives the operator
         a quick affordance to log out without scrolling to the sidebar. -->
    <section class="min-w-0 flex flex-col sm:flex-row sm:items-end sm:justify-between gap-4">
      <div class="min-w-0">
        <p class="text-xs font-medium uppercase tracking-widest text-arang-500">
          Panel Admin Desa Gerbosari
        </p>
        <h2
          class="mt-1 font-serif text-xl sm:text-2xl md:text-3xl font-semibold text-arang-900 break-words"
        >
          Selamat datang, {$authState.user?.username ?? 'Admin'}
        </h2>
        <p class="mt-1.5 text-sm text-arang-500">{todayLong}</p>
      </div>

      {#if $authState.user}
        <div
          class="hidden sm:inline-flex items-center gap-2 rounded-full border border-krem-200
                 bg-white px-3 py-1.5 text-xs text-arang-700 shrink-0"
        >
          <span class="text-arang-500">Kelola sebagai</span>
          <span class="font-medium text-arang-900 truncate max-w-[140px]">
            {$authState.user.username}
          </span>
        </div>
      {/if}
    </section>

    <!-- Section: KPI stat cards -->
    <section class="min-w-0">
      <h3 class="font-serif text-lg font-semibold text-arang-900 mt-0 mb-4">Statistik Cepat</h3>

      <div class="grid grid-cols-2 lg:grid-cols-4 gap-3">
        {#each cards as card}
          <a
            href={card.href}
            on:click|preventDefault={() => goTo(card.href)}
            class="group flex flex-col gap-3 rounded-lg border border-krem-200 bg-white p-5
                   hover:border-menoreh-500 hover:bg-krem-50
                   transition-colors duration-200 ease-out"
            aria-label="Kelola {card.title}"
          >
            <div class="flex items-start justify-between gap-3">
              <div
                class="w-10 h-10 rounded-md flex items-center justify-center {card.accent}"
                aria-hidden="true"
              >
                <svelte:component this={card.icon} class="w-5 h-5" strokeWidth={1.75} />
              </div>
              <ArrowRight
                class="w-4 h-4 text-arang-300 group-hover:text-menoreh-700 transition-colors duration-200 ease-out"
                strokeWidth={2}
                aria-hidden="true"
              />
            </div>

            <div class="flex flex-col gap-1">
              <span class="text-xs font-medium uppercase tracking-widest text-arang-500">
                {card.title}
              </span>
              <div class="flex items-baseline gap-1.5">
                <span
                  class="font-serif text-3xl md:text-4xl font-semibold text-arang-900 tabular-nums leading-none"
                >
                  {formatCount(counts[card.key])}
                </span>
                <span class="text-xs font-medium text-arang-500">{unitFor(card.key)}</span>
              </div>
            </div>

            <p class="hidden sm:block text-sm text-arang-700 leading-snug">{card.description}</p>

            <span
              class="mt-auto inline-flex items-center justify-end gap-1 text-sm font-medium text-menoreh-700
                     group-hover:text-menoreh-800 transition-colors duration-200 ease-out"
            >
              Kelola
              <ArrowRight class="w-3.5 h-3.5" strokeWidth={2} aria-hidden="true" />
            </span>
          </a>
        {/each}
      </div>
    </section>

    <!-- Section: Visualisasi Data. Three cards, stacked on mobile and laid out
         in a 3-column grid on lg+. Each card carries its own title/subtitle
         pair so the underlying chart components stay agnostic about copy. -->
    <section class="min-w-0">
      <h3 class="font-serif text-lg font-semibold text-arang-900 mt-0 mb-4">Visualisasi Data</h3>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <!-- Donut: demografi -->
        <article class="rounded-lg border border-krem-200 bg-white p-5 flex flex-col gap-4">
          <header>
            <h4 class="font-serif text-base font-semibold text-arang-900">Komposisi Penduduk</h4>
            <p class="text-xs text-arang-500 mt-0.5">Laki-laki vs Perempuan</p>
          </header>

          {#if chartsLoading}
            <div class="flex flex-col items-center gap-4 py-2">
              <div class="w-36 h-36 sm:w-56 sm:h-56 rounded-full bg-krem-100 animate-pulse"></div>
              <div class="w-full space-y-2">
                <div class="h-3 rounded bg-krem-100 animate-pulse"></div>
                <div class="h-3 rounded bg-krem-100 animate-pulse"></div>
              </div>
            </div>
          {:else if pendudukRingkasan}
            <DonutChart
              data={demografiData}
              centerValue={pendudukRingkasan.total_penduduk.toLocaleString('id-ID')}
              centerLabel="jiwa"
            />
          {:else}
            <div class="w-full flex items-center justify-center py-10 text-sm text-arang-500">
              Belum ada data
            </div>
          {/if}
        </article>

        <!-- Bar (horizontal): lowongan per kategori -->
        <article class="rounded-lg border border-krem-200 bg-white p-5 flex flex-col gap-4">
          <header>
            <h4 class="font-serif text-base font-semibold text-arang-900">Lowongan per Kategori</h4>
            <p class="text-xs text-arang-500 mt-0.5">Distribusi UMKM, formal, freelance</p>
          </header>

          {#if chartsLoading}
            <div class="flex flex-col gap-3">
              {#each Array(3) as _}
                <div class="space-y-1.5">
                  <div class="h-3 w-1/2 rounded bg-krem-100 animate-pulse"></div>
                  <div class="h-2.5 rounded-full bg-krem-100 animate-pulse"></div>
                </div>
              {/each}
            </div>
          {:else}
            <BarChart data={lowonganByKategori} orientation="horizontal" unit="lowongan" />
          {/if}
        </article>

        <!-- Bar (vertical): publikasi berita 6 bulan terakhir -->
        <article class="rounded-lg border border-krem-200 bg-white p-5 flex flex-col gap-4">
          <header>
            <h4 class="font-serif text-base font-semibold text-arang-900">Publikasi Berita</h4>
            <p class="text-xs text-arang-500 mt-0.5">6 bulan terakhir</p>
          </header>

          {#if chartsLoading}
            <div class="h-40 sm:h-48 flex items-end gap-2">
              {#each Array(6) as _, i}
                <div
                  class="flex-1 rounded-t bg-krem-100 animate-pulse"
                  style="height: {30 + ((i * 13) % 60)}%"
                ></div>
              {/each}
            </div>
          {:else}
            <BarChart data={beritaPerBulan} orientation="vertical" unit="berita" />
          {/if}
        </article>
      </div>
    </section>
  </div>
</AdminShell>

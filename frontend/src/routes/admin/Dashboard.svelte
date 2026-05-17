<script lang="ts">
  import { onMount } from 'svelte';
  import { navigate } from '../../lib/router';
  import { authState, requireAuth } from '../../lib/auth';
  import { listGaleri } from '../../lib/api/galeri';
  import { listLowongan } from '../../lib/api/lowongan';
  import { listBerita } from '../../lib/api/berita';
  import { getRingkasan } from '../../lib/api/penduduk';
  import AdminShell from '../../lib/admin/AdminShell.svelte';
  import {
    ArrowRight,
    ImageIcon,
    Briefcase,
    Newspaper,
    Users
  } from '../../lib/components/icons';

  interface SummaryCard {
    key: 'galeri' | 'lowongan' | 'berita' | 'penduduk';
    title: string;
    description: string;
    href: string;
    icon: typeof ImageIcon;
    accent: string; // background swatch class
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

  async function loadCounts(): Promise<void> {
    const results = await Promise.allSettled([
      listGaleri(),
      listLowongan(),
      listBerita(),
      getRingkasan()
    ]);

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
  }

  onMount(() => {
    if (!requireAuth()) return;
    loadCounts();
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
</script>

<AdminShell title="Dashboard">
  <div class="flex flex-col gap-6 md:gap-8 max-w-6xl">
    <!-- Greeting block. Uses the cached username from the auth store so it
         renders instantly on mount without waiting for any network round-trip.
         min-w-0 + break-words on the username prevents extra-long handles from
         pushing the layout out of the viewport on narrow phones. -->
    <section class="min-w-0">
      <p class="text-xs font-medium uppercase tracking-widest text-arang-500">Selamat datang</p>
      <h2 class="mt-1 font-serif text-xl sm:text-2xl md:text-3xl font-semibold text-arang-900 break-words">
        Halo, {$authState.user?.username ?? 'Admin'}
      </h2>
      <p class="mt-2 text-sm text-arang-700 max-w-2xl">
        Pilih modul di bawah untuk mengelola konten dinamis situs Desa Gerbosari. Statistik di
        setiap kartu diambil langsung dari API publik agar Anda selalu melihat angka terkini.
      </p>
    </section>

    <section>
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
        {#each cards as card}
          <a
            href={card.href}
            on:click|preventDefault={() => goTo(card.href)}
            class="group flex flex-col gap-3 sm:gap-4 rounded-lg border border-krem-200 bg-white p-5
                   min-h-24
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
                <span class="font-serif text-3xl font-semibold text-arang-900 tnum leading-none">
                  {formatCount(counts[card.key])}
                </span>
                <span class="text-xs font-medium text-arang-500">{unitFor(card.key)}</span>
              </div>
            </div>

            <p class="text-sm text-arang-700 leading-snug">{card.description}</p>

            <span
              class="mt-auto inline-flex items-center gap-1 text-sm font-medium text-menoreh-700
                     group-hover:text-menoreh-800 transition-colors duration-200 ease-out"
            >
              Kelola
              <ArrowRight class="w-3.5 h-3.5" strokeWidth={2} aria-hidden="true" />
            </span>
          </a>
        {/each}
      </div>
    </section>
  </div>
</AdminShell>

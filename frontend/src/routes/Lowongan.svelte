<script lang="ts">
  /**
   * Lowongan kerja board for Desa Gerbosari - UMKM, formal, freelance.
   * Client-side filter/search/sort over a single API call. No detail page;
   * cards are visually interactive but do not link out.
   */
  import { onMount, onDestroy } from 'svelte';
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import SectionShell from '../lib/components/ui/SectionShell.svelte';
  import Card from '../lib/components/ui/Card.svelte';
  import Chip from '../lib/components/ui/Chip.svelte';
  import Badge from '../lib/components/ui/Badge.svelte';
  import Skeleton from '../lib/components/ui/Skeleton.svelte';
  import EmptyState from '../lib/components/ui/EmptyState.svelte';
  import { listLowongan } from '../lib/api/lowongan';
  import { isApiError } from '../lib/api/client';
  import type { Lowongan } from '../lib/types';
  import {
    MapPin,
    Calendar,
    Mail,
    Phone,
    Search,
    AlertCircle,
    Briefcase
  } from '../lib/components/icons';

  let items: Lowongan[] = [];
  let loading = true;
  let errorMessage: string | null = null;
  let controller: AbortController | undefined;

  type Kategori = 'all' | 'umkm' | 'formal' | 'freelance';
  let kategori: Kategori = 'all';

  type SortKey = 'terbaru' | 'tenggat' | 'gaji';
  let sortKey: SortKey = 'terbaru';

  // Search input is debounced into `searchTerm` to avoid filter thrash.
  let searchInput = '';
  let searchTerm = '';
  let debounceHandle: ReturnType<typeof setTimeout> | undefined;

  const nf = new Intl.NumberFormat('id-ID', { maximumFractionDigits: 0 });
  const dateFmt = new Intl.DateTimeFormat('id-ID', { dateStyle: 'long' });

  async function load() {
    controller?.abort();
    controller = new AbortController();
    loading = true;
    errorMessage = null;
    try {
      items = await listLowongan({}, { signal: controller.signal });
    } catch (err) {
      if ((err as DOMException)?.name === 'AbortError') return;
      errorMessage = isApiError(err)
        ? err.detail ?? err.message
        : 'Gagal memuat lowongan.';
    } finally {
      loading = false;
    }
  }

  onMount(load);
  onDestroy(() => {
    controller?.abort();
    if (debounceHandle) clearTimeout(debounceHandle);
  });

  function onSearchInput(e: Event) {
    const value = (e.target as HTMLInputElement).value;
    searchInput = value;
    if (debounceHandle) clearTimeout(debounceHandle);
    debounceHandle = setTimeout(() => {
      searchTerm = value.trim().toLowerCase();
    }, 150);
  }

  // Reactive counts per kategori - drives chip badges.
  $: counts = items.reduce(
    (acc, it) => {
      acc.all += 1;
      const k = (it.kategori as Kategori) ?? 'umkm';
      if (k === 'umkm' || k === 'formal' || k === 'freelance') acc[k] += 1;
      return acc;
    },
    { all: 0, umkm: 0, formal: 0, freelance: 0 } as Record<Kategori, number>
  );

  $: filtered = (() => {
    let list = items.slice();
    if (kategori !== 'all') {
      list = list.filter((it) => it.kategori === kategori);
    }
    if (searchTerm) {
      list = list.filter((it) => {
        const hay = `${it.judul} ${it.instansi} ${it.deskripsi}`.toLowerCase();
        return hay.includes(searchTerm);
      });
    }
    if (sortKey === 'tenggat') {
      list.sort((a, b) => {
        const ad = a.deadline ? new Date(a.deadline).getTime() : Number.POSITIVE_INFINITY;
        const bd = b.deadline ? new Date(b.deadline).getTime() : Number.POSITIVE_INFINITY;
        return ad - bd;
      });
    } else if (sortKey === 'gaji') {
      list.sort((a, b) => {
        const av = a.gaji_max ?? a.gaji_min ?? -1;
        const bv = b.gaji_max ?? b.gaji_min ?? -1;
        return bv - av;
      });
    } else {
      list.sort(
        (a, b) =>
          new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
      );
    }
    return list;
  })();

  function formatGaji(min: number | null, max: number | null): string {
    if (!min && !max) return 'Akan didiskusikan';
    if (min && max) return `Rp ${nf.format(min)} - ${nf.format(max)}`;
    if (min) return `Mulai Rp ${nf.format(min)}`;
    return `Hingga Rp ${nf.format(max as number)}`;
  }

  function formatDeadline(d: string | null): string {
    if (!d) return 'Tanpa tenggat';
    try {
      return dateFmt.format(new Date(d));
    } catch {
      return d;
    }
  }

  function deadlineStatus(d: string | null): 'expired' | 'soon' | 'open' | 'none' {
    if (!d) return 'none';
    const target = new Date(d).getTime();
    if (Number.isNaN(target)) return 'none';
    const diff = target - Date.now();
    if (diff < 0) return 'expired';
    if (diff <= 7 * 24 * 60 * 60 * 1000) return 'soon';
    return 'open';
  }

  // Detect kontak channel - phone/WA digits vs email vs free-text fallback.
  function kontakKind(k: string): 'email' | 'phone' | 'text' {
    if (k.includes('@')) return 'email';
    if (/(\+?\d[\d\s-]{6,})/.test(k)) return 'phone';
    return 'text';
  }

  function kontakHref(k: string): string | undefined {
    const kind = kontakKind(k);
    if (kind === 'email') return `mailto:${k.trim()}`;
    if (kind === 'phone') {
      // Strip non-digits, preserve leading +. Bias to WhatsApp wa.me link for ID numbers.
      const digits = k.replace(/[^\d+]/g, '');
      const normalized = digits.startsWith('+')
        ? digits.slice(1)
        : digits.startsWith('0')
          ? `62${digits.slice(1)}`
          : digits;
      return `https://wa.me/${normalized}`;
    }
    return undefined;
  }

  function kategoriVariant(k: string): 'umkm' | 'formal' | 'freelance' | 'neutral' {
    if (k === 'umkm' || k === 'formal' || k === 'freelance') return k;
    return 'neutral';
  }
</script>

<PageHeader
  eyebrow="Peluang Karier"
  title="Lowongan Kerja"
  description="Kesempatan kerja di UMKM, BUMDes, dan kelompok usaha Desa Gerbosari."
/>

<!-- Controls strip: filters, search, sort on paper biome. -->
<SectionShell variant="default" padding="md">
  <div class="space-y-4">
    <div class="flex flex-wrap items-center gap-2">
      <Chip
        label="Semua"
        count={counts.all}
        active={kategori === 'all'}
        on:click={() => (kategori = 'all')}
      />
      <Chip
        label="UMKM"
        count={counts.umkm}
        active={kategori === 'umkm'}
        on:click={() => (kategori = 'umkm')}
      />
      <Chip
        label="Formal"
        count={counts.formal}
        active={kategori === 'formal'}
        on:click={() => (kategori = 'formal')}
      />
      <Chip
        label="Freelance"
        count={counts.freelance}
        active={kategori === 'freelance'}
        on:click={() => (kategori = 'freelance')}
      />
    </div>

    <div class="flex flex-col md:flex-row md:items-center gap-3">
      <label class="relative flex-1">
        <span class="sr-only">Cari lowongan</span>
        <Search
          class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-arang-400"
          strokeWidth={1.75}
          aria-hidden="true"
        />
        <input
          type="search"
          value={searchInput}
          on:input={onSearchInput}
          placeholder="Cari judul, instansi, atau kata kunci..."
          class="h-10 w-full rounded-md border border-krem-300 bg-white pl-9 pr-3 text-sm text-arang-900 placeholder:text-arang-400 focus:border-menoreh-500"
        />
      </label>
      <label class="md:w-56 flex items-center gap-2 text-sm text-arang-700">
        <span class="text-xs font-semibold uppercase tracking-widest text-arang-500">Urutkan</span>
        <select
          bind:value={sortKey}
          class="h-10 flex-1 rounded-md border border-krem-300 bg-white px-2 text-sm text-arang-900 focus:border-menoreh-500"
          aria-label="Urutkan lowongan"
        >
          <option value="terbaru">Terbaru</option>
          <option value="tenggat">Tenggat Terdekat</option>
          <option value="gaji">Gaji Tertinggi</option>
        </select>
      </label>
    </div>
  </div>
</SectionShell>

<!-- Cards on batik motif: evokes "papan pengumuman desa" feel. -->
<SectionShell variant="batik" padding="lg">
  {#if loading}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      {#each Array(4) as _, i (i)}
        <div class="rounded-lg border border-krem-300 bg-white p-6 space-y-4">
          <div class="flex items-start justify-between gap-3">
            <div class="space-y-2 flex-1">
              <Skeleton class="h-6 w-3/4" />
              <Skeleton class="h-3 w-1/3" />
            </div>
            <Skeleton class="h-5 w-16" />
          </div>
          <Skeleton class="h-4 w-full" />
          <Skeleton class="h-4 w-5/6" />
          <Skeleton class="h-4 w-2/3" />
        </div>
      {/each}
    </div>
  {:else if errorMessage}
    <EmptyState
      variant="error"
      title="Tidak dapat memuat lowongan"
      description={errorMessage}
      actionLabel="Coba lagi"
      onAction={load}
    />
  {:else if filtered.length === 0}
    <EmptyState
      title="Belum ada lowongan"
      description={searchTerm || kategori !== 'all'
        ? 'Tidak ada lowongan yang cocok dengan filter saat ini. Coba ubah kata kunci atau kategori.'
        : 'Belum ada lowongan terdaftar saat ini. Pantau kembali secara berkala.'}
    />
  {:else}
    <ul class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      {#each filtered as item (item.id)}
        {@const status = deadlineStatus(item.deadline)}
        {@const expired = status === 'expired'}
        {@const kontak = item.kontak?.trim()}
        {@const kind = kontak ? kontakKind(kontak) : 'text'}
        {@const href = kontak ? kontakHref(kontak) : undefined}
        <li class="h-full {expired ? 'opacity-60' : ''}">
          <Card as="article" interactive padding="md" class="h-full flex flex-col border-krem-300">
            <svelte:fragment slot="header">
              <div class="flex items-start justify-between gap-3">
                <div class="min-w-0">
                  <p class="text-xs font-semibold uppercase tracking-widest text-arang-500 truncate">
                    {item.instansi}
                  </p>
                  <h2 class="mt-1 font-serif text-xl font-semibold text-arang-900 leading-snug">
                    {item.judul}
                  </h2>
                </div>
                <Badge variant={kategoriVariant(item.kategori)} label={item.kategori} />
              </div>
            </svelte:fragment>

            <p class="text-sm text-arang-700 leading-relaxed line-clamp-3">
              {item.deskripsi}
            </p>

            <dl class="mt-4 space-y-2 text-sm text-arang-700">
              <div class="flex items-center gap-2">
                <MapPin class="h-4 w-4 text-arang-400 shrink-0" strokeWidth={1.75} aria-hidden="true" />
                <dt class="sr-only">Lokasi</dt>
                <dd class="truncate">{item.lokasi_pedukuhan ?? '-'}</dd>
              </div>
              <div class="flex items-center gap-2">
                <Calendar class="h-4 w-4 text-arang-400 shrink-0" strokeWidth={1.75} aria-hidden="true" />
                <dt class="sr-only">Tenggat</dt>
                <dd>Tenggat: {formatDeadline(item.deadline)}</dd>
              </div>
              <div class="flex items-center gap-2">
                <Briefcase class="h-4 w-4 text-arang-400 shrink-0" strokeWidth={1.75} aria-hidden="true" />
                <dt class="sr-only">Gaji</dt>
                <dd class="tnum">{formatGaji(item.gaji_min, item.gaji_max)}</dd>
              </div>
            </dl>

            <!-- Status pills -->
            {#if status === 'soon' || status === 'expired'}
              <div class="mt-4 flex flex-wrap gap-2">
                {#if status === 'expired'}
                  <span class="inline-flex items-center gap-1.5 rounded bg-arang-100 px-2.5 py-0.5 text-xs font-medium uppercase tracking-wider text-arang-700">
                    <AlertCircle class="h-3 w-3" strokeWidth={2} aria-hidden="true" />
                    Tenggat berakhir
                  </span>
                {:else}
                  <span class="inline-flex items-center gap-1.5 rounded bg-terakota-50 px-2.5 py-0.5 text-xs font-medium uppercase tracking-wider text-terakota-700">
                    <AlertCircle class="h-3 w-3" strokeWidth={2} aria-hidden="true" />
                    Segera berakhir
                  </span>
                {/if}
              </div>
            {/if}

            <svelte:fragment slot="footer">
              {#if kontak}
                <div class="flex items-center justify-between gap-3">
                  <span class="text-xs font-semibold uppercase tracking-widest text-arang-500">
                    Kontak
                  </span>
                  {#if href}
                    <a
                      href={href}
                      target={kind === 'phone' ? '_blank' : undefined}
                      rel={kind === 'phone' ? 'noopener noreferrer' : undefined}
                      class="inline-flex items-center gap-1.5 text-sm font-medium text-menoreh-700 hover:text-menoreh-800"
                    >
                      {#if kind === 'email'}
                        <Mail class="h-4 w-4" strokeWidth={1.75} aria-hidden="true" />
                      {:else}
                        <Phone class="h-4 w-4" strokeWidth={1.75} aria-hidden="true" />
                      {/if}
                      <span>{kontak}</span>
                    </a>
                  {:else}
                    <span class="text-sm text-arang-700">{kontak}</span>
                  {/if}
                </div>
              {:else}
                <span class="text-xs italic text-arang-500">Kontak belum dicantumkan.</span>
              {/if}
            </svelte:fragment>
          </Card>
        </li>
      {/each}
    </ul>
  {/if}
</SectionShell>

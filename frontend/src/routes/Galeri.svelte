<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import EmptyState from '../lib/components/ui/EmptyState.svelte';
  import Skeleton from '../lib/components/ui/Skeleton.svelte';
  import Badge from '../lib/components/ui/Badge.svelte';
  import Button from '../lib/components/ui/Button.svelte';
  import { listGaleri } from '../lib/api/galeri';
  import { isApiError } from '../lib/api/client';
  import type { Galeri } from '../lib/types';
  import { X, ArrowLeft, ArrowRight, Calendar, ImageIcon } from '../lib/components/icons';

  // ---------------------------------------------------------------------------
  // State
  // ---------------------------------------------------------------------------
  let items: Galeri[] = [];
  let loading = true;
  let errorMessage: string | null = null;
  let activeKategori: string | null = null; // null = "Semua"
  let lightboxIndex: number | null = null;
  let controller: AbortController | undefined;
  let lightboxCloseBtn: HTMLButtonElement | undefined;
  let lastTriggerEl: HTMLElement | null = null;

  // Canonical filter order — keep stable even if backend returns out of order
  const KATEGORI_ORDER = ['Wisata', 'Budaya', 'Agrowisata', 'Kegiatan'];

  // Indonesian long-date formatter, e.g. "12 Februari 2025"
  const dateFmt = new Intl.DateTimeFormat('id-ID', { dateStyle: 'long' });

  // ---------------------------------------------------------------------------
  // Derived
  // ---------------------------------------------------------------------------
  $: kategoriCounts = items.reduce<Record<string, number>>((acc, it) => {
    const k = normalize(it.kategori);
    acc[k] = (acc[k] ?? 0) + 1;
    return acc;
  }, {});

  $: kategoriList = (() => {
    const present = Object.keys(kategoriCounts);
    const ordered = KATEGORI_ORDER.filter((k) => present.includes(k));
    const extras = present.filter((k) => !KATEGORI_ORDER.includes(k)).sort();
    return [...ordered, ...extras];
  })();

  $: filtered = activeKategori === null ? items : items.filter((it) => normalize(it.kategori) === activeKategori);

  function normalize(s: string): string {
    if (!s) return s;
    return s.charAt(0).toUpperCase() + s.slice(1).toLowerCase();
  }

  function formatDate(raw: string | null | undefined): string {
    if (!raw) return '';
    const d = new Date(raw);
    if (Number.isNaN(d.getTime())) return '';
    return dateFmt.format(d);
  }

// ---------------------------------------------------------------------------
  // Data loading
  // ---------------------------------------------------------------------------
  async function load() {
    controller?.abort();
    controller = new AbortController();
    loading = true;
    errorMessage = null;
    try {
      const result = await listGaleri({}, { signal: controller.signal });
      items = Array.isArray(result) ? result : [];
    } catch (err) {
      if ((err as DOMException)?.name === 'AbortError') return;
      errorMessage = isApiError(err)
        ? err.detail ?? err.message
        : 'Gagal memuat galeri. Periksa koneksi Anda lalu coba lagi.';
      items = [];
    } finally {
      loading = false;
    }
  }

  onMount(load);

  onDestroy(() => {
    controller?.abort();
    if (typeof document !== 'undefined') document.body.style.overflow = '';
  });

  // ---------------------------------------------------------------------------
  // Lightbox
  // ---------------------------------------------------------------------------
  async function openLightbox(idx: number, trigger: HTMLElement) {
    lastTriggerEl = trigger;
    lightboxIndex = idx;
    if (typeof document !== 'undefined') document.body.style.overflow = 'hidden';
    await tick();
    lightboxCloseBtn?.focus();
  }

  function closeLightbox() {
    lightboxIndex = null;
    if (typeof document !== 'undefined') document.body.style.overflow = '';
    lastTriggerEl?.focus();
    lastTriggerEl = null;
  }

  function prev() {
    if (lightboxIndex === null || filtered.length === 0) return;
    lightboxIndex = (lightboxIndex - 1 + filtered.length) % filtered.length;
  }

  function next() {
    if (lightboxIndex === null || filtered.length === 0) return;
    lightboxIndex = (lightboxIndex + 1) % filtered.length;
  }

  function onKey(e: KeyboardEvent) {
    if (lightboxIndex === null) return;
    if (e.key === 'Escape') {
      e.preventDefault();
      closeLightbox();
    } else if (e.key === 'ArrowLeft') {
      e.preventDefault();
      prev();
    } else if (e.key === 'ArrowRight') {
      e.preventDefault();
      next();
    }
  }

  function setKategori(k: string | null) {
    activeKategori = k;
  }

  $: active = lightboxIndex !== null ? filtered[lightboxIndex] ?? null : null;
</script>

<svelte:window on:keydown={onKey} />

<PageHeader
  eyebrow="Dokumentasi Desa"
  title="Galeri Foto Kegiatan"
  description="Dokumentasi kegiatan, wisata, budaya, dan agrowisata Desa Gerbosari."
/>

<section class="container-page py-10 md:py-14">
  <!-- Filter row -->
  {#if !loading && !errorMessage && items.length > 0}
    <div class="-mx-6 md:mx-0 overflow-x-auto md:overflow-visible mb-8">
      <div class="flex items-center gap-2 px-6 md:px-0 min-w-max md:min-w-0 md:flex-wrap">
        <button
          type="button"
          on:click={() => setKategori(null)}
          aria-pressed={activeKategori === null}
          class="inline-flex items-center gap-2 h-9 px-3.5 rounded-full text-sm border transition-colors {activeKategori === null
            ? 'bg-arang-900 text-krem-50 border-arang-900'
            : 'bg-white text-arang-700 border-krem-200 hover:border-arang-300'}"
        >
          Semua
          <span class="text-[11px] opacity-70 tnum">· {items.length}</span>
        </button>
        {#each kategoriList as k (k)}
          {@const isActive = activeKategori === k}
          <button
            type="button"
            on:click={() => setKategori(k)}
            aria-pressed={isActive}
            class="inline-flex items-center gap-2 h-9 px-3.5 rounded-full text-sm border transition-colors {isActive
              ? 'bg-arang-900 text-krem-50 border-arang-900'
              : 'bg-white text-arang-700 border-krem-200 hover:border-arang-300'}"
          >
            {k}
            <span class="text-[11px] opacity-70 tnum">· {kategoriCounts[k]}</span>
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Content states -->
  {#if loading}
    <div class="columns-1 sm:columns-2 lg:columns-3 xl:columns-4 gap-4">
      {#each Array(8) as _, i (i)}
        <div class="mb-4 break-inside-avoid" style="height:{180 + ((i * 47) % 140)}px">
          <Skeleton class="w-full h-full rounded-lg" />
        </div>
      {/each}
    </div>
  {:else if errorMessage}
    <EmptyState
      variant="error"
      title="Tidak dapat memuat galeri"
      description={errorMessage}
    >
      <Button variant="secondary" size="sm" on:click={load}>Coba lagi</Button>
    </EmptyState>
  {:else if items.length === 0}
    <EmptyState
      variant="empty"
      title="Belum ada dokumentasi"
      description="Foto kegiatan, wisata, dan budaya akan tampil di sini setelah unggahan pertama."
    />
  {:else if filtered.length === 0}
    <EmptyState
      variant="empty"
      title="Tidak ada foto pada kategori ini"
      description="Pilih kategori lain atau kembali ke Semua untuk melihat seluruh dokumentasi."
    >
      <Button variant="secondary" size="sm" on:click={() => setKategori(null)}>Lihat semua</Button>
    </EmptyState>
  {:else}
    <div class="columns-1 sm:columns-2 lg:columns-3 xl:columns-4 gap-4">
      {#each filtered as item, i (item.id)}
        <button
          type="button"
          on:click={(e) => openLightbox(i, e.currentTarget)}
          class="group relative mb-4 block w-full break-inside-avoid overflow-hidden rounded-lg border border-krem-200 bg-krem-100 text-left focus:outline-none focus-visible:ring-2 focus-visible:ring-menoreh-500 focus-visible:ring-offset-2"
          aria-label={`Buka foto: ${item.judul}`}
        >
          <img
            src={item.file_path}
            alt={item.judul}
            loading="lazy"
            decoding="async"
            class="block w-full h-auto"
          />
          <!-- Hover gradient overlay with caption -->
          <div
            class="pointer-events-none absolute inset-x-0 bottom-0 p-4 bg-gradient-to-t from-arang-900/85 via-arang-900/40 to-transparent opacity-0 group-hover:opacity-100 group-focus-visible:opacity-100 transition-opacity duration-200"
          >
            <div class="flex items-center gap-2">
              <Badge variant={(item.kategori ?? '').toLowerCase()}>{normalize(item.kategori)}</Badge>
            </div>
            <div class="mt-2 font-serif text-base font-semibold text-krem-50 line-clamp-2 leading-snug">
              {item.judul}
            </div>
          </div>
        </button>
      {/each}
    </div>
  {/if}
</section>

<!-- Lightbox -->
{#if active}
  <div
    class="fixed inset-0 z-50 bg-arang-900/95 flex flex-col"
    role="dialog"
    aria-modal="true"
    aria-label={active.judul}
  >
    <!-- Top bar -->
    <div class="flex items-center justify-between px-4 md:px-6 py-3 text-krem-50">
      <div class="flex items-center gap-2 text-xs uppercase tracking-[0.14em] text-krem-100/70">
        <ImageIcon class="w-3.5 h-3.5" strokeWidth={1.75} />
        {(lightboxIndex ?? 0) + 1} / {filtered.length}
      </div>
      <button
        bind:this={lightboxCloseBtn}
        type="button"
        on:click={closeLightbox}
        aria-label="Tutup"
        class="inline-flex h-10 w-10 items-center justify-center rounded-full text-krem-50 hover:bg-krem-50/10 focus:outline-none focus-visible:ring-2 focus-visible:ring-krem-50"
      >
        <X class="w-5 h-5" strokeWidth={2} />
      </button>
    </div>

    <!-- Image area -->
    <div class="flex-1 flex items-center justify-center gap-3 md:gap-6 px-4 md:px-6 min-h-0">
      <button
        type="button"
        on:click={prev}
        aria-label="Foto sebelumnya"
        class="hidden md:inline-flex flex-none h-12 w-12 items-center justify-center rounded-full text-krem-50 hover:bg-krem-50/10 focus:outline-none focus-visible:ring-2 focus-visible:ring-krem-50"
      >
        <ArrowLeft class="w-5 h-5" strokeWidth={2} />
      </button>

      <img
        src={active.file_path}
        alt={active.judul}
        class="max-w-7xl max-h-[calc(100vh-12rem)] w-auto h-auto object-contain rounded"
      />

      <button
        type="button"
        on:click={next}
        aria-label="Foto berikutnya"
        class="hidden md:inline-flex flex-none h-12 w-12 items-center justify-center rounded-full text-krem-50 hover:bg-krem-50/10 focus:outline-none focus-visible:ring-2 focus-visible:ring-krem-50"
      >
        <ArrowRight class="w-5 h-5" strokeWidth={2} />
      </button>
    </div>

    <!-- Caption -->
    <div class="px-4 md:px-8 py-5 md:py-6 border-t border-krem-50/10 bg-arang-900/60 backdrop-blur-sm">
      <div class="max-w-4xl mx-auto">
        <div class="flex flex-wrap items-center gap-2 mb-2">
          <Badge variant={(active.kategori ?? '').toLowerCase()}>{normalize(active.kategori)}</Badge>
          {#if active.taken_at}
            <span class="inline-flex items-center gap-1.5 text-xs text-krem-100/70">
              <Calendar class="w-3.5 h-3.5" strokeWidth={1.75} />
              {formatDate(active.taken_at)}
            </span>
          {/if}
        </div>
        <h2 class="font-serif text-xl md:text-2xl font-semibold text-krem-50 leading-snug">
          {active.judul}
        </h2>
        {#if active.deskripsi}
          <p class="mt-2 text-sm text-krem-100/80 leading-relaxed max-w-2xl">{active.deskripsi}</p>
        {/if}
      </div>

      <!-- Mobile prev/next under caption -->
      <div class="md:hidden mt-4 flex items-center justify-between max-w-4xl mx-auto">
        <button
          type="button"
          on:click={prev}
          aria-label="Foto sebelumnya"
          class="inline-flex h-10 w-10 items-center justify-center rounded-full text-krem-50 bg-krem-50/10 hover:bg-krem-50/20"
        >
          <ArrowLeft class="w-5 h-5" strokeWidth={2} />
        </button>
        <button
          type="button"
          on:click={next}
          aria-label="Foto berikutnya"
          class="inline-flex h-10 w-10 items-center justify-center rounded-full text-krem-50 bg-krem-50/10 hover:bg-krem-50/20"
        >
          <ArrowRight class="w-5 h-5" strokeWidth={2} />
        </button>
      </div>
    </div>
  </div>
{/if}

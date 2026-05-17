<script lang="ts">
  /**
   * Long-form berita / agenda detail page. Renders markdown konten via
   * `marked` inside the shared Prose wrapper. Konten is admin-authored from
   * our own DB, so for v1 we render with {@html} without DOMPurify; this
   * trust boundary should be revisited if user-submitted content ever lands.
   */
  import { onDestroy } from 'svelte';
  import { marked } from 'marked';
  import Badge from '../lib/components/ui/Badge.svelte';
  import Button from '../lib/components/ui/Button.svelte';
  import Card from '../lib/components/ui/Card.svelte';
  import Prose from '../lib/components/ui/Prose.svelte';
  import Skeleton from '../lib/components/ui/Skeleton.svelte';
  import EmptyState from '../lib/components/ui/EmptyState.svelte';
  import { getBerita } from '../lib/api/berita';
  import { isApiError, type RequestOptions } from '../lib/api/client';
  import type { ApiError, Berita } from '../lib/types';
  import {
    Calendar,
    Clock,
    MapPin,
    ArrowLeft,
    ChevronRight,
    User,
    Newspaper
  } from '../lib/components/icons';

  // Hash router params prop (matched in lib/router).
  export let params: { slug?: string } = {};

  let item: Berita | null = null;
  let loading = true;
  let errorMessage: string | null = null;
  let notFound = false;
  let controller: AbortController | undefined;

  const longDate = new Intl.DateTimeFormat('id-ID', { dateStyle: 'long' });
  const fullDateTime = new Intl.DateTimeFormat('id-ID', {
    dateStyle: 'full',
    timeStyle: 'short'
  });

  // GFM on, single newlines kept as plain breaks (no <br>) per spec.
  marked.setOptions({ gfm: true, breaks: false });

  $: htmlContent = item ? (marked.parse(item.konten, { async: false }) as string) : '';

  async function load(slug: string) {
    controller?.abort();
    controller = new AbortController();
    loading = true;
    errorMessage = null;
    notFound = false;
    item = null;
    try {
      const opts: RequestOptions = { signal: controller.signal };
      item = await getBerita(slug, opts);
    } catch (err) {
      if ((err as DOMException)?.name === 'AbortError') return;
      if (isApiError(err) && (err as ApiError).status === 404) {
        notFound = true;
      } else {
        errorMessage = isApiError(err)
          ? err.detail ?? err.message
          : 'Gagal memuat artikel.';
      }
    } finally {
      loading = false;
    }
  }

  // Reactively refetch when slug changes (e.g., user navigates between articles).
  $: if (params?.slug) {
    load(params.slug);
  }

  onDestroy(() => controller?.abort());

  function formatDate(d: string): string {
    try {
      return longDate.format(new Date(d));
    } catch {
      return d;
    }
  }

  function formatDateTime(d: string): string {
    try {
      return fullDateTime.format(new Date(d));
    } catch {
      return d;
    }
  }

  // Heuristic: tanggal may carry a meaningful time component. If the
  // ISO string contains a time slot beyond midnight, surface it via Clock.
  function hasTimeComponent(iso: string): boolean {
    const m = iso.match(/T(\d{2}):(\d{2})/);
    if (!m) return false;
    return !(m[1] === '00' && m[2] === '00');
  }

  function reload() {
    if (params?.slug) load(params.slug);
  }
</script>

<section class="container-page py-8 md:py-12">
  <!-- Breadcrumb -->
  <nav aria-label="Breadcrumb" class="mb-6 text-xs text-arang-500">
    <ol class="flex flex-wrap items-center gap-1.5">
      <li><a href="#/" class="hover:text-menoreh-700">Beranda</a></li>
      <li aria-hidden="true"><ChevronRight class="h-3 w-3" strokeWidth={2} /></li>
      <li><a href="#/berita" class="hover:text-menoreh-700">Berita & Agenda</a></li>
      <li aria-hidden="true"><ChevronRight class="h-3 w-3" strokeWidth={2} /></li>
      <li class="max-w-[20ch] truncate text-arang-700" aria-current="page">
        {item?.judul ?? (loading ? 'Memuat...' : 'Artikel')}
      </li>
    </ol>
  </nav>

  {#if loading}
    <div class="space-y-6">
      <Skeleton class="h-5 w-24" />
      <Skeleton class="h-12 w-full" />
      <Skeleton class="h-12 w-3/4" />
      <Skeleton class="h-5 w-1/2" />
      <Skeleton class="aspect-video w-full" />
      <div class="space-y-3 max-w-prose">
        <Skeleton class="h-4 w-full" />
        <Skeleton class="h-4 w-11/12" />
        <Skeleton class="h-4 w-10/12" />
        <Skeleton class="h-4 w-full" />
        <Skeleton class="h-4 w-9/12" />
      </div>
    </div>
  {:else if notFound}
    <div class="pt-6">
      <EmptyState
        title="Berita tidak ditemukan"
        description="Tautan yang Anda buka mungkin sudah berubah atau dihapus."
      >
        <Button href="#/berita" variant="primary">
          <ArrowLeft class="h-4 w-4" strokeWidth={2} aria-hidden="true" />
          Kembali ke Berita
        </Button>
      </EmptyState>
    </div>
  {:else if errorMessage}
    <div class="pt-6">
      <EmptyState
        variant="error"
        title="Artikel tidak dapat dimuat"
        description={errorMessage}
        actionLabel="Coba lagi"
        onAction={reload}
      />
    </div>
  {:else if item}
    <article>
      <!-- Eyebrow & title -->
      <p class="flex items-center gap-2">
        <Badge variant={item.kategori === 'agenda' ? 'agenda' : 'berita'} label={item.kategori} />
      </p>
      <h1 class="mt-3 font-serif text-4xl md:text-5xl font-semibold text-arang-900 leading-tight text-balance">
        {item.judul}
      </h1>

      <!-- Meta strip -->
      <dl class="mt-4 flex flex-wrap items-center gap-x-5 gap-y-2 text-sm text-arang-700">
        {#if item.author}
          <div class="inline-flex items-center gap-1.5">
            <User class="h-4 w-4 text-arang-400" strokeWidth={1.75} aria-hidden="true" />
            <dt class="sr-only">Penulis</dt>
            <dd>{item.author}</dd>
          </div>
        {/if}
        <div class="inline-flex items-center gap-1.5">
          <Calendar class="h-4 w-4 text-arang-400" strokeWidth={1.75} aria-hidden="true" />
          <dt class="sr-only">Diterbitkan</dt>
          <dd>{formatDate(item.published_at)}</dd>
        </div>
      </dl>

      <!-- Hero image -->
      <figure class="mt-8 overflow-hidden rounded-lg border border-krem-200 bg-krem-100">
        {#if item.gambar_url}
          <img
            src={item.gambar_url}
            alt=""
            class="aspect-video w-full object-cover"
            loading="eager"
          />
        {:else}
          <div class="flex aspect-video w-full items-center justify-center text-arang-400">
            <Newspaper class="h-12 w-12" strokeWidth={1.25} aria-hidden="true" />
          </div>
        {/if}
      </figure>

      <!-- Agenda info card -->
      {#if item.kategori === 'agenda' && item.tanggal}
        <div class="mt-8">
          <Card padding="md" class="border-menoreh-200 bg-menoreh-50/60">
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
              <div class="flex items-start gap-3">
                <Calendar class="h-5 w-5 mt-0.5 text-menoreh-700" strokeWidth={1.75} aria-hidden="true" />
                <div>
                  <p class="text-xs font-semibold uppercase tracking-widest text-menoreh-800">Tanggal</p>
                  <p class="mt-0.5 text-sm font-medium text-arang-900">
                    {formatDate(item.tanggal)}
                  </p>
                </div>
              </div>
              {#if hasTimeComponent(item.tanggal)}
                <div class="flex items-start gap-3">
                  <Clock class="h-5 w-5 mt-0.5 text-menoreh-700" strokeWidth={1.75} aria-hidden="true" />
                  <div>
                    <p class="text-xs font-semibold uppercase tracking-widest text-menoreh-800">Waktu</p>
                    <p class="mt-0.5 text-sm font-medium text-arang-900">
                      {formatDateTime(item.tanggal)}
                    </p>
                  </div>
                </div>
              {/if}
              <div class="flex items-start gap-3">
                <MapPin class="h-5 w-5 mt-0.5 text-menoreh-700" strokeWidth={1.75} aria-hidden="true" />
                <div>
                  <p class="text-xs font-semibold uppercase tracking-widest text-menoreh-800">Lokasi</p>
                  <p class="mt-0.5 text-sm font-medium text-arang-900">
                    Lihat detail di dalam artikel
                  </p>
                </div>
              </div>
            </div>
          </Card>
        </div>
      {/if}

      <!-- Ringkasan lead -->
      {#if item.ringkasan}
        <p class="mt-8 max-w-prose text-lg leading-relaxed text-arang-700 font-serif italic">
          {item.ringkasan}
        </p>
      {/if}

      <!-- Konten markdown (trusted admin source - see file header note) -->
      <div class="mt-6">
        <Prose>
          {@html htmlContent}
        </Prose>
      </div>

      <!-- Footer -->
      <footer class="mt-12 border-t border-krem-200 pt-6">
        <a
          href="#/berita"
          class="inline-flex items-center gap-1.5 text-sm font-medium text-menoreh-700 hover:text-menoreh-800"
        >
          <ArrowLeft class="h-4 w-4" strokeWidth={2} aria-hidden="true" />
          Kembali ke Berita & Agenda
        </a>
      </footer>
    </article>
  {/if}
</section>


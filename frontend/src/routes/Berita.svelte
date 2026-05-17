<script lang="ts">
  /**
   * Civic news + agenda feed. Featured (most-recent) entry rendered as a
   * 2-column hero, remainder in a 1/2/3-column grid. Hash router links.
   */
  import { onMount, onDestroy } from 'svelte';
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import Tabs from '../lib/components/ui/Tabs.svelte';
  import Card from '../lib/components/ui/Card.svelte';
  import Badge from '../lib/components/ui/Badge.svelte';
  import Button from '../lib/components/ui/Button.svelte';
  import Skeleton from '../lib/components/ui/Skeleton.svelte';
  import EmptyState from '../lib/components/ui/EmptyState.svelte';
  import { listBerita } from '../lib/api/berita';
  import { isApiError } from '../lib/api/client';
  import type { Berita } from '../lib/types';
  import { Calendar, ArrowRight, Newspaper } from '../lib/components/icons';

  let items: Berita[] = [];
  let loading = true;
  let errorMessage: string | null = null;
  let controller: AbortController | undefined;

  let activeTab: 'semua' | 'berita' | 'agenda' = 'semua';

  const dateFmt = new Intl.DateTimeFormat('id-ID', { dateStyle: 'long' });

  const tabs = [
    { id: 'semua', label: 'Semua' },
    { id: 'berita', label: 'Berita' },
    { id: 'agenda', label: 'Agenda' }
  ];

  async function load() {
    controller?.abort();
    controller = new AbortController();
    loading = true;
    errorMessage = null;
    try {
      items = await listBerita({}, { signal: controller.signal });
    } catch (err) {
      if ((err as DOMException)?.name === 'AbortError') return;
      errorMessage = isApiError(err)
        ? err.detail ?? err.message
        : 'Gagal memuat berita.';
    } finally {
      loading = false;
    }
  }

  onMount(load);
  onDestroy(() => controller?.abort());

  function relevantDate(it: Berita): string {
    return it.tanggal ?? it.published_at;
  }

  // Sorts by relevant date desc. Agenda items use tanggal when present
  // so upcoming events surface above older publish-dated rows.
  $: filteredItems = (() => {
    const base =
      activeTab === 'semua'
        ? items
        : items.filter((it) => it.kategori === activeTab);
    return [...base].sort(
      (a, b) =>
        new Date(relevantDate(b)).getTime() - new Date(relevantDate(a)).getTime()
    );
  })();

  $: featured = filteredItems[0];
  $: rest = filteredItems.slice(1);

  function formatDate(d: string): string {
    try {
      return dateFmt.format(new Date(d));
    } catch {
      return d;
    }
  }

  function agendaStatus(it: Berita): 'upcoming' | 'past' | null {
    if (it.kategori !== 'agenda' || !it.tanggal) return null;
    const t = new Date(it.tanggal).getTime();
    if (Number.isNaN(t)) return null;
    // Treat same-day events as upcoming until the day rolls over.
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    return t >= today.getTime() ? 'upcoming' : 'past';
  }

  function categoryVariant(k: string): 'berita' | 'agenda' | 'neutral' {
    if (k === 'berita' || k === 'agenda') return k;
    return 'neutral';
  }

  function onTabChange(e: CustomEvent<{ value: string }>) {
    const v = e.detail.value;
    if (v === 'semua' || v === 'berita' || v === 'agenda') {
      activeTab = v;
    }
  }
</script>

<PageHeader
  eyebrow="Informasi Desa"
  title="Berita & Agenda"
  description="Kabar terbaru dan agenda kegiatan Desa Gerbosari."
/>

<section class="container-page py-10 md:py-14 space-y-10">
  <Tabs
    tabs={tabs}
    value={activeTab}
    ariaLabel="Filter kategori berita"
    on:change={onTabChange}
  />

  {#if loading}
    <!-- Featured skeleton -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 md:gap-10 items-center">
      <Skeleton class="aspect-[16/10] w-full" />
      <div class="space-y-4">
        <Skeleton class="h-5 w-24" />
        <Skeleton class="h-10 w-full" />
        <Skeleton class="h-10 w-3/4" />
        <Skeleton class="h-4 w-full" />
        <Skeleton class="h-4 w-5/6" />
        <Skeleton class="h-10 w-40" />
      </div>
    </div>
    <!-- List skeleton -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each Array(6) as _, i (i)}
        <div class="space-y-3 rounded-lg border border-krem-200 bg-white p-4">
          <Skeleton class="aspect-[16/9] w-full" />
          <Skeleton class="h-5 w-3/4" />
          <Skeleton class="h-4 w-full" />
          <Skeleton class="h-4 w-1/2" />
        </div>
      {/each}
    </div>
  {:else if errorMessage}
    <EmptyState
      variant="error"
      title="Tidak dapat memuat berita"
      description={errorMessage}
      actionLabel="Coba lagi"
      onAction={load}
    />
  {:else if filteredItems.length === 0}
    <EmptyState
      title="Belum ada publikasi"
      description={activeTab === 'agenda'
        ? 'Belum ada agenda yang dijadwalkan saat ini.'
        : activeTab === 'berita'
          ? 'Belum ada berita yang dipublikasikan saat ini.'
          : 'Belum ada berita maupun agenda yang dipublikasikan.'}
    />
  {:else}
    <!-- Featured -->
    {#if featured}
      {@const fStatus = agendaStatus(featured)}
      <article class="grid grid-cols-1 md:grid-cols-2 gap-6 md:gap-10 items-center">
        <a
          href={`#/berita/${featured.slug}`}
          class="block overflow-hidden rounded-lg border border-krem-200 bg-krem-100 aspect-[16/10]"
          aria-label={featured.judul}
        >
          {#if featured.gambar_url}
            <img
              src={featured.gambar_url}
              alt=""
              loading="eager"
              class="h-full w-full object-cover transition-[filter] duration-300 hover:brightness-105"
            />
          {:else}
            <div class="flex h-full w-full items-center justify-center text-arang-400">
              <Newspaper class="h-10 w-10" strokeWidth={1.25} aria-hidden="true" />
            </div>
          {/if}
        </a>
        <div>
          <div class="flex flex-wrap items-center gap-2">
            <Badge variant={categoryVariant(featured.kategori)} label={featured.kategori} />
            {#if fStatus === 'upcoming'}
              <Badge variant="aktif" label="Akan datang" />
            {:else if fStatus === 'past'}
              <Badge variant="arsip" label="Selesai" />
            {/if}
          </div>
          <h2 class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight text-balance">
            <a href={`#/berita/${featured.slug}`} class="hover:text-menoreh-800">
              {featured.judul}
            </a>
          </h2>
          <p class="mt-4 text-base text-arang-700 leading-relaxed">
            {featured.ringkasan}
          </p>
          <dl class="mt-4 flex flex-wrap items-center gap-x-4 gap-y-2 text-sm text-arang-700">
            <div class="inline-flex items-center gap-1.5">
              <Calendar class="h-4 w-4 text-arang-400" strokeWidth={1.75} aria-hidden="true" />
              <dt class="sr-only">Tanggal</dt>
              <dd>{formatDate(relevantDate(featured))}</dd>
            </div>
            {#if featured.author}
              <div class="inline-flex items-center gap-1.5">
                <dt class="sr-only">Penulis</dt>
                <dd class="text-arang-500">oleh {featured.author}</dd>
              </div>
            {/if}
          </dl>
          <div class="mt-6">
            <Button href={`#/berita/${featured.slug}`} variant="primary">
              Baca selengkapnya
              <ArrowRight class="h-4 w-4" strokeWidth={2} aria-hidden="true" />
            </Button>
          </div>
        </div>
      </article>
    {/if}

    <!-- Remainder grid -->
    {#if rest.length > 0}
      <ul class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each rest as item (item.id)}
          {@const aStatus = agendaStatus(item)}
          <li class="h-full">
            <a
              href={`#/berita/${item.slug}`}
              class="block h-full"
              aria-label={item.judul}
            >
              <Card as="article" interactive padding="sm" class="h-full overflow-hidden p-0">
                <div class="relative aspect-[16/9] overflow-hidden bg-krem-100">
                  {#if item.gambar_url}
                    <img
                      src={item.gambar_url}
                      alt=""
                      loading="lazy"
                      class="h-full w-full object-cover"
                    />
                  {:else}
                    <div class="flex h-full w-full items-center justify-center text-arang-400">
                      <Newspaper class="h-8 w-8" strokeWidth={1.25} aria-hidden="true" />
                    </div>
                  {/if}
                  <span class="absolute left-3 top-3">
                    <Badge variant={categoryVariant(item.kategori)} label={item.kategori} />
                  </span>
                </div>
                <div class="p-5">
                  {#if aStatus}
                    <div class="mb-2">
                      <Badge
                        variant={aStatus === 'upcoming' ? 'aktif' : 'arsip'}
                        label={aStatus === 'upcoming' ? 'Akan datang' : 'Selesai'}
                      />
                    </div>
                  {/if}
                  <h3 class="font-serif text-lg font-semibold text-arang-900 leading-snug line-clamp-2">
                    {item.judul}
                  </h3>
                  <p class="mt-2 text-sm text-arang-700 leading-relaxed line-clamp-2">
                    {item.ringkasan}
                  </p>
                  <dl class="mt-3 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-arang-500">
                    <div class="inline-flex items-center gap-1">
                      <Calendar class="h-3.5 w-3.5" strokeWidth={1.75} aria-hidden="true" />
                      <dt class="sr-only">Tanggal</dt>
                      <dd>{formatDate(relevantDate(item))}</dd>
                    </div>
                  </dl>
                </div>
              </Card>
            </a>
          </li>
        {/each}
      </ul>
    {/if}
  {/if}
</section>

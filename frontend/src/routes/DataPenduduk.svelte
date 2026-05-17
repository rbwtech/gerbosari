<script lang="ts">
  /**
   * Civic demographic dashboard for Desa Gerbosari.
   * Renders aggregate summary stats, a horizontal ratio bar, and a
   * sortable table of per-pedukuhan figures (with mobile card fallback).
   */
  import { onMount, onDestroy } from 'svelte';
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import SectionShell from '../lib/components/ui/SectionShell.svelte';
  import Stat from '../lib/components/ui/Stat.svelte';
  import Skeleton from '../lib/components/ui/Skeleton.svelte';
  import EmptyState from '../lib/components/ui/EmptyState.svelte';
  import { getRingkasan } from '../lib/api/penduduk';
  import { isApiError } from '../lib/api/client';
  import type { PendudukRingkasan, PendudukPedukuhan } from '../lib/types';
  import { Users, ChevronDown, ChevronUp } from '../lib/components/icons';

  let ringkasan: PendudukRingkasan | null = null;
  let loading = true;
  let errorMessage: string | null = null;
  let controller: AbortController | undefined;

  type SortKey = keyof PendudukPedukuhan;
  let sortKey: SortKey = 'total_penduduk';
  let sortDir: 'asc' | 'desc' = 'desc';

  const nf = new Intl.NumberFormat('id-ID');
  const pf = new Intl.NumberFormat('id-ID', { minimumFractionDigits: 1, maximumFractionDigits: 1 });

  async function load() {
    controller?.abort();
    controller = new AbortController();
    loading = true;
    errorMessage = null;
    try {
      ringkasan = await getRingkasan({ signal: controller.signal });
    } catch (err) {
      if ((err as DOMException)?.name === 'AbortError') return;
      errorMessage = isApiError(err)
        ? err.detail ?? err.message
        : 'Gagal memuat data penduduk.';
    } finally {
      loading = false;
    }
  }

  onMount(load);
  onDestroy(() => controller?.abort());

  function toggleSort(key: SortKey) {
    if (sortKey === key) {
      sortDir = sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      sortKey = key;
      // Numeric columns default to desc (largest first) for civic usability.
      sortDir = key === 'pedukuhan' ? 'asc' : 'desc';
    }
  }

  $: sortedPedukuhan = (() => {
    if (!ringkasan?.per_pedukuhan?.length) return [] as PendudukPedukuhan[];
    const arr = [...ringkasan.per_pedukuhan];
    arr.sort((a, b) => {
      const av = a[sortKey];
      const bv = b[sortKey];
      if (typeof av === 'number' && typeof bv === 'number') {
        return sortDir === 'asc' ? av - bv : bv - av;
      }
      return sortDir === 'asc'
        ? String(av).localeCompare(String(bv), 'id')
        : String(bv).localeCompare(String(av), 'id');
    });
    return arr;
  })();

  // Derived totals used by both summary cards and the table footer.
  $: totalPenduduk = ringkasan?.total_penduduk ?? 0;
  $: totalLaki = ringkasan?.total_laki ?? 0;
  $: totalPerempuan = ringkasan?.total_perempuan ?? 0;
  $: totalKK = ringkasan?.total_kk ?? 0;

  $: pctLaki = totalPenduduk > 0 ? (totalLaki / totalPenduduk) * 100 : 0;
  $: pctPerempuan = totalPenduduk > 0 ? (totalPerempuan / totalPenduduk) * 100 : 0;

  interface Col {
    key: SortKey;
    label: string;
    align: 'left' | 'right';
    short?: string;
  }
  const cols: Col[] = [
    { key: 'pedukuhan', label: 'Pedukuhan', align: 'left' },
    { key: 'total_kk', label: 'KK', align: 'right' },
    { key: 'total_laki', label: 'Laki', align: 'right' },
    { key: 'total_perempuan', label: 'Perempuan', align: 'right' },
    { key: 'total_penduduk', label: 'Total', align: 'right' }
  ];

  function formatTanggal(s: string): string {
    try {
      return new Date(s).toLocaleDateString('id-ID', { dateStyle: 'long' });
    } catch {
      return s;
    }
  }
</script>

<PageHeader
  eyebrow="Demografi"
  title="Data Penduduk Desa Gerbosari"
  description="Ringkasan demografis 19 pedukuhan."
/>

<!-- Summary stats on hijau mist — signals civic / landscape framing. -->
<SectionShell variant="mist" padding="md">
  <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
    {#if loading}
      {#each Array(4) as _, i (i)}
        <div class="space-y-3">
          <Skeleton class="h-5 w-5" />
          <Skeleton class="h-10 w-32" />
          <Skeleton class="h-3 w-24" />
        </div>
      {/each}
    {:else if ringkasan}
      <div>
        <Stat
          icon={Users}
          value={nf.format(totalPenduduk)}
          unit="jiwa"
          label="Total Penduduk"
        />
      </div>
      <div>
        <Stat value={nf.format(totalLaki)} unit="jiwa" label="Laki-laki" />
        {#if totalPenduduk > 0}
          <p class="mt-1 text-xs text-arang-500 tnum">{pf.format(pctLaki)}% dari total</p>
        {/if}
      </div>
      <div>
        <Stat value={nf.format(totalPerempuan)} unit="jiwa" label="Perempuan" />
        {#if totalPenduduk > 0}
          <p class="mt-1 text-xs text-arang-500 tnum">{pf.format(pctPerempuan)}% dari total</p>
        {/if}
      </div>
      <div>
        <Stat value={nf.format(totalKK)} unit="KK" label="Kepala Keluarga" />
        {#if totalPenduduk > 0 && totalKK > 0}
          <p class="mt-1 text-xs text-arang-500 tnum">
            Rata-rata {pf.format(totalPenduduk / totalKK)} jiwa per KK
          </p>
        {/if}
      </div>
    {/if}
  </div>
</SectionShell>

<!-- Distribution bar on plain paper. -->
{#if !loading && ringkasan && totalPenduduk > 0}
  <SectionShell variant="default" padding="md">
    <div class="space-y-3">
      <div class="flex items-baseline justify-between">
        <h2 class="font-serif text-xl font-semibold text-arang-900">Distribusi Penduduk</h2>
        <span class="text-xs uppercase tracking-widest text-arang-500">Laki-laki vs Perempuan</span>
      </div>
      <div
        class="flex h-3 w-full overflow-hidden rounded-full border border-krem-200 bg-krem-100"
        role="img"
        aria-label="Proporsi laki-laki {pf.format(pctLaki)} persen, perempuan {pf.format(pctPerempuan)} persen"
      >
        <div
          class="h-full bg-menoreh-600"
          style="width: {pctLaki}%"
          title="Laki-laki: {nf.format(totalLaki)} jiwa ({pf.format(pctLaki)}%)"
        ></div>
        <div
          class="h-full bg-terakota-600"
          style="width: {pctPerempuan}%"
          title="Perempuan: {nf.format(totalPerempuan)} jiwa ({pf.format(pctPerempuan)}%)"
        ></div>
      </div>
      <div class="flex flex-wrap gap-x-6 gap-y-1 text-sm text-arang-700">
        <span class="inline-flex items-center gap-2">
          <span class="inline-block h-2.5 w-2.5 rounded-full bg-menoreh-600" aria-hidden="true"></span>
          Laki-laki <span class="tnum text-arang-500">{pf.format(pctLaki)}%</span>
        </span>
        <span class="inline-flex items-center gap-2">
          <span class="inline-block h-2.5 w-2.5 rounded-full bg-terakota-600" aria-hidden="true"></span>
          Perempuan <span class="tnum text-arang-500">{pf.format(pctPerempuan)}%</span>
        </span>
      </div>
    </div>
  </SectionShell>
{/if}

<!-- Per-pedukuhan table on batik motif — reads like printed civic document. -->
<SectionShell variant="batik" padding="lg">
  <div class="space-y-4">
    <div class="flex items-baseline justify-between">
      <h2 class="font-serif text-xl font-semibold text-arang-900">Sebaran per Pedukuhan</h2>
      {#if !loading && ringkasan}
        <span class="text-xs uppercase tracking-widest text-arang-500 tnum">
          {nf.format(ringkasan.per_pedukuhan.length)} pedukuhan
        </span>
      {/if}
    </div>

    {#if loading}
      <!-- Loading: table skeleton -->
      <div class="overflow-hidden rounded-lg border border-krem-200">
        {#each Array(6) as _, i (i)}
          <div class="flex items-center gap-4 border-b border-krem-200 p-4 last:border-b-0">
            <Skeleton class="h-4 w-32" />
            <div class="ml-auto flex gap-6">
              <Skeleton class="h-4 w-12" />
              <Skeleton class="h-4 w-12" />
              <Skeleton class="h-4 w-12" />
              <Skeleton class="h-4 w-16" />
            </div>
          </div>
        {/each}
      </div>
    {:else if errorMessage}
      <EmptyState
        variant="error"
        title="Tidak dapat memuat data"
        description={errorMessage}
        actionLabel="Coba lagi"
        onAction={load}
      />
    {:else if !ringkasan || sortedPedukuhan.length === 0}
      <EmptyState
        title="Belum ada data"
        description="Data per pedukuhan belum tersedia."
      />
    {:else}
      <!-- Desktop / tablet: sortable semantic table -->
      <div class="hidden md:block overflow-x-auto rounded-lg border border-krem-200 bg-white">
        <table class="w-full text-sm">
          <caption class="sr-only">
            Tabel jumlah KK, laki-laki, perempuan, dan total penduduk per pedukuhan di Desa Gerbosari.
          </caption>
          <thead class="sticky top-0 bg-krem-50 text-arang-700">
            <tr>
              {#each cols as col (col.key)}
                <th
                  scope="col"
                  class="border-b border-krem-200 {col.align === 'right' ? 'text-right' : 'text-left'}"
                  aria-sort={sortKey === col.key
                    ? sortDir === 'asc'
                      ? 'ascending'
                      : 'descending'
                    : 'none'}
                >
                  <button
                    type="button"
                    class="inline-flex w-full items-center gap-1.5 px-4 py-3 text-xs font-semibold uppercase tracking-widest {col.align === 'right' ? 'justify-end' : 'justify-start'} hover:text-menoreh-700"
                    on:click={() => toggleSort(col.key)}
                  >
                    <span>{col.label}</span>
                    {#if sortKey === col.key}
                      {#if sortDir === 'asc'}
                        <ChevronUp class="h-3.5 w-3.5" strokeWidth={2} aria-hidden="true" />
                      {:else}
                        <ChevronDown class="h-3.5 w-3.5" strokeWidth={2} aria-hidden="true" />
                      {/if}
                    {/if}
                  </button>
                </th>
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each sortedPedukuhan as row (row.pedukuhan)}
              <tr class="border-b border-krem-200 last:border-b-0 even:bg-krem-50 hover:bg-menoreh-50/40">
                <td class="px-4 py-3 font-medium text-arang-900">{row.pedukuhan}</td>
                <td class="px-4 py-3 text-right tnum text-arang-700">{nf.format(row.total_kk)}</td>
                <td class="px-4 py-3 text-right tnum text-arang-700">{nf.format(row.total_laki)}</td>
                <td class="px-4 py-3 text-right tnum text-arang-700">{nf.format(row.total_perempuan)}</td>
                <td class="px-4 py-3 text-right tnum font-semibold text-arang-900">{nf.format(row.total_penduduk)}</td>
              </tr>
            {/each}
            <tr class="border-t-2 border-arang-200 bg-krem-100/70">
              <td class="px-4 py-3 text-xs font-semibold uppercase tracking-widest text-arang-700">Jumlah</td>
              <td class="px-4 py-3 text-right tnum font-semibold text-arang-900">{nf.format(totalKK)}</td>
              <td class="px-4 py-3 text-right tnum font-semibold text-arang-900">{nf.format(totalLaki)}</td>
              <td class="px-4 py-3 text-right tnum font-semibold text-arang-900">{nf.format(totalPerempuan)}</td>
              <td class="px-4 py-3 text-right tnum font-semibold text-arang-900">{nf.format(totalPenduduk)}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Mobile: stacked cards per pedukuhan -->
      <ul class="md:hidden space-y-3" aria-label="Daftar pedukuhan">
        {#each sortedPedukuhan as row (row.pedukuhan)}
          <li class="rounded-lg border border-krem-200 bg-white p-4">
            <div class="flex items-center justify-between">
              <h3 class="font-medium text-arang-900">{row.pedukuhan}</h3>
              <span class="text-xs uppercase tracking-widest text-arang-500">Total {nf.format(row.total_penduduk)}</span>
            </div>
            <dl class="mt-3 grid grid-cols-3 gap-3 text-sm">
              <div>
                <dt class="text-[0.65rem] uppercase tracking-widest text-arang-500">KK</dt>
                <dd class="tnum font-medium text-arang-900">{nf.format(row.total_kk)}</dd>
              </div>
              <div>
                <dt class="text-[0.65rem] uppercase tracking-widest text-arang-500">Laki</dt>
                <dd class="tnum font-medium text-arang-900">{nf.format(row.total_laki)}</dd>
              </div>
              <div>
                <dt class="text-[0.65rem] uppercase tracking-widest text-arang-500">Perempuan</dt>
                <dd class="tnum font-medium text-arang-900">{nf.format(row.total_perempuan)}</dd>
              </div>
            </dl>
          </li>
        {/each}
        <li class="rounded-lg border border-arang-200 bg-krem-100/70 p-4">
          <div class="flex items-center justify-between">
            <h3 class="text-xs font-semibold uppercase tracking-widest text-arang-700">Jumlah</h3>
            <span class="tnum font-semibold text-arang-900">{nf.format(totalPenduduk)} jiwa</span>
          </div>
          <dl class="mt-3 grid grid-cols-3 gap-3 text-sm">
            <div>
              <dt class="text-[0.65rem] uppercase tracking-widest text-arang-500">KK</dt>
              <dd class="tnum font-semibold text-arang-900">{nf.format(totalKK)}</dd>
            </div>
            <div>
              <dt class="text-[0.65rem] uppercase tracking-widest text-arang-500">Laki</dt>
              <dd class="tnum font-semibold text-arang-900">{nf.format(totalLaki)}</dd>
            </div>
            <div>
              <dt class="text-[0.65rem] uppercase tracking-widest text-arang-500">Perempuan</dt>
              <dd class="tnum font-semibold text-arang-900">{nf.format(totalPerempuan)}</dd>
            </div>
          </dl>
        </li>
      </ul>
    {/if}
  </div>
</SectionShell>

<!-- Catatan kaki: small muted text, no shell wrap. -->
{#if !loading && ringkasan}
  <div class="container-page py-6">
    <p class="text-xs italic text-arang-500 leading-relaxed max-w-prose">
      {#if ringkasan?.per_pedukuhan?.[0]?.updated_at}
        Data diperbarui {formatTanggal(ringkasan.per_pedukuhan[0].updated_at)}.
      {/if}
      Distribusi per pedukuhan merupakan estimasi proporsional berdasarkan rekapitulasi
      terakhir dan dapat berubah pada pemutakhiran berikutnya.
    </p>
  </div>
{/if}

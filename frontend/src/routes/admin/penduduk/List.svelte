<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import DataTable, { type ColumnDef } from '../../../lib/admin/components/DataTable.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import EmptyState from '../../../lib/components/ui/EmptyState.svelte';
  import { Search, X as XIcon } from '../../../lib/components/icons';
  import { getRingkasan } from '../../../lib/api/penduduk';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';
  import type { PendudukPedukuhan } from '../../../lib/types';

  let rows: PendudukPedukuhan[] = [];
  let loading = true;
  let errorMessage: string | null = null;
  let controller: AbortController | undefined;

  // Search state - debounced 150ms to keep keystrokes snappy on slower devices.
  let searchInput = '';
  let query = '';
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  const numFmt = new Intl.NumberFormat('id-ID');

  const columns: ColumnDef<PendudukPedukuhan>[] = [
    { key: 'pedukuhan', label: 'Pedukuhan' },
    { key: 'total_kk', label: 'Jumlah KK', align: 'right', width: 'w-28' },
    { key: 'total_laki', label: 'Laki-laki', align: 'right', width: 'w-28' },
    { key: 'total_perempuan', label: 'Perempuan', align: 'right', width: 'w-28' },
    { key: 'total_penduduk', label: 'Total', align: 'right', width: 'w-28' },
    { key: 'actions', label: 'Aksi', align: 'right', sortable: false, width: 'w-28', hideOnMobile: true }
  ];

  async function load() {
    controller?.abort();
    controller = new AbortController();
    loading = true;
    errorMessage = null;
    try {
      const result = await getRingkasan({ signal: controller.signal });
      // Default sort: pedukuhan A-Z using Indonesian collation.
      rows = [...(result.per_pedukuhan ?? [])].sort((a, b) =>
        a.pedukuhan.localeCompare(b.pedukuhan, 'id')
      );
    } catch (err) {
      if ((err as DOMException)?.name === 'AbortError') return;
      errorMessage = isApiError(err) ? err.detail ?? err.message : 'Gagal memuat data penduduk.';
      rows = [];
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    if (!requireAuth()) return;
    load();
  });

  onDestroy(() => {
    controller?.abort();
    if (debounceTimer) clearTimeout(debounceTimer);
  });

  /** Debounce search input by 150ms to throttle reactive filter recomputes. */
  function onSearchInput(event: Event) {
    const value = (event.target as HTMLInputElement).value;
    searchInput = value;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      query = value.trim().toLowerCase();
    }, 150);
  }

  function clearSearch() {
    searchInput = '';
    query = '';
    if (debounceTimer) clearTimeout(debounceTimer);
  }

  // Case-insensitive substring match on the pedukuhan name only - the only
  // human-readable field on this read+update list.
  $: filtered = query
    ? rows.filter((row) => (row.pedukuhan ?? '').toLowerCase().includes(query))
    : rows;

  function goEdit(pedukuhan: string) {
    navigate(`/admin/penduduk/${encodeURIComponent(pedukuhan)}`);
  }
</script>

<AdminShell>
  <div class="mb-5 md:mb-6">
    <h1 class="font-serif text-xl md:text-2xl font-semibold text-arang-900">Data Penduduk</h1>
    <p class="mt-1 text-sm text-arang-600">
      Perbarui jumlah KK dan jumlah penduduk per pedukuhan. 19 pedukuhan dikelola tetap; tidak ada penambahan atau penghapusan.
    </p>
  </div>

  <div class="flex flex-col sm:flex-row sm:items-center gap-3 mb-4">
    <div class="relative flex-1 max-w-md">
      <Search
        class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-arang-500 pointer-events-none"
        strokeWidth={2}
        aria-hidden="true"
      />
      <input
        type="search"
        autocomplete="off"
        spellcheck="false"
        value={searchInput}
        on:input={onSearchInput}
        placeholder="Cari nama pedukuhan..."
        aria-label="Cari pedukuhan"
        class="w-full min-h-11 pl-10 pr-10 rounded-lg border border-krem-300 bg-white text-base md:text-sm text-arang-900 placeholder:text-arang-400 focus:outline-none focus:ring-2 focus:ring-menoreh-500 focus:border-menoreh-500"
      />
      {#if searchInput}
        <button
          type="button"
          on:click={clearSearch}
          aria-label="Hapus pencarian"
          class="absolute right-2 top-1/2 -translate-y-1/2 inline-flex items-center justify-center w-8 h-8 rounded-md text-arang-500 hover:text-arang-900 hover:bg-krem-100 focus:outline-none focus-visible:ring-2 focus-visible:ring-menoreh-500"
        >
          <XIcon class="w-4 h-4" strokeWidth={2} aria-hidden="true" />
        </button>
      {/if}
    </div>
    {#if !loading && !errorMessage && rows.length > 0}
      <p class="text-[11px] font-semibold uppercase tracking-wider text-arang-500 tnum">
        {filtered.length} dari {rows.length} hasil
      </p>
    {/if}
  </div>

  {#if !loading && !errorMessage && rows.length > 0 && filtered.length === 0}
    <EmptyState
      variant="empty"
      title="Tidak ada hasil"
      description="Tidak ada pedukuhan yang cocok dengan kata kunci. Coba kata kunci lain atau hapus filter."
      actionLabel="Hapus filter"
      onAction={clearSearch}
    />
  {:else}
    <DataTable
      {columns}
      rows={filtered}
      {loading}
      error={errorMessage}
      getId={(r) => r.pedukuhan}
      emptyTitle="Data pedukuhan belum tersedia"
      emptyDescription="Pastikan migrasi seed penduduk telah dijalankan di backend."
      on:retry={load}
    >
      <svelte:fragment slot="cell" let:row let:col>
        {#if col.key === 'pedukuhan'}
          <span class="font-medium text-arang-900">{row.pedukuhan}</span>
        {:else if col.key === 'total_kk'}
          <span class="tnum text-arang-700">{numFmt.format(row.total_kk)}</span>
        {:else if col.key === 'total_laki'}
          <span class="tnum text-arang-700">{numFmt.format(row.total_laki)}</span>
        {:else if col.key === 'total_perempuan'}
          <span class="tnum text-arang-700">{numFmt.format(row.total_perempuan)}</span>
        {:else if col.key === 'total_penduduk'}
          <span class="tnum font-semibold text-arang-900">{numFmt.format(row.total_penduduk)}</span>
        {:else if col.key === 'actions'}
          <div class="flex justify-end">
            <Button variant="ghost" size="sm" on:click={() => goEdit(row.pedukuhan)}>Ubah</Button>
          </div>
        {/if}
      </svelte:fragment>

      <svelte:fragment slot="mobileActions" let:row>
        <Button variant="secondary" size="md" on:click={() => goEdit(row.pedukuhan)} class="w-full">
          Ubah
        </Button>
      </svelte:fragment>
    </DataTable>
  {/if}
</AdminShell>

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import DataTable, { type ColumnDef } from '../../../lib/admin/components/DataTable.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import { getRingkasan } from '../../../lib/api/penduduk';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';
  import type { PendudukPedukuhan } from '../../../lib/types';

  let rows: PendudukPedukuhan[] = [];
  let loading = true;
  let errorMessage: string | null = null;
  let controller: AbortController | undefined;

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
      rows = result.per_pedukuhan ?? [];
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

  onDestroy(() => controller?.abort());

  function goEdit(pedukuhan: string) {
    navigate(`/admin/penduduk/${encodeURIComponent(pedukuhan)}`);
  }
</script>

<AdminShell>
  <div class="flex items-start justify-between gap-4 mb-6">
    <div>
      <h1 class="font-serif text-2xl font-semibold text-arang-900">Data Penduduk</h1>
      <p class="mt-1 text-sm text-arang-600">
        Perbarui jumlah KK dan jumlah penduduk per pedukuhan. 19 pedukuhan dikelola tetap; tidak ada penambahan atau penghapusan.
      </p>
    </div>
  </div>

  <DataTable
    {columns}
    {rows}
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
      <div class="flex justify-end">
        <Button variant="secondary" size="sm" on:click={() => goEdit(row.pedukuhan)}>Ubah</Button>
      </div>
    </svelte:fragment>
  </DataTable>
</AdminShell>

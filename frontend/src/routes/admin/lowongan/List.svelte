<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import DataTable, { type ColumnDef } from '../../../lib/admin/components/DataTable.svelte';
  import DeleteConfirm from '../../../lib/admin/components/DeleteConfirm.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import Badge from '../../../lib/components/ui/Badge.svelte';
  import { listLowongan } from '../../../lib/api/lowongan';
  import { deleteLowongan } from '../../../lib/api/admin/lowongan';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';
  import type { Lowongan } from '../../../lib/types';

  let items: Lowongan[] = [];
  let loading = true;
  let errorMessage: string | null = null;
  let controller: AbortController | undefined;

  let pending: Lowongan | null = null;
  let deleting = false;
  let deleteError: string | null = null;

  const dateFmt = new Intl.DateTimeFormat('id-ID', { dateStyle: 'medium' });
  const rupiahFmt = new Intl.NumberFormat('id-ID');

  const columns: ColumnDef<Lowongan>[] = [
    { key: 'judul', label: 'Judul' },
    { key: 'instansi', label: 'Instansi' },
    { key: 'kategori', label: 'Kategori', width: 'w-32' },
    { key: 'gaji', label: 'Gaji', sortable: false, width: 'w-40' },
    { key: 'deadline', label: 'Deadline', width: 'w-32' },
    { key: 'status', label: 'Status', width: 'w-24' },
    { key: 'actions', label: 'Aksi', align: 'right', sortable: false, width: 'w-32', hideOnMobile: true }
  ];

  async function load() {
    controller?.abort();
    controller = new AbortController();
    loading = true;
    errorMessage = null;
    try {
      items = await listLowongan({}, { signal: controller.signal });
    } catch (err) {
      if ((err as DOMException)?.name === 'AbortError') return;
      errorMessage = isApiError(err) ? err.detail ?? err.message : 'Gagal memuat lowongan.';
      items = [];
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    if (!requireAuth()) return;
    load();
  });

  onDestroy(() => controller?.abort());

  function formatDate(raw: string | null | undefined): string {
    if (!raw) return '-';
    const d = new Date(raw);
    return Number.isNaN(d.getTime()) ? '-' : dateFmt.format(d);
  }

  function formatGaji(min: number | null, max: number | null): string {
    if (min == null && max == null) return 'Tidak disebutkan';
    if (min != null && max != null) return `Rp ${rupiahFmt.format(min)} - ${rupiahFmt.format(max)}`;
    if (min != null) return `Mulai Rp ${rupiahFmt.format(min)}`;
    return `Hingga Rp ${rupiahFmt.format(max ?? 0)}`;
  }

  function goEdit(id: string) {
    navigate(`/admin/lowongan/${id}`);
  }

  function askDelete(row: Lowongan) {
    pending = row;
    deleteError = null;
  }

  async function confirmDelete() {
    if (!pending) return;
    deleting = true;
    deleteError = null;
    try {
      await deleteLowongan(pending.id);
      pending = null;
      await load();
    } catch (err) {
      deleteError = isApiError(err) ? err.detail ?? err.message : 'Gagal menghapus lowongan.';
    } finally {
      deleting = false;
    }
  }

  function cancelDelete() {
    if (deleting) return;
    pending = null;
    deleteError = null;
  }
</script>

<AdminShell>
  <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3 sm:gap-4 mb-5 md:mb-6">
    <div class="min-w-0">
      <h1 class="font-serif text-xl md:text-2xl font-semibold text-arang-900">Lowongan Kerja</h1>
      <p class="mt-1 text-sm text-arang-600">Kelola lowongan UMKM, formal, dan freelance warga desa.</p>
    </div>
    <Button href="#/admin/lowongan/new" variant="primary" size="md" class="w-full sm:w-auto">
      Tambah Lowongan
    </Button>
  </div>

  <DataTable
    {columns}
    rows={items}
    {loading}
    error={errorMessage}
    getId={(r) => r.id}
    emptyTitle="Belum ada lowongan"
    emptyDescription="Tambahkan lowongan kerja pertama agar warga dapat melihatnya."
    on:retry={load}
  >
    <svelte:fragment slot="cell" let:row let:col>
      {#if col.key === 'judul'}
        <span class="font-medium text-arang-900">{row.judul}</span>
        {#if row.lokasi_pedukuhan}
          <p class="text-xs text-arang-500 mt-0.5">Pedukuhan {row.lokasi_pedukuhan}</p>
        {/if}
      {:else if col.key === 'instansi'}
        <span class="text-arang-700">{row.instansi}</span>
      {:else if col.key === 'kategori'}
        <Badge variant={((row.kategori ?? 'neutral').toLowerCase()) as never}>{row.kategori}</Badge>
      {:else if col.key === 'gaji'}
        <span class="text-xs text-arang-700 tnum">{formatGaji(row.gaji_min, row.gaji_max)}</span>
      {:else if col.key === 'deadline'}
        <span class="text-arang-700 tnum">{formatDate(row.deadline)}</span>
      {:else if col.key === 'status'}
        <Badge variant={((row.status ?? 'neutral').toLowerCase()) as never}>{row.status}</Badge>
      {:else if col.key === 'actions'}
        <div class="flex justify-end gap-1.5">
          <Button variant="ghost" size="sm" on:click={() => goEdit(row.id)}>Ubah</Button>
          <Button variant="ghost" size="sm" on:click={() => askDelete(row)}>Hapus</Button>
        </div>
      {/if}
    </svelte:fragment>

    <svelte:fragment slot="mobileActions" let:row>
      <div class="grid grid-cols-2 gap-2">
        <Button variant="secondary" size="md" on:click={() => goEdit(row.id)} class="w-full">Ubah</Button>
        <Button variant="danger" size="md" on:click={() => askDelete(row)} class="w-full">Hapus</Button>
      </div>
    </svelte:fragment>
  </DataTable>

  {#if deleteError}
    <p class="mt-3 text-sm text-terakota-700" role="alert">{deleteError}</p>
  {/if}
</AdminShell>

<DeleteConfirm
  open={pending !== null}
  title="Hapus lowongan?"
  description={pending ? `Lowongan "${pending.judul}" dari ${pending.instansi} akan dihapus permanen.` : ''}
  loading={deleting}
  onConfirm={confirmDelete}
  onCancel={cancelDelete}
/>

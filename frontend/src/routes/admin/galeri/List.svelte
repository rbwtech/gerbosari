<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import DataTable, { type ColumnDef } from '../../../lib/admin/components/DataTable.svelte';
  import DeleteConfirm from '../../../lib/admin/components/DeleteConfirm.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import Badge from '../../../lib/components/ui/Badge.svelte';
  import { listGaleri } from '../../../lib/api/galeri';
  import { deleteGaleri } from '../../../lib/api/admin/galeri';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';
  import type { Galeri } from '../../../lib/types';

  let items: Galeri[] = [];
  let loading = true;
  let errorMessage: string | null = null;
  let controller: AbortController | undefined;

  // Delete-flow state. `pending` doubles as both the open flag and the row id
  // so the modal can render the row title in the confirmation copy.
  let pending: Galeri | null = null;
  let deleting = false;
  let deleteError: string | null = null;

  const dateFmt = new Intl.DateTimeFormat('id-ID', { dateStyle: 'medium' });

  const columns: ColumnDef<Galeri>[] = [
    { key: 'thumb', label: 'Pratinjau', sortable: false, width: 'w-20', hideOnMobile: false },
    { key: 'judul', label: 'Judul' },
    { key: 'kategori', label: 'Kategori', width: 'w-32' },
    { key: 'taken_at', label: 'Tanggal', width: 'w-32' },
    { key: 'actions', label: 'Aksi', align: 'right', sortable: false, width: 'w-32', hideOnMobile: true }
  ];

  async function load() {
    controller?.abort();
    controller = new AbortController();
    loading = true;
    errorMessage = null;
    try {
      items = await listGaleri({}, { signal: controller.signal });
    } catch (err) {
      if ((err as DOMException)?.name === 'AbortError') return;
      errorMessage = isApiError(err) ? err.detail ?? err.message : 'Gagal memuat galeri.';
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

  function goEdit(id: string) {
    navigate(`/admin/galeri/${id}`);
  }

  function askDelete(row: Galeri) {
    pending = row;
    deleteError = null;
  }

  async function confirmDelete() {
    if (!pending) return;
    deleting = true;
    deleteError = null;
    try {
      await deleteGaleri(pending.id);
      pending = null;
      await load();
    } catch (err) {
      deleteError = isApiError(err) ? err.detail ?? err.message : 'Gagal menghapus foto.';
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
      <h1 class="font-serif text-xl md:text-2xl font-semibold text-arang-900">Galeri</h1>
      <p class="mt-1 text-sm text-arang-600">Kelola dokumentasi foto desa per kategori.</p>
    </div>
    <Button href="#/admin/galeri/new" variant="primary" size="md" class="w-full sm:w-auto">
      Tambah Galeri
    </Button>
  </div>

  <DataTable
    {columns}
    rows={items}
    {loading}
    error={errorMessage}
    getId={(r) => r.id}
    emptyTitle="Belum ada foto"
    emptyDescription="Tambahkan foto pertama untuk mulai mengisi galeri desa."
    on:retry={load}
  >
    <svelte:fragment slot="cell" let:row let:col>
      {#if col.key === 'thumb'}
        <img
          src={row.file_path}
          alt={row.judul}
          loading="lazy"
          decoding="async"
          class="w-14 h-14 object-cover rounded border border-krem-200 bg-krem-100"
        />
      {:else if col.key === 'judul'}
        <span class="font-medium text-arang-900">{row.judul}</span>
        {#if row.deskripsi}
          <p class="text-xs text-arang-500 line-clamp-1 mt-0.5">{row.deskripsi}</p>
        {/if}
      {:else if col.key === 'kategori'}
        <Badge variant={((row.kategori ?? 'neutral').toLowerCase()) as never}>{row.kategori}</Badge>
      {:else if col.key === 'taken_at'}
        <span class="text-arang-700 tnum">{formatDate(row.taken_at)}</span>
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
  title="Hapus foto galeri?"
  description={pending ? `Foto "${pending.judul}" akan dihapus permanen dan tidak dapat dikembalikan.` : ''}
  loading={deleting}
  onConfirm={confirmDelete}
  onCancel={cancelDelete}
/>

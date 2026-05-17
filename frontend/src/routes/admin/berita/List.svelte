<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import DataTable, { type ColumnDef } from '../../../lib/admin/components/DataTable.svelte';
  import DeleteConfirm from '../../../lib/admin/components/DeleteConfirm.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import Badge from '../../../lib/components/ui/Badge.svelte';
  import { listBerita } from '../../../lib/api/berita';
  import { deleteBerita } from '../../../lib/api/admin/berita';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';
  import type { Berita } from '../../../lib/types';

  let items: Berita[] = [];
  let loading = true;
  let errorMessage: string | null = null;
  let controller: AbortController | undefined;

  let pending: Berita | null = null;
  let deleting = false;
  let deleteError: string | null = null;

  const dateFmt = new Intl.DateTimeFormat('id-ID', { dateStyle: 'medium' });

  const columns: ColumnDef<Berita>[] = [
    { key: 'judul', label: 'Judul' },
    { key: 'kategori', label: 'Kategori', width: 'w-28' },
    { key: 'author', label: 'Penulis', width: 'w-44' },
    { key: 'published_at', label: 'Diterbitkan', width: 'w-32' },
    { key: 'actions', label: 'Aksi', align: 'right', sortable: false, width: 'w-32', hideOnMobile: true }
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
      errorMessage = isApiError(err) ? err.detail ?? err.message : 'Gagal memuat berita.';
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

  function goEdit(slug: string) {
    navigate(`/admin/berita/${encodeURIComponent(slug)}`);
  }

  function askDelete(row: Berita) {
    pending = row;
    deleteError = null;
  }

  async function confirmDelete() {
    if (!pending) return;
    deleting = true;
    deleteError = null;
    try {
      await deleteBerita(pending.slug);
      pending = null;
      await load();
    } catch (err) {
      deleteError = isApiError(err) ? err.detail ?? err.message : 'Gagal menghapus berita.';
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
  <div class="flex items-start justify-between gap-4 mb-6">
    <div>
      <h1 class="font-serif text-2xl font-semibold text-arang-900">Berita & Agenda</h1>
      <p class="mt-1 text-sm text-arang-600">Kelola artikel berita dan agenda kegiatan desa.</p>
    </div>
    <Button href="#/admin/berita/new" variant="primary" size="md">Tambah Berita</Button>
  </div>

  <DataTable
    {columns}
    rows={items}
    {loading}
    error={errorMessage}
    getId={(r) => r.slug}
    emptyTitle="Belum ada berita"
    emptyDescription="Tulis berita atau agenda pertama untuk warga desa."
    on:retry={load}
  >
    <svelte:fragment slot="cell" let:row let:col>
      {#if col.key === 'judul'}
        <span class="font-medium text-arang-900">{row.judul}</span>
        <p class="text-xs text-arang-500 mt-0.5 font-mono">{row.slug}</p>
      {:else if col.key === 'kategori'}
        <Badge variant={((row.kategori ?? 'neutral').toLowerCase()) as never}>{row.kategori}</Badge>
      {:else if col.key === 'author'}
        <span class="text-arang-700">{row.author}</span>
      {:else if col.key === 'published_at'}
        <span class="text-arang-700 tnum">{formatDate(row.published_at)}</span>
      {:else if col.key === 'actions'}
        <div class="flex justify-end gap-1.5">
          <Button variant="ghost" size="sm" on:click={() => goEdit(row.slug)}>Ubah</Button>
          <Button variant="ghost" size="sm" on:click={() => askDelete(row)}>Hapus</Button>
        </div>
      {/if}
    </svelte:fragment>

    <svelte:fragment slot="mobileActions" let:row>
      <div class="flex justify-end gap-2">
        <Button variant="secondary" size="sm" on:click={() => goEdit(row.slug)}>Ubah</Button>
        <Button variant="danger" size="sm" on:click={() => askDelete(row)}>Hapus</Button>
      </div>
    </svelte:fragment>
  </DataTable>

  {#if deleteError}
    <p class="mt-3 text-sm text-terakota-700" role="alert">{deleteError}</p>
  {/if}
</AdminShell>

<DeleteConfirm
  open={pending !== null}
  title="Hapus berita?"
  description={pending ? `Berita "${pending.judul}" akan dihapus permanen dan tidak dapat dikembalikan.` : ''}
  loading={deleting}
  onConfirm={confirmDelete}
  onCancel={cancelDelete}
/>

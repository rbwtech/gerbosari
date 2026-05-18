<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import DataTable, { type ColumnDef } from '../../../lib/admin/components/DataTable.svelte';
  import DeleteConfirm from '../../../lib/admin/components/DeleteConfirm.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import Badge from '../../../lib/components/ui/Badge.svelte';
  import EmptyState from '../../../lib/components/ui/EmptyState.svelte';
  import { Search, X } from '../../../lib/components/icons';
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

  // Client-side search. `searchInput` is bound to the field; `searchTerm` is
  // the debounced, trimmed lowercase value used for filtering to avoid
  // re-computing the haystack on every keystroke.
  let searchInput = '';
  let searchTerm = '';
  let debounceHandle: ReturnType<typeof setTimeout> | undefined;

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

  function clearSearch() {
    searchInput = '';
    if (debounceHandle) clearTimeout(debounceHandle);
    searchTerm = '';
  }

  // Concatenated haystack across the fields most likely to identify a posting.
  $: filtered = searchTerm
    ? items.filter((it) => {
        const hay = `${it.judul ?? ''} ${it.instansi ?? ''} ${it.kategori ?? ''} ${it.lokasi_pedukuhan ?? ''} ${it.deskripsi ?? ''}`.toLowerCase();
        return hay.includes(searchTerm);
      })
    : items;

  $: isSearching = searchTerm.length > 0;
  $: hasNoResults = !loading && !errorMessage && isSearching && filtered.length === 0;

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
    <Button href="#/admin/lowongan/new" variant="primary" size="md" class="w-full sm:w-auto min-h-11">
      Tambah Lowongan
    </Button>
  </div>

  <!-- Toolbar: client-side search field. Stacks above table on every breakpoint
       so the table retains full width on desktop. -->
  <div class="flex flex-col sm:flex-row sm:items-center gap-3 mb-4">
    <label class="relative flex-1 max-w-md sm:max-w-sm">
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
        placeholder="Cari judul, instansi, atau kategori..."
        autocomplete="off"
        spellcheck="false"
        class="min-h-11 h-11 w-full rounded-md border border-krem-300 bg-white pl-10 pr-10 text-base md:text-sm text-arang-900 placeholder:text-arang-400 focus:border-menoreh-500 focus:outline-none focus:ring-2 focus:ring-menoreh-500/30"
      />
      {#if searchInput}
        <button
          type="button"
          on:click={clearSearch}
          aria-label="Bersihkan pencarian"
          class="absolute right-2 top-1/2 -translate-y-1/2 inline-flex h-7 w-7 items-center justify-center rounded text-arang-500 hover:text-arang-900 hover:bg-krem-100 focus:outline-none focus-visible:ring-2 focus-visible:ring-menoreh-500"
        >
          <X class="h-4 w-4" strokeWidth={2} aria-hidden="true" />
        </button>
      {/if}
    </label>
    {#if isSearching && !loading}
      <span class="text-xs font-medium uppercase tracking-wider text-arang-500 tnum">
        {filtered.length} dari {items.length} hasil
      </span>
    {/if}
  </div>

  {#if hasNoResults}
    <EmptyState
      variant="empty"
      title="Tidak ada hasil"
      description="Coba ubah kata kunci pencarian atau hapus filter."
      actionLabel="Hapus filter"
      onAction={clearSearch}
    />
  {:else}
    <DataTable
      {columns}
      rows={filtered}
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
          <Button variant="secondary" size="md" on:click={() => goEdit(row.id)} class="w-full flex-1">Ubah</Button>
          <Button variant="danger" size="md" on:click={() => askDelete(row)} class="w-full flex-1">Hapus</Button>
        </div>
      </svelte:fragment>
    </DataTable>
  {/if}

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

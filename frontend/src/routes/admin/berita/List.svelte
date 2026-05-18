<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import DataTable, { type ColumnDef } from '../../../lib/admin/components/DataTable.svelte';
  import DeleteConfirm from '../../../lib/admin/components/DeleteConfirm.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import Badge from '../../../lib/components/ui/Badge.svelte';
  import EmptyState from '../../../lib/components/ui/EmptyState.svelte';
  import Tabs from '../../../lib/components/ui/Tabs.svelte';
  import { Search, X as XIcon } from '../../../lib/components/icons';
  import { listBerita } from '../../../lib/api/berita';
  import { deleteBerita } from '../../../lib/api/admin/berita';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';
  import type { Berita } from '../../../lib/types';

  type TabId = 'semua' | 'berita' | 'agenda';

  let items: Berita[] = [];
  let loading = true;
  let errorMessage: string | null = null;
  let controller: AbortController | undefined;

  let pending: Berita | null = null;
  let deleting = false;
  let deleteError: string | null = null;

  // Search + tab filter state. `searchInput` is the raw input value; `query`
  // is the debounced lowercase value that drives the filter so typing stays
  // responsive on older Android devices.
  let searchInput = '';
  let query = '';
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;
  let activeTab: TabId = 'semua';

  const tabs: { id: TabId; label: string }[] = [
    { id: 'semua', label: 'Semua' },
    { id: 'berita', label: 'Berita' },
    { id: 'agenda', label: 'Agenda' }
  ];

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
      const result = await listBerita({}, { signal: controller.signal });
      // Default sort: latest published first. Stable when published_at is null.
      items = [...result].sort((a, b) => {
        const av = a.published_at ? Date.parse(a.published_at) : 0;
        const bv = b.published_at ? Date.parse(b.published_at) : 0;
        return bv - av;
      });
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

  onDestroy(() => {
    controller?.abort();
    if (debounceTimer) clearTimeout(debounceTimer);
  });

  /**
   * Debounce search keystrokes to 150ms. Prevents thrashing the reactive
   * filter pipeline while the user is mid-typing on slower devices.
   */
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

  function resetFilters() {
    clearSearch();
    activeTab = 'semua';
  }

  // Tab + search compose: filter by kategori first, then by substring match
  // across the searchable text fields. Case-insensitive substring per spec.
  $: filtered = items.filter((row) => {
    if (activeTab !== 'semua' && (row.kategori ?? '').toLowerCase() !== activeTab) {
      return false;
    }
    if (!query) return true;
    const haystack = [row.judul, row.slug, row.ringkasan, row.kategori, row.author]
      .map((v) => (v ?? '').toLowerCase())
      .join('  ');
    return haystack.includes(query);
  });

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
  <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3 sm:gap-4 mb-5 md:mb-6">
    <div class="min-w-0">
      <h1 class="font-serif text-xl md:text-2xl font-semibold text-arang-900">Berita & Agenda</h1>
      <p class="mt-1 text-sm text-arang-600">Kelola artikel berita dan agenda kegiatan desa.</p>
    </div>
    <Button href="#/admin/berita/new" variant="primary" size="md" class="w-full sm:w-auto">
      Tambah Berita
    </Button>
  </div>

  <Tabs
    {tabs}
    value={activeTab}
    ariaLabel="Filter kategori berita"
    class="mb-4"
    on:change={(e) => (activeTab = e.detail.value as TabId)}
  />

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
        placeholder="Cari judul, ringkasan, atau kategori..."
        aria-label="Cari berita"
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
    {#if !loading && !errorMessage && items.length > 0}
      <p class="text-[11px] font-semibold uppercase tracking-wider text-arang-500 tnum">
        {filtered.length} dari {items.length} hasil
      </p>
    {/if}
  </div>

  {#if !loading && !errorMessage && items.length > 0 && filtered.length === 0}
    <EmptyState
      variant="empty"
      title="Tidak ada hasil"
      description="Tidak ada berita yang cocok dengan filter aktif. Coba kata kunci lain atau hapus filter."
      actionLabel="Hapus filter"
      onAction={resetFilters}
    />
  {:else}
    <DataTable
      {columns}
      rows={filtered}
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
          <p class="text-xs text-arang-600 mt-1 line-clamp-2 md:hidden">{row.ringkasan}</p>
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
        <div class="grid grid-cols-2 gap-2">
          <Button variant="secondary" size="md" on:click={() => goEdit(row.slug)} class="w-full">Ubah</Button>
          <Button variant="danger" size="md" on:click={() => askDelete(row)} class="w-full">Hapus</Button>
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
  title="Hapus berita?"
  description={pending ? `Berita "${pending.judul}" akan dihapus permanen dan tidak dapat dikembalikan.` : ''}
  loading={deleting}
  onConfirm={confirmDelete}
  onCancel={cancelDelete}
/>

<script lang="ts">
  import { onMount } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import FormField from '../../../lib/admin/components/FormField.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import Card from '../../../lib/components/ui/Card.svelte';
  import EmptyState from '../../../lib/components/ui/EmptyState.svelte';
  import { getGaleri } from '../../../lib/api/galeri';
  import { createGaleri, updateGaleri, type CreateGaleriBody } from '../../../lib/api/admin/galeri';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';

  /** Route params injected by Router. id present = edit, absent = create. */
  export let params: { id?: string } = {};

  $: editing = Boolean(params.id);
  $: title = editing ? 'Ubah Foto Galeri' : 'Tambah Foto Galeri';

  // Form state - mirrored to CreateGaleriBody shape on submit.
  let judul = '';
  let deskripsi = '';
  let file_path = '';
  let kategori = 'wisata';
  let taken_at = '';

  let loadError: string | null = null;
  let loading = false;
  let submitting = false;
  let submitError: string | null = null;
  // Field-level validation errors. Keys mirror form field names.
  let fieldErrors: Partial<Record<keyof CreateGaleriBody, string>> = {};

  const kategoriOptions = [
    { value: 'wisata', label: 'Wisata' },
    { value: 'budaya', label: 'Budaya' },
    { value: 'agrowisata', label: 'Agrowisata' },
    { value: 'kegiatan', label: 'Kegiatan' }
  ];

  async function load() {
    if (!params.id) return;
    loading = true;
    loadError = null;
    try {
      const row = await getGaleri(params.id);
      judul = row.judul;
      deskripsi = row.deskripsi ?? '';
      file_path = row.file_path;
      kategori = row.kategori;
      // The backend serializes timestamps as ISO strings - slice to YYYY-MM-DD
      // because <input type="date"> only accepts that format.
      taken_at = row.taken_at ? row.taken_at.slice(0, 10) : '';
    } catch (err) {
      loadError = isApiError(err) ? err.detail ?? err.message : 'Gagal memuat data foto.';
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    if (!requireAuth()) return;
    load();
  });

  function validate(): boolean {
    fieldErrors = {};
    if (!judul.trim()) fieldErrors.judul = 'Judul wajib diisi.';
    if (!file_path.trim()) fieldErrors.file_path = 'Path file wajib diisi.';
    if (!kategori) fieldErrors.kategori = 'Kategori wajib dipilih.';
    return Object.keys(fieldErrors).length === 0;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!validate()) return;
    submitting = true;
    submitError = null;
    const body: CreateGaleriBody = {
      judul: judul.trim(),
      deskripsi: deskripsi.trim() ? deskripsi.trim() : null,
      file_path: file_path.trim(),
      kategori,
      taken_at: taken_at ? taken_at : null
    };
    try {
      if (editing && params.id) {
        await updateGaleri(params.id, body);
      } else {
        await createGaleri(body);
      }
      navigate('/admin/galeri');
    } catch (err) {
      submitError = isApiError(err) ? err.detail ?? err.message : 'Gagal menyimpan foto.';
    } finally {
      submitting = false;
    }
  }

  function cancel() {
    navigate('/admin/galeri');
  }
</script>

<AdminShell>
  <div class="mb-5 md:mb-6">
    <h1 class="font-serif text-xl md:text-2xl font-semibold text-arang-900 break-words">{title}</h1>
    <p class="mt-1 text-sm text-arang-600">
      {editing ? 'Perbarui metadata foto galeri.' : 'Lengkapi metadata untuk foto baru.'}
    </p>
  </div>

  {#if loading}
    <Card>
      <p class="text-sm text-arang-600">Memuat data...</p>
    </Card>
  {:else if loadError}
    <EmptyState variant="error" title="Tidak dapat memuat data" description={loadError}>
      <Button variant="secondary" size="sm" on:click={load}>Coba lagi</Button>
    </EmptyState>
  {:else}
    <form on:submit={handleSubmit} class="space-y-6 max-w-2xl mx-auto sm:mx-0">
      <Card padding="md">
        <div class="space-y-5">
          <FormField
            label="Judul"
            name="judul"
            required
            bind:value={judul}
            error={fieldErrors.judul}
            placeholder="Misal: Panen kopi di Pedukuhan Sumberejo"
          />

          <FormField
            type="textarea"
            label="Deskripsi"
            name="deskripsi"
            rows={3}
            bind:value={deskripsi}
            hint="Opsional. Tampil di lightbox publik."
          />

          <FormField
            label="Path File"
            name="file_path"
            required
            mono
            bind:value={file_path}
            error={fieldErrors.file_path}
            placeholder="/images/gallery/panen-kopi-2025.jpg"
            hint="Path relatif terhadap folder publik frontend, atau URL lengkap."
          />

          <FormField
            type="select"
            label="Kategori"
            name="kategori"
            required
            options={kategoriOptions}
            bind:value={kategori}
            error={fieldErrors.kategori}
          />

          <FormField
            type="date"
            label="Tanggal Pengambilan"
            name="taken_at"
            bind:value={taken_at}
            hint="Opsional. Format YYYY-MM-DD."
          />
        </div>
      </Card>

      {#if submitError}
        <p class="text-sm text-terakota-700" role="alert">{submitError}</p>
      {/if}

      <!-- Stacked on mobile (primary on top), inline on >=sm. Cancel is rendered
           as a secondary button so it remains tap-friendly, not a tiny link. -->
      <div class="flex flex-col-reverse sm:flex-row sm:items-center sm:justify-end gap-2">
        <Button
          variant="secondary"
          size="md"
          on:click={cancel}
          disabled={submitting}
          class="w-full sm:w-auto"
        >
          Batal
        </Button>
        <Button
          type="submit"
          variant="primary"
          size="md"
          loading={submitting}
          class="w-full sm:w-auto"
        >
          {editing ? 'Simpan Perubahan' : 'Tambah Foto'}
        </Button>
      </div>
    </form>
  {/if}
</AdminShell>

<script lang="ts">
  import { onMount } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import FormField from '../../../lib/admin/components/FormField.svelte';
  import ImageUploader from '../../../lib/admin/components/ImageUploader.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import Card from '../../../lib/components/ui/Card.svelte';
  import EmptyState from '../../../lib/components/ui/EmptyState.svelte';
  import { getBerita } from '../../../lib/api/berita';
  import { createBerita, updateBerita, type CreateBeritaBody } from '../../../lib/api/admin/berita';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';

  /** Slug is the canonical identity for berita - matches public route. */
  export let params: { slug?: string } = {};

  $: editing = Boolean(params.slug);
  $: title = editing ? 'Ubah Berita' : 'Tambah Berita';
  // Forwarded to AdminShell so the top-bar breadcrumb/title surfaces the
  // entity's judul once loaded instead of the slug fallback.
  $: crumbTitle = editing ? (judul.trim() || 'Ubah Berita') : 'Tambah Berita';

  let judul = '';
  let slug = '';
  let kategori = 'berita';
  let ringkasan = '';
  let konten = '';
  let gambar_url = '';
  let tanggal = '';
  let lokasi = '';
  let author = 'Pemerintah Desa Gerbosari';
  let published_at = '';
  // Track whether the user manually edited the slug. Auto-slugging stops once
  // they do so the URL doesn't keep churning while they polish the title.
  let slugTouched = false;

  let loadError: string | null = null;
  let loading = false;
  let submitting = false;
  let submitError: string | null = null;
  let fieldErrors: Partial<Record<keyof CreateBeritaBody, string>> = {};

  const kategoriOptions = [
    { value: 'berita', label: 'Berita' },
    { value: 'agenda', label: 'Agenda' }
  ];

  /** Strict lowercase-hyphen pattern enforced by the backend. */
  const SLUG_PATTERN = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;

  function slugify(s: string): string {
    return s
      .toLowerCase()
      .normalize('NFKD')
      .replace(/[^a-z0-9\s-]/g, '')
      .trim()
      .replace(/\s+/g, '-')
      .replace(/-+/g, '-');
  }

  // Auto-derive slug from judul while the user hasn't customized it.
  $: if (!editing && !slugTouched) {
    slug = slugify(judul);
  }

  async function load() {
    if (!params.slug) return;
    loading = true;
    loadError = null;
    try {
      const row = await getBerita(params.slug);
      judul = row.judul;
      slug = row.slug;
      slugTouched = true; // Editing: never auto-rewrite the slug.
      kategori = row.kategori;
      ringkasan = row.ringkasan;
      konten = row.konten;
      gambar_url = row.gambar_url ?? '';
      tanggal = row.tanggal ? row.tanggal.slice(0, 10) : '';
      lokasi = (row as unknown as { lokasi?: string | null }).lokasi ?? '';
      author = row.author;
      published_at = row.published_at ? row.published_at.slice(0, 10) : '';
    } catch (err) {
      loadError = isApiError(err) ? err.detail ?? err.message : 'Gagal memuat berita.';
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
    if (!slug.trim()) fieldErrors.slug = 'Slug wajib diisi.';
    else if (!SLUG_PATTERN.test(slug)) fieldErrors.slug = 'Slug hanya boleh huruf kecil, angka, dan tanda hubung.';
    if (!ringkasan.trim()) fieldErrors.ringkasan = 'Ringkasan wajib diisi.';
    if (!konten.trim()) fieldErrors.konten = 'Konten wajib diisi.';
    if (!author.trim()) fieldErrors.author = 'Penulis wajib diisi.';
    return Object.keys(fieldErrors).length === 0;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!validate()) return;
    submitting = true;
    submitError = null;
    const body: CreateBeritaBody = {
      judul: judul.trim(),
      slug: slug.trim(),
      kategori,
      ringkasan: ringkasan.trim(),
      konten,
      gambar_url: gambar_url.trim() ? gambar_url.trim() : null,
      tanggal: tanggal ? tanggal : null,
      lokasi: lokasi.trim() ? lokasi.trim() : null,
      author: author.trim(),
      // Backend `published_at` is DateTime<Utc>; the <input type="date"> only
      // gives a YYYY-MM-DD string. Promote it to an ISO-8601 instant at UTC
      // midnight so serde accepts it. Sending the bare date triggers 422.
      published_at: published_at ? `${published_at}T00:00:00Z` : null
    };
    try {
      if (editing && params.slug) {
        await updateBerita(params.slug, body);
      } else {
        await createBerita(body);
      }
      navigate('/admin/berita');
    } catch (err) {
      submitError = isApiError(err) ? err.detail ?? err.message : 'Gagal menyimpan berita.';
    } finally {
      submitting = false;
    }
  }

  function cancel() {
    navigate('/admin/berita');
  }

  function onSlugInput() {
    slugTouched = true;
  }
</script>

<AdminShell title={crumbTitle}>
  <div class="mb-5 md:mb-6">
    <h1 class="font-serif text-xl md:text-2xl font-semibold text-arang-900 break-words">{title}</h1>
    <p class="mt-1 text-sm text-arang-600">
      {editing ? 'Perbarui artikel berita atau agenda.' : 'Tulis artikel berita atau agenda baru.'}
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
    <form on:submit={handleSubmit} class="space-y-6 max-w-3xl mx-auto sm:mx-0">
      <Card padding="md">
        <div class="space-y-5">
          <FormField
            label="Judul"
            name="judul"
            required
            bind:value={judul}
            error={fieldErrors.judul}
            placeholder="Misal: Pelatihan Batik untuk Ibu PKK"
          />

          <div on:input={onSlugInput}>
            <FormField
              label="Slug"
              name="slug"
              required
              mono
              bind:value={slug}
              error={fieldErrors.slug}
              placeholder="pelatihan-batik-pkk"
              pattern="^[a-z0-9]+(?:-[a-z0-9]+)*$"
              hint="Hanya huruf kecil, angka, dan tanda hubung. Digunakan pada URL publik."
            />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-5">
            <FormField
              type="select"
              label="Kategori"
              name="kategori"
              required
              options={kategoriOptions}
              bind:value={kategori}
            />

            <FormField
              label="Penulis"
              name="author"
              required
              bind:value={author}
              error={fieldErrors.author}
            />
          </div>

          <FormField
            type="textarea"
            label="Ringkasan"
            name="ringkasan"
            rows={3}
            required
            bind:value={ringkasan}
            error={fieldErrors.ringkasan}
            placeholder="Satu hingga dua kalimat. Tampil di kartu daftar berita."
          />

          <FormField
            type="textarea"
            label="Konten"
            name="konten"
            rows={14}
            required
            mono
            tall
            bind:value={konten}
            error={fieldErrors.konten}
            placeholder={`Tulis dalam format Markdown.\n\n## Subjudul\n\nParagraf...`}
            hint="Format Markdown. Akan dirender sebagai HTML pada halaman publik."
          />

          <ImageUploader
            label="Gambar Sampul"
            name="gambar_url"
            entity="berita"
            bind:value={gambar_url}
            hint="Opsional. JPG, PNG, atau WebP (maks 5 MiB)."
          />

          <div class="grid grid-cols-1 sm:grid-cols-3 gap-5">
            <FormField
              type="date"
              label="Tanggal Kegiatan"
              name="tanggal"
              bind:value={tanggal}
              hint="Relevan untuk agenda."
            />

            <FormField
              type="date"
              label="Tanggal Terbit"
              name="published_at"
              bind:value={published_at}
              hint="Kosongkan untuk menggunakan tanggal sekarang."
            />

            <FormField
              label="Lokasi"
              name="lokasi"
              bind:value={lokasi}
              placeholder="Balai Desa Gerbosari"
            />
          </div>
        </div>
      </Card>

      {#if submitError}
        <p class="text-sm text-terakota-700" role="alert">{submitError}</p>
      {/if}

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
          {editing ? 'Simpan Perubahan' : 'Tambah Berita'}
        </Button>
      </div>
    </form>
  {/if}
</AdminShell>

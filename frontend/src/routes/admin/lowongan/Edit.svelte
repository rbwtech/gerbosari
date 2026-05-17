<script lang="ts">
  import { onMount } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import FormField from '../../../lib/admin/components/FormField.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import Card from '../../../lib/components/ui/Card.svelte';
  import EmptyState from '../../../lib/components/ui/EmptyState.svelte';
  import { getLowongan } from '../../../lib/api/lowongan';
  import { createLowongan, updateLowongan, type CreateLowonganBody } from '../../../lib/api/admin/lowongan';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';

  export let params: { id?: string } = {};

  $: editing = Boolean(params.id);
  $: title = editing ? 'Ubah Lowongan' : 'Tambah Lowongan';

  let judul = '';
  let instansi = '';
  let kategori = 'umkm';
  let deskripsi = '';
  let kontak = '';
  let gaji_min: number | null = null;
  let gaji_max: number | null = null;
  let deadline = '';
  let lokasi_pedukuhan = '';
  let status = 'aktif';

  let loadError: string | null = null;
  let loading = false;
  let submitting = false;
  let submitError: string | null = null;
  let fieldErrors: Partial<Record<keyof CreateLowonganBody, string>> = {};

  const kategoriOptions = [
    { value: 'umkm', label: 'UMKM' },
    { value: 'formal', label: 'Formal' },
    { value: 'freelance', label: 'Freelance' }
  ];

  const statusOptions = [
    { value: 'aktif', label: 'Aktif' },
    { value: 'tutup', label: 'Tutup' },
    { value: 'arsip', label: 'Arsip' }
  ];

  async function load() {
    if (!params.id) return;
    loading = true;
    loadError = null;
    try {
      const row = await getLowongan(params.id);
      judul = row.judul;
      instansi = row.instansi;
      kategori = row.kategori;
      deskripsi = row.deskripsi;
      kontak = row.kontak;
      gaji_min = row.gaji_min;
      gaji_max = row.gaji_max;
      deadline = row.deadline ? row.deadline.slice(0, 10) : '';
      lokasi_pedukuhan = row.lokasi_pedukuhan ?? '';
      status = row.status;
    } catch (err) {
      loadError = isApiError(err) ? err.detail ?? err.message : 'Gagal memuat lowongan.';
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
    if (!instansi.trim()) fieldErrors.instansi = 'Instansi wajib diisi.';
    if (!deskripsi.trim()) fieldErrors.deskripsi = 'Deskripsi wajib diisi.';
    if (!kontak.trim()) fieldErrors.kontak = 'Kontak wajib diisi.';
    if (gaji_min != null && gaji_max != null && gaji_min > gaji_max) {
      fieldErrors.gaji_max = 'Gaji maksimum tidak boleh lebih kecil dari minimum.';
    }
    return Object.keys(fieldErrors).length === 0;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!validate()) return;
    submitting = true;
    submitError = null;
    const body: CreateLowonganBody = {
      judul: judul.trim(),
      instansi: instansi.trim(),
      kategori,
      deskripsi: deskripsi.trim(),
      kontak: kontak.trim(),
      gaji_min: gaji_min === null || Number.isNaN(gaji_min) ? null : Number(gaji_min),
      gaji_max: gaji_max === null || Number.isNaN(gaji_max) ? null : Number(gaji_max),
      deadline: deadline ? deadline : null,
      lokasi_pedukuhan: lokasi_pedukuhan.trim() ? lokasi_pedukuhan.trim() : null,
      status
    };
    try {
      if (editing && params.id) {
        await updateLowongan(params.id, body);
      } else {
        await createLowongan(body);
      }
      navigate('/admin/lowongan');
    } catch (err) {
      submitError = isApiError(err) ? err.detail ?? err.message : 'Gagal menyimpan lowongan.';
    } finally {
      submitting = false;
    }
  }

  function cancel() {
    navigate('/admin/lowongan');
  }
</script>

<AdminShell>
  <div class="mb-6">
    <h1 class="font-serif text-2xl font-semibold text-arang-900">{title}</h1>
    <p class="mt-1 text-sm text-arang-600">
      {editing ? 'Perbarui detail lowongan kerja.' : 'Lengkapi detail lowongan kerja yang baru.'}
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
    <form on:submit={handleSubmit} class="space-y-6 max-w-2xl">
      <Card>
        <div class="space-y-5">
          <FormField
            label="Judul"
            name="judul"
            required
            bind:value={judul}
            error={fieldErrors.judul}
            placeholder="Misal: Penjaga Kebun Teh"
          />

          <FormField
            label="Instansi / Pemberi Kerja"
            name="instansi"
            required
            bind:value={instansi}
            error={fieldErrors.instansi}
            placeholder="Misal: PT Pagilaran"
          />

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
              type="select"
              label="Status"
              name="status"
              required
              options={statusOptions}
              bind:value={status}
            />
          </div>

          <FormField
            type="textarea"
            label="Deskripsi"
            name="deskripsi"
            rows={5}
            required
            bind:value={deskripsi}
            error={fieldErrors.deskripsi}
            placeholder="Uraian tugas, persyaratan, dan benefit."
          />

          <FormField
            label="Kontak"
            name="kontak"
            required
            bind:value={kontak}
            error={fieldErrors.kontak}
            placeholder="Nama PIC, nomor HP, atau email."
          />

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-5">
            <FormField
              type="number"
              label="Gaji Minimum (Rp)"
              name="gaji_min"
              bind:value={gaji_min}
              min={0}
              step={10000}
              placeholder="Opsional"
              hint="Kosongkan jika tidak disebutkan."
            />

            <FormField
              type="number"
              label="Gaji Maksimum (Rp)"
              name="gaji_max"
              bind:value={gaji_max}
              error={fieldErrors.gaji_max}
              min={0}
              step={10000}
              placeholder="Opsional"
            />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-5">
            <FormField
              type="date"
              label="Deadline"
              name="deadline"
              bind:value={deadline}
              hint="Opsional. Tanggal terakhir penerimaan lamaran."
            />

            <FormField
              label="Lokasi Pedukuhan"
              name="lokasi_pedukuhan"
              bind:value={lokasi_pedukuhan}
              placeholder="Misal: Sumberejo"
              hint="Opsional. Hanya nama pedukuhan."
            />
          </div>
        </div>
      </Card>

      {#if submitError}
        <p class="text-sm text-terakota-700" role="alert">{submitError}</p>
      {/if}

      <div class="flex items-center justify-end gap-2">
        <Button variant="ghost" size="md" on:click={cancel} disabled={submitting}>Batal</Button>
        <Button type="submit" variant="primary" size="md" loading={submitting}>
          {editing ? 'Simpan Perubahan' : 'Tambah Lowongan'}
        </Button>
      </div>
    </form>
  {/if}
</AdminShell>

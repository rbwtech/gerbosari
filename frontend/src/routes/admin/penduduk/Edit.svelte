<script lang="ts">
  import { onMount } from 'svelte';
  import AdminShell from '../../../lib/admin/AdminShell.svelte';
  import FormField from '../../../lib/admin/components/FormField.svelte';
  import Button from '../../../lib/components/ui/Button.svelte';
  import Card from '../../../lib/components/ui/Card.svelte';
  import EmptyState from '../../../lib/components/ui/EmptyState.svelte';
  import { getRingkasan } from '../../../lib/api/penduduk';
  import { updatePedukuhan, type UpdatePendudukBody } from '../../../lib/api/admin/penduduk';
  import { isApiError } from '../../../lib/api/client';
  import { requireAuth } from '../../../lib/auth/guard';
  import { navigate } from '../../../lib/router';

  /** Pedukuhan name from URL - already decoded by Router. */
  export let params: { pedukuhan?: string } = {};

  $: pedukuhanName = params.pedukuhan ?? '';

  let jumlah_kk: number | null = null;
  let laki: number | null = null;
  let perempuan: number | null = null;

  let loading = true;
  let loadError: string | null = null;
  let notFound = false;
  let submitting = false;
  let submitError: string | null = null;
  let fieldErrors: Partial<Record<keyof UpdatePendudukBody, string>> = {};

  // Derived total surfaces the impact of edits without a separate write path.
  $: total = (laki ?? 0) + (perempuan ?? 0);

  async function load() {
    if (!params.pedukuhan) return;
    loading = true;
    loadError = null;
    notFound = false;
    try {
      const ringkasan = await getRingkasan();
      const target = (ringkasan.per_pedukuhan ?? []).find((p) => p.pedukuhan === params.pedukuhan);
      if (!target) {
        notFound = true;
        return;
      }
      jumlah_kk = target.total_kk;
      laki = target.total_laki;
      perempuan = target.total_perempuan;
    } catch (err) {
      loadError = isApiError(err) ? err.detail ?? err.message : 'Gagal memuat data pedukuhan.';
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
    // Non-negative integers only. null is allowed so admin can clear a field.
    if (jumlah_kk != null && (Number.isNaN(jumlah_kk) || jumlah_kk < 0)) {
      fieldErrors.jumlah_kk = 'Jumlah KK harus angka non-negatif.';
    }
    if (laki != null && (Number.isNaN(laki) || laki < 0)) {
      fieldErrors.laki = 'Jumlah laki-laki harus angka non-negatif.';
    }
    if (perempuan != null && (Number.isNaN(perempuan) || perempuan < 0)) {
      fieldErrors.perempuan = 'Jumlah perempuan harus angka non-negatif.';
    }
    return Object.keys(fieldErrors).length === 0;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!params.pedukuhan) return;
    if (!validate()) return;
    submitting = true;
    submitError = null;
    const body: UpdatePendudukBody = {
      jumlah_kk: jumlah_kk === null ? null : Number(jumlah_kk),
      laki: laki === null ? null : Number(laki),
      perempuan: perempuan === null ? null : Number(perempuan)
    };
    try {
      await updatePedukuhan(params.pedukuhan, body);
      navigate('/admin/penduduk');
    } catch (err) {
      submitError = isApiError(err) ? err.detail ?? err.message : 'Gagal menyimpan data pedukuhan.';
    } finally {
      submitting = false;
    }
  }

  function cancel() {
    navigate('/admin/penduduk');
  }
</script>

<AdminShell>
  <div class="mb-5 md:mb-6">
    <p class="text-xs uppercase tracking-widest text-arang-500">Pedukuhan</p>
    <h1 class="font-serif text-xl md:text-2xl font-semibold text-arang-900 break-words">
      {pedukuhanName || 'Tanpa Nama'}
    </h1>
    <p class="mt-1 text-sm text-arang-600">Perbarui jumlah KK dan jumlah penduduk.</p>
  </div>

  {#if loading}
    <Card>
      <p class="text-sm text-arang-600">Memuat data...</p>
    </Card>
  {:else if notFound}
    <EmptyState
      variant="empty"
      title="Pedukuhan tidak ditemukan"
      description="Nama pedukuhan tidak cocok dengan data yang ada."
    >
      <Button variant="secondary" size="sm" on:click={cancel}>Kembali ke daftar</Button>
    </EmptyState>
  {:else if loadError}
    <EmptyState variant="error" title="Tidak dapat memuat data" description={loadError}>
      <Button variant="secondary" size="sm" on:click={load}>Coba lagi</Button>
    </EmptyState>
  {:else}
    <form on:submit={handleSubmit} class="space-y-6 max-w-md mx-auto sm:mx-0">
      <Card padding="md">
        <div class="space-y-5">
          <FormField
            type="number"
            label="Jumlah KK"
            name="jumlah_kk"
            bind:value={jumlah_kk}
            error={fieldErrors.jumlah_kk}
            min={0}
            step={1}
            required
          />

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-5">
            <FormField
              type="number"
              label="Jumlah Laki-laki"
              name="laki"
              bind:value={laki}
              error={fieldErrors.laki}
              min={0}
              step={1}
              required
            />

            <FormField
              type="number"
              label="Jumlah Perempuan"
              name="perempuan"
              bind:value={perempuan}
              error={fieldErrors.perempuan}
              min={0}
              step={1}
              required
            />
          </div>

          <div class="flex items-center justify-between pt-2 border-t border-krem-200">
            <span class="text-sm text-arang-600">Total Penduduk</span>
            <span class="font-serif text-xl font-semibold text-arang-900 tnum">{total.toLocaleString('id-ID')}</span>
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
          Simpan Perubahan
        </Button>
      </div>
    </form>
  {/if}
</AdminShell>

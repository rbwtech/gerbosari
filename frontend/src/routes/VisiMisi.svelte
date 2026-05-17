<script lang="ts">
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import { getVisiMisi } from '../lib/content';
  import { AlertTriangle } from '../lib/components/icons';

  /**
   * Visi & Misi — tabbed layout (Desa | Kabupaten) with a numbered serif list
   * for misi. Renders a disclaimer chip when the desa visi is editorially
   * inferred from the tagline rather than sourced from RPJM.
   */
  const data = getVisiMisi() as any;
  const desa = data?.desa ?? {};
  const kabupaten = data?.kabupaten ?? {};

  type TabId = 'desa' | 'kabupaten';
  const tabs: Array<{ id: TabId; label: string }> = [
    { id: 'desa', label: 'Desa Gerbosari' },
    { id: 'kabupaten', label: 'Kabupaten Kulon Progo' }
  ];
  let active: TabId = 'desa';

  $: visi = active === 'desa' ? desa?.visi : kabupaten?.visi;
  $: misi = (active === 'desa' ? desa?.misi : kabupaten?.misi) ?? [];
  $: visiInferred = active === 'desa' && desa?._visi_inferred === true;
  $: visiInferredNote = active === 'desa' ? desa?._visi_inferred_note : undefined;
  $: misiInferred = active === 'desa' && desa?._misi_inferred === true;
  $: misiInferredNote = active === 'desa' ? desa?._misi_inferred_note : undefined;
</script>

<PageHeader
  eyebrow="Arah Pembangunan"
  title="Visi & Misi"
  description="Cita-cita pembangunan jangka panjang dan langkah strategis di tingkat desa maupun kabupaten."
/>

<section class="container-page py-12 md:py-16" aria-labelledby="visimisi-tabs">
  <!-- Tabs -->
  <h2 id="visimisi-tabs" class="sr-only">Pilih level pemerintahan</h2>
  <div
    role="tablist"
    aria-labelledby="visimisi-tabs"
    class="inline-flex items-center gap-1 rounded-md border border-krem-200 bg-krem-50 p-1"
  >
    {#each tabs as t}
      <button
        type="button"
        role="tab"
        aria-selected={active === t.id}
        aria-controls="panel-{t.id}"
        id="tab-{t.id}"
        class="rounded px-4 h-9 text-sm font-medium transition-colors {active === t.id
          ? 'bg-menoreh-500 text-krem-50'
          : 'text-arang-700 hover:text-menoreh-700'}"
        on:click={() => (active = t.id)}
      >
        {t.label}
      </button>
    {/each}
  </div>

  <!-- Panel -->
  <div
    id="panel-{active}"
    role="tabpanel"
    aria-labelledby="tab-{active}"
    class="mt-10 grid grid-cols-1 lg:grid-cols-12 gap-10 lg:gap-16"
  >
    <!-- Visi column -->
    <div class="lg:col-span-5">
      <p class="eyebrow">Visi</p>
      <blockquote class="mt-4">
        <p class="font-serif text-2xl md:text-3xl font-medium text-arang-900 leading-snug text-balance">
          <span class="font-serif text-5xl leading-none text-terakota-500 align-top mr-1">&ldquo;</span>{visi ?? '—'}<span class="font-serif text-5xl leading-none text-terakota-500 align-bottom ml-1">&rdquo;</span>
        </p>
      </blockquote>

      {#if visiInferred}
        <div class="mt-6 flex items-start gap-2.5 rounded-md border border-terakota-500/20 bg-terakota-50 px-3.5 py-2.5 text-xs text-terakota-700">
          <AlertTriangle class="h-4 w-4 mt-0.5 shrink-0" strokeWidth={1.75} />
          <div>
            <span class="font-semibold">Diturunkan dari tagline desa.</span>
            {#if visiInferredNote}<span class="block mt-0.5 leading-relaxed text-terakota-700/90">{visiInferredNote}</span>{/if}
          </div>
        </div>
      {/if}
    </div>

    <!-- Misi column -->
    <div class="lg:col-span-7">
      <p class="eyebrow">Misi</p>
      <ol class="mt-4 space-y-5">
        {#each misi as point, i}
          <li class="grid grid-cols-[3.5rem_1fr] gap-4 border-b border-krem-200 pb-5 last:border-b-0 last:pb-0">
            <span class="font-serif text-3xl font-semibold text-terakota-600 tnum leading-none pt-1">
              {String(i + 1).padStart(2, '0')}
            </span>
            <p class="text-arang-800 leading-relaxed pt-1">{point}</p>
          </li>
        {/each}
      </ol>

      {#if misiInferred}
        <div class="mt-8 flex items-start gap-2.5 rounded-md border border-terakota-500/20 bg-terakota-50 px-3.5 py-2.5 text-xs text-terakota-700">
          <AlertTriangle class="h-4 w-4 mt-0.5 shrink-0" strokeWidth={1.75} />
          <div>
            <span class="font-semibold">Misi disusun berdasarkan dokumen profil.</span>
            {#if misiInferredNote}<span class="block mt-0.5 leading-relaxed text-terakota-700/90">{misiInferredNote}</span>{/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
</section>

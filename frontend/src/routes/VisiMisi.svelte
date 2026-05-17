<script lang="ts">
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import { getVisiMisi } from '../lib/content';

  /**
   * Visi & Misi - tabbed layout (Desa | Kabupaten) with a numbered serif list
   * for misi text content.
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
          <span class="font-serif text-5xl leading-none text-terakota-500 align-top mr-1">&ldquo;</span>{visi ?? '-'}<span class="font-serif text-5xl leading-none text-terakota-500 align-bottom ml-1">&rdquo;</span>
        </p>
      </blockquote>
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
    </div>
  </div>
</section>

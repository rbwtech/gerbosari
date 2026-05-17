<script lang="ts">
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import SectionShell from '../lib/components/ui/SectionShell.svelte';
  import { getVisiMisi } from '../lib/content';

  /**
   * Visi & Misi - tabbed layout (Desa | Kabupaten). The page deliberately
   * leans on typography over chrome: large serif visi blockquote, oversized
   * terakota numerals for misi items. Background biome stays minimal (mist
   * for visi, paper for misi) so the type carries the page.
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
  $: misi = ((active === 'desa' ? desa?.misi : kabupaten?.misi) ?? []) as string[];
</script>

<PageHeader
  eyebrow="Arah Pembangunan"
  title="Visi & Misi"
  description="Cita-cita pembangunan jangka panjang dan langkah strategis di tingkat desa maupun kabupaten."
/>

<!-- ============================================================ TABS ============================================================ -->
<!-- Tabs sit on a calm paper band so the underline indicator carries the
     emphasis. Each tab is a serif label; active gains a terakota-500 rule. -->
<SectionShell variant="default" padding="sm">
  <h2 id="visimisi-tabs" class="sr-only">Pilih level pemerintahan</h2>
  <!--
    Mobile fallback: scroll horizontally when labels overflow. Negative inset
    flushes the runway to the page edge inside .container-page, sm+ restores
    aligned layout.
  -->
  <div
    role="tablist"
    aria-labelledby="visimisi-tabs"
    class="flex items-center gap-1 border-b border-krem-300 overflow-x-auto -mx-4 px-4 sm:mx-0 sm:px-0 sm:overflow-visible"
  >
    {#each tabs as t}
      {@const isActive = active === t.id}
      <button
        type="button"
        role="tab"
        aria-selected={isActive}
        aria-controls="panel-{t.id}"
        id="tab-{t.id}"
        tabindex={isActive ? 0 : -1}
        class="relative shrink-0 px-4 sm:px-5 h-11 min-h-11 font-serif text-base md:text-lg transition-colors duration-200 ease-out
               {isActive ? 'text-arang-900' : 'text-arang-700 hover:text-arang-900'}"
        on:click={() => (active = t.id)}
      >
        {t.label}
        {#if isActive}
          <span
            class="absolute left-4 right-4 sm:left-5 sm:right-5 -bottom-px h-[2px] bg-terakota-500"
            aria-hidden="true"
          ></span>
        {/if}
      </button>
    {/each}
  </div>
</SectionShell>

<!-- ============================================================ VISI ============================================================ -->
<!-- Mist biome lifts the visi quote - mountain-morning paper, terakota
     opening quote SVG floats to the left like a printed flourish. -->
<SectionShell variant="mist" padding="lg">
  <div
    id="panel-{active}"
    role="tabpanel"
    aria-labelledby="tab-{active}"
    aria-label="Visi {active === 'desa' ? 'Desa Gerbosari' : 'Kabupaten Kulon Progo'}"
  >
    <p class="eyebrow">Visi</p>
    <blockquote class="mt-3 md:mt-4">
      <!-- Block quote mark above the text, with airy spacing instead of overlap. -->
      <svg
        aria-hidden="true"
        focusable="false"
        class="h-10 w-10 md:h-12 md:w-12 text-terakota-500 mb-3"
        viewBox="0 0 32 32"
        fill="currentColor"
      >
        <path d="M9.5 22H4.8c0-3.2.3-5.5 1-7s2.1-2.7 4-3.8L11 13c-1.1.7-1.9 1.4-2.4 2.2-.5.8-.8 1.7-.8 2.6h1.7v4.2zm12.7 0h-4.7c0-3.2.3-5.5 1-7s2.1-2.7 4-3.8l1.2 1.8c-1.1.7-1.9 1.4-2.4 2.2-.5.8-.8 1.7-.8 2.6h1.7V22z" />
      </svg>
      <p class="font-serif italic font-medium text-3xl md:text-5xl text-menoreh-900 leading-tight text-balance break-words">
        {visi ?? '-'}
      </p>
    </blockquote>
  </div>
</SectionShell>

<!-- ============================================================ MISI ============================================================ -->
<!-- Misi: oversized terakota-300 serif numerals beside Inter body - reads as
     a numbered manifesto on paper, not a SaaS feature list. -->
<SectionShell variant="default" padding="lg">
  <div aria-labelledby="misi-title">
    <div class="flex items-baseline gap-3 flex-wrap">
      <p class="eyebrow">Misi</p>
      <span aria-hidden="true" class="h-px flex-1 bg-krem-300 min-w-[2rem]"></span>
    </div>
    <h2
      id="misi-title"
      class="mt-2 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight"
    >
      Langkah Strategis
    </h2>
    <p class="mt-2 text-sm md:text-base text-arang-600 max-w-2xl leading-relaxed">
      Cita-cita visi diterjemahkan menjadi langkah-langkah pembangunan yang dapat dievaluasi setiap tahun.
    </p>

    <ol class="mt-6 md:mt-8 space-y-6 md:space-y-8">
      {#each misi as point, i}
        <li class="grid grid-cols-1 md:grid-cols-[6rem_1fr] gap-3 md:gap-8 border-b border-krem-200 pb-6 md:pb-8 last:border-b-0 last:pb-0">
          <!-- Huge serif numeral - terakota-300 keeps it as a watermark, not a
               competing label. Tabular for alignment, scaled down on mobile so
               360px viewports don't get a 90px glyph eating the column. -->
          <div
            class="font-serif text-4xl md:text-6xl font-semibold text-terakota-300 tnum leading-none"
            aria-hidden="true"
          >
            {String(i + 1).padStart(2, '0')}
          </div>
          <p class="text-base md:text-lg text-arang-800 leading-relaxed pt-0 md:pt-2 text-pretty break-words">
            <span class="sr-only">Misi {i + 1}.</span>{point}
          </p>
        </li>
      {/each}
    </ol>
  </div>
</SectionShell>

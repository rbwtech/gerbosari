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
  <div
    role="tablist"
    aria-labelledby="visimisi-tabs"
    class="flex items-center gap-1 border-b border-krem-300"
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
        class="relative px-5 h-11 font-serif text-base md:text-lg transition-colors duration-200 ease-out
               {isActive ? 'text-arang-900' : 'text-arang-700 hover:text-arang-900'}"
        on:click={() => (active = t.id)}
      >
        {t.label}
        {#if isActive}
          <span
            class="absolute left-5 right-5 -bottom-px h-[2px] bg-terakota-500"
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
    <blockquote class="mt-6 relative">
      <!-- Floating terakota opening quote mark (decorative, hidden from AT). -->
      <svg
        aria-hidden="true"
        focusable="false"
        class="pointer-events-none absolute -left-2 md:-left-4 -top-6 md:-top-8 h-12 w-12 md:h-16 md:w-16 text-terakota-500"
        viewBox="0 0 32 32"
        fill="currentColor"
      >
        <path d="M9.5 22H4.8c0-3.2.3-5.5 1-7s2.1-2.7 4-3.8L11 13c-1.1.7-1.9 1.4-2.4 2.2-.5.8-.8 1.7-.8 2.6h1.7v4.2zm12.7 0h-4.7c0-3.2.3-5.5 1-7s2.1-2.7 4-3.8l1.2 1.8c-1.1.7-1.9 1.4-2.4 2.2-.5.8-.8 1.7-.8 2.6h1.7V22z" />
      </svg>
      <p class="font-serif italic font-medium text-4xl md:text-5xl text-menoreh-900 leading-tight text-balance">
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
    <p class="eyebrow">Misi</p>
    <h2
      id="misi-title"
      class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight"
    >
      Langkah Strategis
    </h2>

    <ol class="mt-10 space-y-10 md:space-y-12">
      {#each misi as point, i}
        <li class="grid grid-cols-1 md:grid-cols-[10rem_1fr] gap-4 md:gap-10 border-b border-krem-200 pb-10 last:border-b-0 last:pb-0">
          <!-- Huge serif number - terakota-300 keeps it as a watermark, not a
               competing label. text-7xl per spec, tabular for alignment. -->
          <div
            class="font-serif text-6xl md:text-7xl font-semibold text-terakota-300 tnum leading-none"
            aria-hidden="true"
          >
            {String(i + 1).padStart(2, '0')}
          </div>
          <p class="text-base md:text-lg text-arang-800 leading-relaxed pt-2 md:pt-4">
            <span class="sr-only">Misi {i + 1}.</span>{point}
          </p>
        </li>
      {/each}
    </ol>
  </div>
</SectionShell>

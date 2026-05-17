<script lang="ts">
  /**
   * Page section wrapper with village-themed background "biomes".
   *
   * Routes should NEVER hard-code biome utility classes (.bg-paper,
   * .bg-menoreh-mist, etc.) on a <section> - pick a SectionShell variant
   * instead. This keeps biome decisions centralised so the palette and
   * texture intensity can be retuned in one place without touching routes.
   *
   * Variant intent:
   *   - default      neutral paper surface, used between richer biomes
   *   - mist         airy mountain-mist gradient, ideal for landing intros
   *   - tanah        warm tinted paper, reserved for sejarah / legenda
   *   - batik        krem with subtle batik dot motif, texture behind cards
   *   - menoreh-deep dark menoreh ridge, hero variants only (inverts text)
   */
  export let variant:
    | 'default'
    | 'mist'
    | 'tanah'
    | 'batik'
    | 'menoreh-deep' = 'default';

  /** Vertical padding scale. Use 'none' when the slot owns its own spacing. */
  export let padding: 'sm' | 'md' | 'lg' | 'none' = 'md';

  /** Wrap inner content with .container-page (max-width + horizontal padding). */
  export let contained = true;

  /** Optional landmark id for in-page anchors. */
  export let id: string | undefined = undefined;

  /** Extra classes appended to the <section> element. */
  let cls = '';
  export { cls as class };

  // Mobile vertical rhythm tightens so consecutive textured sections don't
  // stack 160px of empty space on a 360px viewport. Desktop values unchanged.
  // Tighter scale. Earlier values (py-12 md:py-20, py-14 md:py-28) produced
  // 160px+ gaps between consecutive sections because route content often
  // adds its own vertical padding inside the shell — paddings stack.
  // These values keep the page rhythm without creating empty oceans.
  const paddings = {
    none: '',
    sm: 'py-4 md:py-6',
    md: 'py-6 md:py-10',
    lg: 'py-8 md:py-14'
  } as const;

  // Variants resolve to SOLID warm-paper tones - no gradients, no SVG noise,
  // no pattern motifs. Differences are deliberately small so the eye reads
  // them as continuous warm paper, not as switching between AI-generated
  // background slabs. Variant *names* are preserved for backward compat with
  // routes that already chose a biome label, but every page now feels like
  // a single typeset publication, not a mosaic.
  const variantClass = {
    default:        'bg-krem-50',
    mist:           'bg-menoreh-50',
    tanah:          'bg-krem-100',
    batik:          'bg-krem-50',
    'menoreh-deep': 'bg-menoreh-900 text-krem-50'
  } as const;
</script>

<section {id} class="{variantClass[variant]} {paddings[padding]} {cls}">
  {#if contained}
    <div class="container-page">
      <slot />
    </div>
  {:else}
    <slot />
  {/if}
</section>

<script lang="ts">
  /**
   * Page section wrapper with village-themed background "biomes".
   *
   * Routes should NEVER hard-code biome utility classes (.bg-paper,
   * .bg-menoreh-mist, etc.) on a <section> — pick a SectionShell variant
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

  const paddings = {
    none: '',
    sm: 'py-8 md:py-10',
    md: 'py-16 md:py-20',
    lg: 'py-20 md:py-28'
  } as const;

  const variantClass = {
    default: 'bg-paper',
    mist: 'bg-menoreh-mist',
    tanah: 'bg-tanah-paper',
    batik: 'bg-batik-motif',
    'menoreh-deep': 'bg-menoreh-deep'
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

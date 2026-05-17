<script lang="ts">
  /** Visual treatment. `hero` is reserved for Beranda + Sejarah top-of-page. */
  export let variant: 'default' | 'hero' = 'default';
  /** Small uppercase category label rendered above the title. */
  export let eyebrow: string | undefined = undefined;
  export let title: string;
  /** Lead paragraph under the title. */
  export let subtitle: string | undefined = undefined;
  /** Legacy alias for subtitle — kept so existing routes don't churn. */
  export let description: string | undefined = undefined;
  /** Background image URL for hero variant. Falls back to flat surface. */
  export let image: string | undefined = undefined;

  let cls = '';
  export { cls as class };

  $: lead = subtitle ?? description;
</script>

{#if variant === 'hero'}
  <header
    class="relative overflow-hidden border-b border-krem-200 {cls}"
    style={image ? `background-image: url('${image}'); background-size: cover; background-position: center;` : undefined}
  >
    {#if image}
      <!-- Alpha gradient overlay — the only place a gradient is permitted -->
      <div
        class="absolute inset-0 bg-gradient-to-b from-arang-900/70 via-arang-900/55 to-arang-900/75"
        aria-hidden="true"
      ></div>
    {:else}
      <div class="absolute inset-0 bg-menoreh-900" aria-hidden="true"></div>
    {/if}

    <div class="relative container-page py-20 md:py-28">
      {#if $$slots.breadcrumb}
        <div class="mb-6 text-xs uppercase tracking-widest text-krem-100/80">
          <slot name="breadcrumb" />
        </div>
      {/if}

      {#if eyebrow}
        <p class="text-xs font-semibold uppercase tracking-widest text-terakota-300">
          {eyebrow}
        </p>
      {/if}

      <h1
        class="mt-4 font-serif font-semibold text-krem-50 text-balance
               text-5xl md:text-6xl max-w-4xl"
        style="letter-spacing: -0.03em; line-height: 1.05;"
      >
        {title}
      </h1>

      {#if lead}
        <p class="mt-6 text-base md:text-lg text-krem-100/90 max-w-2xl leading-relaxed">
          {lead}
        </p>
      {/if}

      {#if $$slots.default}
        <div class="mt-8">
          <slot />
        </div>
      {/if}
    </div>
  </header>
{:else}
  <header class="border-b border-krem-200 bg-krem-50 {cls}">
    <div class="container-page py-12 md:py-16">
      {#if $$slots.breadcrumb}
        <div class="mb-5 text-xs uppercase tracking-widest text-arang-500">
          <slot name="breadcrumb" />
        </div>
      {/if}

      {#if eyebrow}
        <p class="eyebrow">{eyebrow}</p>
      {/if}

      <h1
        class="mt-3 font-serif text-4xl md:text-5xl font-semibold text-arang-900 max-w-3xl text-balance"
        style="letter-spacing: -0.02em; line-height: 1.1;"
      >
        {title}
      </h1>

      {#if lead}
        <p class="mt-4 text-base md:text-lg text-arang-700 max-w-2xl leading-relaxed">
          {lead}
        </p>
      {/if}

      {#if $$slots.default}
        <div class="mt-6">
          <slot />
        </div>
      {/if}
    </div>
  </header>
{/if}

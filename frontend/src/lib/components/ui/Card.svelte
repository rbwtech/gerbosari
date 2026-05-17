<script lang="ts">
  /** Adds hover affordance for cards that act as links / clickable surfaces. */
  export let interactive = false;
  /** Padding tier. Pass 'none' externally via class override if needed. */
  export let padding: 'sm' | 'md' | 'lg' = 'md';
  export let as: 'div' | 'article' | 'section' = 'div';

  let cls = '';
  export { cls as class };

  const paddings: Record<string, string> = {
    sm: 'p-4',
    md: 'p-6',
    lg: 'p-8'
  };

  // No drop shadows - flat surface with 1px border, subtle hover swap.
  const base =
    'bg-white border border-krem-200 rounded-lg transition-colors duration-200 ease-out';
  const interactiveCls = interactive
    ? 'hover:border-menoreh-500 hover:bg-krem-50 cursor-pointer'
    : '';

  $: composed = `${base} ${interactiveCls} ${paddings[padding]} ${cls}`;
</script>

{#if as === 'article'}
  <article class={composed}>
    {#if $$slots.header}
      <header class="mb-4">
        <slot name="header" />
      </header>
    {/if}
    <slot />
    {#if $$slots.footer}
      <footer class="mt-4 pt-4 border-t border-krem-200">
        <slot name="footer" />
      </footer>
    {/if}
  </article>
{:else if as === 'section'}
  <section class={composed}>
    {#if $$slots.header}
      <header class="mb-4">
        <slot name="header" />
      </header>
    {/if}
    <slot />
    {#if $$slots.footer}
      <footer class="mt-4 pt-4 border-t border-krem-200">
        <slot name="footer" />
      </footer>
    {/if}
  </section>
{:else}
  <div class={composed}>
    {#if $$slots.header}
      <div class="mb-4">
        <slot name="header" />
      </div>
    {/if}
    <slot />
    {#if $$slots.footer}
      <div class="mt-4 pt-4 border-t border-krem-200">
        <slot name="footer" />
      </div>
    {/if}
  </div>
{/if}

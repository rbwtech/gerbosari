<script lang="ts">
  import type { ComponentType, SvelteComponent } from 'svelte';
  import { link } from '../../router';
  import { ArrowRight } from '../icons';

  /** Internal SPA route. External URLs should use a plain `<a>` instead. */
  export let href: string;
  export let title: string;
  export let description: string;
  /** Lucide icon component. Rendered in a tinted square above the title. */
  export let icon: ComponentType<SvelteComponent> | undefined = undefined;

  let cls = '';
  export { cls as class };
</script>

<a
  {href}
  use:link
  class="group block bg-white border border-krem-200 rounded-lg p-6 md:p-7
         transition-colors duration-200 ease-out
         hover:border-menoreh-500 hover:bg-krem-50 {cls}"
>
  {#if icon}
    <div
      class="inline-flex items-center justify-center w-11 h-11 rounded-md
             bg-menoreh-50 text-menoreh-700
             transition-colors duration-200 ease-out
             group-hover:bg-menoreh-700 group-hover:text-white"
    >
      <svelte:component this={icon} class="w-5 h-5" strokeWidth={1.75} aria-hidden="true" />
    </div>
  {/if}

  <h3 class="mt-5 font-serif text-xl md:text-2xl font-semibold text-arang-900 text-balance">
    {title}
  </h3>

  <p class="mt-2 text-sm text-arang-700 leading-relaxed text-pretty">
    {description}
  </p>

  <span
    class="mt-5 inline-flex items-center gap-1.5 text-sm font-medium text-menoreh-700
           transition-colors duration-200 ease-out
           group-hover:text-menoreh-800"
  >
    Pelajari lebih lanjut
    <ArrowRight class="w-4 h-4" strokeWidth={2} aria-hidden="true" />
  </span>
</a>

<script lang="ts">
  import type { ComponentType, SvelteComponent } from 'svelte';

  /** Big number - caller is responsible for formatting (locale, separators). */
  export let value: string | number;
  /** Short descriptor below the value. Rendered in caption style. */
  export let label: string;
  /** Unit suffix (e.g., 'jiwa', 'KK', 'ha'). Smaller, after value. */
  export let unit: string | undefined = undefined;
  /** Optional Lucide icon component rendered above the value. */
  export let icon: ComponentType<SvelteComponent> | undefined = undefined;

  let cls = '';
  export { cls as class };
</script>

<div class="flex flex-col gap-2 min-w-0 {cls}">
  {#if icon}
    <svelte:component this={icon} class="w-5 h-5 text-menoreh-600" strokeWidth={1.75} aria-hidden="true" />
  {/if}
  <div class="flex flex-col">
    <span class="font-serif text-3xl md:text-4xl font-semibold text-arang-900 tnum leading-none break-words">
      {value}
    </span>
    {#if unit}
      <span class="mt-1 text-sm font-medium text-arang-700">{unit}</span>
    {/if}
  </div>
  <span class="text-xs font-medium uppercase tracking-widest text-arang-500 break-words">
    {label}
  </span>
</div>

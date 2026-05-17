<script lang="ts">
  import type { ComponentType, SvelteComponent } from 'svelte';

  /** Big number — caller is responsible for formatting (locale, separators). */
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

<div class="flex flex-col gap-2 {cls}">
  {#if icon}
    <svelte:component this={icon} class="w-5 h-5 text-menoreh-600" strokeWidth={1.75} aria-hidden="true" />
  {/if}
  <div class="flex items-baseline gap-2">
    <span class="font-serif text-4xl md:text-5xl font-semibold text-arang-900 tnum leading-none">
      {value}
    </span>
    {#if unit}
      <span class="text-sm font-medium text-arang-700">{unit}</span>
    {/if}
  </div>
  <span class="text-xs font-medium uppercase tracking-widest text-arang-500">
    {label}
  </span>
</div>

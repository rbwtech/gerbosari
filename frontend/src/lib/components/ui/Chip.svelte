<script lang="ts">
  /** Visually highlights when the chip's facet is the current filter. */
  export let active = false;
  export let label: string;
  /** Optional count badge - e.g., '12' next to "Wisata". */
  export let count: number | undefined = undefined;
  export let disabled = false;

  let cls = '';
  export { cls as class };

  const base =
    'inline-flex items-center gap-1.5 px-3 h-8 rounded-full text-sm font-medium ' +
    'border transition-colors duration-200 ease-out ' +
    'disabled:opacity-50 disabled:cursor-not-allowed';

  $: state = active
    ? 'bg-menoreh-700 text-white border-menoreh-700 hover:bg-menoreh-800'
    : 'bg-white text-arang-700 border-krem-300 hover:border-menoreh-500 hover:text-menoreh-700';
</script>

<button
  type="button"
  class="{base} {state} {cls}"
  aria-pressed={active}
  {disabled}
  on:click
>
  <span>{label}</span>
  {#if count !== undefined}
    <span
      class="tnum text-xs {active ? 'text-menoreh-100' : 'text-arang-500'}"
    >
      {count}
    </span>
  {/if}
</button>

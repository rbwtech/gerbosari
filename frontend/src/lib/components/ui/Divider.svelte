<script lang="ts">
  /** Optional centered text label. When omitted, renders a plain thin rule. */
  export let label: string | undefined = undefined;

  let cls = '';
  export { cls as class };
</script>

{#if label}
  <div class="flex items-center gap-4 {cls}" role="separator" aria-label={label}>
    <span class="h-px flex-1 bg-krem-300"></span>
    <span class="inline-flex items-center gap-2">
      <span class="w-1.5 h-1.5 rounded-full bg-terakota-500" aria-hidden="true"></span>
      <span class="text-xs font-medium uppercase tracking-widest text-arang-500">{label}</span>
      <span class="w-1.5 h-1.5 rounded-full bg-terakota-500" aria-hidden="true"></span>
    </span>
    <span class="h-px flex-1 bg-krem-300"></span>
  </div>
{:else}
  <hr class="border-0 border-t border-krem-300 {cls}" />
{/if}

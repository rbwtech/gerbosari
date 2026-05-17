<script lang="ts">
  /**
   * Sizing is controlled entirely by the `class` prop so callers can match
   * the exact shape of the loading content (e.g. 'h-4 w-32', 'h-48 w-full').
   */
  let cls = '';
  export { cls as class };
</script>

<div
  class="animate-pulse bg-krem-200 rounded {cls}"
  aria-hidden="true"
></div>

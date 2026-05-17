<script lang="ts">
  /**
   * Thin wrapper that applies `.prose-village` typography to long-form
   * content blocks. Use for sejarah, legenda, berita detail bodies.
   */
  let cls = '';
  export { cls as class };
</script>

<div class="prose-village {cls}">
  <slot />
</div>

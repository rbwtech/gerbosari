<script lang="ts">
  /**
   * Reusable search primitive for admin list pages. Wraps a native `type=search`
   * input with a leading magnifier glyph, a trailing clear affordance, and a
   * debounce-aware event surface so consumers can drive remote filtering
   * without re-implementing timer bookkeeping on each list view.
   *
   * Emits three discrete events:
   *  - `input`           native bubbled event for raw keystroke wiring
   *  - `debouncedinput`  CustomEvent<{ value: string }> after the debounce window
   *  - `clear`           dispatched when the inline clear button is invoked
   */
  import { createEventDispatcher } from 'svelte';
  import { Search, X } from '$lib/components/icons';

  export let value = '';
  export let placeholder = 'Cari...';
  export let ariaLabel = 'Pencarian';
  /** Debounce window in milliseconds for the `debouncedinput` event. */
  export let debounce = 150;
  export let disabled = false;

  // `class` is a reserved word in module scope, so alias via `export { ... as class }`
  // to keep the consumer-facing API ergonomic (`<SearchInput class="..." />`).
  let cls = '';
  export { cls as class };

  const dispatch = createEventDispatcher<{
    debouncedinput: { value: string };
    clear: void;
  }>();

  let timer: ReturnType<typeof setTimeout> | undefined;

  function handleInput(_e: Event) {
    if (timer) clearTimeout(timer);
    // Capture value at dispatch-schedule time. Trim on emit so downstream
    // query builders never receive trailing whitespace from sloppy paste.
    const v = value;
    timer = setTimeout(() => {
      dispatch('debouncedinput', { value: v.trim() });
    }, debounce);
  }

  function handleClear() {
    value = '';
    if (timer) clearTimeout(timer);
    // Emit synchronously so consumers can reset their query state immediately.
    dispatch('debouncedinput', { value: '' });
    dispatch('clear');
  }
</script>

<label class="relative block {cls}">
  <span class="sr-only">{ariaLabel}</span>
  <Search
    class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-arang-400"
    strokeWidth={1.75}
    aria-hidden="true"
  />
  <input
    type="search"
    {placeholder}
    {disabled}
    bind:value
    on:input={handleInput}
    on:input
    autocomplete="off"
    spellcheck="false"
    class="w-full min-h-11 pl-10 pr-10 rounded-md border border-krem-300 bg-white
           text-base md:text-sm text-arang-900 placeholder:text-arang-400
           focus:outline-none focus:border-menoreh-500 focus:ring-2 focus:ring-menoreh-500/20
           disabled:bg-krem-50 disabled:cursor-not-allowed
           transition-colors duration-200 ease-out"
  />
  {#if value}
    <button
      type="button"
      on:click={handleClear}
      class="absolute right-2 top-1/2 -translate-y-1/2 inline-flex items-center justify-center
             h-7 w-7 rounded-md text-arang-500 hover:text-arang-900 hover:bg-krem-100
             transition-colors duration-200 ease-out"
      aria-label="Hapus pencarian"
    >
      <X class="h-4 w-4" strokeWidth={2} aria-hidden="true" />
    </button>
  {/if}
</label>

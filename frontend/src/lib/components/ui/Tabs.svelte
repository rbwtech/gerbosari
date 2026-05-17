<script lang="ts">
  import { createEventDispatcher, tick } from 'svelte';

  interface Tab {
    id: string;
    label: string;
  }

  /** List of tabs to render - stable identity by `id`. */
  export let tabs: Tab[] = [];
  /** Active tab id. Controlled - the parent updates this from the change event. */
  export let value: string;
  /** Optional aria label for the tablist when no visible heading is nearby. */
  export let ariaLabel: string | undefined = undefined;

  let cls = '';
  export { cls as class };

  const dispatch = createEventDispatcher<{ change: { value: string } }>();

  let tabRefs: HTMLButtonElement[] = [];

  function select(id: string) {
    if (id === value) return;
    value = id;
    dispatch('change', { value: id });
  }

  // Arrow-key roving focus per WAI-ARIA Authoring Practices for tabs.
  async function onKeydown(event: KeyboardEvent, index: number) {
    const key = event.key;
    const last = tabs.length - 1;
    let target = index;

    if (key === 'ArrowRight') target = index === last ? 0 : index + 1;
    else if (key === 'ArrowLeft') target = index === 0 ? last : index - 1;
    else if (key === 'Home') target = 0;
    else if (key === 'End') target = last;
    else return;

    event.preventDefault();
    select(tabs[target].id);
    await tick();
    tabRefs[target]?.focus();
  }
</script>

<!--
  Mobile fallback: when tab labels exceed the viewport width, swipe to scroll
  horizontally. Negative inset margins on small screens let the scroll runway
  flush to the page edge while keeping desktop alignment unchanged.
-->
<div
  role="tablist"
  aria-label={ariaLabel}
  class="flex items-center gap-1 border-b border-krem-300 overflow-x-auto -mx-4 px-4 sm:mx-0 sm:px-0 sm:overflow-visible {cls}"
>
  {#each tabs as tab, i (tab.id)}
    {@const active = tab.id === value}
    <button
      bind:this={tabRefs[i]}
      role="tab"
      type="button"
      id={`tab-${tab.id}`}
      aria-selected={active}
      aria-controls={`panel-${tab.id}`}
      tabindex={active ? 0 : -1}
      class="relative shrink-0 px-4 h-11 min-h-11 text-sm font-medium transition-colors duration-200 ease-out
             {active ? 'text-menoreh-800' : 'text-arang-700 hover:text-menoreh-700'}"
      on:click={() => select(tab.id)}
      on:keydown={(e) => onKeydown(e, i)}
    >
      {tab.label}
      {#if active}
        <span
          class="absolute left-3 right-3 -bottom-px h-0.5 bg-menoreh-700 rounded-full"
          aria-hidden="true"
        ></span>
      {/if}
    </button>
  {/each}
</div>

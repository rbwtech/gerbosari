<script context="module" lang="ts">
  /** Supported field types - kept in module scope so it can be re-exported. */
  export type FieldType = 'text' | 'number' | 'date' | 'select' | 'textarea';
</script>

<script lang="ts">
  /**
   * Labeled form input with consistent styling, accessibility, and inline
   * error rendering. Two-way binding via `bind:value` mirrors native input
   * semantics so consumers can treat the component as a transparent control.
   */
  export let label: string;
  export let name: string;
  export let value: string | number | null | undefined = '';
  export let type: FieldType = 'text';
  export let required = false;
  export let options: { value: string | number; label: string }[] = [];
  export let error: string | undefined = undefined;
  export let placeholder = '';
  export let rows = 4;
  /** Optional helper line shown when there is no error. */
  export let hint: string | undefined = undefined;
  /** Pass through native attributes for number/date constraints. */
  export let min: number | string | undefined = undefined;
  export let max: number | string | undefined = undefined;
  export let step: number | string | undefined = undefined;
  /** Inline pattern hint for slug fields etc. - HTML5 pattern attr. */
  export let pattern: string | undefined = undefined;
  /** Marks the input read-only without disabling form submission. */
  export let readonly = false;
  /** Adds a monospace hint suitable for slug / markdown / path inputs. */
  export let mono = false;
  /**
   * Opt-in tall treatment for long-form textareas (markdown editors). Enforces
   * a generous min-height that scales with breakpoint so mobile editing of
   * berita konten remains comfortable without forcing manual resize.
   */
  export let tall = false;

  // The describedby id is unique per instance so multiple FormFields can
  // coexist without colliding aria attributes.
  const uid = `ff-${Math.random().toString(36).slice(2, 9)}`;
  $: descId = error ? `${uid}-err` : hint ? `${uid}-hint` : undefined;

  // Tailwind class composition. Errored state swaps the ring + border color.
  // text-base (16px) on mobile is mandatory to prevent iOS Safari from
  // auto-zooming the viewport when an input gains focus. min-h-11 keeps every
  // control at the >=44px touch target threshold.
  const baseInput =
    'w-full rounded-md border bg-white px-3 py-2 min-h-11 text-base md:text-sm text-arang-900 ' +
    'placeholder:text-arang-400 transition-colors duration-150 ' +
    'focus:outline-none focus:ring-2 focus:ring-offset-1 ' +
    'disabled:bg-krem-100 disabled:text-arang-500 disabled:cursor-not-allowed ' +
    'read-only:bg-krem-50 read-only:text-arang-700';

  $: stateInput = error
    ? 'border-terakota-500 focus:border-terakota-600 focus:ring-terakota-500'
    : 'border-krem-300 focus:border-menoreh-600 focus:ring-menoreh-500';

  // Monospace tier keeps the readable size on mobile - drops only on md+.
  $: monoCls = mono ? 'font-mono text-sm md:text-[13px]' : '';
  $: composed = `${baseInput} ${stateInput} ${monoCls}`;
</script>

<div class="flex flex-col">
  <label for={uid} class="block text-sm font-medium text-arang-800 mb-1.5">
    {label}
    {#if required}
      <span class="text-terakota-500" aria-hidden="true">*</span>
    {/if}
  </label>

  {#if type === 'textarea'}
    <textarea
      id={uid}
      {name}
      {placeholder}
      {rows}
      {required}
      {readonly}
      aria-invalid={error ? 'true' : undefined}
      aria-describedby={descId}
      class="{composed} h-auto resize-y leading-relaxed {tall ? 'min-h-64 md:min-h-96' : ''}"
      bind:value
    ></textarea>
  {:else if type === 'select'}
    <select
      id={uid}
      {name}
      {required}
      aria-invalid={error ? 'true' : undefined}
      aria-describedby={descId}
      class={composed}
      bind:value
    >
      {#if placeholder}
        <option value="" disabled>{placeholder}</option>
      {/if}
      {#each options as opt (opt.value)}
        <option value={opt.value}>{opt.label}</option>
      {/each}
    </select>
  {:else if type === 'number'}
    <input
      id={uid}
      type="number"
      {name}
      {placeholder}
      {required}
      {readonly}
      {min}
      {max}
      {step}
      aria-invalid={error ? 'true' : undefined}
      aria-describedby={descId}
      class={composed}
      bind:value
    />
  {:else if type === 'date'}
    <input
      id={uid}
      type="date"
      {name}
      {required}
      {readonly}
      {min}
      {max}
      aria-invalid={error ? 'true' : undefined}
      aria-describedby={descId}
      class={composed}
      bind:value
    />
  {:else}
    <input
      id={uid}
      type="text"
      {name}
      {placeholder}
      {required}
      {readonly}
      {pattern}
      aria-invalid={error ? 'true' : undefined}
      aria-describedby={descId}
      class={composed}
      bind:value
    />
  {/if}

  {#if error}
    <p id="{uid}-err" class="mt-1.5 text-xs text-terakota-700" role="alert">{error}</p>
  {:else if hint}
    <p id="{uid}-hint" class="mt-1.5 text-xs text-arang-500">{hint}</p>
  {/if}
</div>

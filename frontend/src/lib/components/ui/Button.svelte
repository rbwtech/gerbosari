<script lang="ts">
  import { Loader2 } from '../icons';

  /** Visual treatment. `danger` reserved for destructive actions (future use). */
  export let variant: 'primary' | 'secondary' | 'ghost' | 'danger' = 'primary';
  /** Height tier. sm = inline / table actions, md = default, lg = hero CTA. */
  export let size: 'sm' | 'md' | 'lg' = 'md';
  /** When provided, renders an anchor instead of a button. */
  export let href: string | undefined = undefined;
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let disabled = false;
  /** Renders inline spinner and blocks interaction without changing layout. */
  export let loading = false;
  /** Optional anchor target — only applied when `href` is set. */
  export let target: string | undefined = undefined;
  export let rel: string | undefined = undefined;

  let cls = '';
  export { cls as class };

  const base =
    'inline-flex items-center justify-center gap-2 font-medium rounded-md ' +
    'transition-colors duration-200 ease-out ' +
    'disabled:opacity-50 disabled:cursor-not-allowed select-none';

  const variants: Record<string, string> = {
    primary:
      'bg-menoreh-700 text-white hover:bg-menoreh-800 active:bg-menoreh-900',
    secondary:
      'bg-transparent border border-menoreh-700 text-menoreh-700 hover:bg-menoreh-50 active:bg-menoreh-100',
    ghost:
      'text-arang-700 hover:bg-krem-100 active:bg-krem-200',
    danger:
      'bg-terakota-600 text-white hover:bg-terakota-700 active:bg-terakota-800'
  };

  const sizes: Record<string, string> = {
    sm: 'h-8 px-3 text-sm',
    md: 'h-10 px-4 text-sm',
    lg: 'h-12 px-6 text-base'
  };

  $: composed = `${base} ${variants[variant]} ${sizes[size]} ${cls}`;
  $: isDisabled = disabled || loading;
</script>

{#if href}
  <a
    {href}
    {target}
    rel={target === '_blank' ? rel ?? 'noopener noreferrer' : rel}
    class={composed}
    aria-disabled={isDisabled || undefined}
    tabindex={isDisabled ? -1 : 0}
    on:click
  >
    {#if loading}
      <Loader2 class="w-4 h-4 animate-spin" strokeWidth={2} aria-hidden="true" />
    {/if}
    <slot />
  </a>
{:else}
  <button {type} disabled={isDisabled} class={composed} on:click>
    {#if loading}
      <Loader2 class="w-4 h-4 animate-spin" strokeWidth={2} aria-hidden="true" />
    {/if}
    <slot />
  </button>
{/if}

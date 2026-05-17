<script lang="ts">
  import { Inbox, AlertCircle, Loader2 } from '../icons';
  import Button from './Button.svelte';

  export let variant: 'empty' | 'error' | 'loading' = 'empty';
  export let title: string;
  export let description: string | undefined = undefined;
  export let actionLabel: string | undefined = undefined;
  /** Optional callback wired to the action button. */
  export let onAction: (() => void) | undefined = undefined;

  $: role =
    variant === 'error' ? 'alert' : variant === 'loading' ? 'status' : undefined;
</script>

<div
  class="flex flex-col items-center justify-center text-center py-16 px-6 border border-dashed border-krem-300 rounded-lg bg-krem-50 max-w-prose mx-auto"
  {role}
  aria-live={variant === 'loading' ? 'polite' : undefined}
>
  {#if variant === 'error'}
    <AlertCircle class="w-10 h-10 text-terakota-600 mb-4" strokeWidth={1.5} aria-hidden="true" />
  {:else if variant === 'loading'}
    <Loader2 class="w-10 h-10 text-menoreh-600 mb-4 animate-spin" strokeWidth={1.5} aria-hidden="true" />
  {:else}
    <Inbox class="w-10 h-10 text-arang-400 mb-4" strokeWidth={1.5} aria-hidden="true" />
  {/if}

  <h3 class="font-serif text-xl font-semibold text-arang-900">{title}</h3>

  {#if description}
    <p class="mt-2 text-sm text-arang-700 leading-relaxed">{description}</p>
  {/if}

  {#if actionLabel && onAction}
    <div class="mt-6">
      <Button variant="secondary" size="sm" on:click={onAction}>{actionLabel}</Button>
    </div>
  {/if}

  {#if $$slots.default}
    <div class="mt-6">
      <slot />
    </div>
  {/if}
</div>

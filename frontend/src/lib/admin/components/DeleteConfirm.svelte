<script lang="ts">
  import { tick, onDestroy } from 'svelte';
  import Button from '../../components/ui/Button.svelte';
  import { AlertTriangle, X } from '../../components/icons';

  /**
   * Confirmation modal for destructive actions. Trap-focus is intentionally
   * minimal - the confirm button is autofocused so Enter triggers it and
   * Escape always cancels. Backdrop click also cancels.
   */
  export let open = false;
  export let title: string;
  export let description: string;
  export let confirmLabel = 'Hapus';
  export let cancelLabel = 'Batal';
  export let loading = false;
  export let onConfirm: () => void;
  export let onCancel: () => void;

  let dialogRoot: HTMLDivElement | undefined;
  let lastActiveEl: HTMLElement | null = null;

  // Reactively manage body scroll lock + focus restoration on open/close.
  $: void manageOpen(open);

  async function manageOpen(isOpen: boolean) {
    if (typeof document === 'undefined') return;
    if (isOpen) {
      lastActiveEl = document.activeElement as HTMLElement | null;
      document.body.style.overflow = 'hidden';
      await tick();
      // Autofocus the destructive confirm action (last footer button) so
      // Enter triggers delete and Tab cycles between cancel/confirm normally.
      const buttons = dialogRoot?.querySelectorAll<HTMLButtonElement>('footer button, [data-dialog-footer] button');
      const last = buttons?.[buttons.length - 1];
      last?.focus();
    } else {
      document.body.style.overflow = '';
      lastActiveEl?.focus?.();
      lastActiveEl = null;
    }
  }

  onDestroy(() => {
    if (typeof document !== 'undefined') document.body.style.overflow = '';
  });

  function onKey(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      e.preventDefault();
      if (!loading) onCancel();
    }
  }

  function onBackdrop(e: MouseEvent) {
    // Only cancel when clicking the backdrop itself, not the card.
    if (e.target === e.currentTarget && !loading) onCancel();
  }
</script>

<svelte:window on:keydown={onKey} />

{#if open}
  <div
    bind:this={dialogRoot}
    class="fixed inset-0 z-50 flex items-center justify-center px-4 py-8 bg-arang-900/60"
    role="dialog"
    aria-modal="true"
    aria-labelledby="delete-confirm-title"
    aria-describedby="delete-confirm-desc"
    tabindex="-1"
    on:click={onBackdrop}
    on:keydown|self={(e) => {
      // Backdrop keyboard fallback - global keydown handles Escape, but the
      // a11y linter requires a paired handler when click is wired up.
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        if (!loading) onCancel();
      }
    }}
  >
    <div
      class="w-full max-w-md bg-white rounded-lg border border-krem-200 overflow-hidden"
      role="document"
    >
      <div class="flex items-start justify-between gap-3 px-6 pt-6">
        <div class="flex items-start gap-3">
          <div class="flex-none mt-0.5 w-10 h-10 rounded-full bg-terakota-50 flex items-center justify-center">
            <AlertTriangle class="w-5 h-5 text-terakota-600" strokeWidth={2} aria-hidden="true" />
          </div>
          <div>
            <h2 id="delete-confirm-title" class="font-serif text-lg font-semibold text-arang-900">
              {title}
            </h2>
          </div>
        </div>
        <button
          type="button"
          on:click={() => !loading && onCancel()}
          aria-label="Tutup"
          disabled={loading}
          class="flex-none -mr-2 -mt-2 inline-flex h-8 w-8 items-center justify-center rounded-md text-arang-500 hover:bg-krem-100 disabled:opacity-50"
        >
          <X class="w-4 h-4" strokeWidth={2} aria-hidden="true" />
        </button>
      </div>

      <p id="delete-confirm-desc" class="px-6 pt-3 pb-6 text-sm text-arang-700 leading-relaxed">
        {description}
      </p>

      <div
        data-dialog-footer
        class="flex justify-end gap-2 px-6 py-4 bg-krem-50 border-t border-krem-200"
      >
        <Button variant="ghost" size="md" disabled={loading} on:click={onCancel}>
          {cancelLabel}
        </Button>
        <Button variant="danger" size="md" loading={loading} on:click={onConfirm}>
          {confirmLabel}
        </Button>
      </div>
    </div>
  </div>
{/if}

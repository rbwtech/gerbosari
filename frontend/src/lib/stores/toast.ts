import { writable } from 'svelte/store';

export interface ToastMessage {
  id: number;
  tone: 'info' | 'error' | 'success';
  message: string;
}

function createToastStore() {
  const { subscribe, update } = writable<ToastMessage[]>([]);
  let counter = 0;

  function push(tone: ToastMessage['tone'], message: string, timeoutMs = 4500): void {
    const id = ++counter;
    update((list) => [...list, { id, tone, message }]);
    if (timeoutMs > 0) {
      setTimeout(() => dismiss(id), timeoutMs);
    }
  }

  function dismiss(id: number): void {
    update((list) => list.filter((t) => t.id !== id));
  }

  return {
    subscribe,
    info: (msg: string) => push('info', msg),
    error: (msg: string) => push('error', msg),
    success: (msg: string) => push('success', msg),
    dismiss
  };
}

export const toast = createToastStore();

import { writable } from 'svelte/store';

function readHashPath(): string {
  if (typeof window === 'undefined') return '/';
  const raw = window.location.hash.replace(/^#/, '');
  if (!raw || raw === '/') return '/';
  return raw.startsWith('/') ? raw : `/${raw}`;
}

export const location = writable<string>(readHashPath());

if (typeof window !== 'undefined') {
  window.addEventListener('hashchange', () => {
    location.set(readHashPath());
  });
}

export function navigate(path: string): void {
  if (typeof window === 'undefined') return;
  const next = path.startsWith('/') ? path : `/${path}`;
  if (window.location.hash === `#${next}`) {
    // hashchange won't fire if identical - force store update
    location.set(next);
  } else {
    window.location.hash = `#${next}`;
  }
}

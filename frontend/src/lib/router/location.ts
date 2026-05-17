import { writable } from 'svelte/store';

// Hash-router compat shim. If the visitor arrives on a real path like
// `/admin/login` (typed in the address bar, bookmark, external link, etc.),
// rewrite the URL to the hash form BEFORE we read the initial location.
// nginx already falls back to index.html for unknown paths, so we always
// reach this module — the rewrite happens once at module evaluation.
if (typeof window !== 'undefined') {
  const { pathname, search, hash } = window.location;
  if (!hash && pathname && pathname !== '/' && pathname !== '/index.html') {
    history.replaceState(null, '', '/#' + pathname + search);
  }
}

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

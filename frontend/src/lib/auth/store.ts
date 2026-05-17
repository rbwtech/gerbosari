import { derived, get, writable, type Readable } from 'svelte/store';
import type { AdminUser } from '../types';

/**
 * Persistent admin session. The Bearer token, its expiry, and the principal
 * are kept together so `isAuthenticated()` can short-circuit on either missing
 * or expired tokens without a separate clock store.
 *
 * SECURITY: localStorage is readable by any script executing on this origin,
 * so a successful XSS would yield session compromise. Acceptable for the v1
 * cloud-computing assignment scope; harden later with httpOnly cookies +
 * CSRF tokens once we ship a real reverse proxy in front of the Rust API.
 */
export interface AuthState {
  token: string | null;
  expiresAt: string | null;
  user: AdminUser | null;
}

const STORAGE_KEY = 'gerbosari.auth';
const EMPTY: AuthState = { token: null, expiresAt: null, user: null };

function readFromStorage(): AuthState {
  if (typeof window === 'undefined') return EMPTY;
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) return EMPTY;
    const parsed = JSON.parse(raw) as Partial<AuthState>;
    if (
      typeof parsed?.token !== 'string' ||
      typeof parsed?.expiresAt !== 'string' ||
      !parsed?.user ||
      typeof (parsed.user as AdminUser).id !== 'string' ||
      typeof (parsed.user as AdminUser).username !== 'string'
    ) {
      return EMPTY;
    }
    return {
      token: parsed.token,
      expiresAt: parsed.expiresAt,
      user: parsed.user as AdminUser
    };
  } catch {
    // Corrupt payload - drop it so the user is forced through the login flow.
    return EMPTY;
  }
}

function writeToStorage(state: AuthState): void {
  if (typeof window === 'undefined') return;
  if (!state.token) {
    window.localStorage.removeItem(STORAGE_KEY);
    return;
  }
  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
}

/** Internal writable; consumers should subscribe via `authState` (readonly). */
const _authState = writable<AuthState>(readFromStorage());

/** Read-only view of the current auth state for UI subscriptions. */
export const authState: Readable<AuthState> = { subscribe: _authState.subscribe };

/**
 * True when a non-empty token is present AND its expiry is in the future.
 * Exposed as both a pure helper (for guards / interceptors) and a derived
 * store (for reactive template bindings).
 */
export function isAuthenticated(state: AuthState): boolean {
  if (!state.token || !state.expiresAt) return false;
  const expiry = Date.parse(state.expiresAt);
  if (Number.isNaN(expiry)) return false;
  return expiry > Date.now();
}

export const isAuthed: Readable<boolean> = derived(_authState, ($s) => isAuthenticated($s));

/** Persist a fresh login result. Called from the login route on 200 OK. */
export function setAuth(token: string, expiresAt: string, user: AdminUser): void {
  const next: AuthState = { token, expiresAt, user };
  _authState.set(next);
  writeToStorage(next);
}

/** Drop the session. Used by logout, expiry sweep, and 401 interceptor. */
export function clearAuth(): void {
  _authState.set(EMPTY);
  writeToStorage(EMPTY);
}

/** Snapshot helper - avoids `get(authState)` boilerplate in non-Svelte code. */
export function getAuth(): AuthState {
  return get(_authState);
}

// Background expiry sweep. Once the access token's TTL elapses we eagerly
// clear the session so the next render redirects to /admin/login instead of
// letting the user click into a doomed protected page first.
let sweepHandle: ReturnType<typeof setInterval> | null = null;

function startExpirySweep(): void {
  if (typeof window === 'undefined' || sweepHandle !== null) return;
  sweepHandle = setInterval(() => {
    const current = get(_authState);
    if (current.token && !isAuthenticated(current)) {
      clearAuth();
      // Lazy import keeps router out of the store's hard dependency graph.
      import('../router').then(({ navigate, location }) => {
        const path = get(location);
        if (!path.startsWith('/admin/login')) {
          navigate('/admin/login');
        }
      });
    }
  }, 60_000);
}

if (typeof window !== 'undefined') {
  startExpirySweep();
}

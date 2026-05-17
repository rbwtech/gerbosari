import { navigate } from '../router';
import { getAuth, isAuthenticated } from './store';

/**
 * Imperative auth guard for admin pages. Call from `onMount` and short-circuit
 * any data fetching when it returns false:
 *
 *   onMount(() => {
 *     if (!requireAuth()) return;
 *     // ...load protected data
 *   });
 *
 * Returns true when the session is valid; otherwise navigates to /admin/login
 * and returns false so the caller can bail.
 */
export function requireAuth(): boolean {
  const state = getAuth();
  if (isAuthenticated(state)) return true;
  navigate('/admin/login');
  return false;
}

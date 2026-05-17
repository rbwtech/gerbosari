/**
 * Barrel for the admin auth module. Importers should never reach into the
 * individual files - keeping a single entry point lets us refactor the
 * persistence layer (cookies, IndexedDB) without rippling through routes.
 */
export {
  authState,
  isAuthed,
  setAuth,
  clearAuth,
  getAuth,
  isAuthenticated,
  type AuthState
} from './store';

export {
  apiAuthGet,
  apiAuthPost,
  apiAuthPatch,
  apiAuthDelete
} from './client';

export { requireAuth } from './guard';

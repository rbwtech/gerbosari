import type { ApiError } from '../types';
import type { RequestOptions } from '../api/client';
import { clearAuth, getAuth } from './store';

const BASE_URL = import.meta.env.VITE_API_BASE ?? '/api';

function joinUrl(path: string): string {
  const trimmedBase = BASE_URL.replace(/\/$/, '');
  const normalizedPath = path.startsWith('/') ? path : `/${path}`;
  return `${trimmedBase}${normalizedPath}`;
}

async function parseError(response: Response): Promise<ApiError> {
  let detail: string | undefined;
  try {
    const data = await response.json();
    detail = typeof data?.message === 'string' ? data.message : undefined;
  } catch {
    detail = await response.text().catch(() => undefined);
  }
  return {
    status: response.status,
    message: response.statusText || 'Permintaan gagal',
    detail
  };
}

/**
 * Centralised 401 handler. Clears the local session and bounces the user to
 * the login screen so a stale token cannot keep a protected page mounted.
 * Router is lazy-imported to avoid a cycle with the store.
 */
async function handleUnauthorized(): Promise<void> {
  clearAuth();
  if (typeof window === 'undefined') return;
  const { navigate, location } = await import('../router');
  const { get } = await import('svelte/store');
  const path = get(location);
  if (!path.startsWith('/admin/login')) {
    navigate('/admin/login');
  }
}

interface RawRequestInit {
  method: 'GET' | 'POST' | 'PATCH' | 'DELETE';
  body?: unknown;
  options: RequestOptions;
}

async function authenticatedRequest<TRes>(path: string, init: RawRequestInit): Promise<TRes> {
  const { token } = getAuth();
  if (!token) {
    // Treat a missing token like a 401 - the user must be redirected before
    // any protected endpoint can leak a response.
    await handleUnauthorized();
    const err: ApiError = {
      status: 401,
      message: 'Unauthorized',
      detail: 'Sesi telah berakhir. Silakan masuk kembali.'
    };
    throw err;
  }

  const joined = joinUrl(path);
  const url = joined.startsWith('http')
    ? new URL(joined)
    : new URL(joined, window.location.origin);
  if (init.options.query) {
    for (const [key, value] of Object.entries(init.options.query)) {
      if (value !== undefined && value !== null && value !== '') {
        url.searchParams.set(key, String(value));
      }
    }
  }

  const headers: Record<string, string> = {
    Accept: 'application/json',
    Authorization: `Bearer ${token}`
  };
  let body: BodyInit | undefined;
  if (init.body !== undefined) {
    headers['Content-Type'] = 'application/json';
    body = JSON.stringify(init.body);
  }

  const response = await fetch(url.toString(), {
    method: init.method,
    headers,
    body,
    signal: init.options.signal
  });

  if (response.status === 401) {
    await handleUnauthorized();
    throw await parseError(response);
  }

  if (!response.ok) {
    throw await parseError(response);
  }

  // 204 No Content is a legal success - return undefined cast to TRes so
  // delete/patch handlers don't blow up parsing an empty body as JSON.
  if (response.status === 204) {
    return undefined as TRes;
  }

  const text = await response.text();
  if (!text) return undefined as TRes;
  return JSON.parse(text) as TRes;
}

export function apiAuthGet<TRes>(path: string, options: RequestOptions = {}): Promise<TRes> {
  return authenticatedRequest<TRes>(path, { method: 'GET', options });
}

export function apiAuthPost<TReq, TRes>(
  path: string,
  body: TReq,
  options: RequestOptions = {}
): Promise<TRes> {
  return authenticatedRequest<TRes>(path, { method: 'POST', body, options });
}

export function apiAuthPatch<TReq, TRes>(
  path: string,
  body: TReq,
  options: RequestOptions = {}
): Promise<TRes> {
  return authenticatedRequest<TRes>(path, { method: 'PATCH', body, options });
}

export function apiAuthDelete<TRes = void>(
  path: string,
  options: RequestOptions = {}
): Promise<TRes> {
  return authenticatedRequest<TRes>(path, { method: 'DELETE', options });
}

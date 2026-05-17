import type { ApiError } from '../types';

const BASE_URL = import.meta.env.VITE_API_BASE ?? '/api';

/**
 * Strip trailing slash so callers can use leading-slash paths without doubling.
 */
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

export interface RequestOptions {
  signal?: AbortSignal;
  query?: Record<string, string | number | boolean | undefined>;
}

/**
 * Typed fetch wrapper. Throws ApiError on non-2xx so route components
 * can render an explicit error UI instead of guessing on undefined.
 */
export async function apiGet<T>(path: string, options: RequestOptions = {}): Promise<T> {
  // Relative base ('/api') needs window.location as the base; absolute base
  // (e.g. http://localhost:3000/api in dev) is used as-is.
  const joined = joinUrl(path);
  const url = joined.startsWith('http')
    ? new URL(joined)
    : new URL(joined, window.location.origin);
  if (options.query) {
    for (const [key, value] of Object.entries(options.query)) {
      if (value !== undefined && value !== null && value !== '') {
        url.searchParams.set(key, String(value));
      }
    }
  }

  const response = await fetch(url.toString(), {
    method: 'GET',
    headers: { Accept: 'application/json' },
    signal: options.signal
  });

  if (!response.ok) {
    throw await parseError(response);
  }

  return (await response.json()) as T;
}

export function isApiError(value: unknown): value is ApiError {
  return (
    typeof value === 'object' &&
    value !== null &&
    'status' in value &&
    'message' in value
  );
}

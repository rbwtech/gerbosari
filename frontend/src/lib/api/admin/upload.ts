/**
 * Admin image upload client.
 *
 * Bypasses `apiAuth*` because those helpers always set
 * `Content-Type: application/json` and stringify the body — for multipart
 * uploads we need the browser to set its own `multipart/form-data; boundary=…`
 * header and to stream the raw `FormData`. We still reuse the same auth token
 * and 401 redirect semantics as the JSON path.
 */
import { get } from 'svelte/store';
import type { ApiError } from '../../types';
import { clearAuth, getAuth } from '../../auth/store';
import { navigate, location } from '../../router';

const BASE_URL = import.meta.env.VITE_API_BASE ?? '/api';

export type UploadEntity = 'gallery' | 'berita';

export interface UploadResponse {
  /** Public URL ready to drop into the entity's image column. */
  url: string;
  /** Bytes written, surfaced for UI confirmation. */
  size: number;
  /** UUID-derived on-disk filename. */
  filename: string;
}

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
    message: response.statusText || 'Unggahan gagal',
    detail
  };
}

function handleUnauthorized(): void {
  clearAuth();
  if (typeof window === 'undefined') return;
  const path = get(location);
  if (!path.startsWith('/admin/login')) navigate('/admin/login');
}

/**
 * Uploads a single image to `POST /api/admin/upload`. Resolves with the
 * server-side URL the SPA should persist into the entity row. Rejects with
 * `ApiError` on any non-2xx so callers can render `err.detail ?? err.message`.
 */
export async function uploadImage(
  file: File,
  entity: UploadEntity,
  options: { signal?: AbortSignal } = {}
): Promise<UploadResponse> {
  const { token } = getAuth();
  if (!token) {
    handleUnauthorized();
    const err: ApiError = {
      status: 401,
      message: 'Unauthorized',
      detail: 'Sesi telah berakhir. Silakan masuk kembali.'
    };
    throw err;
  }

  const form = new FormData();
  form.append('entity', entity);
  form.append('file', file, file.name);

  const url = joinUrl('/admin/upload');
  const response = await fetch(url, {
    method: 'POST',
    headers: {
      // Do NOT set Content-Type — the browser must add the multipart boundary.
      Accept: 'application/json',
      Authorization: `Bearer ${token}`
    },
    body: form,
    signal: options.signal
  });

  if (response.status === 401) {
    handleUnauthorized();
    throw await parseError(response);
  }
  if (!response.ok) throw await parseError(response);
  return (await response.json()) as UploadResponse;
}

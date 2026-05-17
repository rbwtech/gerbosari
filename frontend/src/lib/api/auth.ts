import type { ApiError, AuthToken } from '../types';

const BASE_URL = import.meta.env.VITE_API_BASE ?? '/api';

function joinUrl(path: string): string {
  const trimmedBase = BASE_URL.replace(/\/$/, '');
  const normalizedPath = path.startsWith('/') ? path : `/${path}`;
  return `${trimmedBase}${normalizedPath}`;
}

/**
 * Exchange credentials for a Bearer token. Maps backend error shapes to
 * Indonesian-language `ApiError` payloads so callers can render copy directly
 * without re-translating on every screen.
 */
export async function login(username: string, password: string): Promise<AuthToken> {
  const joined = joinUrl('/auth/login');
  const url = joined.startsWith('http')
    ? joined
    : new URL(joined, window.location.origin).toString();

  let response: Response;
  try {
    response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Accept: 'application/json'
      },
      body: JSON.stringify({ username, password })
    });
  } catch {
    // fetch only throws for true network failures (DNS, CORS preflight, offline).
    const err: ApiError = {
      status: 0,
      message: 'Tidak dapat terhubung ke server',
      detail: 'Periksa koneksi internet Anda lalu coba lagi.'
    };
    throw err;
  }

  if (response.status === 401) {
    const err: ApiError = {
      status: 401,
      message: 'Kredensial tidak valid',
      detail: 'Username atau password salah.'
    };
    throw err;
  }

  if (!response.ok) {
    let detail: string | undefined;
    try {
      const data = await response.json();
      detail = typeof data?.message === 'string' ? data.message : undefined;
    } catch {
      detail = undefined;
    }
    const err: ApiError = {
      status: response.status,
      message: 'Login gagal',
      detail: detail ?? 'Terjadi kesalahan pada server. Silakan coba lagi.'
    };
    throw err;
  }

  return (await response.json()) as AuthToken;
}

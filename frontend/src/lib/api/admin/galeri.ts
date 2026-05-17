/**
 * Admin API client for Galeri (photo gallery) CRUD.
 * All mutating calls require a Bearer token - parallel agent 4 owns the
 * `apiAuth*` helpers in `$lib/auth/client` which attach the token from the
 * auth store and surface `ApiError` on non-2xx responses.
 */
import { apiAuthPost, apiAuthPatch, apiAuthDelete } from '../../auth/client';
import type { Galeri } from '../../types';

export interface CreateGaleriBody {
  judul: string;
  deskripsi?: string | null;
  file_path: string;
  kategori: string;
  taken_at?: string | null;
}

export type UpdateGaleriBody = Partial<CreateGaleriBody>;

export function createGaleri(body: CreateGaleriBody): Promise<Galeri> {
  return apiAuthPost<CreateGaleriBody, Galeri>('/admin/galeri', body);
}

export function updateGaleri(id: string, body: UpdateGaleriBody): Promise<Galeri> {
  return apiAuthPatch<UpdateGaleriBody, Galeri>(`/admin/galeri/${encodeURIComponent(id)}`, body);
}

export function deleteGaleri(id: string): Promise<void> {
  return apiAuthDelete<void>(`/admin/galeri/${encodeURIComponent(id)}`);
}

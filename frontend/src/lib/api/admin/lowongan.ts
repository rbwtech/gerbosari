/**
 * Admin API client for Lowongan (job postings) CRUD.
 * Gaji fields are nullable - backend treats null/undefined as "tidak disebutkan".
 */
import { apiAuthPost, apiAuthPatch, apiAuthDelete } from '../../auth/client';
import type { Lowongan } from '../../types';

export interface CreateLowonganBody {
  judul: string;
  instansi: string;
  kategori: string;
  deskripsi: string;
  kontak: string;
  gaji_min?: number | null;
  gaji_max?: number | null;
  deadline?: string | null;
  lokasi_pedukuhan?: string | null;
  status: string;
}

export type UpdateLowonganBody = Partial<CreateLowonganBody>;

export function createLowongan(body: CreateLowonganBody): Promise<Lowongan> {
  return apiAuthPost<CreateLowonganBody, Lowongan>('/admin/lowongan', body);
}

export function updateLowongan(id: string, body: UpdateLowonganBody): Promise<Lowongan> {
  return apiAuthPatch<UpdateLowonganBody, Lowongan>(`/admin/lowongan/${encodeURIComponent(id)}`, body);
}

export function deleteLowongan(id: string): Promise<void> {
  return apiAuthDelete<void>(`/admin/lowongan/${encodeURIComponent(id)}`);
}

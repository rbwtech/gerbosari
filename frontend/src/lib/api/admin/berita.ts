/**
 * Admin API client for Berita / Agenda CRUD.
 * Identity is the URL-safe slug, not a numeric id - the slug is part of the
 * public route (#/berita/:slug) so it must remain stable across edits.
 */
import { apiAuthPost, apiAuthPatch, apiAuthDelete } from '../../auth/client';
import type { Berita } from '../../types';

export interface CreateBeritaBody {
  judul: string;
  slug: string;
  kategori: string;
  ringkasan: string;
  konten: string;
  gambar_url?: string | null;
  tanggal?: string | null;
  lokasi?: string | null;
  author: string;
  published_at?: string | null;
}

export type UpdateBeritaBody = Partial<CreateBeritaBody>;

export function createBerita(body: CreateBeritaBody): Promise<Berita> {
  return apiAuthPost<CreateBeritaBody, Berita>('/admin/berita', body);
}

export function updateBerita(slug: string, body: UpdateBeritaBody): Promise<Berita> {
  return apiAuthPatch<UpdateBeritaBody, Berita>(`/admin/berita/${encodeURIComponent(slug)}`, body);
}

export function deleteBerita(slug: string): Promise<void> {
  return apiAuthDelete<void>(`/admin/berita/${encodeURIComponent(slug)}`);
}

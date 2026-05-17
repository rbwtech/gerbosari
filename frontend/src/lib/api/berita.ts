import { apiGet, type RequestOptions } from './client';
import type { Berita } from '../types';

export interface BeritaQuery {
  kategori?: string;
}

export function listBerita(query: BeritaQuery = {}, options: RequestOptions = {}): Promise<Berita[]> {
  return apiGet<Berita[]>('/berita', { ...options, query });
}

export function getBerita(slug: string, options: RequestOptions = {}): Promise<Berita> {
  return apiGet<Berita>(`/berita/${encodeURIComponent(slug)}`, options);
}

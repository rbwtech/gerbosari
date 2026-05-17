import { apiGet, type RequestOptions } from './client';
import type { Galeri } from '../types';

export interface GaleriQuery {
  kategori?: string;
  limit?: number;
}

export function listGaleri(query: GaleriQuery = {}, options: RequestOptions = {}): Promise<Galeri[]> {
  return apiGet<Galeri[]>('/galeri', { ...options, query });
}

export function getGaleri(id: string, options: RequestOptions = {}): Promise<Galeri> {
  return apiGet<Galeri>(`/galeri/${encodeURIComponent(id)}`, options);
}

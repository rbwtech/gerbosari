import { apiGet, type RequestOptions } from './client';
import type { Lowongan } from '../types';

export interface LowonganQuery {
  kategori?: string;
}

export function listLowongan(query: LowonganQuery = {}, options: RequestOptions = {}): Promise<Lowongan[]> {
  return apiGet<Lowongan[]>('/lowongan', { ...options, query });
}

export function getLowongan(id: string, options: RequestOptions = {}): Promise<Lowongan> {
  return apiGet<Lowongan>(`/lowongan/${encodeURIComponent(id)}`, options);
}

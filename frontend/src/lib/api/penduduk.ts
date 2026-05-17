import { apiGet, type RequestOptions } from './client';
import type { PendudukRingkasan } from '../types';

export function getRingkasan(options: RequestOptions = {}): Promise<PendudukRingkasan> {
  return apiGet<PendudukRingkasan>('/penduduk/ringkasan', options);
}

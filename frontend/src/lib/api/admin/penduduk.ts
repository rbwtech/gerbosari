/**
 * Admin API client for Penduduk (population) per-pedukuhan updates.
 * Only PATCH is exposed - the 19 pedukuhan rows are seeded by migration so
 * neither create nor delete is part of the admin surface.
 */
import { apiAuthPatch } from '../../auth/client';
import type { PendudukPedukuhan } from '../../types';

export interface UpdatePendudukBody {
  jumlah_kk?: number | null;
  laki?: number | null;
  perempuan?: number | null;
}

export function updatePedukuhan(pedukuhan: string, body: UpdatePendudukBody): Promise<PendudukPedukuhan> {
  return apiAuthPatch<UpdatePendudukBody, PendudukPedukuhan>(
    `/admin/penduduk/${encodeURIComponent(pedukuhan)}`,
    body
  );
}

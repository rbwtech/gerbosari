/**
 * Shared DTOs mirroring the Rust backend's serde-serialized response shape.
 * Field names are kept verbatim - do not rename without updating the backend
 * `presentation/dto/*` modules and the consuming routes in lockstep.
 */

export interface Galeri {
  id: string;
  judul: string;
  deskripsi: string | null;
  file_path: string;
  kategori: 'wisata' | 'budaya' | 'agrowisata' | 'kegiatan' | string;
  taken_at: string | null;
  created_at: string;
}

export interface PendudukPedukuhan {
  pedukuhan: string;
  total_penduduk: number;
  total_laki: number;
  total_perempuan: number;
  total_kk: number;
  updated_at: string;
}

export interface PendudukRingkasan {
  total_penduduk: number;
  total_laki: number;
  total_perempuan: number;
  total_kk: number;
  per_pedukuhan: PendudukPedukuhan[];
}

export interface Lowongan {
  id: string;
  judul: string;
  instansi: string;
  kategori: 'umkm' | 'formal' | 'freelance' | string;
  deskripsi: string;
  kontak: string;
  gaji_min: number | null;
  gaji_max: number | null;
  deadline: string | null;
  lokasi_pedukuhan: string | null;
  status: 'aktif' | 'tutup' | 'arsip' | string;
  created_at: string;
}

export interface BeritaRingkasan {
  id: string;
  judul: string;
  slug: string;
  kategori: 'berita' | 'agenda' | string;
  ringkasan: string;
  gambar_url: string | null;
  tanggal: string | null;
  author: string;
  published_at: string;
}

export interface Berita extends BeritaRingkasan {
  konten: string;
}

export interface ApiError {
  status: number;
  message: string;
  detail?: string;
}

/**
 * Authenticated admin principal. Mirrors the backend's `presentation/dto/auth.rs`
 * AdminUserDto - `id` is a UUID string, `username` is lowercase alphanumeric.
 */
export interface AdminUser {
  id: string;
  username: string;
}

/**
 * Response payload from POST /api/auth/login. `expires_at` is an ISO-8601
 * timestamp; the auth store evicts the token client-side once it elapses so the
 * UI redirects to /admin/login before the backend issues a 401.
 */
export interface AuthToken {
  token: string;
  expires_at: string;
  user: AdminUser;
}

export interface Beranda {
  tagline: string;
  deskripsi_singkat: string;
  statistik: {
    luas_ha: number;
    penduduk_total: number;
    pedukuhan_count: number;
    rt_count: number;
    rw_count: number;
  };
}

export interface Sejarah {
  asal_usul: string;
  etimologi: {
    ger: string;
    bo: string;
    sa: string;
    ri: string;
  };
  peristiwa_1949: {
    judul: string;
    narator: string;
    isi: string;
  };
  legenda: Array<{ nama: string; isi: string }>;
  pemerintahan: Array<{ periode: string; pemimpin: string; jabatan: string }>;
}

export interface VisiMisi {
  visi_desa: string;
  misi_desa: string[];
  visi_kabupaten: string;
  misi_kabupaten: string[];
}

export interface StrukturOrganisasi {
  kepala_desa: string;
  sekretaris: string;
  kasi: Array<{ jabatan: string; nama: string }>;
  kadus: Array<{ pedukuhan: string; nama: string }>;
}

export interface PetaWilayah {
  koordinat: { lat: number; lon: number };
  luas_ha: number;
  ketinggian_min: number;
  ketinggian_max: number;
  suhu_rata_celsius: number;
  batas: { utara: string; timur: string; selatan: string; barat: string };
  pedukuhan: Array<{ nama: string; zona: string; deskripsi: string }>;
  zona: Array<{ nomor: number; tema: string; lokasi: string[] }>;
}

# content/

Single source of truth untuk seluruh konten Website Profil Desa Gerbosari (Kecamatan Samigaluh, Kabupaten Kulon Progo). Konten di sini diekstrak dari dokumen profil desa (`Desa Gerbosari.docx`), jurnal pengabdian UGM (Devi & Hidayati, 2020), serta situs lama `gerbosari-kulonprogo.desa.id`.

## Struktur

```
content/
├── pages/                     # Konten statis halaman utama (frontend baca langsung)
│   ├── beranda.json           # Hero, alamat, quick stats, highlights
│   ├── sejarah.json           # Asal usul, etimologi, peristiwa 1949, 6 legenda, pemerintahan
│   ├── visi-misi.json         # Visi-misi desa (inferred dari tagline) + Kabupaten
│   ├── struktur-organisasi.json   # Kepala desa, kasi, kaur, kadus (kebanyakan TBD)
│   └── peta-wilayah.json      # Koordinat, luas, ketinggian, batas, 19 pedukuhan, 4 zona
├── seed/                      # Data awal untuk database (backend seed)
│   ├── galeri.json            # 12 entri foto kegiatan/wisata/budaya/agrowisata
│   ├── penduduk-ringkasan.json # 19 pedukuhan, KK + L/P (jumlah = 4.802 jiwa)
│   ├── lowongan.json          # 6 lowongan sampel terikat ekonomi Gerbosari
│   └── berita.json            # 5 entri campuran berita & agenda
└── images/
    ├── gallery/               # 12 foto kegiatan dengan nama deskriptif
    ├── cover/                 # 2 foto hero landscape
    └── peta/                  # 3 PNG peta (administrasi, zona, pedukuhan)
```

## Audiens & alur

- **Frontend (svelte/vite)** mengimpor JSON di `pages/` apa adanya untuk render halaman statis. File peta dan cover dipakai sebagai aset background dan section image.
- **Backend (Rust + MySQL)** menjalankan one-time seeder yang membaca file di `seed/` dan menulisnya ke tabel `galeri`, `penduduk_pedukuhan`, `lowongan`, dan `berita`. File JSON tetap menjadi sumber otoritatif - bila perlu reset DB, jalankan seeder ulang.
- **Tim deploy** menyalin folder `images/` ke storage statis (S3/CloudFront, atau folder publik web server).

## Cara update

1. **Edit konten kecil (typo, tambah berita)**: edit langsung file JSON, validasi sintaks dengan `jq . file.json` atau editor JSON-aware, lalu commit.
2. **Tambah foto galeri**: letakkan file di `images/gallery/`, tambahkan entri baru di `seed/galeri.json` dengan id berurutan (`gal-013`, dst.) dan jalur relatif dari root repo.
3. **Update data resmi (penduduk, pejabat)**: ganti angka/nama di file terkait. Pertahankan field `_meta` dan `_inferred` agar transparansi sumber tetap terjaga.
4. **Field bertanda `_inferred: true`** harus diverifikasi dengan pihak desa sebelum dianggap final.

## Catatan integritas data

- Total penduduk per pedukuhan (`penduduk-ringkasan.json`) **dikalibrasi presisi** ke agregat resmi: 1.268 KK, 2.380 laki-laki, 2.422 perempuan, 4.802 jiwa. Distribusi per pedukuhan masih estimasi proporsional.
- Visi-misi desa adalah **interpretasi** dari tagline "Sejahtera Mandiri - Desa Wisata Berbasis Budaya dan Ekonomi Kreatif". Dokumen sumber hanya memuat visi-misi Kabupaten Kulon Progo.
- Koordinat di `peta-wilayah.json` mengandung anomali (bujur 116 BT, di luar rentang DIY). Field `koordinat.catatan` memberi peringatan.
- Total agama (5.328) di dokumen sumber lebih besar dari total penduduk (4.802); angka tetap disajikan apa adanya dengan catatan.
- Nama personalia di `struktur-organisasi.json` mayoritas `TBD` karena hanya kepala desa periode 2015-2021 (Damar, A.Md) yang tercantum di sumber.

## Penyusun & sumber

- **Disusun ulang**: tim konten tugas akhir Komputasi Awan, Mei 2026.
- **Sumber primer**: Buku Profil Desa Gerbosari (penyusun: Damar, A.Md dan Dyah Yekti Indrajati, S.Si), `Desa Gerbosari.docx`.
- **Sumber sekunder**: Devi, L. Y., & Hidayati, W. (2020). Pembuatan Profil Desa Gerbosari. *Jurnal Pengabdian dan Pengembangan Masyarakat*, 3(2), 445-454.
- **Referensi tambahan**: BPS Kulon Progo 2017; BPTP Yogyakarta 2012; gerbosari-kulonprogo.desa.id.

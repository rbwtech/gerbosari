-- Migration: seed initial data
-- Idempotent seed keyed on natural unique columns (pedukuhan, slug).
-- UUIDs are hardcoded so reruns do not produce duplicate logical rows.
-- Population numbers below sum to 2380 laki + 2422 perempuan = 4802 total,
-- across 1268 KK distributed over 19 pedukuhan.

-- ---------------------------------------------------------------------------
-- penduduk_ringkasan: 19 pedukuhan
-- ---------------------------------------------------------------------------
INSERT IGNORE INTO penduduk_ringkasan (id, pedukuhan, jumlah_kk, laki, perempuan) VALUES
    ('11111111-0000-4000-8000-000000000001', 'Sarimulyo',     78, 145, 148),
    ('11111111-0000-4000-8000-000000000002', 'Kemiriombo',    65, 122, 125),
    ('11111111-0000-4000-8000-000000000003', 'Jeruk',         70, 132, 135),
    ('11111111-0000-4000-8000-000000000004', 'Manggis',       60, 113, 115),
    ('11111111-0000-4000-8000-000000000005', 'Pengos A',      72, 135, 138),
    ('11111111-0000-4000-8000-000000000006', 'Pengos B',      68, 128, 130),
    ('11111111-0000-4000-8000-000000000007', 'Ketaon',        55, 103, 105),
    ('11111111-0000-4000-8000-000000000008', 'Ngroto',        58, 110, 112),
    ('11111111-0000-4000-8000-000000000009', 'Clumprit',      52,  98, 100),
    ('11111111-0000-4000-8000-000000000010', 'Karang',        75, 142, 145),
    ('11111111-0000-4000-8000-000000000011', 'Jetis',         80, 150, 152),
    ('11111111-0000-4000-8000-000000000012', 'Tlogo',         62, 116, 119),
    ('11111111-0000-4000-8000-000000000013', 'Jati',          50,  95,  96),
    ('11111111-0000-4000-8000-000000000014', 'Sumbo',         48,  90,  92),
    ('11111111-0000-4000-8000-000000000015', 'Dukuh',         67, 126, 128),
    ('11111111-0000-4000-8000-000000000016', 'Sendat',        70, 132, 134),
    ('11111111-0000-4000-8000-000000000017', 'Kayugede',      85, 160, 162),
    ('11111111-0000-4000-8000-000000000018', 'Menggermalang', 73, 137, 140),
    ('11111111-0000-4000-8000-000000000019', 'Keceme',        80, 146, 146);

-- ---------------------------------------------------------------------------
-- galeri: 11 rows. file_path is the URL path served by nginx, which aliases
-- /images/ -> /var/www/gerbosari/content/images/. Filenames mirror those
-- actually present under content/images/gallery/ (single source of truth).
-- ---------------------------------------------------------------------------
-- Galeri has no natural unique key, so we deduplicate by stable UUID via INSERT IGNORE
-- on the primary key.
INSERT IGNORE INTO galeri (id, judul, deskripsi, file_path, kategori, taken_at) VALUES
    ('22222222-0000-4000-8000-000000000001', 'Gardu Pandang Puncak Suroloyo',                  'Gardu pandang Puncak Suroloyo di ketinggian 1.000 mdpl di Pedukuhan Keceme — ikon wisata utama Desa Gerbosari.',                                  '/images/gallery/gerbosari-puncak-suroloyo.png',         'wisata',     '2024-08-17'),
    ('22222222-0000-4000-8000-000000000002', 'Panorama Perbukitan Menoreh',                    'Hamparan perbukitan Menoreh dilihat dari atas bukit batu di wilayah Gerbosari.',                                                                  '/images/gallery/gerbosari-view-bukit-batu.jpg',         'wisata',     '2024-09-22'),
    ('22222222-0000-4000-8000-000000000003', 'Sendang Batu Air Hijau',                         'Sendang alami berbatu dengan air hijau jernih, tujuan wisata alam warga sekitar.',                                                                '/images/gallery/gerbosari-sendang-batu-hijau.jpg',      'wisata',     '2024-11-09'),
    ('22222222-0000-4000-8000-000000000004', 'Sungai Jernih Berbatu',                          'Aliran sungai berbatu dengan air jernih — bagian dari kekayaan hidrologi Menoreh.',                                                               '/images/gallery/gerbosari-sungai-jernih.jpg',           'wisata',     '2024-12-03'),
    ('22222222-0000-4000-8000-000000000005', 'Sendang Kawidodaren',                            'Lokasi ritual jamasan pusaka kraton Yogyakarta tiap 1 Suro di Pedukuhan Sendat.',                                                                 '/images/gallery/gerbosari-sendang-kawidodaren.png',     'budaya',     '2024-07-08'),
    ('22222222-0000-4000-8000-000000000006', 'Tugu Agrowisata Krisan',                         'Gerbang Agrowisata Krisan Gerbosari di kawasan Zona III, sentra ekonomi kreatif desa.',                                                           '/images/gallery/gerbosari-tugu-agrowisata-krisan.jpg',  'agrowisata', '2025-02-11'),
    ('22222222-0000-4000-8000-000000000007', 'Pengajian Akbar Warga',                          'Pengajian akbar warga Desa Gerbosari di lapangan terbuka, dipimpin ustadz tamu.',                                                                 '/images/gallery/gerbosari-pengajian-akbar.jpg',         'budaya',     '2025-03-29'),
    ('22222222-0000-4000-8000-000000000008', 'Peringatan Maulid Nabi Muhammad SAW',            'Peringatan Maulid Nabi Muhammad SAW oleh masyarakat Pedukuhan Gerbosari.',                                                                        '/images/gallery/gerbosari-maulid-nabi.jpg',             'budaya',     '2024-10-15'),
    ('22222222-0000-4000-8000-000000000009', 'Hari Olahraga Nasional 2025',                    'Peringatan Hari Olahraga Nasional 2025 tingkat Kapanewon Samigaluh, dihadiri perangkat desa.',                                                    '/images/gallery/gerbosari-haornas-2025.jpg',            'kegiatan',   '2025-09-09'),
    ('22222222-0000-4000-8000-00000000000A', 'Sekolah Sepak Bola Binaan Desa',                 'Tim Sekolah Sepak Bola anak binaan desa, salah satu wadah pembinaan generasi muda.',                                                              '/images/gallery/gerbosari-ssb-sepakbola-anak.jpg',      'kegiatan',   '2025-04-12'),
    ('22222222-0000-4000-8000-00000000000B', 'Kerja Bakti Pembersihan Saluran',                'Warga gotong royong membersihkan saluran air di kompleks balai desa.',                                                                            '/images/gallery/gerbosari-kerja-bakti-saluran.jpg',     'kegiatan',   '2025-05-18');

-- ---------------------------------------------------------------------------
-- lowongan: 6 rows tied to real local economy.
-- Natural uniqueness is by UUID (no unique business key on this table).
-- ---------------------------------------------------------------------------
INSERT IGNORE INTO lowongan (id, judul, instansi, kategori, deskripsi, kontak, gaji_min, gaji_max, deadline, lokasi_pedukuhan, status) VALUES
    ('33333333-0000-4000-8000-000000000001',
     'Barista Kedai Kopi Suroloyo',
     'UMKM Kopi Suroloyo',
     'umkm',
     'Dibutuhkan barista untuk kedai kopi di kawasan agrowisata Suroloyo. Mampu meracik kopi manual brew (V60, French Press) dan espresso. Diutamakan warga Gerbosari.',
     'WA: 0812-1111-0001 (Bp. Wahyu)',
     1800000.00, 2500000.00, '2026-03-31', 'Keceme', 'aktif'),
    ('33333333-0000-4000-8000-000000000002',
     'Pengrajin Batik Tulis Motif Menoreh',
     'Sanggar Batik Menoreh',
     'umkm',
     'Sanggar batik membuka kesempatan bagi pengrajin batik tulis berpengalaman minimal 1 tahun untuk produksi reguler motif Menoreh.',
     'WA: 0812-1111-0002 (Ibu Sri)',
     1500000.00, 2200000.00, '2026-04-15', 'Jetis', 'aktif'),
    ('33333333-0000-4000-8000-000000000003',
     'Peternak Kambing Peranakan Etawa',
     'Kelompok Tani Ngudi Makmur',
     'formal',
     'Kelompok tani mencari tenaga peternak kambing PE untuk pengelolaan kandang komunal. Diutamakan memiliki pengalaman pemerahan dan pakan fermentasi.',
     'Email: ngudimakmur.gerbosari@gmail.com',
     2200000.00, 3000000.00, '2026-04-30', 'Kayugede', 'aktif'),
    ('33333333-0000-4000-8000-000000000004',
     'Pengelola Homestay Agrowisata',
     'BUMDes Gerbosari',
     'formal',
     'BUMDes membuka rekrutmen pengelola operasional homestay agrowisata: penerimaan tamu, kebersihan, koordinasi paket wisata kopi-teh-batik.',
     'Email: bumdes.gerbosari@desa.id',
     2500000.00, 3500000.00, '2026-05-15', 'Keceme', 'aktif'),
    ('33333333-0000-4000-8000-000000000005',
     'Pembudidaya Ikan Air Tawar',
     'Kelompok Mina Tirta Lestari',
     'umkm',
     'Dibutuhkan tenaga budidaya ikan nila dan lele pada kolam komunal kelompok mina. Sistem bagi hasil panen.',
     'WA: 0812-1111-0005 (Bp. Sukirno)',
     NULL, NULL, '2026-04-10', 'Pengos A', 'aktif'),
    ('33333333-0000-4000-8000-000000000006',
     'Admin Sosial Media BUMDes',
     'BUMDes Gerbosari',
     'freelance',
     'Mengelola konten Instagram dan TikTok BUMDes untuk promosi produk UMKM dan paket agrowisata. Kerja remote, 10-15 jam per minggu.',
     'Email: bumdes.gerbosari@desa.id',
     800000.00, 1500000.00, '2026-03-25', 'Karang', 'aktif');

-- ---------------------------------------------------------------------------
-- berita: 5 rows (2 berita + 3 agenda). Deduplicated by slug.
-- ---------------------------------------------------------------------------
INSERT IGNORE INTO berita (id, judul, slug, kategori, ringkasan, konten, gambar_url, tanggal_acara, lokasi, author, published_at) VALUES
    ('44444444-0000-4000-8000-000000000001',
     'Pembangunan Jalan Akses Agrowisata Menoreh Tahap I Selesai',
     'pembangunan-jalan-akses-agrowisata-menoreh-tahap-1',
     'berita',
     'Ruas jalan sepanjang 2,3 km menuju kawasan agrowisata kopi dan teh di Pedukuhan Keceme rampung dikerjakan dan siap dilewati kendaraan roda empat.',
     '## Akses Agrowisata Kini Lebih Mudah\n\nPemerintah Desa Gerbosari bersama Dinas PUPR Kulon Progo telah menyelesaikan tahap pertama pembangunan jalan akses agrowisata sepanjang **2,3 km** dari simpang Pedukuhan Karang menuju kawasan kebun kopi dan teh di Pedukuhan Keceme.\n\nProyek senilai Rp 1,8 miliar ini diharapkan mendorong arus kunjungan wisatawan ke sentra agrowisata Menoreh, sekaligus mempermudah petani mengangkut hasil panen.\n\nTahap II direncanakan dimulai pada triwulan ketiga 2026, mencakup pengerasan jalan penghubung antar kebun.',
     '/images/gallery/gerbosari-view-bukit-batu.jpg',
     NULL,
     'Sarimulyo, Gerbosari',
     'Pemerintah Desa Gerbosari',
     '2026-02-18 09:00:00'),
    ('44444444-0000-4000-8000-000000000002',
     'Panen Raya Kopi Robusta 2026 di Pedukuhan Keceme',
     'panen-raya-kopi-robusta-2026-keceme',
     'berita',
     'Kelompok tani kopi Keceme membukukan panen 12 ton biji merah pada musim 2026, naik 18 persen dibanding tahun sebelumnya.',
     '## Kopi Menoreh Naik Kelas\n\nMusim panen kopi robusta 2026 di Pedukuhan **Keceme** menorehkan hasil panen sebesar **12 ton biji merah (cherry)**, naik sekitar 18 persen dibanding musim 2025. Kenaikan diatribusikan pada curah hujan yang lebih stabil dan pendampingan intensif dari penyuluh pertanian.\n\nSebagian hasil panen sudah diserap oleh kedai kopi lokal di Suroloyo dan jaringan roastery di Yogyakarta. BUMDes Gerbosari juga mulai membuka kanal direct-trade ke pembeli kafe di Jakarta dan Bandung.',
     '/images/gallery/gerbosari-tugu-agrowisata-krisan.jpg',
     NULL,
     'Keceme, Gerbosari',
     'Pemerintah Desa Gerbosari',
     '2026-03-02 08:30:00'),
    ('44444444-0000-4000-8000-000000000003',
     'Tanggap Warso 1 Suro: Kirab Pusaka Kraton di Sendang Kawidodaren',
     'agenda-tanggap-warso-1-suro-kirab-pusaka',
     'agenda',
     'Tradisi tahunan jamasan dan kirab pusaka kraton di Sendang Kawidodaren, terbuka untuk umum dengan pengaturan jalur kirab.',
     '## Tanggap Warso 1 Suro\n\nPemerintah Desa Gerbosari kembali menyelenggarakan **Tanggap Warso 1 Suro** dengan acara utama berupa **kirab pusaka kraton** menuju **Sendang Kawidodaren** untuk prosesi jamasan.\n\n- Waktu: 16 Juli 2026, pukul 19.00 WIB\n- Titik kumpul: Balai Desa Gerbosari\n- Rute kirab: Balai Desa - Pedukuhan Sendat - Sendang Kawidodaren\n- Dress code peserta kirab: busana adat Jawa\n\nMasyarakat umum dipersilakan menyaksikan di sepanjang rute. Parkir terpusat di lapangan Pedukuhan Sendat.',
     '/images/gallery/gerbosari-sendang-kawidodaren.png',
     '2026-07-16',
     'Sendang Kawidodaren, Pedukuhan Sendat',
     'Pemerintah Desa Gerbosari',
     '2026-04-01 10:00:00'),
    ('44444444-0000-4000-8000-000000000004',
     'Bersih Dusun Pedukuhan Keceme Bulan Agustus',
     'agenda-bersih-dusun-keceme-agustus',
     'agenda',
     'Gotong royong tahunan pembersihan lingkungan Pedukuhan Keceme dilanjutkan kenduri bersama warga.',
     '## Bersih Dusun Keceme\n\nWarga Pedukuhan Keceme menyelenggarakan agenda **Bersih Dusun** sebagai wujud rasa syukur dan pelestarian tradisi gotong royong.\n\n- Waktu: 17 Agustus 2026, mulai pukul 06.30 WIB\n- Lokasi: Sepanjang jalan utama dan kawasan Sendang Pedukuhan Keceme\n- Acara: Kerja bakti, doa bersama, kenduri\n- Catatan: Warga harap membawa peralatan kebersihan dari rumah masing-masing.',
     '/images/gallery/gerbosari-kerja-bakti-saluran.jpg',
     '2026-08-17',
     'Pedukuhan Keceme',
     'Pemerintah Desa Gerbosari',
     '2026-07-20 08:00:00'),
    ('44444444-0000-4000-8000-000000000005',
     'Saparan Bulan Sapar di Balai Desa Gerbosari',
     'agenda-saparan-bulan-sapar',
     'agenda',
     'Peringatan adat Saparan dengan kenduri dan pertunjukan jathilan terbuka untuk seluruh warga desa dan tamu undangan.',
     '## Saparan Gerbosari\n\nTradisi **Saparan** kembali digelar pada bulan Sapar di Balai Desa Gerbosari sebagai bentuk pelestarian budaya Jawa.\n\n- Waktu: 22 September 2026, pukul 19.30 WIB\n- Lokasi: Halaman Balai Desa Gerbosari\n- Susunan acara: Kenduri, doa bersama, pertunjukan jathilan oleh sanggar lokal\n- Open public, tidak memungut biaya\n\nDimohon kerja sama warga sekitar untuk pengaturan parkir dan keamanan.',
     '/images/gallery/gerbosari-pengajian-akbar.jpg',
     '2026-09-22',
     'Balai Desa Gerbosari',
     'Pemerintah Desa Gerbosari',
     '2026-08-25 09:00:00');

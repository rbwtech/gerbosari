-- Migration: create lowongan table
-- Local job postings (UMKM, formal, freelance) tied to Desa Gerbosari economy.

CREATE TABLE IF NOT EXISTS lowongan (
    id CHAR(36) NOT NULL,
    judul VARCHAR(200) NOT NULL,
    instansi VARCHAR(200) NOT NULL,
    kategori ENUM('umkm', 'formal', 'freelance') NOT NULL,
    deskripsi TEXT NOT NULL,
    kontak VARCHAR(200) NOT NULL,
    gaji_min DECIMAL(12, 2),
    gaji_max DECIMAL(12, 2),
    deadline DATE,
    lokasi_pedukuhan VARCHAR(50),
    status ENUM('aktif', 'tutup', 'arsip') NOT NULL DEFAULT 'aktif',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    INDEX idx_lowongan_kategori (kategori),
    INDEX idx_lowongan_status (status),
    INDEX idx_lowongan_deadline (deadline),
    CONSTRAINT chk_lowongan_gaji_range CHECK (
        gaji_min IS NULL
        OR gaji_max IS NULL
        OR gaji_min <= gaji_max
    )
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;

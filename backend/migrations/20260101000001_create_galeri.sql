-- Migration: create galeri table
-- Stores metadata for gallery images. Actual files live under content/images/gallery/
-- and are served by nginx (see deploy/nginx.conf).

CREATE TABLE IF NOT EXISTS galeri (
    id CHAR(36) NOT NULL,
    judul VARCHAR(200) NOT NULL,
    deskripsi TEXT,
    file_path VARCHAR(500) NOT NULL,
    kategori ENUM('kegiatan', 'wisata', 'budaya', 'agrowisata') NOT NULL,
    taken_at DATE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    INDEX idx_galeri_kategori (kategori),
    INDEX idx_galeri_taken_at (taken_at)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;

-- Migration: create berita table
-- Combined news/agenda store. `kategori='agenda'` rows populate the upcoming-events
-- panel and require tanggal_acara; `kategori='berita'` rows are standard articles.

CREATE TABLE IF NOT EXISTS berita (
    id CHAR(36) NOT NULL,
    judul VARCHAR(200) NOT NULL,
    slug VARCHAR(200) NOT NULL,
    kategori ENUM('berita', 'agenda') NOT NULL,
    ringkasan VARCHAR(500) NOT NULL,
    konten MEDIUMTEXT NOT NULL,
    gambar_url VARCHAR(500),
    tanggal_acara DATE,
    lokasi VARCHAR(200),
    author VARCHAR(100) NOT NULL DEFAULT 'Pemerintah Desa Gerbosari',
    published_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY uq_berita_slug (slug),
    INDEX idx_berita_kategori (kategori),
    INDEX idx_berita_published_at (published_at)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;

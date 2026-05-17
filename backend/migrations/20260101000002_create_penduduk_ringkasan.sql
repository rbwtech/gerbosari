-- Migration: create penduduk_ringkasan table
-- Aggregate population summary per pedukuhan. Not individual records;
-- the public profile page only shows totals per dusun.

CREATE TABLE IF NOT EXISTS penduduk_ringkasan (
    id CHAR(36) NOT NULL,
    pedukuhan VARCHAR(50) NOT NULL,
    jumlah_kk INT UNSIGNED NOT NULL,
    laki INT UNSIGNED NOT NULL,
    perempuan INT UNSIGNED NOT NULL,
    total INT UNSIGNED GENERATED ALWAYS AS (laki + perempuan) STORED,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY uq_penduduk_ringkasan_pedukuhan (pedukuhan)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::berita::{Berita, BeritaRingkasan};

#[derive(Debug, Deserialize)]
pub struct BeritaListQuery {
    pub kategori: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BeritaRingkasanResponse {
    pub id: Uuid,
    pub judul: String,
    pub slug: String,
    pub kategori: String,
    pub ringkasan: String,
    pub gambar_url: Option<String>,
    pub tanggal: Option<NaiveDate>,
    pub author: String,
    pub published_at: DateTime<Utc>,
}

impl From<BeritaRingkasan> for BeritaRingkasanResponse {
    fn from(b: BeritaRingkasan) -> Self {
        Self {
            id: b.id,
            judul: b.judul,
            slug: b.slug,
            kategori: b.kategori.as_str().to_string(),
            ringkasan: b.ringkasan,
            gambar_url: b.gambar_url,
            tanggal: b.tanggal_acara,
            author: b.author,
            published_at: b.published_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BeritaResponse {
    pub id: Uuid,
    pub judul: String,
    pub slug: String,
    pub kategori: String,
    pub ringkasan: String,
    pub konten: String,
    pub gambar_url: Option<String>,
    pub tanggal: Option<NaiveDate>,
    pub author: String,
    pub published_at: DateTime<Utc>,
}

impl From<Berita> for BeritaResponse {
    fn from(b: Berita) -> Self {
        Self {
            id: b.id,
            judul: b.judul,
            slug: b.slug,
            kategori: b.kategori.as_str().to_string(),
            ringkasan: b.ringkasan,
            konten: b.konten,
            gambar_url: b.gambar_url,
            tanggal: b.tanggal_acara,
            author: b.author,
            published_at: b.published_at,
        }
    }
}

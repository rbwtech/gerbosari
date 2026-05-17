use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KategoriBerita {
    Berita,
    Agenda,
}

impl KategoriBerita {
    pub fn as_str(self) -> &'static str {
        match self {
            KategoriBerita::Berita => "berita",
            KategoriBerita::Agenda => "agenda",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "berita" => Some(Self::Berita),
            "agenda" => Some(Self::Agenda),
            _ => None,
        }
    }
}

/// Summary projection used in list endpoints. Excludes `konten` to keep
/// payloads small; full body is fetched via `get_by_slug`.
#[derive(Debug, Clone)]
pub struct BeritaRingkasan {
    pub id: Uuid,
    pub judul: String,
    pub slug: String,
    pub kategori: KategoriBerita,
    pub ringkasan: String,
    pub gambar_url: Option<String>,
    pub tanggal_acara: Option<NaiveDate>,
    pub author: String,
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Berita {
    pub id: Uuid,
    pub judul: String,
    pub slug: String,
    pub kategori: KategoriBerita,
    pub ringkasan: String,
    pub konten: String,
    pub gambar_url: Option<String>,
    pub tanggal_acara: Option<NaiveDate>,
    pub author: String,
    pub published_at: DateTime<Utc>,
}

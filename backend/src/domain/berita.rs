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
    pub lokasi: Option<String>,
    pub author: String,
    pub published_at: DateTime<Utc>,
}

/// Input payload for inserting a new `berita` row. The repository assigns
/// `id`; `published_at` defaults to `Utc::now()` when omitted so admins can
/// publish instantly without supplying a timestamp.
#[derive(Debug, Clone)]
pub struct NewBerita {
    pub judul: String,
    pub slug: String,
    pub kategori: KategoriBerita,
    pub ringkasan: String,
    pub konten: String,
    pub gambar_url: Option<String>,
    pub tanggal: Option<NaiveDate>,
    pub lokasi: Option<String>,
    pub author: String,
    pub published_at: Option<DateTime<Utc>>,
}

/// Partial update for an existing `berita` row. Outer `Option` distinguishes
/// "unchanged" from "set"; inner `Option` on nullable columns distinguishes
/// "set to NULL" from "no change". `slug` is the natural identity and is not
/// patchable here — delete + create when renaming.
#[derive(Debug, Clone, Default)]
pub struct BeritaPatch {
    pub judul: Option<String>,
    pub kategori: Option<KategoriBerita>,
    pub ringkasan: Option<String>,
    pub konten: Option<String>,
    pub gambar_url: Option<Option<String>>,
    pub tanggal: Option<Option<NaiveDate>>,
    pub lokasi: Option<Option<String>>,
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

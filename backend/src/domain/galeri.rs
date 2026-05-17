use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KategoriGaleri {
    Kegiatan,
    Wisata,
    Budaya,
    Agrowisata,
}

impl KategoriGaleri {
    pub fn as_str(self) -> &'static str {
        match self {
            KategoriGaleri::Kegiatan => "kegiatan",
            KategoriGaleri::Wisata => "wisata",
            KategoriGaleri::Budaya => "budaya",
            KategoriGaleri::Agrowisata => "agrowisata",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "kegiatan" => Some(Self::Kegiatan),
            "wisata" => Some(Self::Wisata),
            "budaya" => Some(Self::Budaya),
            "agrowisata" => Some(Self::Agrowisata),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Galeri {
    pub id: Uuid,
    pub judul: String,
    pub deskripsi: Option<String>,
    pub file_path: String,
    pub kategori: KategoriGaleri,
    pub taken_at: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

/// Payload for inserting a new `galeri` row. The repository generates `id`
/// and `created_at`, so they are intentionally omitted here.
#[derive(Debug, Clone)]
pub struct NewGaleri {
    pub judul: String,
    pub deskripsi: Option<String>,
    pub file_path: String,
    pub kategori: KategoriGaleri,
    pub taken_at: Option<NaiveDate>,
}

/// Partial update for an existing `galeri` row. `Option::None` means
/// "do not change"; `Option::Some(None)` (for nullable columns) clears the
/// value. Distinguished via `Option<Option<T>>` for nullable fields.
#[derive(Debug, Clone, Default)]
pub struct GaleriPatch {
    pub judul: Option<String>,
    pub deskripsi: Option<Option<String>>,
    pub file_path: Option<String>,
    pub kategori: Option<KategoriGaleri>,
    pub taken_at: Option<Option<NaiveDate>>,
}

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::galeri::Galeri;

#[derive(Debug, Deserialize)]
pub struct GaleriListQuery {
    pub kategori: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GaleriResponse {
    pub id: Uuid,
    pub judul: String,
    pub deskripsi: Option<String>,
    pub file_path: String,
    pub kategori: String,
    pub taken_at: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

impl From<Galeri> for GaleriResponse {
    fn from(g: Galeri) -> Self {
        Self {
            id: g.id,
            judul: g.judul,
            deskripsi: g.deskripsi,
            file_path: g.file_path,
            kategori: g.kategori.as_str().to_string(),
            taken_at: g.taken_at,
            created_at: g.created_at,
        }
    }
}

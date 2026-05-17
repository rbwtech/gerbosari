use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::lowongan::Lowongan;

#[derive(Debug, Deserialize)]
pub struct LowonganListQuery {
    pub kategori: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LowonganResponse {
    pub id: Uuid,
    pub judul: String,
    pub instansi: String,
    pub kategori: String,
    pub deskripsi: String,
    pub kontak: String,
    pub gaji_min: Option<i64>,
    pub gaji_max: Option<i64>,
    pub deadline: Option<NaiveDate>,
    pub lokasi_pedukuhan: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<Lowongan> for LowonganResponse {
    fn from(l: Lowongan) -> Self {
        Self {
            id: l.id,
            judul: l.judul,
            instansi: l.instansi,
            kategori: l.kategori.as_str().to_string(),
            deskripsi: l.deskripsi,
            kontak: l.kontak,
            gaji_min: l.gaji_min,
            gaji_max: l.gaji_max,
            deadline: l.deadline,
            lokasi_pedukuhan: l.lokasi_pedukuhan,
            status: l.status.as_str().to_string(),
            created_at: l.created_at,
        }
    }
}

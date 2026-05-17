use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KategoriLowongan {
    Umkm,
    Formal,
    Freelance,
}

impl KategoriLowongan {
    pub fn as_str(self) -> &'static str {
        match self {
            KategoriLowongan::Umkm => "umkm",
            KategoriLowongan::Formal => "formal",
            KategoriLowongan::Freelance => "freelance",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "umkm" => Some(Self::Umkm),
            "formal" => Some(Self::Formal),
            "freelance" => Some(Self::Freelance),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusLowongan {
    Aktif,
    Tutup,
    Arsip,
}

impl StatusLowongan {
    pub fn as_str(self) -> &'static str {
        match self {
            StatusLowongan::Aktif => "aktif",
            StatusLowongan::Tutup => "tutup",
            StatusLowongan::Arsip => "arsip",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "aktif" => Some(Self::Aktif),
            "tutup" => Some(Self::Tutup),
            "arsip" => Some(Self::Arsip),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lowongan {
    pub id: Uuid,
    pub judul: String,
    pub instansi: String,
    pub kategori: KategoriLowongan,
    pub deskripsi: String,
    pub kontak: String,
    pub gaji_min: Option<i64>,
    pub gaji_max: Option<i64>,
    pub deadline: Option<NaiveDate>,
    pub lokasi_pedukuhan: Option<String>,
    pub status: StatusLowongan,
    pub created_at: DateTime<Utc>,
}

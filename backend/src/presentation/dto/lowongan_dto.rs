use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::lowongan::{
    KategoriLowongan, Lowongan, LowonganPatch, NewLowongan, StatusLowongan,
};
use crate::error::AppError;
use crate::presentation::dto::double_option;

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

// ============================================================================
// Admin write DTOs
// ============================================================================

/// JSON body for `POST /api/admin/lowongan`. `kategori` and `status` arrive as
/// strings and are mapped to their enum equivalents during `into_domain()`,
/// surfacing user-friendly Indonesian errors for invalid values.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateLowonganRequest {
    #[validate(length(min = 1, max = 200, message = "judul wajib 1-200 karakter"))]
    pub judul: String,
    #[validate(length(min = 1, max = 200, message = "instansi wajib 1-200 karakter"))]
    pub instansi: String,
    #[validate(length(min = 1, message = "kategori wajib diisi"))]
    pub kategori: String,
    #[validate(length(min = 1, max = 10000, message = "deskripsi wajib 1-10000 karakter"))]
    pub deskripsi: String,
    #[validate(length(min = 1, max = 200, message = "kontak wajib 1-200 karakter"))]
    pub kontak: String,
    pub gaji_min: Option<i64>,
    pub gaji_max: Option<i64>,
    pub deadline: Option<NaiveDate>,
    #[validate(length(max = 50, message = "lokasi_pedukuhan maksimal 50 karakter"))]
    pub lokasi_pedukuhan: Option<String>,
    /// Defaults to "aktif" when omitted.
    pub status: Option<String>,
}

impl CreateLowonganRequest {
    pub fn into_domain(self) -> Result<NewLowongan, AppError> {
        let kategori = KategoriLowongan::parse(&self.kategori).ok_or_else(|| {
            AppError::Validation(format!(
                "kategori tidak dikenal: '{}'. Gunakan umkm|formal|freelance",
                self.kategori
            ))
        })?;

        let status = match self.status {
            Some(raw) => StatusLowongan::parse(&raw).ok_or_else(|| {
                AppError::Validation(format!(
                    "status tidak dikenal: '{}'. Gunakan aktif|tutup|arsip",
                    raw
                ))
            })?,
            None => StatusLowongan::Aktif,
        };

        // Belt-and-suspenders gaji range check; DB also enforces via CHECK
        // constraint, but failing fast yields a cleaner 400 message.
        if let (Some(min), Some(max)) = (self.gaji_min, self.gaji_max) {
            if min > max {
                return Err(AppError::Validation(
                    "gaji_min tidak boleh lebih besar dari gaji_max".to_string(),
                ));
            }
        }
        if self.gaji_min.map(|v| v < 0).unwrap_or(false)
            || self.gaji_max.map(|v| v < 0).unwrap_or(false)
        {
            return Err(AppError::Validation(
                "gaji tidak boleh negatif".to_string(),
            ));
        }

        Ok(NewLowongan {
            judul: self.judul,
            instansi: self.instansi,
            kategori,
            deskripsi: self.deskripsi,
            kontak: self.kontak,
            gaji_min: self.gaji_min,
            gaji_max: self.gaji_max,
            deadline: self.deadline,
            lokasi_pedukuhan: self.lokasi_pedukuhan,
            status,
        })
    }
}

/// JSON body for `PATCH /api/admin/lowongan/:id`. All fields optional.
/// Nullable columns use `double_option` so an explicit `null` clears them.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLowonganRequest {
    #[validate(length(min = 1, max = 200, message = "judul wajib 1-200 karakter"))]
    pub judul: Option<String>,

    #[validate(length(min = 1, max = 200, message = "instansi wajib 1-200 karakter"))]
    pub instansi: Option<String>,

    pub kategori: Option<String>,

    #[validate(length(min = 1, max = 10000, message = "deskripsi wajib 1-10000 karakter"))]
    pub deskripsi: Option<String>,

    #[validate(length(min = 1, max = 200, message = "kontak wajib 1-200 karakter"))]
    pub kontak: Option<String>,

    #[serde(default, deserialize_with = "double_option")]
    pub gaji_min: Option<Option<i64>>,

    #[serde(default, deserialize_with = "double_option")]
    pub gaji_max: Option<Option<i64>>,

    #[serde(default, deserialize_with = "double_option")]
    pub deadline: Option<Option<NaiveDate>>,

    #[serde(default, deserialize_with = "double_option")]
    pub lokasi_pedukuhan: Option<Option<String>>,

    pub status: Option<String>,
}

impl UpdateLowonganRequest {
    pub fn into_domain(self) -> Result<LowonganPatch, AppError> {
        let kategori = match self.kategori {
            Some(raw) => Some(KategoriLowongan::parse(&raw).ok_or_else(|| {
                AppError::Validation(format!(
                    "kategori tidak dikenal: '{}'. Gunakan umkm|formal|freelance",
                    raw
                ))
            })?),
            None => None,
        };

        let status = match self.status {
            Some(raw) => Some(StatusLowongan::parse(&raw).ok_or_else(|| {
                AppError::Validation(format!(
                    "status tidak dikenal: '{}'. Gunakan aktif|tutup|arsip",
                    raw
                ))
            })?),
            None => None,
        };

        // Same negative-gaji guard as create. We can only cross-check min<=max
        // if both are explicitly provided in this patch.
        if let (Some(Some(min)), Some(Some(max))) = (self.gaji_min, self.gaji_max) {
            if min > max {
                return Err(AppError::Validation(
                    "gaji_min tidak boleh lebih besar dari gaji_max".to_string(),
                ));
            }
        }
        if matches!(self.gaji_min, Some(Some(v)) if v < 0)
            || matches!(self.gaji_max, Some(Some(v)) if v < 0)
        {
            return Err(AppError::Validation(
                "gaji tidak boleh negatif".to_string(),
            ));
        }

        Ok(LowonganPatch {
            judul: self.judul,
            instansi: self.instansi,
            kategori,
            deskripsi: self.deskripsi,
            kontak: self.kontak,
            gaji_min: self.gaji_min,
            gaji_max: self.gaji_max,
            deadline: self.deadline,
            lokasi_pedukuhan: self.lokasi_pedukuhan,
            status,
        })
    }
}

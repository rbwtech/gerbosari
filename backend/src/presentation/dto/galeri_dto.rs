use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::galeri::{Galeri, GaleriPatch, KategoriGaleri, NewGaleri};
use crate::error::AppError;
use crate::presentation::dto::double_option;

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

// ============================================================================
// Admin write DTOs
// ============================================================================

/// JSON body for `POST /api/admin/galeri`. All required fields must be present;
/// `kategori` is a free-form string that maps onto `KategoriGaleri` at
/// validation time so we can return a user-friendly Indonesian error.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateGaleriRequest {
    #[validate(length(min = 1, max = 200, message = "judul wajib 1-200 karakter"))]
    pub judul: String,
    #[validate(length(max = 5000, message = "deskripsi maksimal 5000 karakter"))]
    pub deskripsi: Option<String>,
    #[validate(length(min = 1, max = 500, message = "file_path wajib 1-500 karakter"))]
    pub file_path: String,
    #[validate(length(min = 1, message = "kategori wajib diisi"))]
    pub kategori: String,
    pub taken_at: Option<NaiveDate>,
}

impl CreateGaleriRequest {
    pub fn into_domain(self) -> Result<NewGaleri, AppError> {
        let kategori = KategoriGaleri::parse(&self.kategori).ok_or_else(|| {
            AppError::Validation(format!(
                "kategori tidak dikenal: '{}'. Gunakan kegiatan|wisata|budaya|agrowisata",
                self.kategori
            ))
        })?;
        Ok(NewGaleri {
            judul: self.judul,
            deskripsi: self.deskripsi,
            file_path: self.file_path,
            kategori,
            taken_at: self.taken_at,
        })
    }
}

/// JSON body for `PATCH /api/admin/galeri/:id`. Every field is optional.
/// Nullable columns use `double_option` so a `null` payload clears them.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGaleriRequest {
    #[validate(length(min = 1, max = 200, message = "judul wajib 1-200 karakter"))]
    pub judul: Option<String>,

    #[serde(default, deserialize_with = "double_option")]
    pub deskripsi: Option<Option<String>>,

    #[validate(length(min = 1, max = 500, message = "file_path wajib 1-500 karakter"))]
    pub file_path: Option<String>,

    pub kategori: Option<String>,

    #[serde(default, deserialize_with = "double_option")]
    pub taken_at: Option<Option<NaiveDate>>,
}

impl UpdateGaleriRequest {
    pub fn into_domain(self) -> Result<GaleriPatch, AppError> {
        let kategori = match self.kategori {
            Some(raw) => Some(KategoriGaleri::parse(&raw).ok_or_else(|| {
                AppError::Validation(format!(
                    "kategori tidak dikenal: '{}'. Gunakan kegiatan|wisata|budaya|agrowisata",
                    raw
                ))
            })?),
            None => None,
        };

        Ok(GaleriPatch {
            judul: self.judul,
            deskripsi: self.deskripsi,
            file_path: self.file_path,
            kategori,
            taken_at: self.taken_at,
        })
    }
}

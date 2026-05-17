use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::berita::{Berita, BeritaPatch, BeritaRingkasan, KategoriBerita, NewBerita};
use crate::error::AppError;
use crate::presentation::dto::double_option;

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
    pub lokasi: Option<String>,
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
            lokasi: b.lokasi,
            author: b.author,
            published_at: b.published_at,
        }
    }
}

// ============================================================================
// Admin write DTOs
// ============================================================================

/// URL-safe slug enforcement: lowercase ASCII letters, digits, and hyphens
/// only, with no leading/trailing/consecutive hyphens. Enforced at the DTO
/// layer so malformed slugs never reach the SQL prepare step. Implemented
/// inline (no `regex` crate) to keep the dependency surface small.
fn validate_slug(value: &str) -> Result<(), validator::ValidationError> {
    fn invalid() -> validator::ValidationError {
        let mut err = validator::ValidationError::new("slug_format");
        err.message = Some(
            "slug hanya boleh huruf kecil, angka, dan tanda hubung tunggal (a-z, 0-9, -)".into(),
        );
        err
    }

    if value.is_empty() {
        return Err(invalid());
    }
    let bytes = value.as_bytes();
    if bytes[0] == b'-' || bytes[bytes.len() - 1] == b'-' {
        return Err(invalid());
    }
    let mut prev_hyphen = false;
    for &b in bytes {
        let is_alnum = b.is_ascii_lowercase() || b.is_ascii_digit();
        let is_hyphen = b == b'-';
        if !is_alnum && !is_hyphen {
            return Err(invalid());
        }
        if is_hyphen && prev_hyphen {
            return Err(invalid());
        }
        prev_hyphen = is_hyphen;
    }
    Ok(())
}

/// JSON body for `POST /api/admin/berita`. `konten` is stored as raw Markdown
/// — trusted admin content rendered downstream by the frontend.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateBeritaRequest {
    #[validate(length(min = 1, max = 200, message = "judul wajib 1-200 karakter"))]
    pub judul: String,

    #[validate(
        length(min = 1, max = 200, message = "slug wajib 1-200 karakter"),
        custom(function = "validate_slug")
    )]
    pub slug: String,

    #[validate(length(min = 1, message = "kategori wajib diisi"))]
    pub kategori: String,

    #[validate(length(min = 1, max = 500, message = "ringkasan wajib 1-500 karakter"))]
    pub ringkasan: String,

    #[validate(length(min = 1, message = "konten wajib diisi"))]
    pub konten: String,

    #[validate(length(max = 500, message = "gambar_url maksimal 500 karakter"))]
    pub gambar_url: Option<String>,

    pub tanggal: Option<NaiveDate>,

    #[validate(length(max = 200, message = "lokasi maksimal 200 karakter"))]
    pub lokasi: Option<String>,

    #[validate(length(min = 1, max = 100, message = "author wajib 1-100 karakter"))]
    pub author: String,

    pub published_at: Option<DateTime<Utc>>,
}

impl CreateBeritaRequest {
    pub fn into_domain(self) -> Result<NewBerita, AppError> {
        let kategori = KategoriBerita::parse(&self.kategori).ok_or_else(|| {
            AppError::Validation(format!(
                "kategori tidak dikenal: '{}'. Gunakan berita|agenda",
                self.kategori
            ))
        })?;
        Ok(NewBerita {
            judul: self.judul,
            slug: self.slug,
            kategori,
            ringkasan: self.ringkasan,
            konten: self.konten,
            gambar_url: self.gambar_url,
            tanggal: self.tanggal,
            lokasi: self.lokasi,
            author: self.author,
            published_at: self.published_at,
        })
    }
}

/// JSON body for `PATCH /api/admin/berita/:slug`. Every field is optional;
/// nullable columns use `double_option` so a `null` payload explicitly clears
/// them while an absent key leaves them untouched.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBeritaRequest {
    #[validate(length(min = 1, max = 200, message = "judul wajib 1-200 karakter"))]
    pub judul: Option<String>,

    pub kategori: Option<String>,

    #[validate(length(min = 1, max = 500, message = "ringkasan wajib 1-500 karakter"))]
    pub ringkasan: Option<String>,

    #[validate(length(min = 1, message = "konten tidak boleh kosong"))]
    pub konten: Option<String>,

    #[serde(default, deserialize_with = "double_option")]
    pub gambar_url: Option<Option<String>>,

    #[serde(default, deserialize_with = "double_option")]
    pub tanggal: Option<Option<NaiveDate>>,

    #[serde(default, deserialize_with = "double_option")]
    pub lokasi: Option<Option<String>>,

    #[validate(length(min = 1, max = 100, message = "author wajib 1-100 karakter"))]
    pub author: Option<String>,

    pub published_at: Option<DateTime<Utc>>,
}

impl UpdateBeritaRequest {
    pub fn into_domain(self) -> Result<BeritaPatch, AppError> {
        let kategori = match self.kategori {
            Some(raw) => Some(KategoriBerita::parse(&raw).ok_or_else(|| {
                AppError::Validation(format!(
                    "kategori tidak dikenal: '{}'. Gunakan berita|agenda",
                    raw
                ))
            })?),
            None => None,
        };

        Ok(BeritaPatch {
            judul: self.judul,
            kategori,
            ringkasan: self.ringkasan,
            konten: self.konten,
            gambar_url: self.gambar_url,
            tanggal: self.tanggal,
            lokasi: self.lokasi,
            author: self.author,
            published_at: self.published_at,
        })
    }
}

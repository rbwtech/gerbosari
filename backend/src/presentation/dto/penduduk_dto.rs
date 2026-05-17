use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::penduduk::{PendudukPedukuhan, PendudukRingkasan};

#[derive(Debug, Serialize)]
pub struct PendudukPedukuhanResponse {
    pub pedukuhan: String,
    pub total_penduduk: i64,
    pub total_laki: i64,
    pub total_perempuan: i64,
    pub total_kk: i64,
    pub updated_at: DateTime<Utc>,
}

impl From<PendudukPedukuhan> for PendudukPedukuhanResponse {
    fn from(p: PendudukPedukuhan) -> Self {
        Self {
            pedukuhan: p.pedukuhan,
            total_penduduk: p.total_penduduk,
            total_laki: p.total_laki,
            total_perempuan: p.total_perempuan,
            total_kk: p.total_kk,
            updated_at: p.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PendudukRingkasanResponse {
    pub total_penduduk: i64,
    pub total_laki: i64,
    pub total_perempuan: i64,
    pub total_kk: i64,
    pub per_pedukuhan: Vec<PendudukPedukuhanResponse>,
}

impl From<PendudukRingkasan> for PendudukRingkasanResponse {
    fn from(r: PendudukRingkasan) -> Self {
        Self {
            total_penduduk: r.total_penduduk,
            total_laki: r.total_laki,
            total_perempuan: r.total_perempuan,
            total_kk: r.total_kk,
            per_pedukuhan: r.per_pedukuhan.into_iter().map(Into::into).collect(),
        }
    }
}

// ============================================================================
// Admin write DTOs
// ============================================================================

/// JSON body for `PATCH /api/admin/penduduk/:pedukuhan`. Field names mirror
/// the raw DB columns (`jumlah_kk`, `laki`, `perempuan`); the derived `total`
/// column is computed by MySQL and not patchable. Counts are validated as
/// non-negative; an upper bound is enforced loosely to guard against typos
/// (a single pedukuhan is unlikely to exceed 1_000_000 people).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePendudukRequest {
    #[validate(range(min = 0, max = 1_000_000, message = "jumlah_kk harus 0-1.000.000"))]
    pub jumlah_kk: Option<i64>,

    #[validate(range(min = 0, max = 1_000_000, message = "laki harus 0-1.000.000"))]
    pub laki: Option<i64>,

    #[validate(range(min = 0, max = 1_000_000, message = "perempuan harus 0-1.000.000"))]
    pub perempuan: Option<i64>,
}

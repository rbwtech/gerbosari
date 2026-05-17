use chrono::{DateTime, Utc};
use serde::Serialize;

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

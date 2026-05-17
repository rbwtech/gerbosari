use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PendudukPedukuhan {
    pub pedukuhan: String,
    pub total_penduduk: i64,
    pub total_laki: i64,
    pub total_perempuan: i64,
    pub total_kk: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PendudukRingkasan {
    pub total_penduduk: i64,
    pub total_laki: i64,
    pub total_perempuan: i64,
    pub total_kk: i64,
    pub per_pedukuhan: Vec<PendudukPedukuhan>,
}

/// Partial update for a `penduduk_ringkasan` pedukuhan row. The 19 pedukuhan
/// list is fixed at seed time, so only column updates are supported — no
/// create or delete. Field names match the raw DB columns (`jumlah_kk`,
/// `laki`, `perempuan`); the `total` column is GENERATED and not patchable.
#[derive(Debug, Clone, Default)]
pub struct PendudukPatch {
    pub jumlah_kk: Option<i64>,
    pub laki: Option<i64>,
    pub perempuan: Option<i64>,
}

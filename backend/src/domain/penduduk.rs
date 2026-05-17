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

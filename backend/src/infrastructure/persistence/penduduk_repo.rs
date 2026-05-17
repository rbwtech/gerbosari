use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;

use crate::domain::penduduk::{PendudukPedukuhan, PendudukRingkasan};
use crate::domain::repository::PendudukRepository;
use crate::error::AppError;

pub struct MysqlPendudukRepository {
    pool: MySqlPool,
}

impl MysqlPendudukRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct PendudukRow {
    pedukuhan: String,
    total_penduduk: i64,
    total_laki: i64,
    total_perempuan: i64,
    total_kk: i64,
    updated_at: NaiveDateTime,
}

impl From<PendudukRow> for PendudukPedukuhan {
    fn from(r: PendudukRow) -> Self {
        Self {
            pedukuhan: r.pedukuhan,
            total_penduduk: r.total_penduduk,
            total_laki: r.total_laki,
            total_perempuan: r.total_perempuan,
            total_kk: r.total_kk,
            updated_at: r.updated_at.and_utc(),
        }
    }
}

#[async_trait]
impl PendudukRepository for MysqlPendudukRepository {
    async fn ringkasan(&self) -> Result<PendudukRingkasan, AppError> {
        // Schema columns: jumlah_kk, laki, perempuan, total (UNSIGNED INT generated).
        // Cast to SIGNED so they decode as i64 cleanly across all rows.
        let rows: Vec<PendudukRow> = sqlx::query_as::<_, PendudukRow>(
            "SELECT pedukuhan, \
                    CAST(total AS SIGNED)     AS total_penduduk, \
                    CAST(laki AS SIGNED)      AS total_laki, \
                    CAST(perempuan AS SIGNED) AS total_perempuan, \
                    CAST(jumlah_kk AS SIGNED) AS total_kk, \
                    updated_at \
             FROM penduduk_ringkasan \
             ORDER BY pedukuhan ASC",
        )
        .fetch_all(&self.pool)
        .await?;

        let per_pedukuhan: Vec<PendudukPedukuhan> =
            rows.into_iter().map(PendudukPedukuhan::from).collect();

        let total_penduduk = per_pedukuhan.iter().map(|p| p.total_penduduk).sum();
        let total_laki = per_pedukuhan.iter().map(|p| p.total_laki).sum();
        let total_perempuan = per_pedukuhan.iter().map(|p| p.total_perempuan).sum();
        let total_kk = per_pedukuhan.iter().map(|p| p.total_kk).sum();

        Ok(PendudukRingkasan {
            total_penduduk,
            total_laki,
            total_perempuan,
            total_kk,
            per_pedukuhan,
        })
    }
}

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPool;
use sqlx::{FromRow, QueryBuilder};

use crate::domain::penduduk::{PendudukPatch, PendudukPedukuhan, PendudukRingkasan};
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

/// Casts UNSIGNED columns to SIGNED so they decode as `i64` cleanly. Reused
/// by both `ringkasan` (list) and the single-row reads after update.
const SELECT_PEDUKUHAN_COLUMNS: &str =
    "SELECT pedukuhan, \
            CAST(total AS SIGNED)     AS total_penduduk, \
            CAST(laki AS SIGNED)      AS total_laki, \
            CAST(perempuan AS SIGNED) AS total_perempuan, \
            CAST(jumlah_kk AS SIGNED) AS total_kk, \
            updated_at \
     FROM penduduk_ringkasan";

#[async_trait]
impl PendudukRepository for MysqlPendudukRepository {
    async fn ringkasan(&self) -> Result<PendudukRingkasan, AppError> {
        let sql = format!("{SELECT_PEDUKUHAN_COLUMNS} ORDER BY pedukuhan ASC");
        let rows: Vec<PendudukRow> = sqlx::query_as::<_, PendudukRow>(&sql)
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

    async fn update_pedukuhan(
        &self,
        pedukuhan: &str,
        patch: PendudukPatch,
    ) -> Result<Option<PendudukPedukuhan>, AppError> {
        // Empty patch is a no-op; still report whether the row exists.
        let is_empty = patch.jumlah_kk.is_none()
            && patch.laki.is_none()
            && patch.perempuan.is_none();

        if !is_empty {
            let mut qb: QueryBuilder<sqlx::MySql> =
                QueryBuilder::new("UPDATE penduduk_ringkasan SET ");
            let mut sep = qb.separated(", ");

            if let Some(v) = patch.jumlah_kk {
                sep.push("jumlah_kk = ").push_bind_unseparated(v);
            }
            if let Some(v) = patch.laki {
                sep.push("laki = ").push_bind_unseparated(v);
            }
            if let Some(v) = patch.perempuan {
                sep.push("perempuan = ").push_bind_unseparated(v);
            }

            qb.push(" WHERE pedukuhan = ").push_bind(pedukuhan);

            qb.build().execute(&self.pool).await?;
        }

        let sql = format!("{SELECT_PEDUKUHAN_COLUMNS} WHERE pedukuhan = ?");
        let row: Option<PendudukRow> = sqlx::query_as::<_, PendudukRow>(&sql)
            .bind(pedukuhan)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(PendudukPedukuhan::from))
    }
}

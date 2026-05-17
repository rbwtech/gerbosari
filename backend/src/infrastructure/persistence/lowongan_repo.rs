use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::mysql::MySqlPool;
use sqlx::{FromRow, QueryBuilder};
use uuid::Uuid;

use crate::domain::lowongan::{KategoriLowongan, Lowongan, StatusLowongan};
use crate::domain::repository::{LowonganFilter, LowonganRepository};
use crate::error::AppError;

pub struct MysqlLowonganRepository {
    pool: MySqlPool,
}

impl MysqlLowonganRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct LowonganRow {
    id: String,
    judul: String,
    instansi: String,
    kategori: String,
    deskripsi: String,
    kontak: String,
    gaji_min: Option<i64>,
    gaji_max: Option<i64>,
    deadline: Option<NaiveDate>,
    lokasi_pedukuhan: Option<String>,
    status: String,
    created_at: NaiveDateTime,
}

impl TryFrom<LowonganRow> for Lowongan {
    type Error = AppError;

    fn try_from(row: LowonganRow) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&row.id)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("invalid lowongan id: {e}")))?;
        let kategori = KategoriLowongan::parse(&row.kategori).ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!(
                "unknown lowongan kategori in db: {}",
                row.kategori
            ))
        })?;
        let status = StatusLowongan::parse(&row.status).ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!(
                "unknown lowongan status in db: {}",
                row.status
            ))
        })?;
        Ok(Lowongan {
            id,
            judul: row.judul,
            instansi: row.instansi,
            kategori,
            deskripsi: row.deskripsi,
            kontak: row.kontak,
            gaji_min: row.gaji_min,
            gaji_max: row.gaji_max,
            deadline: row.deadline,
            lokasi_pedukuhan: row.lokasi_pedukuhan,
            status,
            created_at: row.created_at.and_utc(),
        })
    }
}

const SELECT_COLUMNS: &str =
    "SELECT id, judul, instansi, kategori, deskripsi, kontak, \
            CAST(gaji_min AS SIGNED) AS gaji_min, \
            CAST(gaji_max AS SIGNED) AS gaji_max, \
            deadline, lokasi_pedukuhan, status, created_at \
     FROM lowongan";

#[async_trait]
impl LowonganRepository for MysqlLowonganRepository {
    async fn list(&self, filter: LowonganFilter) -> Result<Vec<Lowongan>, AppError> {
        let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(SELECT_COLUMNS);
        qb.push(" WHERE 1=1");

        if let Some(k) = filter.kategori {
            qb.push(" AND kategori = ").push_bind(k.as_str());
        }
        if let Some(s) = filter.status {
            qb.push(" AND status = ").push_bind(s.as_str());
        }

        qb.push(" ORDER BY created_at DESC");

        let rows: Vec<LowonganRow> = qb
            .build_query_as::<LowonganRow>()
            .fetch_all(&self.pool)
            .await?;

        rows.into_iter().map(Lowongan::try_from).collect()
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Lowongan, AppError> {
        let sql = format!("{SELECT_COLUMNS} WHERE id = ?");
        let row: Option<LowonganRow> = sqlx::query_as::<_, LowonganRow>(&sql)
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => Lowongan::try_from(r),
            None => Err(AppError::NotFound),
        }
    }
}

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::mysql::MySqlPool;
use sqlx::{FromRow, QueryBuilder};
use uuid::Uuid;

use crate::domain::lowongan::{
    KategoriLowongan, Lowongan, LowonganPatch, NewLowongan, StatusLowongan,
};
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

    async fn create(&self, input: NewLowongan) -> Result<Lowongan, AppError> {
        // Domain enforces gaji_min <= gaji_max via the CHECK constraint at DB
        // level; surface MySQL error as a 400 in the handler via AppError mapping.
        let id = Uuid::new_v4();
        let id_str = id.to_string();

        sqlx::query(
            "INSERT INTO lowongan (id, judul, instansi, kategori, deskripsi, kontak, \
                                   gaji_min, gaji_max, deadline, lokasi_pedukuhan, status) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id_str)
        .bind(&input.judul)
        .bind(&input.instansi)
        .bind(input.kategori.as_str())
        .bind(&input.deskripsi)
        .bind(&input.kontak)
        .bind(input.gaji_min)
        .bind(input.gaji_max)
        .bind(input.deadline)
        .bind(&input.lokasi_pedukuhan)
        .bind(input.status.as_str())
        .execute(&self.pool)
        .await?;

        self.get_by_id(id).await
    }

    async fn update(
        &self,
        id: Uuid,
        patch: LowonganPatch,
    ) -> Result<Option<Lowongan>, AppError> {
        if patch.judul.is_none()
            && patch.instansi.is_none()
            && patch.kategori.is_none()
            && patch.deskripsi.is_none()
            && patch.kontak.is_none()
            && patch.gaji_min.is_none()
            && patch.gaji_max.is_none()
            && patch.deadline.is_none()
            && patch.lokasi_pedukuhan.is_none()
            && patch.status.is_none()
        {
            return match self.get_by_id(id).await {
                Ok(l) => Ok(Some(l)),
                Err(AppError::NotFound) => Ok(None),
                Err(e) => Err(e),
            };
        }

        let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE lowongan SET ");
        let mut sep = qb.separated(", ");

        if let Some(judul) = patch.judul.as_ref() {
            sep.push("judul = ").push_bind_unseparated(judul);
        }
        if let Some(instansi) = patch.instansi.as_ref() {
            sep.push("instansi = ").push_bind_unseparated(instansi);
        }
        if let Some(kategori) = patch.kategori {
            sep.push("kategori = ").push_bind_unseparated(kategori.as_str());
        }
        if let Some(deskripsi) = patch.deskripsi.as_ref() {
            sep.push("deskripsi = ").push_bind_unseparated(deskripsi);
        }
        if let Some(kontak) = patch.kontak.as_ref() {
            sep.push("kontak = ").push_bind_unseparated(kontak);
        }
        if let Some(gaji_min) = patch.gaji_min {
            sep.push("gaji_min = ").push_bind_unseparated(gaji_min);
        }
        if let Some(gaji_max) = patch.gaji_max {
            sep.push("gaji_max = ").push_bind_unseparated(gaji_max);
        }
        if let Some(deadline) = patch.deadline {
            sep.push("deadline = ").push_bind_unseparated(deadline);
        }
        if let Some(lokasi) = patch.lokasi_pedukuhan.as_ref() {
            sep.push("lokasi_pedukuhan = ").push_bind_unseparated(lokasi.clone());
        }
        if let Some(status) = patch.status {
            sep.push("status = ").push_bind_unseparated(status.as_str());
        }

        qb.push(" WHERE id = ").push_bind(id.to_string());

        let result = qb.build().execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            return match self.get_by_id(id).await {
                Ok(l) => Ok(Some(l)),
                Err(AppError::NotFound) => Ok(None),
                Err(e) => Err(e),
            };
        }

        match self.get_by_id(id).await {
            Ok(l) => Ok(Some(l)),
            Err(AppError::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM lowongan WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

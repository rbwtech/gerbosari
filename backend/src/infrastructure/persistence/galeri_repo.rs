use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::galeri::{Galeri, KategoriGaleri};
use crate::domain::repository::GaleriRepository;
use crate::error::AppError;

pub struct MysqlGaleriRepository {
    pool: MySqlPool,
}

impl MysqlGaleriRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct GaleriRow {
    id: String,
    judul: String,
    deskripsi: Option<String>,
    file_path: String,
    kategori: String,
    taken_at: Option<NaiveDate>,
    created_at: NaiveDateTime,
}

impl TryFrom<GaleriRow> for Galeri {
    type Error = AppError;

    fn try_from(row: GaleriRow) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&row.id)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("invalid galeri id: {e}")))?;
        let kategori = KategoriGaleri::parse(&row.kategori).ok_or_else(|| {
            AppError::Internal(anyhow::anyhow!(
                "unknown galeri kategori in db: {}",
                row.kategori
            ))
        })?;
        Ok(Galeri {
            id,
            judul: row.judul,
            deskripsi: row.deskripsi,
            file_path: row.file_path,
            kategori,
            taken_at: row.taken_at,
            created_at: row.created_at.and_utc(),
        })
    }
}

#[async_trait]
impl GaleriRepository for MysqlGaleriRepository {
    async fn list(&self, kategori: Option<KategoriGaleri>) -> Result<Vec<Galeri>, AppError> {
        let rows: Vec<GaleriRow> = match kategori {
            Some(k) => sqlx::query_as::<_, GaleriRow>(
                "SELECT id, judul, deskripsi, file_path, kategori, taken_at, created_at \
                 FROM galeri WHERE kategori = ? ORDER BY taken_at DESC, created_at DESC",
            )
            .bind(k.as_str())
            .fetch_all(&self.pool)
            .await?,
            None => sqlx::query_as::<_, GaleriRow>(
                "SELECT id, judul, deskripsi, file_path, kategori, taken_at, created_at \
                 FROM galeri ORDER BY taken_at DESC, created_at DESC",
            )
            .fetch_all(&self.pool)
            .await?,
        };

        rows.into_iter().map(Galeri::try_from).collect()
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Galeri, AppError> {
        let row: Option<GaleriRow> = sqlx::query_as::<_, GaleriRow>(
            "SELECT id, judul, deskripsi, file_path, kategori, taken_at, created_at \
             FROM galeri WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Galeri::try_from(r),
            None => Err(AppError::NotFound),
        }
    }
}

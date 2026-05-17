use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::mysql::MySqlPool;
use sqlx::{FromRow, QueryBuilder};
use uuid::Uuid;

use crate::domain::galeri::{Galeri, GaleriPatch, KategoriGaleri, NewGaleri};
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

    async fn create(&self, input: NewGaleri) -> Result<Galeri, AppError> {
        // Server-side id generation keeps `id` stable even if the client retries
        // an insert. v4 is sufficient for our scale; v7 would need an extra
        // crate feature flag.
        let id = Uuid::new_v4();
        let id_str = id.to_string();

        sqlx::query(
            "INSERT INTO galeri (id, judul, deskripsi, file_path, kategori, taken_at) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&id_str)
        .bind(&input.judul)
        .bind(&input.deskripsi)
        .bind(&input.file_path)
        .bind(input.kategori.as_str())
        .bind(input.taken_at)
        .execute(&self.pool)
        .await?;

        // Re-fetch so created_at reflects the DB DEFAULT timestamp the row was
        // actually written with — avoids client/server clock drift.
        self.get_by_id(id).await
    }

    async fn update(
        &self,
        id: Uuid,
        patch: GaleriPatch,
    ) -> Result<Option<Galeri>, AppError> {
        // No-op patch: confirm the row exists (so callers get 404 vs 200 with
        // no changes) and return it unchanged.
        if patch.judul.is_none()
            && patch.deskripsi.is_none()
            && patch.file_path.is_none()
            && patch.kategori.is_none()
            && patch.taken_at.is_none()
        {
            return match self.get_by_id(id).await {
                Ok(g) => Ok(Some(g)),
                Err(AppError::NotFound) => Ok(None),
                Err(e) => Err(e),
            };
        }

        let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE galeri SET ");
        let mut sep = qb.separated(", ");

        if let Some(judul) = patch.judul.as_ref() {
            sep.push("judul = ").push_bind_unseparated(judul);
        }
        if let Some(deskripsi) = patch.deskripsi.as_ref() {
            sep.push("deskripsi = ").push_bind_unseparated(deskripsi.clone());
        }
        if let Some(file_path) = patch.file_path.as_ref() {
            sep.push("file_path = ").push_bind_unseparated(file_path);
        }
        if let Some(kategori) = patch.kategori {
            sep.push("kategori = ").push_bind_unseparated(kategori.as_str());
        }
        if let Some(taken_at) = patch.taken_at.as_ref() {
            sep.push("taken_at = ").push_bind_unseparated(*taken_at);
        }

        qb.push(" WHERE id = ").push_bind(id.to_string());

        let result = qb.build().execute(&self.pool).await?;

        // rows_affected == 0 could mean either "no such id" or "values matched
        // existing row". Disambiguate with a follow-up SELECT.
        if result.rows_affected() == 0 {
            return match self.get_by_id(id).await {
                Ok(g) => Ok(Some(g)),
                Err(AppError::NotFound) => Ok(None),
                Err(e) => Err(e),
            };
        }

        match self.get_by_id(id).await {
            Ok(g) => Ok(Some(g)),
            Err(AppError::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM galeri WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

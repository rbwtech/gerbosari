use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use sqlx::mysql::MySqlPool;
use sqlx::{FromRow, QueryBuilder};
use uuid::Uuid;

use crate::domain::berita::{Berita, BeritaPatch, BeritaRingkasan, KategoriBerita, NewBerita};
use crate::domain::repository::BeritaRepository;
use crate::error::AppError;

pub struct MysqlBeritaRepository {
    pool: MySqlPool,
}

impl MysqlBeritaRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct BeritaRingkasanRow {
    id: String,
    judul: String,
    slug: String,
    kategori: String,
    ringkasan: String,
    gambar_url: Option<String>,
    tanggal_acara: Option<NaiveDate>,
    author: String,
    published_at: NaiveDateTime,
}

#[derive(Debug, FromRow)]
struct BeritaRow {
    id: String,
    judul: String,
    slug: String,
    kategori: String,
    ringkasan: String,
    konten: String,
    gambar_url: Option<String>,
    tanggal_acara: Option<NaiveDate>,
    lokasi: Option<String>,
    author: String,
    published_at: NaiveDateTime,
}

const FULL_SELECT: &str = "SELECT id, judul, slug, kategori, ringkasan, konten, gambar_url, \
                           tanggal_acara, lokasi, author, published_at FROM berita";

fn parse_uuid(s: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(s).map_err(|e| AppError::Internal(anyhow::anyhow!("invalid berita id: {e}")))
}

fn parse_kategori(s: &str) -> Result<KategoriBerita, AppError> {
    KategoriBerita::parse(s).ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("unknown berita kategori in db: {s}"))
    })
}

impl TryFrom<BeritaRingkasanRow> for BeritaRingkasan {
    type Error = AppError;

    fn try_from(row: BeritaRingkasanRow) -> Result<Self, Self::Error> {
        Ok(BeritaRingkasan {
            id: parse_uuid(&row.id)?,
            judul: row.judul,
            slug: row.slug,
            kategori: parse_kategori(&row.kategori)?,
            ringkasan: row.ringkasan,
            gambar_url: row.gambar_url,
            tanggal_acara: row.tanggal_acara,
            author: row.author,
            published_at: row.published_at.and_utc(),
        })
    }
}

impl TryFrom<BeritaRow> for Berita {
    type Error = AppError;

    fn try_from(row: BeritaRow) -> Result<Self, Self::Error> {
        Ok(Berita {
            id: parse_uuid(&row.id)?,
            judul: row.judul,
            slug: row.slug,
            kategori: parse_kategori(&row.kategori)?,
            ringkasan: row.ringkasan,
            konten: row.konten,
            gambar_url: row.gambar_url,
            tanggal_acara: row.tanggal_acara,
            lokasi: row.lokasi,
            author: row.author,
            published_at: row.published_at.and_utc(),
        })
    }
}

#[async_trait]
impl BeritaRepository for MysqlBeritaRepository {
    async fn list(
        &self,
        kategori: Option<KategoriBerita>,
    ) -> Result<Vec<BeritaRingkasan>, AppError> {
        let rows: Vec<BeritaRingkasanRow> = match kategori {
            Some(k) => sqlx::query_as::<_, BeritaRingkasanRow>(
                "SELECT id, judul, slug, kategori, ringkasan, gambar_url, tanggal_acara, author, \
                 published_at FROM berita WHERE kategori = ? ORDER BY published_at DESC",
            )
            .bind(k.as_str())
            .fetch_all(&self.pool)
            .await?,
            None => sqlx::query_as::<_, BeritaRingkasanRow>(
                "SELECT id, judul, slug, kategori, ringkasan, gambar_url, tanggal_acara, author, \
                 published_at FROM berita ORDER BY published_at DESC",
            )
            .fetch_all(&self.pool)
            .await?,
        };

        rows.into_iter().map(BeritaRingkasan::try_from).collect()
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Berita, AppError> {
        let sql = format!("{FULL_SELECT} WHERE slug = ?");
        let row: Option<BeritaRow> = sqlx::query_as::<_, BeritaRow>(&sql)
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => Berita::try_from(r),
            None => Err(AppError::NotFound),
        }
    }

    async fn create(&self, input: NewBerita) -> Result<Berita, AppError> {
        let id = Uuid::new_v4();
        // Default `published_at` to "now" so admins can publish instantly
        // without supplying a redundant timestamp on the wire.
        let published_at = input.published_at.unwrap_or_else(Utc::now);

        sqlx::query(
            "INSERT INTO berita \
             (id, judul, slug, kategori, ringkasan, konten, gambar_url, tanggal_acara, lokasi, \
              author, published_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id.to_string())
        .bind(&input.judul)
        .bind(&input.slug)
        .bind(input.kategori.as_str())
        .bind(&input.ringkasan)
        .bind(&input.konten)
        .bind(&input.gambar_url)
        .bind(input.tanggal)
        .bind(&input.lokasi)
        .bind(&input.author)
        .bind(published_at.naive_utc())
        .execute(&self.pool)
        .await?;

        // Re-fetch by slug so the returned row reflects DB defaults (e.g.
        // `created_at`) and any column-level coercions.
        self.get_by_slug(&input.slug).await
    }

    async fn update(
        &self,
        slug: &str,
        patch: BeritaPatch,
    ) -> Result<Option<Berita>, AppError> {
        // Short-circuit when there is nothing to update; still verify the row
        // exists so the handler's NotFound contract is preserved.
        if is_empty_patch(&patch) {
            return self.fetch_by_slug_optional(slug).await;
        }

        let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE berita SET ");
        let mut sep = qb.separated(", ");

        if let Some(v) = &patch.judul {
            sep.push("judul = ").push_bind_unseparated(v);
        }
        if let Some(v) = patch.kategori {
            sep.push("kategori = ").push_bind_unseparated(v.as_str());
        }
        if let Some(v) = &patch.ringkasan {
            sep.push("ringkasan = ").push_bind_unseparated(v);
        }
        if let Some(v) = &patch.konten {
            sep.push("konten = ").push_bind_unseparated(v);
        }
        if let Some(v) = &patch.gambar_url {
            sep.push("gambar_url = ").push_bind_unseparated(v);
        }
        if let Some(v) = &patch.tanggal {
            sep.push("tanggal_acara = ").push_bind_unseparated(v);
        }
        if let Some(v) = &patch.lokasi {
            sep.push("lokasi = ").push_bind_unseparated(v);
        }
        if let Some(v) = &patch.author {
            sep.push("author = ").push_bind_unseparated(v);
        }
        if let Some(v) = patch.published_at {
            sep.push("published_at = ").push_bind_unseparated(v.naive_utc());
        }

        qb.push(" WHERE slug = ").push_bind(slug);

        let result = qb.build().execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            // MySQL reports 0 rows_affected both for "row missing" and for
            // "row exists but no column changed". Disambiguate via SELECT.
            return self.fetch_by_slug_optional(slug).await;
        }

        self.fetch_by_slug_optional(slug).await
    }

    async fn delete(&self, slug: &str) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM berita WHERE slug = ?")
            .bind(slug)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

impl MysqlBeritaRepository {
    async fn fetch_by_slug_optional(&self, slug: &str) -> Result<Option<Berita>, AppError> {
        let sql = format!("{FULL_SELECT} WHERE slug = ?");
        let row: Option<BeritaRow> = sqlx::query_as::<_, BeritaRow>(&sql)
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => Ok(Some(Berita::try_from(r)?)),
            None => Ok(None),
        }
    }
}

fn is_empty_patch(p: &BeritaPatch) -> bool {
    p.judul.is_none()
        && p.kategori.is_none()
        && p.ringkasan.is_none()
        && p.konten.is_none()
        && p.gambar_url.is_none()
        && p.tanggal.is_none()
        && p.lokasi.is_none()
        && p.author.is_none()
        && p.published_at.is_none()
}

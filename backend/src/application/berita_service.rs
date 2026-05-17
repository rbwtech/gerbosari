use std::sync::Arc;

use crate::domain::berita::{Berita, BeritaPatch, BeritaRingkasan, KategoriBerita, NewBerita};
use crate::domain::repository::BeritaRepository;
use crate::error::AppError;

pub struct BeritaService {
    repo: Arc<dyn BeritaRepository>,
}

impl BeritaService {
    pub fn new(repo: Arc<dyn BeritaRepository>) -> Self {
        Self { repo }
    }

    pub async fn list(
        &self,
        kategori: Option<KategoriBerita>,
    ) -> Result<Vec<BeritaRingkasan>, AppError> {
        self.repo.list(kategori).await
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Berita, AppError> {
        if slug.is_empty() {
            return Err(AppError::BadRequest("slug tidak boleh kosong".into()));
        }
        self.repo.get_by_slug(slug).await
    }

    /// Persists a new article. Slug uniqueness is enforced by the DB unique
    /// index — duplicate slugs surface as `AppError::Database` and become 500
    /// today; refining to a 409 mapping is a TODO until we add a dedicated
    /// `Conflict` variant.
    pub async fn create(&self, input: NewBerita) -> Result<Berita, AppError> {
        self.repo.create(input).await
    }

    pub async fn update(&self, slug: &str, patch: BeritaPatch) -> Result<Berita, AppError> {
        if slug.is_empty() {
            return Err(AppError::BadRequest("slug tidak boleh kosong".into()));
        }
        self.repo
            .update(slug, patch)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn delete(&self, slug: &str) -> Result<(), AppError> {
        if slug.is_empty() {
            return Err(AppError::BadRequest("slug tidak boleh kosong".into()));
        }
        if self.repo.delete(slug).await? {
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}

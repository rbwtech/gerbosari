use std::sync::Arc;

use crate::domain::berita::{Berita, BeritaRingkasan, KategoriBerita};
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
            return Err(AppError::BadRequest("slug must not be empty".into()));
        }
        self.repo.get_by_slug(slug).await
    }
}

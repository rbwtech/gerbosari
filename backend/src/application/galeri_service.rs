use std::sync::Arc;
use uuid::Uuid;

use crate::domain::galeri::{Galeri, GaleriPatch, KategoriGaleri, NewGaleri};
use crate::domain::repository::GaleriRepository;
use crate::error::AppError;

pub struct GaleriService {
    repo: Arc<dyn GaleriRepository>,
}

impl GaleriService {
    pub fn new(repo: Arc<dyn GaleriRepository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self, kategori: Option<KategoriGaleri>) -> Result<Vec<Galeri>, AppError> {
        self.repo.list(kategori).await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Galeri, AppError> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, input: NewGaleri) -> Result<Galeri, AppError> {
        self.repo.create(input).await
    }

    /// Maps the repository's "no matching id" (`Ok(None)`) into the canonical
    /// `AppError::NotFound`, so handlers don't need to do this dance themselves.
    pub async fn update(&self, id: Uuid, patch: GaleriPatch) -> Result<Galeri, AppError> {
        match self.repo.update(id, patch).await? {
            Some(g) => Ok(g),
            None => Err(AppError::NotFound),
        }
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        if self.repo.delete(id).await? {
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}

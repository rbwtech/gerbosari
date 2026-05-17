use std::sync::Arc;
use uuid::Uuid;

use crate::domain::galeri::{Galeri, KategoriGaleri};
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
}

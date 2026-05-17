use std::sync::Arc;
use uuid::Uuid;

use crate::domain::lowongan::{Lowongan, LowonganPatch, NewLowongan};
use crate::domain::repository::{LowonganFilter, LowonganRepository};
use crate::error::AppError;

pub struct LowonganService {
    repo: Arc<dyn LowonganRepository>,
}

impl LowonganService {
    pub fn new(repo: Arc<dyn LowonganRepository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self, filter: LowonganFilter) -> Result<Vec<Lowongan>, AppError> {
        self.repo.list(filter).await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Lowongan, AppError> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, input: NewLowongan) -> Result<Lowongan, AppError> {
        self.repo.create(input).await
    }

    pub async fn update(&self, id: Uuid, patch: LowonganPatch) -> Result<Lowongan, AppError> {
        match self.repo.update(id, patch).await? {
            Some(l) => Ok(l),
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

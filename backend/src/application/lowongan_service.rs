use std::sync::Arc;
use uuid::Uuid;

use crate::domain::lowongan::Lowongan;
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
}

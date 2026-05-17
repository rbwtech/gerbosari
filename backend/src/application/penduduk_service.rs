use std::sync::Arc;

use crate::domain::penduduk::PendudukRingkasan;
use crate::domain::repository::PendudukRepository;
use crate::error::AppError;

pub struct PendudukService {
    repo: Arc<dyn PendudukRepository>,
}

impl PendudukService {
    pub fn new(repo: Arc<dyn PendudukRepository>) -> Self {
        Self { repo }
    }

    pub async fn ringkasan(&self) -> Result<PendudukRingkasan, AppError> {
        self.repo.ringkasan().await
    }
}

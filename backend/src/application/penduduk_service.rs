use std::sync::Arc;

use crate::domain::penduduk::{PendudukPatch, PendudukPedukuhan, PendudukRingkasan};
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

    /// Updates a single pedukuhan row. Returns `NotFound` when the pedukuhan
    /// is not part of the immutable seed list (the 19 dusun of Desa Gerbosari
    /// are reference data and cannot be created via API).
    pub async fn update_pedukuhan(
        &self,
        pedukuhan: &str,
        patch: PendudukPatch,
    ) -> Result<PendudukPedukuhan, AppError> {
        if pedukuhan.is_empty() {
            return Err(AppError::BadRequest("nama pedukuhan tidak boleh kosong".into()));
        }
        self.repo
            .update_pedukuhan(pedukuhan, patch)
            .await?
            .ok_or(AppError::NotFound)
    }
}

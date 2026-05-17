use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::berita::{Berita, BeritaRingkasan, KategoriBerita};
use crate::domain::galeri::{Galeri, KategoriGaleri};
use crate::domain::lowongan::{KategoriLowongan, Lowongan, StatusLowongan};
use crate::domain::penduduk::PendudukRingkasan;
use crate::error::AppError;

/// Read-only repository for `galeri`. List filter is the kategori facet from
/// the public API; `None` means "all categories".
#[async_trait]
pub trait GaleriRepository: Send + Sync {
    async fn list(&self, kategori: Option<KategoriGaleri>) -> Result<Vec<Galeri>, AppError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Galeri, AppError>;
}

#[async_trait]
pub trait PendudukRepository: Send + Sync {
    async fn ringkasan(&self) -> Result<PendudukRingkasan, AppError>;
}

#[derive(Debug, Default, Clone)]
pub struct LowonganFilter {
    pub kategori: Option<KategoriLowongan>,
    pub status: Option<StatusLowongan>,
}

#[async_trait]
pub trait LowonganRepository: Send + Sync {
    async fn list(&self, filter: LowonganFilter) -> Result<Vec<Lowongan>, AppError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Lowongan, AppError>;
}

#[async_trait]
pub trait BeritaRepository: Send + Sync {
    async fn list(
        &self,
        kategori: Option<KategoriBerita>,
    ) -> Result<Vec<BeritaRingkasan>, AppError>;
    async fn get_by_slug(&self, slug: &str) -> Result<Berita, AppError>;
}

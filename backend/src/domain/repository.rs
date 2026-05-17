use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::admin_user::AdminUser;
use crate::domain::berita::{
    Berita, BeritaPatch, BeritaRingkasan, KategoriBerita, NewBerita,
};
use crate::domain::galeri::{Galeri, GaleriPatch, KategoriGaleri, NewGaleri};
use crate::domain::lowongan::{
    KategoriLowongan, Lowongan, LowonganPatch, NewLowongan, StatusLowongan,
};
use crate::domain::penduduk::{PendudukPatch, PendudukPedukuhan, PendudukRingkasan};
use crate::error::AppError;

// ============================================================================
// Galeri
// ============================================================================

/// Read + write repository for `galeri`. List filter is the kategori facet
/// from the public API; `None` means "all categories". Write methods are
/// invoked only from authenticated admin handlers.
#[async_trait]
pub trait GaleriRepository: Send + Sync {
    async fn list(&self, kategori: Option<KategoriGaleri>) -> Result<Vec<Galeri>, AppError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Galeri, AppError>;

    async fn create(&self, input: NewGaleri) -> Result<Galeri, AppError>;
    async fn update(&self, id: Uuid, patch: GaleriPatch) -> Result<Option<Galeri>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<bool, AppError>;
}

// ============================================================================
// Penduduk
// ============================================================================

#[async_trait]
pub trait PendudukRepository: Send + Sync {
    async fn ringkasan(&self) -> Result<PendudukRingkasan, AppError>;

    /// Updates a single pedukuhan row in place. Returns `None` when the
    /// pedukuhan name is not in the fixed seed list. Creating/deleting
    /// pedukuhan is intentionally not part of the contract — the 19 dusun
    /// of Desa Gerbosari are immutable reference data.
    async fn update_pedukuhan(
        &self,
        pedukuhan: &str,
        patch: PendudukPatch,
    ) -> Result<Option<PendudukPedukuhan>, AppError>;
}

// ============================================================================
// Lowongan
// ============================================================================

#[derive(Debug, Default, Clone)]
pub struct LowonganFilter {
    pub kategori: Option<KategoriLowongan>,
    pub status: Option<StatusLowongan>,
}

#[async_trait]
pub trait LowonganRepository: Send + Sync {
    async fn list(&self, filter: LowonganFilter) -> Result<Vec<Lowongan>, AppError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Lowongan, AppError>;

    async fn create(&self, input: NewLowongan) -> Result<Lowongan, AppError>;
    async fn update(
        &self,
        id: Uuid,
        patch: LowonganPatch,
    ) -> Result<Option<Lowongan>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<bool, AppError>;
}

// ============================================================================
// Berita
// ============================================================================

#[async_trait]
pub trait BeritaRepository: Send + Sync {
    async fn list(
        &self,
        kategori: Option<KategoriBerita>,
    ) -> Result<Vec<BeritaRingkasan>, AppError>;
    async fn get_by_slug(&self, slug: &str) -> Result<Berita, AppError>;

    async fn create(&self, input: NewBerita) -> Result<Berita, AppError>;
    async fn update(
        &self,
        slug: &str,
        patch: BeritaPatch,
    ) -> Result<Option<Berita>, AppError>;
    async fn delete(&self, slug: &str) -> Result<bool, AppError>;
}

// ============================================================================
// Admin user
// ============================================================================

/// Repository for backoffice administrator credentials. `upsert` is used by
/// the bootstrap flow at startup so the operator can rotate the admin
/// password via the `ADMIN_PASSWORD` environment variable.
#[async_trait]
pub trait AdminUserRepository: Send + Sync {
    async fn find_by_username(&self, username: &str) -> Result<Option<AdminUser>, AppError>;
    async fn upsert(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<AdminUser, AppError>;
}

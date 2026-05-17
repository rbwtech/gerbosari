use std::sync::Arc;

use crate::application::berita_service::BeritaService;
use crate::application::galeri_service::GaleriService;
use crate::application::lowongan_service::LowonganService;
use crate::application::penduduk_service::PendudukService;

#[derive(Clone)]
pub struct AppState {
    pub galeri: Arc<GaleriService>,
    pub penduduk: Arc<PendudukService>,
    pub lowongan: Arc<LowonganService>,
    pub berita: Arc<BeritaService>,
}

use std::sync::Arc;

use crate::application::auth_service::AuthService;
use crate::application::berita_service::BeritaService;
use crate::application::galeri_service::GaleriService;
use crate::application::lowongan_service::LowonganService;
use crate::application::penduduk_service::PendudukService;
use crate::infrastructure::auth::JwtEncoder;
use crate::infrastructure::rate_limit::LoginThrottle;

/// Shared, cheaply-cloned handle injected into every Axum handler. All
/// fields are `Arc`-wrapped because Axum clones the state per request.
#[derive(Clone)]
pub struct AppState {
    pub galeri: Arc<GaleriService>,
    pub penduduk: Arc<PendudukService>,
    pub lowongan: Arc<LowonganService>,
    pub berita: Arc<BeritaService>,
    pub auth: Arc<AuthService>,
    /// Exposed so handlers (or future middleware) can issue / verify tokens
    /// without going through `AuthService`. Kept distinct from `auth` to
    /// avoid accidental coupling.
    pub jwt: Arc<JwtEncoder>,
    /// Per-IP login attempt limiter. Single-instance, in-memory.
    pub login_throttle: Arc<LoginThrottle>,
}

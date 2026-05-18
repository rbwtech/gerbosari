use axum::extract::DefaultBodyLimit;
use axum::http::{header, HeaderValue, Method};
use axum::routing::{get, patch, post};
use axum::Router;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::presentation::handlers;
use crate::presentation::state::AppState;

/// Builds the full HTTP router. Public routes (read-only catalogue + login)
/// live alongside admin routes that gate on `AuthGuard` at the handler
/// signature level — no shared layer to keep per-route auth visible.
pub fn build_router(state: AppState, cors_origins: &[String]) -> Router {
    let cors = build_cors(cors_origins);

    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    // ------------------------------------------------------------------
    // Public API
    // ------------------------------------------------------------------
    let public = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/galeri", get(handlers::galeri::list))
        .route("/galeri/:id", get(handlers::galeri::get_by_id))
        .route("/penduduk/ringkasan", get(handlers::penduduk::ringkasan))
        .route("/lowongan", get(handlers::lowongan::list))
        .route("/lowongan/:id", get(handlers::lowongan::get_by_id))
        .route("/berita", get(handlers::berita::list))
        .route("/berita/:slug", get(handlers::berita::get_by_slug))
        .route("/auth/login", post(handlers::auth::login));

    // ------------------------------------------------------------------
    // Admin API — every handler takes `AuthGuard` as an extractor so the
    // 401 path is owned by the middleware. Module paths must match exactly
    // what parallel CRUD agents implement.
    // ------------------------------------------------------------------
    let admin = Router::new()
        .route("/galeri", post(handlers::galeri::admin_create))
        .route(
            "/galeri/:id",
            patch(handlers::galeri::admin_update).delete(handlers::galeri::admin_delete),
        )
        .route("/lowongan", post(handlers::lowongan::admin_create))
        .route(
            "/lowongan/:id",
            patch(handlers::lowongan::admin_update).delete(handlers::lowongan::admin_delete),
        )
        .route("/berita", post(handlers::berita::admin_create))
        .route(
            "/berita/:slug",
            patch(handlers::berita::admin_update).delete(handlers::berita::admin_delete),
        )
        .route(
            "/penduduk/:pedukuhan",
            patch(handlers::penduduk::admin_update),
        )
        // Multipart image upload. The route-level `DefaultBodyLimit` caps the
        // total request body at 8 MiB so a misbehaving client can't tie up
        // memory; the handler additionally enforces a per-file 5 MiB cap.
        .route(
            "/upload",
            post(handlers::upload::admin_upload)
                .layer(DefaultBodyLimit::max(8 * 1024 * 1024)),
        );

    let api = Router::new().merge(public).nest("/admin", admin);

    Router::new()
        .nest("/api", api)
        .with_state(state)
        .layer(cors)
        .layer(trace)
}

fn build_cors(origins: &[String]) -> CorsLayer {
    let parsed: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|o| HeaderValue::from_str(o).ok())
        .collect();

    let allow_origin = if parsed.is_empty() {
        AllowOrigin::list(std::iter::empty::<HeaderValue>())
    } else {
        AllowOrigin::list(parsed)
    };

    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
}

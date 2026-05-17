use axum::http::{header, HeaderValue, Method};
use axum::routing::get;
use axum::Router;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::presentation::handlers;
use crate::presentation::state::AppState;

pub fn build_router(state: AppState, cors_origins: &[String]) -> Router {
    let cors = build_cors(cors_origins);

    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let api = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/galeri", get(handlers::galeri::list))
        .route("/galeri/:id", get(handlers::galeri::get_by_id))
        .route("/penduduk/ringkasan", get(handlers::penduduk::ringkasan))
        .route("/lowongan", get(handlers::lowongan::list))
        .route("/lowongan/:id", get(handlers::lowongan::get_by_id))
        .route("/berita", get(handlers::berita::list))
        .route("/berita/:slug", get(handlers::berita::get_by_slug));

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
        .allow_methods([Method::GET])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT])
}

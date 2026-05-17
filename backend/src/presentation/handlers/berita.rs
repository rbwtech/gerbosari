use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use validator::Validate;

use crate::domain::berita::KategoriBerita;
use crate::error::AppError;
use crate::presentation::dto::berita_dto::{
    BeritaListQuery, BeritaResponse, BeritaRingkasanResponse, CreateBeritaRequest,
    UpdateBeritaRequest,
};
use crate::presentation::middleware::jwt::AuthGuard;
use crate::presentation::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<BeritaListQuery>,
) -> Result<Json<Vec<BeritaRingkasanResponse>>, AppError> {
    let kategori = match q.kategori.as_deref() {
        Some(raw) => Some(
            KategoriBerita::parse(raw)
                .ok_or_else(|| AppError::Validation(format!("kategori tidak dikenal: {raw}")))?,
        ),
        None => None,
    };

    let items = state.berita.list(kategori).await?;
    Ok(Json(
        items
            .into_iter()
            .map(BeritaRingkasanResponse::from)
            .collect(),
    ))
}

pub async fn get_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<BeritaResponse>, AppError> {
    let item = state.berita.get_by_slug(&slug).await?;
    Ok(Json(BeritaResponse::from(item)))
}

// ============================================================================
// Admin handlers (JWT-protected by AuthGuard extractor)
// ============================================================================

pub async fn admin_create(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Json(body): Json<CreateBeritaRequest>,
) -> Result<(StatusCode, Json<BeritaResponse>), AppError> {
    body.validate()?;
    let input = body.into_domain()?;
    let created = state.berita.create(input).await?;
    Ok((StatusCode::CREATED, Json(BeritaResponse::from(created))))
}

pub async fn admin_update(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(body): Json<UpdateBeritaRequest>,
) -> Result<Json<BeritaResponse>, AppError> {
    body.validate()?;
    let patch = body.into_domain()?;
    let updated = state.berita.update(&slug, patch).await?;
    Ok(Json(BeritaResponse::from(updated)))
}

pub async fn admin_delete(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<StatusCode, AppError> {
    state.berita.delete(&slug).await?;
    Ok(StatusCode::NO_CONTENT)
}

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::domain::galeri::KategoriGaleri;
use crate::error::AppError;
use crate::presentation::dto::galeri_dto::{
    CreateGaleriRequest, GaleriListQuery, GaleriResponse, UpdateGaleriRequest,
};
use crate::presentation::middleware::jwt::AuthGuard;
use crate::presentation::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<GaleriListQuery>,
) -> Result<Json<Vec<GaleriResponse>>, AppError> {
    let kategori = match q.kategori.as_deref() {
        Some(raw) => Some(
            KategoriGaleri::parse(raw)
                .ok_or_else(|| AppError::Validation(format!("unknown kategori: {raw}")))?,
        ),
        None => None,
    };

    let items = state.galeri.list(kategori).await?;
    Ok(Json(items.into_iter().map(GaleriResponse::from).collect()))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GaleriResponse>, AppError> {
    let item = state.galeri.get_by_id(id).await?;
    Ok(Json(GaleriResponse::from(item)))
}

// ============================================================================
// Admin handlers (auth-gated by `AuthGuard`)
// ============================================================================

/// `POST /api/admin/galeri` — create a new gallery item. The `AuthGuard`
/// extractor short-circuits with 401 before we touch the body.
pub async fn admin_create(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Json(body): Json<CreateGaleriRequest>,
) -> Result<(StatusCode, Json<GaleriResponse>), AppError> {
    body.validate()?;
    let input = body.into_domain()?;
    let created = state.galeri.create(input).await?;
    Ok((StatusCode::CREATED, Json(GaleriResponse::from(created))))
}

/// `PATCH /api/admin/galeri/:id` — partial update. Returns 404 when no row
/// matches the supplied UUID.
pub async fn admin_update(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateGaleriRequest>,
) -> Result<Json<GaleriResponse>, AppError> {
    body.validate()?;
    let patch = body.into_domain()?;
    let updated = state.galeri.update(id, patch).await?;
    Ok(Json(GaleriResponse::from(updated)))
}

/// `DELETE /api/admin/galeri/:id` — returns 204 on success, 404 when missing.
pub async fn admin_delete(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.galeri.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::domain::lowongan::{KategoriLowongan, StatusLowongan};
use crate::domain::repository::LowonganFilter;
use crate::error::AppError;
use crate::presentation::dto::lowongan_dto::{
    CreateLowonganRequest, LowonganListQuery, LowonganResponse, UpdateLowonganRequest,
};
use crate::presentation::middleware::jwt::AuthGuard;
use crate::presentation::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<LowonganListQuery>,
) -> Result<Json<Vec<LowonganResponse>>, AppError> {
    let kategori = match q.kategori.as_deref() {
        Some(raw) => Some(
            KategoriLowongan::parse(raw)
                .ok_or_else(|| AppError::Validation(format!("unknown kategori: {raw}")))?,
        ),
        None => None,
    };
    let status = match q.status.as_deref() {
        Some(raw) => Some(
            StatusLowongan::parse(raw)
                .ok_or_else(|| AppError::Validation(format!("unknown status: {raw}")))?,
        ),
        None => None,
    };

    let items = state
        .lowongan
        .list(LowonganFilter { kategori, status })
        .await?;
    Ok(Json(items.into_iter().map(LowonganResponse::from).collect()))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<LowonganResponse>, AppError> {
    let item = state.lowongan.get_by_id(id).await?;
    Ok(Json(LowonganResponse::from(item)))
}

// ============================================================================
// Admin handlers (auth-gated by `AuthGuard`)
// ============================================================================

/// `POST /api/admin/lowongan` — create a new job posting.
pub async fn admin_create(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Json(body): Json<CreateLowonganRequest>,
) -> Result<(StatusCode, Json<LowonganResponse>), AppError> {
    body.validate()?;
    let input = body.into_domain()?;
    let created = state.lowongan.create(input).await?;
    Ok((StatusCode::CREATED, Json(LowonganResponse::from(created))))
}

/// `PATCH /api/admin/lowongan/:id` — partial update; 404 when no such id.
pub async fn admin_update(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateLowonganRequest>,
) -> Result<Json<LowonganResponse>, AppError> {
    body.validate()?;
    let patch = body.into_domain()?;
    let updated = state.lowongan.update(id, patch).await?;
    Ok(Json(LowonganResponse::from(updated)))
}

/// `DELETE /api/admin/lowongan/:id` — 204 on success, 404 when missing.
pub async fn admin_delete(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.lowongan.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

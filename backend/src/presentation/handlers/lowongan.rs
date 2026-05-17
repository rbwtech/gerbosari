use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;

use crate::domain::lowongan::{KategoriLowongan, StatusLowongan};
use crate::domain::repository::LowonganFilter;
use crate::error::AppError;
use crate::presentation::dto::lowongan_dto::{LowonganListQuery, LowonganResponse};
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

use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;

use crate::domain::galeri::KategoriGaleri;
use crate::error::AppError;
use crate::presentation::dto::galeri_dto::{GaleriListQuery, GaleriResponse};
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

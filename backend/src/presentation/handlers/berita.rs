use axum::extract::{Path, Query, State};
use axum::Json;

use crate::domain::berita::KategoriBerita;
use crate::error::AppError;
use crate::presentation::dto::berita_dto::{
    BeritaListQuery, BeritaResponse, BeritaRingkasanResponse,
};
use crate::presentation::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<BeritaListQuery>,
) -> Result<Json<Vec<BeritaRingkasanResponse>>, AppError> {
    let kategori = match q.kategori.as_deref() {
        Some(raw) => Some(
            KategoriBerita::parse(raw)
                .ok_or_else(|| AppError::Validation(format!("unknown kategori: {raw}")))?,
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

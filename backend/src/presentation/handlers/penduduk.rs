use axum::extract::State;
use axum::Json;

use crate::error::AppError;
use crate::presentation::dto::penduduk_dto::PendudukRingkasanResponse;
use crate::presentation::state::AppState;

pub async fn ringkasan(
    State(state): State<AppState>,
) -> Result<Json<PendudukRingkasanResponse>, AppError> {
    let r = state.penduduk.ringkasan().await?;
    Ok(Json(PendudukRingkasanResponse::from(r)))
}

use axum::extract::{Path, State};
use axum::Json;
use validator::Validate;

use crate::domain::penduduk::PendudukPatch;
use crate::error::AppError;
use crate::presentation::dto::penduduk_dto::{
    PendudukPedukuhanResponse, PendudukRingkasanResponse, UpdatePendudukRequest,
};
use crate::presentation::middleware::jwt::AuthGuard;
use crate::presentation::state::AppState;

pub async fn ringkasan(
    State(state): State<AppState>,
) -> Result<Json<PendudukRingkasanResponse>, AppError> {
    let r = state.penduduk.ringkasan().await?;
    Ok(Json(PendudukRingkasanResponse::from(r)))
}

// ============================================================================
// Admin handlers (JWT-protected by AuthGuard extractor)
// ============================================================================

pub async fn admin_update(
    _auth: AuthGuard,
    State(state): State<AppState>,
    Path(pedukuhan): Path<String>,
    Json(body): Json<UpdatePendudukRequest>,
) -> Result<Json<PendudukPedukuhanResponse>, AppError> {
    body.validate()?;
    let patch = PendudukPatch {
        jumlah_kk: body.jumlah_kk,
        laki: body.laki,
        perempuan: body.perempuan,
    };
    let updated = state.penduduk.update_pedukuhan(&pedukuhan, patch).await?;
    Ok(Json(PendudukPedukuhanResponse::from(updated)))
}

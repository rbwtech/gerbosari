use axum::Json;
use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    })
}

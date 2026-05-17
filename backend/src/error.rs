use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("resource not found")]
    NotFound,

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message): (StatusCode, &'static str, String) = match &self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "not_found",
                "Resource not found".to_string(),
            ),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            AppError::Database(err) => {
                if matches!(err, sqlx::Error::RowNotFound) {
                    (
                        StatusCode::NOT_FOUND,
                        "not_found",
                        "Resource not found".to_string(),
                    )
                } else {
                    tracing::error!(target: "gerbosari::error", error = ?err, "database error");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "internal_error",
                        "An internal error occurred".to_string(),
                    )
                }
            }
            AppError::Internal(err) => {
                tracing::error!(target: "gerbosari::error", error = ?err, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "An internal error occurred".to_string(),
                )
            }
        };

        (
            status,
            Json(ErrorBody {
                error: code,
                message,
            }),
        )
            .into_response()
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(value: validator::ValidationErrors) -> Self {
        AppError::Validation(value.to_string())
    }
}

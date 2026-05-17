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

    /// 401 with structured Indonesian copy. Used by auth login + JWT middleware.
    /// `code` is a stable machine identifier; `message` is the user-facing text.
    #[error("unauthorized: {message}")]
    Unauthorized {
        code: &'static str,
        message: String,
    },

    /// 429 with a Retry-After hint. Used by the login throttle to slow down
    /// brute-force credential stuffing. `retry_after_secs` becomes the
    /// HTTP `Retry-After` header so clients can back off correctly.
    #[error("too many requests; retry after {retry_after_secs}s")]
    TooManyRequests {
        message: String,
        retry_after_secs: u64,
    },

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
        // TooManyRequests is special: needs Retry-After header, handled below.
        if let AppError::TooManyRequests { message, retry_after_secs } = &self {
            let retry = retry_after_secs.to_string();
            let body = Json(ErrorBody {
                error: "too_many_requests",
                message: message.clone(),
            });
            return (
                StatusCode::TOO_MANY_REQUESTS,
                [(axum::http::header::RETRY_AFTER, retry.as_str())],
                body,
            )
                .into_response();
        }

        let (status, code, message): (StatusCode, &'static str, String) = match &self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "not_found",
                "Resource not found".to_string(),
            ),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            AppError::Unauthorized { code, message } => {
                (StatusCode::UNAUTHORIZED, *code, message.clone())
            }
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
            AppError::TooManyRequests { .. } => {
                // Handled by the early-return branch above; this arm exists only
                // so the outer match is exhaustive.
                unreachable!("TooManyRequests handled above")
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

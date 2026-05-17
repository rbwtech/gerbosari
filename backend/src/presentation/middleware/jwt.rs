//! `AuthGuard` extractor — pulls `Authorization: Bearer <token>` off the
//! request, verifies it via the `AuthService` in `AppState`, and exposes the
//! authenticated principal to handlers. Any failure short-circuits with a
//! structured 401.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::domain::auth::{AuthError, AuthenticatedUser};
use crate::presentation::state::AppState;

/// Extractor that fails the request with 401 if no valid bearer token is
/// present. Handler signatures simply take `AuthGuard(user): AuthGuard` to
/// gate access.
#[derive(Debug, Clone)]
pub struct AuthGuard(pub AuthenticatedUser);

#[derive(Debug, Serialize)]
struct UnauthorizedBody {
    error: &'static str,
    message: &'static str,
}

/// Wire-level rejection for the guard. Crafted by hand so the middleware
/// path never depends on `AppError` ordering / variant churn.
pub struct AuthRejection {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(UnauthorizedBody {
                error: self.code,
                message: self.message,
            }),
        )
            .into_response()
    }
}

impl AuthRejection {
    const MISSING: Self = Self {
        status: StatusCode::UNAUTHORIZED,
        code: "missing_authorization",
        message: "Otorisasi diperlukan untuk mengakses endpoint ini.",
    };

    const INVALID: Self = Self {
        status: StatusCode::UNAUTHORIZED,
        code: "invalid_token",
        message: "Token tidak valid atau telah kedaluwarsa.",
    };
}

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthGuard {
    type Rejection = AuthRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let raw = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(AuthRejection::MISSING)?;

        let token = raw
            .strip_prefix("Bearer ")
            .or_else(|| raw.strip_prefix("bearer "))
            .ok_or(AuthRejection::MISSING)?
            .trim();

        if token.is_empty() {
            return Err(AuthRejection::MISSING);
        }

        match state.auth.verify_token(token) {
            Ok(user) => Ok(AuthGuard(user)),
            Err(AuthError::InvalidToken) | Err(AuthError::InvalidCredentials) => {
                Err(AuthRejection::INVALID)
            }
            Err(AuthError::Internal(msg)) => {
                tracing::error!(target: "gerbosari::auth", error = %msg, "jwt verify internal error");
                Err(AuthRejection::INVALID)
            }
        }
    }
}

use std::net::{IpAddr, Ipv4Addr};

use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use axum::Json;
use std::net::SocketAddr;
use validator::Validate;

use crate::domain::auth::AuthError;
use crate::error::AppError;
use crate::presentation::dto::auth_dto::{LoginRequest, LoginResponse};
use crate::presentation::state::AppState;

/// `POST /api/auth/login`. Validates the payload, delegates to the auth
/// service, and serialises the resulting token. All credential failures map
/// to a single 401 shape so callers cannot distinguish "unknown user" from
/// "wrong password". A per-IP throttle gates the endpoint to slow brute
/// force attempts.
pub async fn login(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let ip = client_ip(&headers, peer);
    if let Err(retry_after) = state.login_throttle.check_and_record(ip) {
        return Err(AppError::TooManyRequests {
            message: "Terlalu banyak percobaan login. Silakan coba lagi nanti.".to_string(),
            retry_after_secs: retry_after.as_secs().max(1),
        });
    }

    body.validate()?;

    match state.auth.login(&body.username, &body.password).await {
        Ok(token) => Ok(Json(LoginResponse::from(token))),
        Err(AuthError::InvalidCredentials) => Err(AppError::Unauthorized {
            code: "invalid_credentials",
            message: "Username atau password salah.".to_string(),
        }),
        Err(AuthError::InvalidToken) => Err(AppError::Unauthorized {
            code: "invalid_token",
            message: "Token tidak valid atau telah kedaluwarsa.".to_string(),
        }),
        Err(AuthError::Internal(msg)) => {
            tracing::error!(target: "gerbosari::auth", error = %msg, "auth internal error");
            Err(AppError::Internal(anyhow::anyhow!("auth internal error")))
        }
    }
}

/// Resolve the real client IP. Behind nginx the TCP peer is always 127.0.0.1,
/// so prefer the first hop in `X-Forwarded-For`. Falls back to the socket
/// peer when no forwarded header is present (direct calls during tests).
fn client_ip(headers: &HeaderMap, peer: SocketAddr) -> IpAddr {
    if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = xff.split(',').next() {
            if let Ok(ip) = first.trim().parse::<IpAddr>() {
                return ip;
            }
        }
    }
    if let Some(real) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        if let Ok(ip) = real.trim().parse::<IpAddr>() {
            return ip;
        }
    }
    if peer.ip().is_unspecified() {
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
    } else {
        peer.ip()
    }
}

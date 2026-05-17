use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Domain-level authentication errors. The presentation layer maps these
/// onto HTTP responses with Indonesian copy; the variants here stay in
/// English to keep them grep-able in logs.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("token invalid or expired")]
    InvalidToken,
    #[error("internal error: {0}")]
    Internal(String),
}

/// Identity attached to an inbound request after JWT verification. This is
/// the principal that admin handlers operate under.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub username: String,
}

/// Successful login envelope. The opaque `token` is the signed JWT the
/// client must send back in `Authorization: Bearer <token>`. The full user
/// is returned to spare the frontend a follow-up fetch.
#[derive(Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub user: AuthenticatedUser,
}

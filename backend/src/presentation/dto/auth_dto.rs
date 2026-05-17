use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::auth::AuthToken;

/// Inbound payload for `POST /api/auth/login`. Length bounds are intentionally
/// loose at the API edge — credential checks happen in `AuthService`.
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, max = 64))]
    pub username: String,
    #[validate(length(min = 1, max = 256))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub user: AuthUserDto,
}

#[derive(Debug, Serialize)]
pub struct AuthUserDto {
    pub id: Uuid,
    pub username: String,
}

impl From<AuthToken> for LoginResponse {
    fn from(token: AuthToken) -> Self {
        Self {
            token: token.token,
            expires_at: token.expires_at,
            user: AuthUserDto {
                id: token.user.user_id,
                username: token.user.username,
            },
        }
    }
}

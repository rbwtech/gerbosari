//! Orchestrates the credential -> JWT login flow. Pure async; depends only
//! on the `AdminUserRepository` trait and the infrastructure-level
//! `JwtEncoder`. Verification is constant-time relative to the bcrypt cost.

use std::sync::Arc;
use uuid::Uuid;

use crate::domain::auth::{AuthError, AuthToken, AuthenticatedUser};
use crate::domain::repository::AdminUserRepository;
use crate::infrastructure::auth::{verify_password, JwtEncoder};

pub struct AuthService {
    repo: Arc<dyn AdminUserRepository>,
    encoder: Arc<JwtEncoder>,
}

impl AuthService {
    pub fn new(repo: Arc<dyn AdminUserRepository>, encoder: Arc<JwtEncoder>) -> Self {
        Self { repo, encoder }
    }

    /// Authenticates `(username, password)` and returns a signed token. A
    /// missing user and a bad password both collapse to
    /// `InvalidCredentials` to avoid leaking which factor failed.
    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthToken, AuthError> {
        let user = self
            .repo
            .find_by_username(username)
            .await
            .map_err(|e| AuthError::Internal(format!("admin lookup failed: {e}")))?
            .ok_or(AuthError::InvalidCredentials)?;

        if !verify_password(password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }

        let (token, expires_at) = self.encoder.issue(user.id, &user.username)?;

        Ok(AuthToken {
            token,
            expires_at,
            user: AuthenticatedUser {
                user_id: user.id,
                username: user.username,
            },
        })
    }

    /// Decodes a JWT and projects its claims into an `AuthenticatedUser`.
    /// Used by the JWT middleware on every admin request.
    pub fn verify_token(&self, token: &str) -> Result<AuthenticatedUser, AuthError> {
        let claims = self.encoder.decode(token)?;
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AuthError::InvalidToken)?;
        Ok(AuthenticatedUser {
            user_id,
            username: claims.username,
        })
    }
}

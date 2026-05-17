//! JWT + bcrypt primitives. Kept in the infrastructure layer because they
//! depend on third-party crates (`jsonwebtoken`, `bcrypt`); the application
//! layer only sees `AuthError` and the encoded string.

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::auth::AuthError;

/// JWT signing configuration. The secret must be at least 32 bytes — that
/// invariant is enforced one layer up in `AppConfig::from_env`.
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry_hours: u32,
}

impl JwtConfig {
    pub fn new(secret: String, expiry_hours: u32) -> Self {
        Self {
            secret,
            expiry_hours,
        }
    }
}

/// JWT claim set. `sub` carries the admin user's UUID; `username` is a
/// convenience copy so handlers do not need a DB round-trip to render the
/// principal name. `exp` and `iat` are unix seconds per RFC 7519.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}

/// Stateless HS256 encoder/decoder. Cheap to clone; share via `Arc` so the
/// secret material is allocated once at boot.
#[derive(Clone)]
pub struct JwtEncoder {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    expiry_hours: u32,
}

impl JwtEncoder {
    pub fn new(config: JwtConfig) -> Self {
        let bytes = config.secret.as_bytes();
        Self {
            encoding_key: EncodingKey::from_secret(bytes),
            decoding_key: DecodingKey::from_secret(bytes),
            // Default validation already checks `exp`; we keep `nbf`/`aud`
            // off because we don't issue those claims.
            validation: Validation::new(Algorithm::HS256),
            expiry_hours: config.expiry_hours,
        }
    }

    /// Builds and signs a token for `user_id`. Returns the encoded string
    /// alongside the absolute expiry timestamp so the caller can echo it to
    /// the client.
    pub fn issue(
        &self,
        user_id: Uuid,
        username: &str,
    ) -> Result<(String, DateTime<Utc>), AuthError> {
        let now = Utc::now();
        let exp = now + Duration::hours(i64::from(self.expiry_hours));

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| AuthError::Internal(format!("jwt encode failed: {e}")))?;

        Ok((token, exp))
    }

    /// Verifies signature + expiry. Any failure collapses to
    /// `AuthError::InvalidToken` to avoid leaking discrimination signals.
    pub fn decode(&self, token: &str) -> Result<Claims, AuthError> {
        let data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(data.claims)
    }
}

/// Cost factor for bcrypt password hashing. 12 yields roughly 250ms on
/// modern CPUs — slow enough to be a meaningful login throttle, fast enough
/// to not stall container startup.
pub const BCRYPT_COST: u32 = 12;

/// Hashes a plain-text password. Used exclusively at admin bootstrap.
pub fn hash_password(plain: &str) -> Result<String, AuthError> {
    bcrypt::hash(plain, BCRYPT_COST)
        .map_err(|e| AuthError::Internal(format!("bcrypt hash failed: {e}")))
}

/// Constant-time bcrypt verification. A malformed hash counts as a failure
/// rather than an internal error to keep the login surface uniform.
pub fn verify_password(plain: &str, hash: &str) -> Result<bool, AuthError> {
    bcrypt::verify(plain, hash).map_err(|_| AuthError::InvalidCredentials)
}

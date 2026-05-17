use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Persistent representation of a backoffice administrator. `password_hash`
/// holds a bcrypt digest produced at cost=12; the plain password is never
/// retained in memory beyond the verification call.
#[derive(Debug, Clone)]
pub struct AdminUser {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

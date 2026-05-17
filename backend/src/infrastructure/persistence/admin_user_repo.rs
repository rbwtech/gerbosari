use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::admin_user::AdminUser;
use crate::domain::repository::AdminUserRepository;
use crate::error::AppError;

pub struct MysqlAdminUserRepository {
    pool: MySqlPool,
}

impl MysqlAdminUserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct AdminUserRow {
    id: String,
    username: String,
    password_hash: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl TryFrom<AdminUserRow> for AdminUser {
    type Error = AppError;

    fn try_from(row: AdminUserRow) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&row.id)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("invalid admin_user id: {e}")))?;
        Ok(AdminUser {
            id,
            username: row.username,
            password_hash: row.password_hash,
            created_at: row.created_at.and_utc(),
            updated_at: row.updated_at.and_utc(),
        })
    }
}

#[async_trait]
impl AdminUserRepository for MysqlAdminUserRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<AdminUser>, AppError> {
        let row: Option<AdminUserRow> = sqlx::query_as::<_, AdminUserRow>(
            "SELECT id, username, password_hash, created_at, updated_at \
             FROM admin_users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(AdminUser::try_from(r)?)),
            None => Ok(None),
        }
    }

    /// Inserts a new admin or rotates the password hash on the existing row
    /// (matched by unique `username`). Used by the startup bootstrap so the
    /// operator can rotate credentials via `ADMIN_PASSWORD` without manual
    /// SQL. Returns the resulting row.
    async fn upsert(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<AdminUser, AppError> {
        let id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO admin_users (id, username, password_hash) \
             VALUES (?, ?, ?) \
             ON DUPLICATE KEY UPDATE password_hash = VALUES(password_hash)",
        )
        .bind(&id)
        .bind(username)
        .bind(password_hash)
        .execute(&self.pool)
        .await?;

        let row: AdminUserRow = sqlx::query_as::<_, AdminUserRow>(
            "SELECT id, username, password_hash, created_at, updated_at \
             FROM admin_users WHERE username = ?",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        AdminUser::try_from(row)
    }
}

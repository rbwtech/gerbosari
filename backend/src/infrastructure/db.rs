use std::time::Duration;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn init_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(16)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Some(Duration::from_secs(300)))
        .test_before_acquire(true)
        .connect(database_url)
        .await
}

/// Applies migrations bundled at compile time from `./migrations` (sibling of
/// `Cargo.toml`). The migrations folder is owned by the database agent; this
/// function only triggers them on startup.
pub async fn run_migrations(pool: &MySqlPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

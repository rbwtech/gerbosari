use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

/// Minimum acceptable JWT secret length (bytes). Below this, HS256 brute-force
/// is trivial on commodity hardware — refuse to boot rather than ship insecure.
pub const JWT_SECRET_MIN_LEN: usize = 32;

/// Default token lifetime (hours). 24h matches typical admin SPA sessions
/// without requiring background refresh wiring.
pub const JWT_DEFAULT_EXPIRY_HOURS: u32 = 24;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub bind_addr: SocketAddr,
    pub cors_origins: Vec<String>,
    pub app_env: AppEnv,
    pub jwt_secret: String,
    pub jwt_expiry_hours: u32,
    pub admin_username: String,
    /// `None` if `ADMIN_PASSWORD` was not set — bootstrap will skip the
    /// upsert path and rely on the admin row already existing in the DB.
    pub admin_password: Option<String>,
    /// Filesystem root where admin image uploads land. Nginx aliases
    /// `/images/` to this same path, so the URL returned to the SPA is just
    /// `/images/<subdir>/<filename>`. Default `../content/images` resolves
    /// relative to the systemd `WorkingDirectory`.
    pub upload_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnv {
    Development,
    Production,
}

impl AppEnv {
    pub fn is_prod(self) -> bool {
        matches!(self, AppEnv::Production)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    Missing(&'static str),

    #[error("invalid value for {var}: {source}")]
    Invalid {
        var: &'static str,
        #[source]
        source: anyhow::Error,
    },

    #[error("JWT_SECRET must be at least {min} bytes; got {got}")]
    WeakJwtSecret { min: usize, got: usize },
}

impl AppConfig {
    /// Loads runtime configuration from environment variables. Intended to be
    /// called once at process startup; failures are unrecoverable.
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| ConfigError::Missing("DATABASE_URL"))?;

        let bind_addr_raw =
            env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
        let bind_addr: SocketAddr =
            bind_addr_raw
                .parse()
                .map_err(|e: std::net::AddrParseError| ConfigError::Invalid {
                    var: "BIND_ADDR",
                    source: anyhow::Error::from(e),
                })?;

        let cors_origins = env::var("CORS_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:5173".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let app_env = match env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" | "prod" => AppEnv::Production,
            _ => AppEnv::Development,
        };

        // --- Auth ---
        let jwt_secret =
            env::var("JWT_SECRET").map_err(|_| ConfigError::Missing("JWT_SECRET"))?;
        if jwt_secret.len() < JWT_SECRET_MIN_LEN {
            return Err(ConfigError::WeakJwtSecret {
                min: JWT_SECRET_MIN_LEN,
                got: jwt_secret.len(),
            });
        }

        let jwt_expiry_hours = match env::var("JWT_EXPIRY_HOURS") {
            Ok(raw) => raw.parse::<u32>().map_err(|e| ConfigError::Invalid {
                var: "JWT_EXPIRY_HOURS",
                source: anyhow::Error::from(e),
            })?,
            Err(_) => JWT_DEFAULT_EXPIRY_HOURS,
        };

        let admin_username =
            env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());

        // ADMIN_PASSWORD is optional — empty / unset just means "don't
        // rotate at boot". Operator can populate the admin row manually or
        // set it on the next deploy.
        let admin_password = env::var("ADMIN_PASSWORD")
            .ok()
            .filter(|s| !s.is_empty());

        let upload_dir = env::var("UPLOAD_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("../content/images"));

        Ok(Self {
            database_url,
            bind_addr,
            cors_origins,
            app_env,
            jwt_secret,
            jwt_expiry_hours,
            admin_username,
            admin_password,
            upload_dir,
        })
    }
}

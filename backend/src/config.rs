use std::env;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub bind_addr: SocketAddr,
    pub cors_origins: Vec<String>,
    pub app_env: AppEnv,
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

        Ok(Self {
            database_url,
            bind_addr,
            cors_origins,
            app_env,
        })
    }
}

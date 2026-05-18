use std::sync::Arc;

use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use gerbosari_backend::application::auth_service::AuthService;
use gerbosari_backend::application::berita_service::BeritaService;
use gerbosari_backend::application::galeri_service::GaleriService;
use gerbosari_backend::application::lowongan_service::LowonganService;
use gerbosari_backend::application::penduduk_service::PendudukService;
use gerbosari_backend::config::{AppConfig, AppEnv};
use gerbosari_backend::domain::repository::AdminUserRepository;
use gerbosari_backend::infrastructure::auth::{hash_password, JwtConfig, JwtEncoder};
use gerbosari_backend::infrastructure::db;
use gerbosari_backend::infrastructure::rate_limit::LoginThrottle;
use gerbosari_backend::infrastructure::persistence::admin_user_repo::MysqlAdminUserRepository;
use gerbosari_backend::infrastructure::persistence::berita_repo::MysqlBeritaRepository;
use gerbosari_backend::infrastructure::persistence::galeri_repo::MysqlGaleriRepository;
use gerbosari_backend::infrastructure::persistence::lowongan_repo::MysqlLowonganRepository;
use gerbosari_backend::infrastructure::persistence::penduduk_repo::MysqlPendudukRepository;
use gerbosari_backend::presentation::router::build_router;
use gerbosari_backend::presentation::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let config = AppConfig::from_env()
        .unwrap_or_else(|e| panic!("invalid configuration on startup: {e}"));

    init_tracing(config.app_env);

    tracing::info!(
        bind_addr = %config.bind_addr,
        env = ?config.app_env,
        "starting gerbosari-backend"
    );

    let pool = db::init_pool(&config.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("failed to connect to database: {e}"))?;

    if let Err(e) = db::run_migrations(&pool).await {
        return Err(anyhow::anyhow!("failed to run migrations: {e}"));
    }

    let galeri_repo = Arc::new(MysqlGaleriRepository::new(pool.clone()));
    let penduduk_repo = Arc::new(MysqlPendudukRepository::new(pool.clone()));
    let lowongan_repo = Arc::new(MysqlLowonganRepository::new(pool.clone()));
    let berita_repo = Arc::new(MysqlBeritaRepository::new(pool.clone()));
    let admin_user_repo: Arc<dyn AdminUserRepository> =
        Arc::new(MysqlAdminUserRepository::new(pool.clone()));

    // --- Auth wiring ---
    let jwt_encoder = Arc::new(JwtEncoder::new(JwtConfig::new(
        config.jwt_secret.clone(),
        config.jwt_expiry_hours,
    )));
    let auth_service = Arc::new(AuthService::new(
        admin_user_repo.clone(),
        jwt_encoder.clone(),
    ));

    // --- Admin bootstrap ---
    // Behaviour matrix:
    //   ADMIN_PASSWORD set    -> hash + upsert the row (rotate on each boot)
    //   ADMIN_PASSWORD unset  -> require the row to already exist, else log
    //                            a loud error but keep serving (operator can
    //                            fix on next deploy without losing uptime).
    bootstrap_admin(
        admin_user_repo.as_ref(),
        &config.admin_username,
        config.admin_password.as_deref(),
    )
    .await?;

    // --- Login rate limiter ---
    // 5 attempts per 60s per client IP. Sweeps stale entries every 5 minutes
    // so the map doesn't grow unbounded on a long-lived process.
    let login_throttle = Arc::new(LoginThrottle::new());
    {
        let throttle = login_throttle.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                throttle.sweep();
            }
        });
    }

    // --- Upload directory ---
    // Eagerly ensure it exists at boot so the first POST /api/admin/upload
    // doesn't race with mkdir. If the path can't be created (perms, missing
    // parent under systemd's ProtectSystem=strict), log loud but keep serving
    // — the upload route will return 500 with the same message on hit.
    let upload_dir = config.upload_dir.clone();
    if let Err(e) = tokio::fs::create_dir_all(&upload_dir).await {
        tracing::error!(
            target: "gerbosari::startup",
            path = %upload_dir.display(),
            error = %e,
            "failed to ensure upload_dir at startup — uploads will fail until this is fixed"
        );
    }

    let state = AppState {
        galeri: Arc::new(GaleriService::new(galeri_repo)),
        penduduk: Arc::new(PendudukService::new(penduduk_repo)),
        lowongan: Arc::new(LowonganService::new(lowongan_repo)),
        berita: Arc::new(BeritaService::new(berita_repo)),
        auth: auth_service,
        jwt: jwt_encoder,
        login_throttle,
        upload_dir: Arc::new(upload_dir),
    };

    let app = build_router(state, &config.cors_origins);

    let listener = TcpListener::bind(config.bind_addr)
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind {}: {e}", config.bind_addr))?;

    tracing::info!(addr = %config.bind_addr, "listening");

    // `into_make_service_with_connect_info::<SocketAddr>()` so the
    // login handler can extract the peer's IP via `ConnectInfo`.
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .map_err(|e| anyhow::anyhow!("server error: {e}"))?;

    Ok(())
}

/// Idempotent admin user provisioning. Runs once at startup. Never panics —
/// any failure is logged so the API can keep serving public routes.
async fn bootstrap_admin(
    repo: &dyn AdminUserRepository,
    username: &str,
    password: Option<&str>,
) -> anyhow::Result<()> {
    match password {
        Some(pw) => {
            // Hashing happens off the request path so the 250ms bcrypt cost
            // is paid at boot, not per-login.
            let hash = hash_password(pw)
                .map_err(|e| anyhow::anyhow!("failed to hash admin password: {e}"))?;
            repo.upsert(username, &hash)
                .await
                .map_err(|e| anyhow::anyhow!("admin upsert failed: {e}"))?;
            tracing::info!(username = %username, "admin bootstrapped");
        }
        None => {
            match repo.find_by_username(username).await {
                Ok(Some(_)) => {
                    tracing::info!(
                        username = %username,
                        "admin already provisioned; skipping bootstrap"
                    );
                }
                Ok(None) => {
                    tracing::error!(
                        username = %username,
                        "ADMIN_PASSWORD not set and no admin row exists — login will fail until populated"
                    );
                }
                Err(e) => {
                    tracing::error!(error = %e, "failed to verify admin presence at boot");
                }
            }
        }
    }
    Ok(())
}

fn init_tracing(env: AppEnv) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn,tower_http=info"));

    let registry = tracing_subscriber::registry().with(filter);

    if env.is_prod() {
        registry
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        registry
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(e) = tokio::signal::ctrl_c().await {
            tracing::error!(error = ?e, "failed to install ctrl_c handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut sig) => {
                sig.recv().await;
            }
            Err(e) => tracing::error!(error = ?e, "failed to install terminate handler"),
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}

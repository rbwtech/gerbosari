use std::sync::Arc;

use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use gerbosari_backend::application::berita_service::BeritaService;
use gerbosari_backend::application::galeri_service::GaleriService;
use gerbosari_backend::application::lowongan_service::LowonganService;
use gerbosari_backend::application::penduduk_service::PendudukService;
use gerbosari_backend::config::{AppConfig, AppEnv};
use gerbosari_backend::infrastructure::db;
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

    let state = AppState {
        galeri: Arc::new(GaleriService::new(galeri_repo)),
        penduduk: Arc::new(PendudukService::new(penduduk_repo)),
        lowongan: Arc::new(LowonganService::new(lowongan_repo)),
        berita: Arc::new(BeritaService::new(berita_repo)),
    };

    let app = build_router(state, &config.cors_origins);

    let listener = TcpListener::bind(config.bind_addr)
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind {}: {e}", config.bind_addr))?;

    tracing::info!(addr = %config.bind_addr, "listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| anyhow::anyhow!("server error: {e}"))?;

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

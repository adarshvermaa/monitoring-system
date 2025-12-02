use anyhow::Result;
use axum::{routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod api;
mod auth;
mod processor;
mod storage;

use config::CollectorConfig;

#[derive(Parser)]
#[command(
    name = "monitoring-collector",
    about = "Central collector server for monitoring data",
    version
)]
struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "/etc/monitoring/collector.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "monitoring_collector=info,axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    // Load configuration
    let config = CollectorConfig::load(&cli.config)?;
    
    info!("Starting monitoring collector");
    info!("WebSocket endpoint: {}", config.server.websocket_addr);
    
    // Build router
    let app = build_router(config.clone());

    // Parse address
    let addr: SocketAddr = config.server.websocket_addr.parse()?;

    info!("Collector listening on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .await?;

    Ok(())
}

fn build_router(config: CollectorConfig) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ingest", axum::routing::any(api::websocket::handle_websocket))
        .with_state(config)
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(tower_http::trace::DefaultOnResponse::new()
                    .level(Level::INFO)),
        )
}

async fn health_check() -> &'static str {
    "OK"
}

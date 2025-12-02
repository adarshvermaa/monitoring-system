use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod collectors;
mod buffer;
mod pipeline;
mod transport;

use config::AgentConfig;

#[derive(Parser)]
#[command(
    name = "monitoring-agent",
    about = "Production-grade monitoring agent for logs, metrics, and traffic",
    version
)]
struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "/etc/monitoring/agent.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the monitoring agent as a daemon
    Start {
        /// Run in foreground (don't daemonize)
        #[arg(short, long)]
        foreground: bool,
    },
    
    /// Stop the running agent
    Stop,
    
    /// Check agent configuration
    Check,
    
    /// Test connection to collector
    TestConnection,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "monitoring_agent=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    // Load configuration
    let config = AgentConfig::load(&cli.config)?;
    
    match cli.command {
        Some(Commands::Start { foreground }) => {
            info!("Starting monitoring agent (foreground: {})", foreground);
            start_agent(config, foreground).await?;
        }
        Some(Commands::Stop) => {
            info!("Stopping monitoring agent");
            stop_agent()?;
        }
        Some(Commands::Check) => {
            info!("Checking configuration");
            check_config(config)?;
        }
        Some(Commands::TestConnection) => {
            info!("Testing connection to collector");
            test_connection(config).await?;
        }
        None => {
            // Default: start in foreground
            info!("Starting monitoring agent in foreground mode");
            start_agent(config, true).await?;
        }
    }

    Ok(())
}

async fn start_agent(config: AgentConfig, foreground: bool) -> Result<()> {
    info!("Agent ID: {}", config.agent.id);
    info!("Hostname: {}", config.agent.hostname);
    info!("Collector endpoint: {}", config.collector.endpoint);

    // Create event buffer
    let buffer = buffer::RingBuffer::new(config.buffer.max_events);
    let buffer = std::sync::Arc::new(buffer);

    // Create shutdown channel
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    // Start collectors
    let mut handles = Vec::new();

    // Log collector
    if config.collectors.logs.enabled {
        info!("Starting log collector");
        let log_collector = collectors::logs::LogCollector::new(
            config.collectors.logs.clone(),
            buffer.clone(),
        );
        let handle = tokio::spawn(async move {
            if let Err(e) = log_collector.run().await {
                error!("Log collector error: {}", e);
            }
        });
        handles.push(handle);
    }

    // Metrics collector
    if config.collectors.metrics.enabled {
        info!("Starting metrics collector");
        let metrics_collector = collectors::metrics::MetricsCollector::new(
            config.collectors.metrics.clone(),
            buffer.clone(),
        );
        let handle = tokio::spawn(async move {
            if let Err(e) = metrics_collector.run().await {
                error!("Metrics collector error: {}", e);
            }
        });
        handles.push(handle);
    }

    // Traffic collector
    if config.collectors.traffic.enabled {
        info!("Starting traffic collector");
        let traffic_collector = collectors::traffic::TrafficCollector::new(
            config.collectors.traffic.clone(),
            buffer.clone(),
        );
        let handle = tokio::spawn(async move {
            if let Err(e) = traffic_collector.run().await {
                error!("Traffic collector error: {}", e);
            }
        });
        handles.push(handle);
    }

    // Start batcher/compressor pipeline
    info!("Starting event pipeline");
    let batcher = pipeline::Batcher::new(
        config.buffer.clone(),
        buffer.clone(),
        config.agent.clone(),
    );
    let (batch_tx, batch_rx) = tokio::sync::mpsc::channel(100);
    let batcher_handle = tokio::spawn(async move {
        if let Err(e) = batcher.run(batch_tx).await {
            error!("Batcher error: {}", e);
        }
    });
    handles.push(batcher_handle);

    // Start transport
    info!("Starting transport layer");
    let transport = transport::Transport::new(config.collector.clone());
    let transport_handle = tokio::spawn(async move {
        if let Err(e) = transport.run(batch_rx).await {
            error!("Transport error: {}", e);
        }
    });
    handles.push(transport_handle);

    // Wait for shutdown signal
    info!("Agent running. Press Ctrl+C to shutdown.");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
        _ = shutdown_rx.recv() => {
            info!("Received shutdown from channel");
        }
    }

    // Shutdown gracefully
    info!("Shutting down agent...");
    let _ = shutdown_tx.send(());
    
    // Wait for all tasks to complete (with timeout)
    for handle in handles {
        let _ = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            handle
        ).await;
    }

    info!("Agent stopped");
    Ok(())
}

fn stop_agent() -> Result<()> {
    // Implementation would send signal to running daemon
    // For now, just a placeholder
    println!("Stop command not yet implemented. Use systemctl stop monitoring-agent");
    Ok(())
}

fn check_config(config: AgentConfig) -> Result<()> {
    println!("✓ Configuration loaded successfully");
    println!("  Agent ID: {}", config.agent.id);
    println!("  Hostname: {}", config.agent.hostname);
    println!("  Collector: {}", config.collector.endpoint);
    println!("  Log collection: {}", config.collectors.logs.enabled);
    println!("  Metrics collection: {}", config.collectors.metrics.enabled);
    println!("  Traffic collection: {}", config.collectors.traffic.enabled);
    println!("  Buffer size: {}", config.buffer.max_events);
    println!("  Flush interval: {}s", config.buffer.flush_interval_secs);
    Ok(())
}

async fn test_connection(config: AgentConfig) -> Result<()> {
    println!("Testing connection to collector: {}", config.collector.endpoint);
    
    // Try to establish WebSocket connection
    let transport = transport::Transport::new(config.collector);
    transport.test_connection().await?;
    
    println!("✓ Connection successful");
    Ok(())
}

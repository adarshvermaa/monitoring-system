pub mod system;
pub mod prometheus;

use crate::config::MetricsCollectorConfig;
use crate::buffer::RingBuffer;
use anyhow::Result;
use std::sync::Arc;
use tracing::{error, info};

pub struct MetricsCollector {
    config: MetricsCollectorConfig,
    buffer: Arc<RingBuffer>,
}

impl MetricsCollector {
    pub fn new(config: MetricsCollectorConfig, buffer: Arc<RingBuffer>) -> Self {
        Self { config, buffer }
    }

    pub async fn run(self) -> Result<()> {
        let mut handles = Vec::new();

        // Start system metrics collector
        info!("Starting system metrics collector (interval: {}s)", 
            self.config.system_interval_secs);
        let system_collector = system::SystemMetrics::new(
            self.config.system_interval_secs,
            self.config.include_process_metrics,
            self.buffer.clone(),
        );
        let handle = tokio::spawn(async move {
            if let Err(e) = system_collector.run().await {
                error!("System metrics collector error: {}", e);
            }
        });
        handles.push(handle);

        // Start Prometheus scrapers
        if !self.config.prometheus_endpoints.is_empty() {
            info!("Starting Prometheus scrapers for {} endpoints",
                self.config.prometheus_endpoints.len());
            
            for endpoint in self.config.prometheus_endpoints.clone() {
                let scraper = prometheus::PrometheusScaper::new(
                    endpoint,
                    self.buffer.clone(),
                );
                let handle = tokio::spawn(async move {
                    if let Err(e) = scraper.run().await {
                        error!("Prometheus scraper error: {}", e);
                    }
                });
                handles.push(handle);
            }
        }

        // Wait for all tasks
        for handle in handles {
            let _ = handle.await;
        }

        Ok(())
    }
}

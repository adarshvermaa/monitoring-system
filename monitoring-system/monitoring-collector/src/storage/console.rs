use super::StorageBackend;
use anyhow::Result;
use async_trait::async_trait;
use monitoring_common::Event;
use tracing::info;

/// Console backend - prints events to stdout (for testing/development)
pub struct ConsoleBackend;

impl ConsoleBackend {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl StorageBackend for ConsoleBackend {
    async fn store_events(&self, events: Vec<Event>) -> Result<()> {
        info!("Storing {} events to console", events.len());
        
        for event in events {
            match event {
                Event::Log(log_event) => {
                    println!("[LOG] {} | {} | {} | {}",
                        chrono::DateTime::from_timestamp_millis(log_event.timestamp)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or_else(|| "unknown".to_string()),
                        log_event.source,
                        format!("{:?}", log_event.level),
                        log_event.message
                    );
                }
                Event::Metric(metric_event) => {
                    println!("[METRIC] {} | {} = {} {:?}",
                        chrono::DateTime::from_timestamp_millis(metric_event.timestamp)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or_else(|| "unknown".to_string()),
                        metric_event.name,
                        metric_event.value,
                        metric_event.unit.as_deref().unwrap_or("")
                    );
                }
                Event::Traffic(traffic_event) => {
                    println!("[TRAFFIC] {} | {:?} | {}:{} -> {}:{}",
                        chrono::DateTime::from_timestamp_millis(traffic_event.timestamp)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or_else(|| "unknown".to_string()),
                        traffic_event.protocol,
                        traffic_event.src_ip,
                        traffic_event.src_port,
                        traffic_event.dst_ip,
                        traffic_event.dst_port
                    );
                }
            }
        }

        Ok(())
    }
}

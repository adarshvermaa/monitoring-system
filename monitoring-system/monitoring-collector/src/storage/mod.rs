mod console;

use crate::config::StorageSettings;
use anyhow::Result;
use async_trait::async_trait;
use monitoring_common::Event;

#[async_trait]
pub trait StorageBackend {
    async fn store_events(&self, events: Vec<Event>) -> Result<()>;
}

pub fn create_backend(config: StorageSettings) -> Box<dyn StorageBackend + Send + Sync> {
    match config.backend.as_str() {
        "console" => Box::new(console::ConsoleBackend::new()),
        // Add more backends here
        // "clickhouse" => Box::new(clickhouse::ClickHouseBackend::new(config)),
        // "postgres" => Box::new(postgres::PostgresBackend::new(config)),
        // "s3" => Box::new(s3::S3Backend::new(config)),
        _ => {
            tracing::warn!("Unknown storage backend '{}', using console", config.backend);
            Box::new(console::ConsoleBackend::new())
        }
    }
}

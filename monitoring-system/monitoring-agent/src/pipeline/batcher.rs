use crate::buffer::RingBuffer;
use crate::config::{AgentSettings, BufferSettings};
use crate::pipeline::Compressor;
use anyhow::Result;
use monitoring_common::{Batch, CompressionType, UncompressedBatch};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tracing::{debug, info};
use uuid::Uuid;

pub struct Batcher {
    config: BufferSettings,
    agent: AgentSettings,
    buffer: Arc<RingBuffer>,
}

impl Batcher {
    pub fn new(
        config: BufferSettings,
        buffer: Arc<RingBuffer>,
        agent: AgentSettings,
    ) -> Self {
        Self {
            config,
            agent,
            buffer,
        }
    }

    pub async fn run(self, batch_tx: Sender<Batch>) -> Result<()> {
        info!("Starting batcher (max_size: {}, flush_interval: {}s)",
            self.config.max_batch_size,
            self.config.flush_interval_secs);

        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(self.config.flush_interval_secs)
        );

        loop {
            interval.tick().await;

            // Check if we have events to batch
            if self.buffer.is_empty() {
                debug!("Buffer empty, skipping batch");
                continue;
            }

            // Drain events from buffer
            let events = self.buffer.drain(self.config.max_batch_size);
            
            if events.is_empty() {
                continue;
            }

            debug!("Creating batch with {} events", events.len());

            // Create uncompressed batch
            let uncompressed_batch = UncompressedBatch {
                batch_id: Uuid::new_v4().to_string(),
                agent_id: self.agent.id.clone(),
                hostname: self.agent.hostname.clone(),
                timestamp: chrono::Utc::now().timestamp_millis(),
                events,
            };

            // Compress batch
            let compression = self.parse_compression_type();
            let batch = match Compressor::compress(uncompressed_batch, compression) {
                Ok(batch) => batch,
                Err(e) => {
                    tracing::error!("Failed to compress batch: {}", e);
                    continue;
                }
            };

            // Send batch to transport
            if let Err(e) = batch_tx.send(batch).await {
                tracing::error!("Failed to send batch to transport: {}", e);
            }
        }
    }

    fn parse_compression_type(&self) -> CompressionType {
        match self.config.compression.as_str() {
            "snappy" => CompressionType::Snappy,
            "lz4" => CompressionType::Lz4,
            "gzip" => CompressionType::Gzip,
            "none" => CompressionType::None,
            _ => CompressionType::Snappy,
        }
    }
}

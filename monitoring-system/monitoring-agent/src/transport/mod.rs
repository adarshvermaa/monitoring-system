mod websocket;
mod retry;

use crate::config::CollectorSettings;
use anyhow::Result;
use monitoring_common::Batch;
use tokio::sync::mpsc::Receiver;
use tracing::info;

pub struct Transport {
    config: CollectorSettings,
}

impl Transport {
    pub fn new(config: CollectorSettings) -> Self {
        Self { config }
    }

    pub async fn run(self, mut batch_rx: Receiver<Batch>) -> Result<()> {
        info!("Starting transport layer");

        // Use WebSocket by default
        let mut ws_client = websocket::WebSocketClient::new(self.config.clone()).await?;

        while let Some(batch) = batch_rx.recv().await {
            if let Err(e) = ws_client.send_batch(batch).await {
                tracing::error!("Failed to send batch: {}", e);
            }
        }

        Ok(())
    }

    pub async fn test_connection(&self) -> Result<()> {
        websocket::WebSocketClient::test_connection(&self.config).await
    }
}

use crate::config::CollectorSettings;
use crate::transport::retry::RetryPolicy;
use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use monitoring_common::{Batch, IngestResponse};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info, warn};

pub struct WebSocketClient {
    config: CollectorSettings,
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    retry_policy: RetryPolicy,
}

impl WebSocketClient {
    pub async fn new(config: CollectorSettings) -> Result<Self> {
        let retry_policy = RetryPolicy::new(
            5,  // max_retries
            std::time::Duration::from_secs(1),  // initial_delay
            std::time::Duration::from_secs(60), // max_delay
        );

        let mut client = Self {
            config,
            ws_stream: None,
            retry_policy,
        };

        client.connect().await?;
        Ok(client)
    }

    async fn connect(&mut self) -> Result<()> {
        info!("Connecting to collector: {}", self.config.endpoint);

        let mut request = self.config.endpoint.parse::<http::Uri>()?;
        
        // Add authorization header if token is provided
        let mut request_builder = http::Request::builder()
            .uri(&self.config.endpoint);

        if let Some(token) = &self.config.auth_token {
            request_builder = request_builder
                .header("Authorization", format!("Bearer {}", token));
        }

        let (ws_stream, response) = connect_async(&self.config.endpoint).await
            .context("Failed to connect to WebSocket")?;

        debug!("WebSocket connected, response: {:?}", response.status());
        
        self.ws_stream = Some(ws_stream);
        self.retry_policy.reset();

        Ok(())
    }

    pub async fn send_batch(&mut self, batch: Batch) -> Result<()> {
        let batch_id = batch.batch_id.clone();
        
        loop {
            match self.try_send_batch(&batch).await {
                Ok(response) => {
                    debug!("Batch {} sent successfully: {:?}", batch_id, response.status);
                    return Ok(());
                }
                Err(e) => {
                    error!("Failed to send batch {}: {}", batch_id, e);
                    
                    // Try to reconnect and retry
                    if let Some(delay) = self.retry_policy.next_delay() {
                        warn!("Retrying in {:?}...", delay);
                        tokio::time::sleep(delay).await;

                        // Reconnect
                        if let Err(e) = self.connect().await {
                            error!("Failed to reconnect: {}", e);
                            continue;
                        }
                    } else {
                        error!("Max retries exceeded for batch {}", batch_id);
                        return Err(e);
                    }
                }
            }
        }
    }

    async fn try_send_batch(&mut self, batch: &Batch) -> Result<IngestResponse> {
        let ws_stream = self.ws_stream.as_mut()
            .context("WebSocket not connected")?;

        // Serialize batch to JSON
        let json = serde_json::to_string(batch)?;
        
        // Send as text message
        ws_stream.send(Message::Text(json)).await?;

        // Wait for response
        match tokio::time::timeout(
            std::time::Duration::from_secs(30),
            ws_stream.next()
        ).await {
            Ok(Some(Ok(Message::Text(response_text)))) => {
                let response: IngestResponse = serde_json::from_str(&response_text)?;
                Ok(response)
            }
            Ok(Some(Ok(msg))) => {
                anyhow::bail!("Unexpected message type: {:?}", msg);
            }
            Ok(Some(Err(e))) => {
                Err(e.into())
            }
            Ok(None) => {
                anyhow::bail!("WebSocket closed");
            }
            Err(_) => {
                anyhow::bail!("Response timeout");
            }
        }
    }

    pub async fn test_connection(config: &CollectorSettings) -> Result<()> {
        info!("Testing connection to: {}", config.endpoint);

        let (ws_stream, response) = connect_async(&config.endpoint).await
            .context("Failed to connect to WebSocket")?;

        info!("Connection successful, status: {:?}", response.status());
        
        // Close the connection  
        drop(ws_stream);

        Ok(())
    }
}

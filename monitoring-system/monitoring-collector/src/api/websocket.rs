use axum::{
    extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use monitoring_common::{Batch, IngestResponse, IngestStatus};
use crate::config::CollectorConfig;
use crate::processor::BatchProcessor;
use tracing::{debug, error, info, warn};

pub async fn handle_websocket(
    ws: WebSocketUpgrade,
    State(config): State<CollectorConfig>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, config))
}

async fn handle_socket(socket: WebSocket, config: CollectorConfig) {
    info!("New WebSocket connection established");

    let (mut sender, mut receiver) = socket.split();
    let processor = BatchProcessor::new(config.processor.clone(), config.storage.clone());

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                debug!("Received text message, length: {}", text.len());
                
                // Parse batch
                let batch: Batch = match serde_json::from_str(&text) {
                    Ok(b) => b,
                    Err(e) => {
                        error!("Failed to parse batch: {}", e);
                        let error_response = IngestResponse {
                            batch_id: "unknown".to_string(),
                            status: IngestStatus::Rejected,
                            error_message: Some(format!("Invalid batch format: {}", e)),
                            received_at: chrono::Utc::now().timestamp_millis(),
                        };
                        
                        if let Ok(response_json) = serde_json::to_string(&error_response) {
                            let _ = sender.send(Message::Text(response_json)).await;
                        }
                        continue;
                    }
                };

                let batch_id = batch.batch_id.clone();
                info!("Received batch: {} with {} events", batch_id, batch.event_count);

                // Process batch
                let response = match processor.process(batch).await {
                    Ok(_) => {
                        info!("Successfully processed batch: {}", batch_id);
                        IngestResponse {
                            batch_id: batch_id.clone(),
                            status: IngestStatus::Success,
                            error_message: None,
                            received_at: chrono::Utc::now().timestamp_millis(),
                        }
                    }
                    Err(e) => {
                        error!("Failed to process batch {}: {}", batch_id, e);
                        IngestResponse {
                            batch_id: batch_id.clone(),
                            status: IngestStatus::Failed,
                            error_message: Some(e.to_string()),
                            received_at: chrono::Utc::now().timestamp_millis(),
                        }
                    }
                };

                // Send response
                if let Ok(response_json) = serde_json::to_string(&response) {
                    if let Err(e) = sender.send(Message::Text(response_json)).await {
                        error!("Failed to send response: {}", e);
                        break;
                    }
                }
            }
            Ok(Message::Binary(data)) => {
                debug!("Received binary message, length: {}", data.len());
                // Could handle protobuf here
            }
            Ok(Message::Ping(data)) => {
                debug!("Received ping");
                if let Err(e) = sender.send(Message::Pong(data)).await {
                    error!("Failed to send pong: {}", e);
                    break;
                }
            }
            Ok(Message::Pong(_)) => {
                debug!("Received pong");
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket closed by client");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    info!("WebSocket connection closed");
}

use crate::auth::TokenValidator;
use crate::config::CollectorConfig;
use crate::processor::BatchProcessor;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use futures_util::{SinkExt, StreamExt};
use monitoring_common::{Batch, IngestResponse, IngestStatus};
use tracing::{debug, error, info};

pub async fn handle_websocket(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    query: Option<Query<std::collections::HashMap<String, String>>>,
    State(config): State<CollectorConfig>,
) -> Response {
    if let Err(status) = authorize_request(&headers, query.as_ref(), &config) {
        return status.into_response();
    }

    ws.on_upgrade(|socket| handle_socket(socket, config))
}

fn authorize_request(
    headers: &HeaderMap,
    query: Option<&Query<std::collections::HashMap<String, String>>>,
    config: &CollectorConfig,
) -> Result<(), StatusCode> {
    if !matches!(config.auth.mode.as_str(), "token" | "hybrid") {
        return Ok(());
    }

    let secret = config
        .auth
        .token_secret
        .as_ref()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = extract_bearer_token(headers)
        .or_else(|| extract_query_token(query))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let validator = TokenValidator::new(secret.clone());
    validator
        .validate_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(())
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    let token = value.strip_prefix("Bearer ")?.trim();

    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

fn extract_query_token(
    query: Option<&Query<std::collections::HashMap<String, String>>>,
) -> Option<&str> {
    query?
        .0
        .get("token")
        .map(String::as_str)
        .filter(|token| !token.trim().is_empty())
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
                info!(
                    "Received batch: {} with {} events",
                    batch_id, batch.event_count
                );

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

#[cfg(test)]
mod tests {
    use super::{extract_bearer_token, extract_query_token};
    use axum::{
        extract::Query,
        http::{header::AUTHORIZATION, HeaderMap, HeaderValue},
    };
    use std::collections::HashMap;

    #[test]
    fn extracts_bearer_token() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer my-token"));

        assert_eq!(extract_bearer_token(&headers), Some("my-token"));
    }

    #[test]
    fn ignores_invalid_authorization_header() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Basic abc"));

        assert_eq!(extract_bearer_token(&headers), None);
    }

    #[test]
    fn extracts_query_token() {
        let query = Query(HashMap::from([(
            String::from("token"),
            String::from("abc"),
        )]));

        assert_eq!(extract_query_token(Some(&query)), Some("abc"));
    }
}

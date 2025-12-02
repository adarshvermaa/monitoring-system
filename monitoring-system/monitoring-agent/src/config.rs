use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub agent: AgentSettings,
    pub collector: CollectorSettings,
    pub buffer: BufferSettings,
    pub collectors: CollectorConfigs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSettings {
    pub id: String,
    pub hostname: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectorSettings {
    pub endpoint: String,
    #[serde(default)]
    pub grpc_endpoint: Option<String>,
    pub auth_token: Option<String>,
    pub tls_ca_cert: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_secs: u64,
    #[serde(default = "default_request_timeout")]
    pub request_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferSettings {
    #[serde(default = "default_max_events")]
    pub max_events: usize,
    #[serde(default = "default_flush_interval")]
    pub flush_interval_secs: u64,
    #[serde(default = "default_max_batch_size")]
    pub max_batch_size: usize,
    #[serde(default = "default_compression")]
    pub compression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectorConfigs {
    pub logs: LogCollectorConfig,
    pub metrics: MetricsCollectorConfig,
    pub traffic: TrafficCollectorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCollectorConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub journald_units: Vec<String>,
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsCollectorConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_system_interval")]
    pub system_interval_secs: u64,
    #[serde(default)]
    pub prometheus_endpoints: Vec<String>,
    #[serde(default)]
    pub include_process_metrics: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficCollectorConfig {
    #[serde(default)]
    pub enabled: bool,
    pub interface: Option<String>,
    #[serde(default)]
    pub protocols: Vec<String>,
    #[serde(default = "default_sample_rate")]
    pub sample_rate: f64,
    #[serde(default)]
    pub capture_payload: bool,
}

// Default values
fn default_max_events() -> usize {
    10000
}

fn default_flush_interval() -> u64 {
    60
}

fn default_max_batch_size() -> usize {
    1000
}

fn default_compression() -> String {
    "snappy".to_string()
}

fn default_system_interval() -> u64 {
    10
}

fn default_sample_rate() -> f64 {
    0.1
}

fn default_connect_timeout() -> u64 {
    30
}

fn default_request_timeout() -> u64 {
    60
}

impl AgentConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let config_str = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;
        
        let mut config: AgentConfig = toml::from_str(&config_str)
            .context("Failed to parse config file")?;

        // Set hostname if not provided
        if config.agent.hostname.is_empty() {
            config.agent.hostname = hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_else(|| "unknown".to_string());
        }

        // Expand environment variables in auth token
        if let Some(token) = &config.collector.auth_token {
            if token.starts_with("${") && token.ends_with("}") {
                let var_name = &token[2..token.len() - 1];
                config.collector.auth_token = std::env::var(var_name).ok();
            }
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AgentConfig {
            agent: AgentSettings {
                id: "test-agent".to_string(),
                hostname: "test-host".to_string(),
                tags: vec!["env:test".to_string()],
            },
            collector: CollectorSettings {
                endpoint: "wss://localhost:8080/ingest".to_string(),
                grpc_endpoint: None,
                auth_token: Some("test-token".to_string()),
                tls_ca_cert: None,
                client_cert: None,
                client_key: None,
                connect_timeout_secs: 30,
                request_timeout_secs: 60,
            },
            buffer: BufferSettings {
                max_events: 10000,
                flush_interval_secs: 60,
                max_batch_size: 1000,
                compression: "snappy".to_string(),
            },
            collectors: CollectorConfigs {
                logs: LogCollectorConfig {
                    enabled: true,
                    files: vec![],
                    journald_units: vec![],
                    exclude_patterns: vec![],
                },
                metrics: MetricsCollectorConfig {
                    enabled: true,
                    system_interval_secs: 10,
                    prometheus_endpoints: vec![],
                    include_process_metrics: false,
                },
                traffic: TrafficCollectorConfig {
                    enabled: false,
                    interface: None,
                    protocols: vec![],
                    sample_rate: 0.1,
                    capture_payload: false,
                },
            },
        };

        assert_eq!(config.agent.id, "test-agent");
    }
}

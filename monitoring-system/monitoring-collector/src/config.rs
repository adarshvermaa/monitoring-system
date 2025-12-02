use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectorConfig {
    pub server: ServerSettings,
    pub auth: AuthSettings,
    pub storage: StorageSettings,
    pub processor: ProcessorSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub websocket_addr: String,
    pub grpc_addr: Option<String>,
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
    pub mtls_ca_cert: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSettings {
    #[serde(default = "default_auth_mode")]
    pub mode: String,  // "token", "mtls", or "hybrid"
    pub token_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSettings {
    pub backend: String,  // "clickhouse", "postgres", "s3", "console"
    pub clickhouse_url: Option<String>,
    pub postgres_url: Option<String>,
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSettings {
    #[serde(default = "default_workers")]
    pub workers: usize,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

fn default_auth_mode() -> String {
    "token".to_string()
}

fn default_workers() -> usize {
    4
}

fn default_batch_size() -> usize {
    1000
}

impl CollectorConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let config_str = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;
        
        let mut config: CollectorConfig = toml::from_str(&config_str)
            .context("Failed to parse config file")?;

        // Expand environment variables
        if let Some(secret) = &config.auth.token_secret {
            if secret.starts_with("${") && secret.ends_with("}") {
                let var_name = &secret[2..secret.len() - 1];
                config.auth.token_secret = std::env::var(var_name).ok();
            }
        }

        Ok(config)
    }
}

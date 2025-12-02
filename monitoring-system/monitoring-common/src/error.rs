use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonitoringError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Transport error: {0}")]
    Transport(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Collector error: {0}")]
    Collector(String),
    
    #[error("Buffer overflow")]
    BufferOverflow,
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, MonitoringError>;

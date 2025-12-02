use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main event enum containing all event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    Log(LogEvent),
    Metric(MetricEvent),
    Traffic(TrafficEvent),
}

/// Log event from file tailing or journald
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub timestamp: i64,
    pub source: String,
    pub level: LogLevel,
    pub message: String,
    pub fields: HashMap<String, String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// Metric event from system or Prometheus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricEvent {
    pub timestamp: i64,
    pub name: String,
    pub value: f64,
    pub metric_type: MetricType,
    pub tags: HashMap<String, String>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Traffic event from packet capture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficEvent {
    pub timestamp: i64,
    pub protocol: Protocol,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub bytes: u64,
    pub packets: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Protocol {
    HTTP,
    HTTPS,
    TCP,
    UDP,
    ICMP,
    Other(String),
}

/// Batch of events with compression info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    pub batch_id: String,
    pub agent_id: String,
    pub hostname: String,
    pub timestamp: i64,
    pub event_count: usize,
    pub compression: CompressionType,
    pub compressed_data: Vec<u8>,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    None,
    Snappy,
    Lz4,
    Gzip,
}

/// Uncompressed batch for internal use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncompressedBatch {
    pub batch_id: String,
    pub agent_id: String,
    pub hostname: String,
    pub timestamp: i64,
    pub events: Vec<Event>,
}

/// Response from collector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestResponse {
    pub batch_id: String,
    pub status: IngestStatus,
    pub error_message: Option<String>,
    pub received_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IngestStatus {
    Success,
    PartialSuccess,
    Failed,
    Rejected,
}

impl Event {
    pub fn timestamp(&self) -> i64 {
        match self {
            Event::Log(e) => e.timestamp,
            Event::Metric(e) => e.timestamp,
            Event::Traffic(e) => e.timestamp,
        }
    }
    
    pub fn event_type(&self) -> &str {
        match self {
            Event::Log(_) => "log",
            Event::Metric(_) => "metric",
            Event::Traffic(_) => "traffic",
        }
    }
}

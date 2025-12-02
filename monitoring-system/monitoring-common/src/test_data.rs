// Test data generator for development and testing

use monitoring_common::{Event, LogEvent, LogLevel, MetricEvent, MetricType, TrafficEvent, Protocol};
use std::collections::HashMap;

/// Generate sample log events
pub fn generate_log_events(count: usize) -> Vec<Event> {
    let mut events = Vec::with_capacity(count);
    let sources = vec![
        "/var/log/nginx/access.log",
        "/var/log/app/application.log",
        "/var/log/syslog",
    ];
    let messages = vec![
        "User login successful",
        "API request completed in 45ms",
        "Database query executed",
        "Cache hit for key: user_123",
        "Started processing job",
    ];

    for i in 0..count {
        let mut fields = HashMap::new();
        fields.insert("request_id".to_string(), format!("req_{}", i));
        fields.insert("user_id".to_string(), format!("user_{}", i % 100));

        let event = Event::Log(LogEvent {
            timestamp: chrono::Utc::now().timestamp_millis() + (i as i64 * 1000),
            source: sources[i % sources.len()].to_string(),
            level: match i % 5 {
                0 => LogLevel::Debug,
                1 | 2 => LogLevel::Info,
                3 => LogLevel::Warning,
                4 => LogLevel::Error,
                _ => LogLevel::Info,
            },
            message: format!("{} ({})", messages[i % messages.len()], i),
            fields,
            tags: vec![
                "env:test".to_string(),
                format!("instance:{}", i % 3),
            ],
        });
        events.push(event);
    }

    events
}

/// Generate sample metric events
pub fn generate_metric_events(count: usize) -> Vec<Event> {
    let mut events = Vec::with_capacity(count);

    for i in 0..count {
        let timestamp = chrono::Utc::now().timestamp_millis() + (i as i64 * 10000);
        
        // CPU metric
        let cpu_event = Event::Metric(MetricEvent {
            timestamp,
            name: "system.cpu.usage".to_string(),
            value: 20.0 + (i as f64 % 60.0),
            metric_type: MetricType::Gauge,
            tags: HashMap::from([
                ("host".to_string(), format!("host-{}", i % 5)),
            ]),
            unit: Some("%".to_string()),
        });
        events.push(cpu_event);

        // Memory metric
        let mem_event = Event::Metric(MetricEvent {
            timestamp,
            name: "system.mem.usage".to_string(),
            value: 50.0 + (i as f64 % 40.0),
            metric_type: MetricType::Gauge,
            tags: HashMap::from([
                ("host".to_string(), format!("host-{}", i % 5)),
            ]),
            unit: Some("%".to_string()),
        });
        events.push(mem_event);

        // Request counter
        let req_event = Event::Metric(MetricEvent {
            timestamp,
            name: "http.requests.total".to_string(),
            value: i as f64,
            metric_type: MetricType::Counter,
            tags: HashMap::from([
                ("method".to_string(), "GET".to_string()),
                ("status".to_string(), "200".to_string()),
            ]),
            unit: None,
        });
        events.push(req_event);
    }

    events
}

/// Generate sample traffic events
pub fn generate_traffic_events(count: usize) -> Vec<Event> {
    let mut events = Vec::with_capacity(count);
    let protocols = vec![Protocol::HTTP, Protocol::HTTPS, Protocol::TCP, Protocol::UDP];
    
    for i in 0..count {
        let event = Event::Traffic(TrafficEvent {
            timestamp: chrono::Utc::now().timestamp_millis() + (i as i64 * 100),
            protocol: protocols[i % protocols.len()].clone(),
            src_ip: format!("192.168.1.{}", (i % 254) + 1),
            dst_ip: format!("10.0.0.{}", (i % 254) + 1),
            src_port: 50000 + (i % 5000) as u16,
            dst_port: match i % 4 {
                0 => 80,
                1 => 443,
                2 => 3306,
                3 => 5432,
                _ => 8080,
            },
            bytes: ((i % 1000) + 100) as u64 * 1024,
            packets: ((i % 50) + 1) as u64,
            metadata: HashMap::new(),
        });
        events.push(event);
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_log_events() {
        let events = generate_log_events(10);
        assert_eq!(events.len(), 10);
        
        for event in events {
            match event {
                Event::Log(log) => {
                    assert!(!log.message.is_empty());
                    assert!(!log.source.is_empty());
                }
                _ => panic!("Expected log event"),
            }
        }
    }

    #[test]
    fn test_generate_metric_events() {
        let events = generate_metric_events(5);
        assert_eq!(events.len(), 15); // 3 metrics per iteration
        
        for event in events {
            match event {
                Event::Metric(metric) => {
                    assert!(!metric.name.is_empty());
                    assert!(metric.value >= 0.0);
                }
                _ => panic!("Expected metric event"),
            }
        }
    }

    #[test]
    fn test_generate_traffic_events() {
        let events = generate_traffic_events(10);
        assert_eq!(events.len(), 10);
        
        for event in events {
            match event {
                Event::Traffic(traffic) => {
                    assert!(!traffic.src_ip.is_empty());
                    assert!(!traffic.dst_ip.is_empty());
                    assert!(traffic.bytes > 0);
                }
                _ => panic!("Expected traffic event"),
            }
        }
    }
}

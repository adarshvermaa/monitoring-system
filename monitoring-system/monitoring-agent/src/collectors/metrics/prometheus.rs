use crate::buffer::RingBuffer;
use anyhow::Result;
use monitoring_common::{Event, MetricEvent, MetricType};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, warn};

pub struct PrometheusScaper {
    endpoint: String,
    buffer: Arc<RingBuffer>,
    client: reqwest::Client,
}

impl PrometheusScaper {
    pub fn new(endpoint: String, buffer: Arc<RingBuffer>) -> Self {
        Self {
            endpoint,
            buffer,
            client: reqwest::Client::new(),
        }
    }

    pub async fn run(self) -> Result<()> {
        // Scrape every 30 seconds
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

        loop {
            interval.tick().await;

            if let Err(e) = self.scrape().await {
                error!("Failed to scrape {}: {}", self.endpoint, e);
            }
        }
    }

    async fn scrape(&self) -> Result<()> {
        debug!("Scraping Prometheus endpoint: {}", self.endpoint);

        let response = self.client
            .get(&self.endpoint)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        let text = response.text().await?;
        let timestamp = chrono::Utc::now().timestamp_millis();

        // Parse Prometheus text format
        self.parse_prometheus_metrics(&text, timestamp)?;

        Ok(())
    }

    fn parse_prometheus_metrics(&self, text: &str, timestamp: i64) -> Result<()> {
        let lines = prometheus_parse::Scrape::parse(text.lines().map(|s| Ok(s.to_owned())))?;

        for sample in lines.samples {
            let mut tags = HashMap::new();
            
            // Add labels as tags
            for (key, value) in &sample.labels {
                tags.insert(key.clone(), value.clone());
            }

            tags.insert("endpoint".to_string(), self.endpoint.clone());

            let metric_type = match sample.metric.as_str() {
                m if m.ends_with("_total") => MetricType::Counter,
                m if m.ends_with("_bucket") || m.ends_with("_sum") || m.ends_with("_count") => {
                    MetricType::Histogram
                }
                _ => MetricType::Gauge,
            };

            let event = Event::Metric(MetricEvent {
                timestamp,
                name: format!("prometheus.{}", sample.metric),
                value: sample.value,
                metric_type,
                tags,
                unit: None,
            });

            if let Err(e) = self.buffer.push(event) {
                warn!("Buffer full, dropping Prometheus metric: {}", e);
            }
        }

        Ok(())
    }
}

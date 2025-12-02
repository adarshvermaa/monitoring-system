use crate::config::{ProcessorSettings, StorageSettings};
use crate::pipeline::Compressor;
use crate::storage::StorageBackend;
use anyhow::Result;
use monitoring_common::{Batch, Event};
use tracing::{debug, info};

pub struct BatchProcessor {
    config: ProcessorSettings,
    storage: Box<dyn StorageBackend + Send + Sync>,
}

impl BatchProcessor {
    pub fn new(config: ProcessorSettings, storage_config: StorageSettings) -> Self {
        let storage = crate::storage::create_backend(storage_config);
        
        Self {
            config,
            storage,
        }
    }

    pub async fn process(&self, batch: Batch) -> Result<()> {
        debug!("Processing batch: {}", batch.batch_id);

        // Decompress batch
        let events = Compressor::decompress(&batch)?;
        
        info!("Decompressed {} events from batch {}", events.len(), batch.batch_id);

        // Parse and enrich events
        let enriched_events = self.parse_and_enrich(events, &batch)?;

        // Store events
        self.storage.store_events(enriched_events).await?;

        Ok(())
    }

    fn parse_and_enrich(&self, events: Vec<Event>, batch: &Batch) -> Result<Vec<Event>> {
        // In a real implementation, this would:
        // - Validate event schemas
        // - Enrich with additional metadata (geo-location, etc.)
        // - Apply transformations
        // - Filter based on rules
        
        // For now, just add batch metadata as tags
        let enriched: Vec<Event> = events.into_iter().map(|mut event| {
            match &mut event {
                Event::Log(log_event) => {
                    log_event.tags.push(format!("agent_id:{}", batch.agent_id));
                    log_event.tags.push(format!("hostname:{}", batch.hostname));
                }
                Event::Metric(metric_event) => {
                    metric_event.tags.insert("agent_id".to_string(), batch.agent_id.clone());
                    metric_event.tags.insert("hostname".to_string(), batch.hostname.clone());
                }
                Event::Traffic(traffic_event) => {
                    traffic_event.metadata.insert("agent_id".to_string(), batch.agent_id.clone());
                    traffic_event.metadata.insert("hostname".to_string(), batch.hostname.clone());
                }
            }
            event
        }).collect();

        Ok(enriched)
    }
}

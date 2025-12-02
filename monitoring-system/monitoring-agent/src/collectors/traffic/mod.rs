mod pcap_collector;

use crate::config::TrafficCollectorConfig;
use crate::buffer::RingBuffer;
use anyhow::Result;
use std::sync::Arc;
use tracing::{info, warn};

pub struct TrafficCollector {
    config: TrafficCollectorConfig,
    buffer: Arc<RingBuffer>,
}

impl TrafficCollector {
    pub fn new(config: TrafficCollectorConfig, buffer: Arc<RingBuffer>) -> Self {
        Self { config, buffer }
    }

    pub async fn run(self) -> Result<()> {
        #[cfg(feature = "pcap-capture")]
        {
            info!("Starting pcap-based traffic collector");
            let collector = pcap_collector::PcapCollector::new(
                self.config.clone(),
                self.buffer,
            )?;
            collector.run().await
        }

        #[cfg(not(feature = "pcap-capture"))]
        {
            warn!("Traffic capture not compiled in");
            Ok(())
        }
    }
}

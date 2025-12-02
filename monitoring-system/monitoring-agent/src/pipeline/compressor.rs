use anyhow::Result;
use monitoring_common::{Batch, CompressionType, UncompressedBatch};
use sha2::{Digest, Sha256};

pub struct Compressor;

impl Compressor {
    pub fn compress(uncompressed: UncompressedBatch, compression: CompressionType) -> Result<Batch> {
        // Serialize events to JSON
        let json_data = serde_json::to_vec(&uncompressed.events)?;

        // Compress based on type
        let compressed_data = match compression {
            CompressionType::None => json_data.clone(),
            CompressionType::Snappy => {
                let mut encoder = snap::raw::Encoder::new();
                encoder.compress_vec(&json_data)?
            }
            #[cfg(feature = "lz4-compression")]
            CompressionType::Lz4 => {
                lz4::block::compress(&json_data, None, false)?
            }
            #[cfg(not(feature = "lz4-compression"))]
            CompressionType::Lz4 => {
                tracing::warn!("LZ4 compression not compiled in, using Snappy");
                let mut encoder = snap::raw::Encoder::new();
                encoder.compress_vec(&json_data)?
            }
            CompressionType::Gzip => {
                use std::io::Write;
                let mut encoder = flate2::write::GzEncoder::new(
                    Vec::new(),
                    flate2::Compression::default(),
                );
                encoder.write_all(&json_data)?;
                encoder.finish()?
            }
        };

        // Calculate checksum
        let mut hasher = Sha256::new();
        hasher.update(&compressed_data);
        let checksum = hex::encode(hasher.finalize());

        // Calculate compression ratio
        let original_size = json_data.len();
        let compressed_size = compressed_data.len();
        let ratio = (compressed_size as f64 / original_size as f64) * 100.0;
        
        tracing::debug!(
            "Compressed batch: {} bytes -> {} bytes ({:.1}%)",
            original_size,
            compressed_size,
            ratio
        );

        Ok(Batch {
            batch_id: uncompressed.batch_id,
            agent_id: uncompressed.agent_id,
            hostname: uncompressed.hostname,
            timestamp: uncompressed.timestamp,
            event_count: uncompressed.events.len(),
            compression,
            compressed_data,
            checksum,
        })
    }

    pub fn decompress(batch: &Batch) -> Result<Vec<monitoring_common::Event>> {
        let decompressed_data = match batch.compression {
            CompressionType::None => batch.compressed_data.clone(),
            CompressionType::Snappy => {
                let mut decoder = snap::raw::Decoder::new();
                decoder.decompress_vec(&batch.compressed_data)?
            }
            #[cfg(feature = "lz4-compression")]
            CompressionType::Lz4 => {
                lz4::block::decompress(&batch.compressed_data, None)?
            }
            #[cfg(not(feature = "lz4-compression"))]
            CompressionType::Lz4 => {
                anyhow::bail!("LZ4 compression not compiled in");
            }
            CompressionType::Gzip => {
                use std::io::Read;
                let mut decoder = flate2::read::GzDecoder::new(&batch.compressed_data[..]);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                decompressed
            }
        };

        // Verify checksum
        let mut hasher = Sha256::new();
        hasher.update(&batch.compressed_data);
        let checksum = hex::encode(hasher.finalize());
        
        if checksum != batch.checksum {
            anyhow::bail!("Checksum mismatch: expected {}, got {}", batch.checksum, checksum);
        }

        // Deserialize events
        let events = serde_json::from_slice(&decompressed_data)?;
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use monitoring_common::{Event, LogEvent, LogLevel};
    use std::collections::HashMap;

    #[test]
    fn test_compress_decompress() {
        let events = vec![Event::Log(LogEvent {
            timestamp: 123,
            source: "test".to_string(),
            level: LogLevel::Info,
            message: "test message".to_string(),
            fields: HashMap::new(),
            tags: vec![],
        })];

        let uncompressed = UncompressedBatch {
            batch_id: "test-batch".to_string(),
            agent_id: "test-agent".to_string(),
            hostname: "test-host".to_string(),
            timestamp: 123,
            events: events.clone(),
        };

        let batch = Compressor::compress(uncompressed, CompressionType::Snappy).unwrap();
        let decompressed = Compressor::decompress(&batch).unwrap();

        assert_eq!(decompressed.len(), events.len());
    }
}

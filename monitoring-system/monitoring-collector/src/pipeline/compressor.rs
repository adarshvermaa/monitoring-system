// Re-use compressor logic from agent
// This is a simplified version for the collector side

use anyhow::Result;
use monitoring_common::{Batch, CompressionType, Event};
use sha2::{Digest, Sha256};

pub struct Compressor;

impl Compressor {
    pub fn decompress(batch: &Batch) -> Result<Vec<Event>> {
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

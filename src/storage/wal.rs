use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::errors::DBError;
use crate::storage::buffer::InsertBuffer;

const WAL_MAGIC: &[u8; 4] = b"AWAL";
const WAL_VERSION: u8 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalEntry {
    pub tx_id: u64,
    pub buffer: InsertBuffer,
}

pub struct Wal {
    path: String,
    next_tx_id: u64,
}

impl Wal {
    pub fn new(wh_path: &str) -> Result<Self, DBError> {
        let path = format!("{}/wal.bin", wh_path);
        Ok(Self {
            path,
            next_tx_id: 1,
        })
    }

    pub fn append(&mut self, buffer: &InsertBuffer) -> Result<u64, DBError> {
        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;

        let entry = WalEntry {
            tx_id,
            buffer: buffer.clone(),
        };

        let data = bincode::serialize(&entry).map_err(|e| {
            DBError::UnsupportedOperation(format!("WAL serialize error: {}", e))
        })?;

        let crc = crc32(&data);
        let len = data.len() as u32;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(&len.to_le_bytes())?;
        writer.write_all(&crc.to_le_bytes())?;
        writer.write_all(&data)?;
        writer.flush()?;

        // fsync for durability
        let file = OpenOptions::new().read(true).open(&self.path)?;
        file.sync_all()?;

        debug!(tx_id = tx_id, bytes = data.len(), "WAL entry written");
        Ok(tx_id)
    }

    pub fn read_entries(&self) -> Result<Vec<WalEntry>, DBError> {
        let path = Path::new(&self.path);
        if !path.exists() {
            return Ok(vec![]);
        }

        let file = File::open(path)?;
        let file_len = file.metadata()?.len();
        if file_len == 0 {
            return Ok(vec![]);
        }

        let mut reader = BufReader::new(file);
        let mut entries = Vec::new();
        let mut pos: u64 = 0;

        while pos < file_len {
            // Read length
            let mut len_buf = [0u8; 4];
            if reader.read_exact(&mut len_buf).is_err() {
                warn!(pos = pos, "WAL truncated at length field, stopping replay");
                break;
            }
            let len = u32::from_le_bytes(len_buf);
            pos += 4;

            // Read checksum
            let mut crc_buf = [0u8; 4];
            if reader.read_exact(&mut crc_buf).is_err() {
                warn!(pos = pos, "WAL truncated at checksum field, stopping replay");
                break;
            }
            let expected_crc = u32::from_le_bytes(crc_buf);
            pos += 4;

            // Read data
            let mut data = vec![0u8; len as usize];
            if reader.read_exact(&mut data).is_err() {
                warn!(pos = pos, len = len, "WAL truncated at data, stopping replay");
                break;
            }
            pos += len as u64;

            // Verify checksum
            let actual_crc = crc32(&data);
            if actual_crc != expected_crc {
                warn!(
                    pos = pos,
                    expected = expected_crc,
                    actual = actual_crc,
                    "WAL checksum mismatch, stopping replay"
                );
                break;
            }

            // Deserialize
            match bincode::deserialize::<WalEntry>(&data) {
                Ok(entry) => {
                    entries.push(entry);
                }
                Err(e) => {
                    warn!(error = %e, "WAL entry deserialization failed, stopping replay");
                    break;
                }
            }
        }

        info!(entries = entries.len(), "WAL entries loaded");
        Ok(entries)
    }

    pub fn truncate(&mut self) -> Result<(), DBError> {
        let path = Path::new(&self.path);
        if path.exists() {
            fs::remove_file(path)?;
        }
        debug!("WAL truncated");
        Ok(())
    }

    pub fn update_tx_counter(&mut self, max_tx_id: u64) {
        if max_tx_id >= self.next_tx_id {
            self.next_tx_id = max_tx_id + 1;
        }
    }
}

fn crc32(data: &[u8]) -> u32 {
    // Simple CRC32 (IEEE) implementation
    let mut crc: u32 = 0xFFFFFFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

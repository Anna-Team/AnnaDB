use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::data_types::item::Item;
use crate::data_types::primitives::link::Link;
use crate::errors::DBError;

const SNAPSHOT_MAGIC: &[u8; 4] = b"ASNP";
const SNAPSHOT_VERSION: u8 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotData {
    pub collections: HashMap<String, HashMap<Link, Item>>,
    pub last_tx_id: u64,
}

pub struct SnapshotManager {
    wh_path: String,
}

impl SnapshotManager {
    pub fn new(wh_path: &str) -> Self {
        Self {
            wh_path: wh_path.to_string(),
        }
    }

    fn snapshot_path(&self) -> String {
        format!("{}/snapshot.bin", self.wh_path)
    }

    fn snapshot_tmp_path(&self) -> String {
        format!("{}/snapshot.bin.tmp", self.wh_path)
    }

    pub fn write(
        &self,
        collections: &HashMap<String, HashMap<Link, Item>>,
        last_tx_id: u64,
    ) -> Result<(), DBError> {
        let data = SnapshotData {
            collections: collections.clone(),
            last_tx_id,
        };

        let encoded = bincode::serialize(&data).map_err(|e| {
            DBError::UnsupportedOperation(format!("snapshot serialize error: {}", e))
        })?;

        // Write atomically: tmp file, then rename
        let tmp_path = self.snapshot_tmp_path();
        let final_path = self.snapshot_path();

        let mut file = fs::File::create(&tmp_path)?;
        file.write_all(SNAPSHOT_MAGIC)?;
        file.write_all(&[SNAPSHOT_VERSION])?;

        let len = encoded.len() as u64;
        file.write_all(&len.to_le_bytes())?;
        file.write_all(&encoded)?;
        file.flush()?;
        file.sync_all()?;

        fs::rename(&tmp_path, &final_path)?;

        info!(
            collections = data.collections.len(),
            tx_id = last_tx_id,
            bytes = encoded.len(),
            "snapshot written"
        );
        Ok(())
    }

    pub fn load(&self) -> Result<Option<SnapshotData>, DBError> {
        let path = self.snapshot_path();
        if !Path::new(&path).exists() {
            return Ok(None);
        }

        let mut file = fs::File::open(&path)?;

        // Check magic
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;
        if &magic != SNAPSHOT_MAGIC {
            warn!("snapshot file has invalid magic bytes, skipping");
            return Ok(None);
        }

        // Check version
        let mut version = [0u8; 1];
        file.read_exact(&mut version)?;
        if version[0] != SNAPSHOT_VERSION {
            warn!(
                version = version[0],
                expected = SNAPSHOT_VERSION,
                "snapshot version mismatch, skipping"
            );
            return Ok(None);
        }

        // Read length
        let mut len_buf = [0u8; 8];
        file.read_exact(&mut len_buf)?;
        let len = u64::from_le_bytes(len_buf);

        // Read data
        let mut data = vec![0u8; len as usize];
        file.read_exact(&mut data)?;

        let snapshot: SnapshotData = bincode::deserialize(&data).map_err(|e| {
            DBError::UnsupportedOperation(format!("snapshot deserialize error: {}", e))
        })?;

        info!(
            collections = snapshot.collections.len(),
            tx_id = snapshot.last_tx_id,
            "snapshot loaded"
        );
        Ok(Some(snapshot))
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.snapshot_path()).exists()
    }
}

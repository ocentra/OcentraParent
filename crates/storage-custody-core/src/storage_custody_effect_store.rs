//! Durable custody effect ledger.
//!
//! The existing retention tombstone store owns the delete publication
//! obligation. This ledger owns the adjacent source-to-effect state so a
//! restart can replay the same typed action without inventing a second event
//! journal or losing whether a local effect was actually applied.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use ocentra_eventing::envelope::StoredEventEnvelope;
use serde::{Deserialize, Serialize};

use crate::storage_custody::{
    StorageCustodyActionPlannedEvent, StorageCustodyEffectKind, StorageCustodyInput,
};

#[path = "storage_custody_effect_store_io.rs"]
mod storage_custody_effect_store_io;
#[path = "storage_custody_effect_store_mutations.rs"]
mod storage_custody_effect_store_mutations;
#[path = "storage_custody_effect_store_prepare.rs"]
mod storage_custody_effect_store_prepare;
#[path = "storage_custody_effect_store_update.rs"]
mod storage_custody_effect_store_update;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageCustodyEffectStatus {
    Prepared,
    Journaled,
    Applying,
    Applied,
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageCustodyEffectRecord {
    pub schema_version: u16,
    pub operation_ref: String,
    pub effect_kind: StorageCustodyEffectKind,
    pub effect_ref: String,
    pub relative_path: Option<String>,
    pub household_id: String,
    pub child_profile_id: String,
    pub target_device_id: String,
    pub authority_generation: u64,
    pub session_generation: u64,
    pub custody_input: StorageCustodyInput,
    pub action: StorageCustodyActionPlannedEvent,
    pub envelope: StoredEventEnvelope,
    pub status: StorageCustodyEffectStatus,
    pub manual_required_reason: Option<String>,
}

#[derive(Clone)]
pub struct StorageCustodyEffectStore {
    path: PathBuf,
}

impl StorageCustodyEffectStore {
    pub fn open(directory: impl AsRef<Path>) -> io::Result<Self> {
        let directory = directory.as_ref();
        storage_custody_effect_store_io::reject_symlink(directory)?;
        fs::create_dir_all(directory)?;
        storage_custody_effect_store_io::reject_symlink(directory)?;
        let directory = directory.canonicalize()?;
        Ok(Self {
            path: directory.join("storage-custody-effects.json"),
        })
    }

    pub fn records(&self) -> io::Result<Vec<StorageCustodyEffectRecord>> {
        self.read_records()
    }

    pub fn pending_records(&self) -> io::Result<Vec<StorageCustodyEffectRecord>> {
        Ok(self
            .read_records()?
            .into_iter()
            .filter(StorageCustodyEffectRecord::is_pending)
            .collect())
    }
}

impl StorageCustodyEffectRecord {
    fn is_pending(&self) -> bool {
        matches!(
            self.status,
            StorageCustodyEffectStatus::Prepared
                | StorageCustodyEffectStatus::Journaled
                | StorageCustodyEffectStatus::Applying
        )
    }
}

impl StorageCustodyEffectStore {
    fn read_records(&self) -> io::Result<Vec<StorageCustodyEffectRecord>> {
        storage_custody_effect_store_io::read_records(&self.path)
    }

    fn write_records(&self, records: &[StorageCustodyEffectRecord]) -> io::Result<()> {
        storage_custody_effect_store_io::write_records(&self.path, records)
    }

    fn lock(&self) -> io::Result<std::fs::File> {
        storage_custody_effect_store_io::lock(&self.path)
    }
}

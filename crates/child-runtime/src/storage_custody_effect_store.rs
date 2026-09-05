//! Child-runtime-owned durable custody effect ledger.
//!
//! This module is intentionally private to the child runtime crate.  The
//! public storage-custody domain exposes decisions and read-only domain data;
//! only this runtime owner may advance an effect to a semantic terminal state.

use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};

use ocentra_eventing::envelope::StoredEventEnvelope;
use serde::{Deserialize, Serialize};

use ocentra_storage_custody_core::storage_custody::{
    StorageCustodyActionPlannedEvent, StorageCustodyEffectKind, StorageCustodyInput,
};

#[path = "storage_custody_effect_store_apply.rs"]
mod storage_custody_effect_store_apply;
#[path = "storage_custody_effect_store_io.rs"]
mod storage_custody_effect_store_io;
#[path = "storage_custody_effect_store_manual.rs"]
mod storage_custody_effect_store_manual;
#[path = "storage_custody_effect_store_manual_lease.rs"]
mod storage_custody_effect_store_manual_lease;
#[path = "storage_custody_effect_store_mutations.rs"]
mod storage_custody_effect_store_mutations;
#[path = "storage_custody_effect_store_prepare.rs"]
mod storage_custody_effect_store_prepare;
#[path = "storage_custody_effect_store_recovery.rs"]
mod storage_custody_effect_store_recovery;
#[path = "storage_custody_effect_store_terminal.rs"]
mod storage_custody_effect_store_terminal;
#[path = "storage_custody_effect_store_update.rs"]
mod storage_custody_effect_store_update;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) enum StorageCustodyEffectStatus {
    Prepared,
    Journaled,
    Applying,
    Applied,
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(super) struct StorageCustodyEffectRecord {
    pub(super) schema_version: u16,
    pub(super) operation_ref: String,
    pub(super) effect_kind: StorageCustodyEffectKind,
    pub(super) effect_ref: String,
    pub(super) relative_path: Option<String>,
    pub(super) household_id: String,
    pub(super) child_profile_id: String,
    pub(super) target_device_id: String,
    pub(super) authority_generation: u64,
    pub(super) session_generation: u64,
    pub(super) custody_input: StorageCustodyInput,
    pub(super) action: StorageCustodyActionPlannedEvent,
    pub(super) envelope: StoredEventEnvelope,
    status: StorageCustodyEffectStatus,
    manual_required_reason: Option<String>,
    /// A persisted owner lease prevents a second runtime from re-entering a
    /// local apply.  A missing lease on Applying is accepted only as a legacy
    /// orphan and is converted to manual recovery while the process lock is
    /// held.
    #[serde(default)]
    apply_lease_id: Option<String>,
}

pub(super) struct StorageCustodyEffectRecordPreparation {
    pub(super) operation_ref: String,
    pub(super) effect_kind: StorageCustodyEffectKind,
    pub(super) effect_ref: String,
    pub(super) relative_path: Option<String>,
    pub(super) household_id: String,
    pub(super) child_profile_id: String,
    pub(super) target_device_id: String,
    pub(super) authority_generation: u64,
    pub(super) session_generation: u64,
    pub(super) custody_input: StorageCustodyInput,
    pub(super) action: StorageCustodyActionPlannedEvent,
    pub(super) envelope: StoredEventEnvelope,
}

#[derive(Clone)]
pub(super) struct StorageCustodyEffectStore {
    path: PathBuf,
    _instance_lock: Arc<std::fs::File>,
}

impl StorageCustodyEffectStore {
    pub(super) fn open(directory: impl AsRef<Path>) -> io::Result<Self> {
        let directory = directory.as_ref();
        storage_custody_effect_store_io::reject_symlink(directory)?;
        fs::create_dir_all(directory)?;
        storage_custody_effect_store_io::reject_symlink(directory)?;
        let directory = directory.canonicalize()?;
        let instance_lock = storage_custody_effect_store_io::open_instance_lock(&directory)?;
        Ok(Self {
            path: directory.join("storage-custody-effects.json"),
            _instance_lock: Arc::new(instance_lock),
        })
    }

    pub(super) fn records(&self) -> io::Result<Vec<StorageCustodyEffectRecord>> {
        self.read_records()
    }

    pub(super) fn pending_records(&self) -> io::Result<Vec<StorageCustodyEffectRecord>> {
        Ok(self
            .read_records()?
            .into_iter()
            .filter(StorageCustodyEffectRecord::is_pending)
            .collect())
    }
}

impl StorageCustodyEffectRecord {
    pub(super) fn prepared(preparation: StorageCustodyEffectRecordPreparation) -> Self {
        Self {
            schema_version: 1,
            operation_ref: preparation.operation_ref,
            effect_kind: preparation.effect_kind,
            effect_ref: preparation.effect_ref,
            relative_path: preparation.relative_path,
            household_id: preparation.household_id,
            child_profile_id: preparation.child_profile_id,
            target_device_id: preparation.target_device_id,
            authority_generation: preparation.authority_generation,
            session_generation: preparation.session_generation,
            custody_input: preparation.custody_input,
            action: preparation.action,
            envelope: preparation.envelope,
            status: StorageCustodyEffectStatus::Prepared,
            manual_required_reason: None,
            apply_lease_id: None,
        }
    }

    pub(super) fn status(&self) -> StorageCustodyEffectStatus {
        self.status
    }

    pub(super) fn manual_required_reason(&self) -> Option<&str> {
        self.manual_required_reason.as_deref()
    }

    fn is_pending(&self) -> bool {
        matches!(
            self.status,
            StorageCustodyEffectStatus::Prepared
                | StorageCustodyEffectStatus::Journaled
                | StorageCustodyEffectStatus::Applying
        )
    }

    fn validate_loaded(&self) -> io::Result<()> {
        if self.schema_version != 1
            || self.operation_ref.trim().is_empty()
            || self.effect_ref.trim().is_empty()
            || self.household_id.trim().is_empty()
            || self.child_profile_id.trim().is_empty()
            || self.target_device_id.trim().is_empty()
            || self.authority_generation == 0
            || self.session_generation == 0
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custody effect record has an invalid binding",
            ));
        }
        if self.effect_kind == StorageCustodyEffectKind::LocalDelete
            && self.relative_path.as_deref().is_none_or(str::is_empty)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "local delete effect requires a relative payload path",
            ));
        }
        if self.effect_kind != StorageCustodyEffectKind::LocalDelete && self.relative_path.is_some()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "non-local custody effects must not carry a filesystem path",
            ));
        }
        self.validate_status_invariants()
    }

    fn validate_status_invariants(&self) -> io::Result<()> {
        let valid = match self.status {
            StorageCustodyEffectStatus::Prepared | StorageCustodyEffectStatus::Journaled => {
                self.apply_lease_id.is_none() && self.manual_required_reason.is_none()
            }
            StorageCustodyEffectStatus::Applying => {
                self.manual_required_reason.is_none()
                    && self
                        .apply_lease_id
                        .as_deref()
                        .is_none_or(|lease| !lease.trim().is_empty())
            }
            StorageCustodyEffectStatus::Applied => {
                self.apply_lease_id.is_none() && self.manual_required_reason.is_none()
            }
            StorageCustodyEffectStatus::ManualRequired => {
                self.apply_lease_id.is_none()
                    && self
                        .manual_required_reason
                        .as_deref()
                        .is_some_and(|reason| !reason.trim().is_empty())
            }
        };
        if valid {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custody effect status, apply lease, and manual reason are inconsistent",
            ))
        }
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

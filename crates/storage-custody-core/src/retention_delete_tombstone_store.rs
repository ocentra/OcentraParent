use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;

mod record;

use ocentra_eventing::envelope::StoredEventEnvelope;

use crate::storage_custody::{
    LocalPayloadRetentionAction, StorageCustodyActionPlannedEvent, StorageTombstoneState,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RetentionDeleteOutboxRecord {
    pub version: u16,
    pub deletion_ref: String,
    pub proof_ref: String,
    pub terminal_pending: bool,
    payload: RetentionDeleteOutboxPayload,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum RetentionDeleteOutboxPayload {
    LegacyVersionOne,
    Typed(Box<TypedTombstoneOutboxPayload>),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct TypedTombstoneOutboxPayload {
    pub(super) action: StorageCustodyActionPlannedEvent,
    pub(super) envelope: StoredEventEnvelope,
}

impl RetentionDeleteOutboxRecord {
    pub fn typed_action_and_envelope(
        &self,
    ) -> Option<(&StorageCustodyActionPlannedEvent, &StoredEventEnvelope)> {
        match &self.payload {
            RetentionDeleteOutboxPayload::LegacyVersionOne => None,
            RetentionDeleteOutboxPayload::Typed(payload) => {
                Some((&payload.action, &payload.envelope))
            }
        }
    }

    fn typed(
        deletion_ref: String,
        proof_ref: String,
        action: StorageCustodyActionPlannedEvent,
        envelope: StoredEventEnvelope,
    ) -> Self {
        record::typed(deletion_ref, proof_ref, action, envelope)
    }

    fn decode(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        record::decode(value)
    }

    fn encode(&self) -> Result<serde_json::Value, serde_json::Error> {
        record::encode(self)
    }
}

#[derive(Clone)]
pub struct RetentionDeleteTombstoneStore {
    path: PathBuf,
}

impl RetentionDeleteTombstoneStore {
    pub fn open(directory: impl AsRef<Path>) -> io::Result<Self> {
        fs::create_dir_all(directory.as_ref())?;
        if fs::symlink_metadata(directory.as_ref())?
            .file_type()
            .is_symlink()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "custody store directory must not be a symlink",
            ));
        }
        let directory = directory.as_ref().canonicalize()?;
        Ok(Self {
            path: directory.join("retention-delete-tombstones.json"),
        })
    }

    pub fn records(&self) -> io::Result<Vec<RetentionDeleteOutboxRecord>> {
        match fs::read(&self.path) {
            Ok(bytes) => {
                let values: Vec<serde_json::Value> = serde_json::from_slice(&bytes)
                    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
                values
                    .into_iter()
                    .map(RetentionDeleteOutboxRecord::decode)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(error),
        }
    }

    pub fn persist_action_plan_intent(
        &self,
        envelope: StoredEventEnvelope,
        action: StorageCustodyActionPlannedEvent,
    ) -> io::Result<()> {
        if action.action_plan.tombstone_state != StorageTombstoneState::Write
            || action.action_plan.local_payload_retention_action
                != LocalPayloadRetentionAction::Delete
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "only a coherent delete custody action can create a tombstone intent",
            ));
        }

        let deletion_ref = format!(
            "storage-custody-delete:{}",
            action.source_decision_id.as_str()
        );
        let proof_ref = action.action_plan_id.as_str().to_owned();
        let lock = self.lock()?;
        lock.lock_exclusive()?;
        let mut records = self.records()?;
        if !records
            .iter()
            .any(|record| record.deletion_ref == deletion_ref)
        {
            records.push(RetentionDeleteOutboxRecord::typed(
                deletion_ref,
                proof_ref,
                action,
                envelope,
            ));
        }
        let result = self.write(&records);
        FileExt::unlock(&lock)?;
        result
    }

    pub fn mark_terminal_published(&self, deletion_ref: &str) -> io::Result<()> {
        let lock = self.lock()?;
        lock.lock_exclusive()?;
        let mut records = self.records()?;
        for record in &mut records {
            if record.deletion_ref == deletion_ref {
                record.terminal_pending = false;
            }
        }
        let result = self.write(&records);
        FileExt::unlock(&lock)?;
        result
    }

    fn write(&self, records: &[RetentionDeleteOutboxRecord]) -> io::Result<()> {
        let encoded = records
            .iter()
            .map(RetentionDeleteOutboxRecord::encode)
            .collect::<Result<Vec<_>, _>>()
            .map_err(io::Error::other)?;
        AtomicFile::new(&self.path, AllowOverwrite)
            .write(|file| {
                serde_json::to_writer(&mut *file, &encoded).map_err(io::Error::other)?;
                file.sync_all()
            })
            .map_err(|error| io::Error::other(error.to_string()))?;
        sync_parent_directory(&self.path)
    }

    fn lock(&self) -> io::Result<std::fs::File> {
        OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(self.path.with_extension("lock"))
    }
}

#[cfg(not(windows))]
fn sync_parent_directory(path: &Path) -> io::Result<()> {
    std::fs::File::open(path.parent().unwrap_or_else(|| Path::new(".")))?.sync_all()
}

#[cfg(windows)]
fn sync_parent_directory(_path: &Path) -> io::Result<()> {
    Ok(())
}

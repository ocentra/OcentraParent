use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;

#[path = "retention_delete_tombstone_store_path.rs"]
mod path;
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
    TerminalMarker,
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
        if let RetentionDeleteOutboxPayload::Typed(payload) = &self.payload {
            Some((&payload.action, &payload.envelope))
        } else {
            None
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
        let directory = directory.as_ref();
        path::reject_symlink(directory, "custody store directory")?;
        fs::create_dir_all(directory)?;
        path::reject_symlink(directory, "custody store directory")?;
        let directory = directory.canonicalize()?;
        let path = directory.join("retention-delete-tombstones.json");
        path::reject_symlink(&path, "custody tombstone record")?;
        path::reject_symlink(&path.with_extension("lock"), "custody tombstone lock")?;
        Ok(Self { path })
    }

    pub fn records(&self) -> io::Result<Vec<RetentionDeleteOutboxRecord>> {
        path::reject_symlink(&self.path, "custody tombstone record")?;
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
        if let Some(existing) = records
            .iter_mut()
            .find(|record| record.deletion_ref == deletion_ref)
        {
            // A v1 pending row has no typed action/envelope to replay. A
            // repeated typed custody event is the migration boundary.
            existing.replace_legacy_pending_with_typed(deletion_ref, proof_ref, action, envelope);
        } else {
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
        if !records
            .iter()
            .any(|record| record.deletion_ref == deletion_ref)
        {
            FileExt::unlock(&lock)?;
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("unknown retention delete tombstone: {deletion_ref}"),
            ));
        }
        for record in &mut records {
            if record.deletion_ref == deletion_ref {
                record.terminal_pending = false;
                record.version = record::TERMINAL_MARKER_STORE_VERSION;
                record.payload = RetentionDeleteOutboxPayload::TerminalMarker;
            }
        }
        let result = self.write(&records);
        FileExt::unlock(&lock)?;
        result
    }

    fn write(&self, records: &[RetentionDeleteOutboxRecord]) -> io::Result<()> {
        path::reject_symlink(&self.path, "custody tombstone record")?;
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
        let lock_path = self.path.with_extension("lock");
        path::reject_symlink(&lock_path, "custody tombstone lock")?;
        OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(lock_path)
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

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use ocentra_eventing::envelope::StoredEventEnvelope;
use ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent;

use crate::{
    child_runtime_tombstone_event_flow::RetentionDeleteTombstoneExecutor,
    service::storage_custody_runtime::StorageCustodyTerminalEffectCapability,
};

#[path = "retention_delete_tombstone_store_path.rs"]
mod path;
mod record;
#[path = "retention_delete_tombstone_store_intent.rs"]
mod retention_delete_tombstone_store_intent;
#[path = "retention_delete_tombstone_store_io.rs"]
mod retention_delete_tombstone_store_io;
#[path = "retention_delete_tombstone_store_terminal.rs"]
mod retention_delete_tombstone_store_terminal;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RetentionDeleteOutboxRecord {
    pub(crate) version: u16,
    pub(crate) deletion_ref: String,
    pub(crate) proof_ref: String,
    pub(crate) terminal_pending: bool,
    payload: RetentionDeleteOutboxPayload,
}

#[derive(Clone, Debug, PartialEq)]
enum RetentionDeleteOutboxPayload {
    LegacyVersionOne,
    Typed(Box<TypedTombstoneOutboxPayload>),
    TerminalMarker,
}

#[derive(Clone, Debug, PartialEq)]
struct TypedTombstoneOutboxPayload {
    action: StorageCustodyActionPlannedEvent,
    envelope: StoredEventEnvelope,
}

impl RetentionDeleteOutboxRecord {
    pub(crate) fn typed_action_and_envelope(
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

    fn validate_loaded(&self) -> io::Result<()> {
        if self.deletion_ref.trim().is_empty() || self.proof_ref.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custody tombstone record has an empty identity",
            ));
        }
        let valid = match &self.payload {
            RetentionDeleteOutboxPayload::LegacyVersionOne => {
                self.version == record::LEGACY_STORE_VERSION && self.terminal_pending
            }
            RetentionDeleteOutboxPayload::Typed(_) => {
                self.version == record::TYPED_STORE_VERSION && self.terminal_pending
            }
            RetentionDeleteOutboxPayload::TerminalMarker => {
                self.version == record::TERMINAL_MARKER_STORE_VERSION && !self.terminal_pending
            }
        };
        if valid {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custody tombstone record has an invalid version, payload, or terminal state",
            ))
        }
    }
}

#[derive(Clone)]
pub(crate) struct RetentionDeleteTombstoneStore {
    path: PathBuf,
}

impl RetentionDeleteTombstoneStore {
    pub(crate) fn open(directory: impl AsRef<Path>) -> io::Result<Self> {
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

    pub(crate) fn records(&self) -> io::Result<Vec<RetentionDeleteOutboxRecord>> {
        retention_delete_tombstone_store_io::records(self)
    }

    pub(crate) fn persist_action_plan_intent(
        &self,
        _executor: &RetentionDeleteTombstoneExecutor,
        envelope: StoredEventEnvelope,
        action: StorageCustodyActionPlannedEvent,
    ) -> io::Result<()> {
        retention_delete_tombstone_store_intent::persist(self, envelope, action)
    }

    pub(crate) fn mark_terminal_published(
        &self,
        _executor: &RetentionDeleteTombstoneExecutor,
        _terminal_effect: &StorageCustodyTerminalEffectCapability,
        deletion_ref: &str,
        action: &StorageCustodyActionPlannedEvent,
    ) -> io::Result<()> {
        retention_delete_tombstone_store_terminal::mark(self, deletion_ref, action)
    }

    fn write(&self, records: &[RetentionDeleteOutboxRecord]) -> io::Result<()> {
        retention_delete_tombstone_store_io::write(self, records)
    }

    fn lock(&self) -> io::Result<std::fs::File> {
        retention_delete_tombstone_store_io::lock(self)
    }
}

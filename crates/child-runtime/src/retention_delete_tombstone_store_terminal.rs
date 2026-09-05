use std::io;

use fs2::FileExt;
use ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent;

use super::{
    RetentionDeleteOutboxPayload, RetentionDeleteOutboxRecord, RetentionDeleteTombstoneStore,
};

pub(super) fn mark(
    store: &RetentionDeleteTombstoneStore,
    deletion_ref: &str,
    action: &StorageCustodyActionPlannedEvent,
) -> io::Result<()> {
    let lock = store.lock()?;
    lock.lock_exclusive()?;
    let mut records = store.records()?;
    let Some(record) = records
        .iter_mut()
        .find(|record| record.deletion_ref == deletion_ref)
    else {
        return unlock_with_error(
            &lock,
            io::ErrorKind::NotFound,
            &format!("unknown retention delete tombstone: {deletion_ref}"),
        );
    };
    if !record.terminal_pending && record.version == super::record::TERMINAL_MARKER_STORE_VERSION {
        super::retention_delete_tombstone_store_io::unlock(&lock)?;
        return Ok(());
    }
    let Some((stored_action, _)) = record.typed_action_and_envelope() else {
        return unlock_with_error(
            &lock,
            io::ErrorKind::InvalidData,
            "legacy retention delete tombstone requires typed terminal proof",
        );
    };
    if !terminal_proof_matches(record, stored_action, action) {
        return unlock_with_error(
            &lock,
            io::ErrorKind::InvalidInput,
            "retention delete terminal proof does not match the durable typed intent",
        );
    }
    record.terminal_pending = false;
    record.version = super::record::TERMINAL_MARKER_STORE_VERSION;
    record.payload = RetentionDeleteOutboxPayload::TerminalMarker;
    let result = store.write(&records);
    super::retention_delete_tombstone_store_io::unlock(&lock)?;
    result
}

fn terminal_proof_matches(
    record: &RetentionDeleteOutboxRecord,
    stored_action: &StorageCustodyActionPlannedEvent,
    action: &StorageCustodyActionPlannedEvent,
) -> bool {
    let expected_deletion_ref = format!(
        "storage-custody-delete:{}",
        stored_action.source_decision_id.as_str()
    );
    stored_action == action
        && expected_deletion_ref == record.deletion_ref
        && record.proof_ref == stored_action.action_plan_id.as_str()
}

fn unlock_with_error(lock: &std::fs::File, kind: io::ErrorKind, message: &str) -> io::Result<()> {
    super::retention_delete_tombstone_store_io::unlock(lock)?;
    Err(io::Error::new(kind, message))
}

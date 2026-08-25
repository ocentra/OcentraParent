use std::io;

use fs2::FileExt;
use ocentra_eventing::envelope::StoredEventEnvelope;
use ocentra_storage_custody_core::storage_custody::{
    LocalPayloadRetentionAction, StorageCustodyActionPlannedEvent, StorageTombstoneState,
};

use super::{RetentionDeleteOutboxRecord, RetentionDeleteTombstoneStore};

pub(super) fn persist(
    store: &RetentionDeleteTombstoneStore,
    envelope: StoredEventEnvelope,
    action: StorageCustodyActionPlannedEvent,
) -> io::Result<()> {
    if !is_coherent_delete(&action) {
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
    let lock = store.lock()?;
    lock.lock_exclusive()?;
    let mut records = store.records()?;
    if let Some(existing) = records
        .iter_mut()
        .find(|record| record.deletion_ref == deletion_ref)
    {
        ensure_reusable_intent(existing, &proof_ref, &action, &lock)?;
        existing.replace_legacy_pending_with_typed(deletion_ref, proof_ref, action, envelope);
    } else {
        records.push(RetentionDeleteOutboxRecord::typed(
            deletion_ref,
            proof_ref,
            action,
            envelope,
        ));
    }
    let result = store.write(&records);
    super::retention_delete_tombstone_store_io::unlock(&lock)?;
    result
}

fn is_coherent_delete(action: &StorageCustodyActionPlannedEvent) -> bool {
    action.action_plan.tombstone_state == StorageTombstoneState::Write
        && action.action_plan.local_payload_retention_action == LocalPayloadRetentionAction::Delete
}

fn ensure_reusable_intent(
    existing: &RetentionDeleteOutboxRecord,
    proof_ref: &str,
    action: &StorageCustodyActionPlannedEvent,
    lock: &std::fs::File,
) -> io::Result<()> {
    if !existing.terminal_pending {
        unlock_with_error(
            lock,
            io::ErrorKind::AlreadyExists,
            "retention delete reference is already terminal",
        )?;
    }
    if existing.proof_ref != proof_ref {
        unlock_with_error(
            lock,
            io::ErrorKind::AlreadyExists,
            "retention delete reference was reused with a different action plan",
        )?;
    }
    if let Some((stored_action, _)) = existing.typed_action_and_envelope() {
        if stored_action != action {
            unlock_with_error(
                lock,
                io::ErrorKind::AlreadyExists,
                "retention delete reference was reused with a different custody action",
            )?;
        }
    }
    Ok(())
}

fn unlock_with_error(lock: &std::fs::File, kind: io::ErrorKind, message: &str) -> io::Result<()> {
    super::retention_delete_tombstone_store_io::unlock(lock)?;
    Err(io::Error::new(kind, message))
}

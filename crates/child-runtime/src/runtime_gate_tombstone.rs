//! Child-runtime boundary for typed retention tombstone publication.

use ocentra_eventing::{
    envelope::StoredEventEnvelope,
    journal::{ndjson::NdjsonEventJournal, JournalAppend},
};
use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;
use ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent;

/// Persists the terminal-publish obligation before journaling the typed custody
/// delete action. If the journal append fails, the durable outbox remains for a
/// restart to replay the same idempotent action.
pub async fn persist_child_runtime_tombstone_action(
    journal: &NdjsonEventJournal,
    store: &RetentionDeleteTombstoneStore,
    envelope: &StoredEventEnvelope,
    action: &StorageCustodyActionPlannedEvent,
) -> std::io::Result<JournalAppend> {
    let journaled_action = envelope
        .decode::<StorageCustodyActionPlannedEvent>()
        .map_err(std::io::Error::other)?;
    if journaled_action.payload != *action {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "child-runtime tombstone journal payload must match the typed custody action",
        ));
    }
    store.persist_action_plan_intent(action)?;
    journal
        .append_idempotent(envelope)
        .await
        .map_err(std::io::Error::other)
}

/// Removes a durable tombstone intent only after the terminal publication is
/// confirmed by the runtime's owning delivery path.
pub fn acknowledge_child_runtime_tombstone_publication(
    store: &RetentionDeleteTombstoneStore,
    deletion_ref: &str,
) -> std::io::Result<()> {
    store.mark_terminal_published(deletion_ref)
}

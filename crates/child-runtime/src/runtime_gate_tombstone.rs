//! Child-runtime boundary for typed retention tombstone publication.

use ocentra_eventing::{
    envelope::{DomainEvent, StoredEventEnvelope},
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
    let journaled = envelope
        .decode::<StorageCustodyActionPlannedEvent>()
        .map_err(std::io::Error::other)?;
    if journaled.payload != *action
        || journaled.aggregate_key != action.aggregate_key().map_err(std::io::Error::other)?
        || journaled.idempotency_key != action.idempotency_key().map_err(std::io::Error::other)?
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "child-runtime tombstone journal envelope must match the typed custody action identity",
        ));
    }
    persist_durable_tombstone_intent(store.clone(), envelope.clone(), action.clone()).await?;
    journal
        .append_idempotent(envelope)
        .await
        .map_err(std::io::Error::other)
}

/// Removes a durable tombstone intent only after the terminal publication is
/// confirmed by the runtime's owning delivery path.
pub async fn acknowledge_child_runtime_tombstone_publication(
    store: &RetentionDeleteTombstoneStore,
    deletion_ref: &str,
) -> std::io::Result<()> {
    let store = store.clone();
    let deletion_ref = deletion_ref.to_owned();
    tokio::task::spawn_blocking(move || store.mark_terminal_published(&deletion_ref))
        .await
        .map_err(std::io::Error::other)?
}

async fn persist_durable_tombstone_intent(
    store: RetentionDeleteTombstoneStore,
    envelope: StoredEventEnvelope,
    action: StorageCustodyActionPlannedEvent,
) -> std::io::Result<()> {
    tokio::task::spawn_blocking(move || store.persist_action_plan_intent(envelope, action))
        .await
        .map_err(std::io::Error::other)?
}

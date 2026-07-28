//! Child-runtime boundary for typed retention tombstone publication.

use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;
use ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent;

/// Bridges a typed custody delete action into the durable terminal-publish
/// outbox. Callers cannot enqueue a tombstone from a UI or raw command string.
pub fn persist_child_runtime_tombstone_action(
    store: &RetentionDeleteTombstoneStore,
    action: &StorageCustodyActionPlannedEvent,
) -> std::io::Result<()> {
    store.persist_action_plan_intent(action)
}

/// Removes a durable tombstone intent only after the terminal publication is
/// confirmed by the runtime's owning delivery path.
pub fn acknowledge_child_runtime_tombstone_publication(
    store: &RetentionDeleteTombstoneStore,
    deletion_ref: &str,
) -> std::io::Result<()> {
    store.mark_terminal_published(deletion_ref)
}

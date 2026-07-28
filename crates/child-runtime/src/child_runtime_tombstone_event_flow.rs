//! Production child-runtime custody event flow for retained deletion actions.
//!
//! The flow owns envelope construction and hands the durable outbox/journal
//! boundary to `runtime_gate_tombstone`; service startup recovery remains owned
//! by the service layer.

use ocentra_eventing::{
    envelope::{EventEnvelope, EventMetadata},
    journal::ndjson::NdjsonEventJournal,
};
use ocentra_storage_custody_core::{
    retention_delete_tombstone_store::RetentionDeleteTombstoneStore,
    storage_custody::StorageCustodyActionPlannedEvent,
};

use crate::runtime_gate_tombstone::{
    persist_child_runtime_tombstone_action_with_milestones, ChildRuntimeTombstonePublicationOutcome,
};

#[derive(Clone)]
pub struct ChildRuntimeTombstoneEventFlow {
    journal: NdjsonEventJournal,
    store: RetentionDeleteTombstoneStore,
}

impl ChildRuntimeTombstoneEventFlow {
    pub fn new(journal: NdjsonEventJournal, store: RetentionDeleteTombstoneStore) -> Self {
        Self { journal, store }
    }

    /// Accepts a real typed custody action, constructs its event envelope, and
    /// exposes the durable-outbox/journal result keyed by the caller's
    /// correlation identity.
    pub async fn publish_action(
        &self,
        action: StorageCustodyActionPlannedEvent,
        metadata: EventMetadata,
    ) -> std::io::Result<ChildRuntimeTombstonePublicationOutcome> {
        let envelope = EventEnvelope::from_event(action.clone(), metadata)
            .and_then(|event| event.store())
            .map_err(std::io::Error::other)?;
        persist_child_runtime_tombstone_action_with_milestones(
            &self.journal,
            &self.store,
            &envelope,
            &action,
        )
        .await
    }
}

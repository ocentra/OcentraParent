//! Production child-runtime custody event flow for retained deletion actions.
//!
//! The flow owns envelope construction and hands the durable outbox/journal
//! boundary to `runtime_gate_tombstone`; service startup recovery remains owned
//! by the service layer.

use ocentra_eventing::{
    envelope::{EventEnvelope, EventMetadata, StoredEventEnvelope},
    journal::ndjson::NdjsonEventJournal,
};
use ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent;

use crate::runtime_gate_tombstone::{
    acknowledge_child_runtime_tombstone_publication,
    persist_child_runtime_tombstone_action_with_milestones,
    replay_pending_child_runtime_tombstones, ChildRuntimeTombstonePublicationOutcome,
    ChildRuntimeTombstoneRecoveryReport,
};

use crate::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;

/// Opaque authority for the runtime-owned tombstone outbox mutation seam.
///
/// The constructor is private to this module. Sibling modules can only reach
/// the store mutation helpers through this event-flow owner; they cannot mint
/// a raw executor token for direct persistence.
#[derive(Clone, Copy)]
pub(crate) struct RetentionDeleteTombstoneExecutor {
    _private: (),
}

impl RetentionDeleteTombstoneExecutor {
    fn new() -> Self {
        Self { _private: () }
    }
}

#[derive(Clone)]
pub(crate) struct ChildRuntimeTombstoneEventFlow {
    journal: NdjsonEventJournal,
    store: RetentionDeleteTombstoneStore,
    executor: RetentionDeleteTombstoneExecutor,
}

impl ChildRuntimeTombstoneEventFlow {
    pub(crate) fn new(journal: NdjsonEventJournal, store: RetentionDeleteTombstoneStore) -> Self {
        Self {
            journal,
            store,
            executor: RetentionDeleteTombstoneExecutor::new(),
        }
    }

    /// Accepts a real typed custody action, constructs its event envelope, and
    /// exposes the durable-outbox/journal result keyed by the caller's
    /// correlation identity.
    pub(crate) async fn publish_action(
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
            &self.executor,
            &envelope,
            &action,
        )
        .await
    }

    /// Journal a custody action that does not create a local retention
    /// tombstone (for example a parent-owned export, report/query, or settings
    /// apply request).  The same hash-chain and idempotent journal owned by
    /// the child runtime is used; provider execution is deliberately kept at
    /// the owning parent/provider boundary.
    pub(crate) async fn publish_action_to_journal(
        &self,
        action: StorageCustodyActionPlannedEvent,
        metadata: EventMetadata,
    ) -> std::io::Result<ocentra_eventing::journal::JournalAppend> {
        let envelope = EventEnvelope::from_event(action, metadata)
            .and_then(|event| event.store())
            .map_err(std::io::Error::other)?;
        self.journal
            .append_idempotent(&envelope)
            .await
            .map_err(std::io::Error::other)
    }

    /// Recovery form of [`Self::publish_action`].  The envelope was already
    /// persisted in the custody effect ledger, so replay must reuse its exact
    /// event/correlation/idempotency identity instead of accepting a newly
    /// caller-minted metadata value.
    pub(crate) async fn publish_stored_action(
        &self,
        envelope: &StoredEventEnvelope,
        action: &StorageCustodyActionPlannedEvent,
    ) -> std::io::Result<ChildRuntimeTombstonePublicationOutcome> {
        persist_child_runtime_tombstone_action_with_milestones(
            &self.journal,
            &self.store,
            &self.executor,
            envelope,
            action,
        )
        .await
    }

    pub(crate) async fn publish_stored_action_to_journal(
        &self,
        envelope: &StoredEventEnvelope,
    ) -> std::io::Result<ocentra_eventing::journal::JournalAppend> {
        self.journal
            .append_idempotent(envelope)
            .await
            .map_err(std::io::Error::other)
    }

    /// Acknowledge the retained delete obligation only after the owning
    /// runtime has durably committed the local terminal effect.  Keeping the
    /// store behind this flow prevents a caller from minting or bypassing the
    /// terminal publication boundary.
    pub(crate) async fn acknowledge_publication(
        &self,
        terminal_effect: &super::service::storage_custody_runtime::
            StorageCustodyTerminalEffectCapability,
        action: &StorageCustodyActionPlannedEvent,
    ) -> std::io::Result<()> {
        acknowledge_child_runtime_tombstone_publication(
            &self.store,
            &self.executor,
            terminal_effect,
            action,
        )
        .await
    }

    /// Service startup recovery entry point. It republishes durable pending
    /// obligations through the idempotent journal and leaves acknowledgement
    /// to the owning terminal-delivery path.
    pub(crate) async fn recover_pending(
        &self,
    ) -> std::io::Result<ChildRuntimeTombstoneRecoveryReport> {
        replay_pending_child_runtime_tombstones(&self.journal, &self.store).await
    }
}

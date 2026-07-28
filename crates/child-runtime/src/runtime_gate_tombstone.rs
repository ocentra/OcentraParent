//! Child-runtime boundary for typed retention tombstone publication.

use ocentra_eventing::{
    envelope::{DomainEvent, StoredEventEnvelope},
    ids::CorrelationId,
    journal::{ndjson::NdjsonEventJournal, JournalAppend},
};
use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;
use ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent;

/// Observable, correlation-bound milestones for a child-runtime tombstone
/// publication attempt. These are deliberately typed rather than log text so
/// the production flow and its lifecycle proof share the same evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChildRuntimeTombstoneMilestone {
    DurableOutboxWritten,
    JournalAppendConfirmed,
    JournalAppendPendingRetry,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChildRuntimeTombstonePublicationReport {
    pub correlation_id: CorrelationId,
    pub milestones: Vec<ChildRuntimeTombstoneMilestone>,
    pub append: Option<JournalAppend>,
}

/// The durable outbox was written, but the journal must be retried by the
/// owning runtime delivery path. This is not a publication success.
#[derive(Clone, Debug, PartialEq)]
pub enum ChildRuntimeTombstonePublicationOutcome {
    Journaled(ChildRuntimeTombstonePublicationReport),
    PendingJournalRetry(ChildRuntimeTombstonePublicationReport),
}

/// Persists the terminal-publish obligation before journaling the typed custody
/// delete action. If the journal append fails, the durable outbox remains for a
/// restart to replay the same idempotent action.
pub async fn persist_child_runtime_tombstone_action(
    journal: &NdjsonEventJournal,
    store: &RetentionDeleteTombstoneStore,
    envelope: &StoredEventEnvelope,
    action: &StorageCustodyActionPlannedEvent,
) -> std::io::Result<JournalAppend> {
    match persist_child_runtime_tombstone_action_with_milestones(journal, store, envelope, action)
        .await?
    {
        ChildRuntimeTombstonePublicationOutcome::Journaled(report) => report
            .append
            .ok_or_else(|| std::io::Error::other("journaled tombstone publication omitted append")),
        ChildRuntimeTombstonePublicationOutcome::PendingJournalRetry(_) => {
            Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "child-runtime tombstone journal append requires retry",
            ))
        }
    }
}

/// Persists the delete intent before journal append and exposes the exact
/// correlated boundary reached. A journal failure leaves a durable retry
/// obligation and returns `PendingJournalRetry`; callers must not treat it as
/// terminal publication.
pub async fn persist_child_runtime_tombstone_action_with_milestones(
    journal: &NdjsonEventJournal,
    store: &RetentionDeleteTombstoneStore,
    envelope: &StoredEventEnvelope,
    action: &StorageCustodyActionPlannedEvent,
) -> std::io::Result<ChildRuntimeTombstonePublicationOutcome> {
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
    let correlation_id = envelope.correlation_id.clone();
    let mut milestones = vec![ChildRuntimeTombstoneMilestone::DurableOutboxWritten];
    match journal.append_idempotent(envelope).await {
        Ok(append) => {
            milestones.push(ChildRuntimeTombstoneMilestone::JournalAppendConfirmed);
            Ok(ChildRuntimeTombstonePublicationOutcome::Journaled(
                ChildRuntimeTombstonePublicationReport {
                    correlation_id,
                    milestones,
                    append: Some(append),
                },
            ))
        }
        Err(_) => {
            milestones.push(ChildRuntimeTombstoneMilestone::JournalAppendPendingRetry);
            Ok(
                ChildRuntimeTombstonePublicationOutcome::PendingJournalRetry(
                    ChildRuntimeTombstonePublicationReport {
                        correlation_id,
                        milestones,
                        append: None,
                    },
                ),
            )
        }
    }
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

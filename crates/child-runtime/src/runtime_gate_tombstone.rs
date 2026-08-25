//! Child-runtime boundary for typed retention tombstone publication.

#[path = "runtime_gate_tombstone_error.rs"]
mod runtime_gate_tombstone_error;
#[path = "runtime_gate_tombstone_recovery.rs"]
pub(crate) mod runtime_gate_tombstone_recovery;
#[path = "runtime_gate_tombstone_recovery_validation.rs"]
mod runtime_gate_tombstone_recovery_validation;

use ocentra_eventing::{
    envelope::{DomainEvent, StoredEventEnvelope},
    ids::CorrelationId,
    journal::{ndjson::NdjsonEventJournal, JournalAppend},
};
use ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent;

use crate::child_runtime_tombstone_event_flow::RetentionDeleteTombstoneExecutor;
use crate::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;
use crate::service::storage_custody_runtime::StorageCustodyTerminalEffectCapability;

use runtime_gate_tombstone_error::is_retryable_journal_error;

pub type ChildRuntimeTombstoneRecoveryReport =
    runtime_gate_tombstone_recovery::ChildRuntimeTombstoneRecoveryReport;

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
pub(crate) async fn persist_child_runtime_tombstone_action(
    journal: &NdjsonEventJournal,
    store: &RetentionDeleteTombstoneStore,
    executor: &RetentionDeleteTombstoneExecutor,
    envelope: &StoredEventEnvelope,
    action: &StorageCustodyActionPlannedEvent,
) -> std::io::Result<JournalAppend> {
    match persist_child_runtime_tombstone_action_with_milestones(
        journal, store, executor, envelope, action,
    )
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
pub(crate) async fn persist_child_runtime_tombstone_action_with_milestones(
    journal: &NdjsonEventJournal,
    store: &RetentionDeleteTombstoneStore,
    executor: &RetentionDeleteTombstoneExecutor,
    envelope: &StoredEventEnvelope,
    action: &StorageCustodyActionPlannedEvent,
) -> std::io::Result<ChildRuntimeTombstonePublicationOutcome> {
    let journaled = envelope
        .decode::<StorageCustodyActionPlannedEvent>()
        .map_err(std::io::Error::other)?;
    if journaled.payload() != action
        || journaled.aggregate_key() != &action.aggregate_key().map_err(std::io::Error::other)?
        || journaled.idempotency_key()
            != &action.idempotency_key().map_err(std::io::Error::other)?
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "child-runtime tombstone journal envelope must match the typed custody action identity",
        ));
    }
    persist_durable_tombstone_intent(
        store.clone(),
        executor.clone(),
        envelope.clone(),
        action.clone(),
    )
    .await?;
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
        Err(error) if is_retryable_journal_error(&error) => {
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
        Err(error) => Err(std::io::Error::other(error.to_string())),
    }
}

pub(crate) async fn replay_pending_child_runtime_tombstones(
    journal: &NdjsonEventJournal,
    store: &RetentionDeleteTombstoneStore,
) -> std::io::Result<ChildRuntimeTombstoneRecoveryReport> {
    runtime_gate_tombstone_recovery::replay_pending_child_runtime_tombstones(journal, store).await
}

/// Removes a durable tombstone intent only after the terminal publication is
/// confirmed by the runtime's owning delivery path.
pub(crate) async fn acknowledge_child_runtime_tombstone_publication(
    store: &RetentionDeleteTombstoneStore,
    executor: &RetentionDeleteTombstoneExecutor,
    terminal_effect: &StorageCustodyTerminalEffectCapability,
    action: &StorageCustodyActionPlannedEvent,
) -> std::io::Result<()> {
    let store = store.clone();
    let executor = executor.clone();
    let terminal_effect = *terminal_effect;
    let deletion_ref = format!(
        "storage-custody-delete:{}",
        action.source_decision_id.as_str()
    );
    let action = action.clone();
    tokio::task::spawn_blocking(move || {
        store.mark_terminal_published(&executor, &terminal_effect, &deletion_ref, &action)
    })
    .await
    .map_err(std::io::Error::other)?
}

async fn persist_durable_tombstone_intent(
    store: RetentionDeleteTombstoneStore,
    executor: RetentionDeleteTombstoneExecutor,
    envelope: StoredEventEnvelope,
    action: StorageCustodyActionPlannedEvent,
) -> std::io::Result<()> {
    tokio::task::spawn_blocking(move || {
        store.persist_action_plan_intent(&executor, envelope, action)
    })
    .await
    .map_err(std::io::Error::other)?
}

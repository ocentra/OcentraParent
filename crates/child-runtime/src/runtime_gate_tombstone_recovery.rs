//! Startup recovery for durable child-runtime tombstone obligations.

use ocentra_eventing::{
    ids::CorrelationId,
    journal::{ndjson::NdjsonEventJournal, JournalAppend},
};
use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;

use super::runtime_gate_tombstone_error::is_retryable_journal_error;

/// Startup replay result for durable tombstone obligations. A replayed
/// journal append is not an acknowledgement: the owning delivery path must
/// still acknowledge after terminal publication is confirmed.
#[derive(Clone, Debug, PartialEq)]
pub struct ChildRuntimeTombstoneRecoveryReport {
    pub journaled: Vec<JournalAppend>,
    pub pending_journal_retry: Vec<CorrelationId>,
}

/// Replays every still-pending typed tombstone obligation after runtime
/// startup. Legacy rows and terminal markers are skipped because they do not
/// contain a typed event that can be safely reconstructed.
pub async fn replay_pending_child_runtime_tombstones(
    journal: &NdjsonEventJournal,
    store: &RetentionDeleteTombstoneStore,
) -> std::io::Result<ChildRuntimeTombstoneRecoveryReport> {
    let records = tokio::task::spawn_blocking({
        let store = store.clone();
        move || store.records()
    })
    .await
    .map_err(std::io::Error::other)??;
    let mut report = ChildRuntimeTombstoneRecoveryReport {
        journaled: Vec::new(),
        pending_journal_retry: Vec::new(),
    };
    for record in records {
        if !record.terminal_pending {
            continue;
        }
        let Some((_, envelope)) = record.typed_action_and_envelope() else {
            continue;
        };
        match journal.append_idempotent(envelope).await {
            Ok(append) => report.journaled.push(append),
            Err(error) if is_retryable_journal_error(&error) => report
                .pending_journal_retry
                .push(envelope.correlation_id.clone()),
            Err(error) => return Err(std::io::Error::other(error.to_string())),
        }
    }
    Ok(report)
}

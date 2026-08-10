use std::path::PathBuf;

use ocentra_eventing::{
    error::EventingError,
    ids::EventType,
    journal::{ndjson::NdjsonEventJournal, ndjson::NdjsonJournalOptions},
    replay::ReplayFilter,
};
use ocentra_parent_agent_protocol::{
    constants::enforcement,
    enforcement::{
        EnforcementAuditEventKind, EnforcementAuditJournalEvent, EnforcementResultStatus,
    },
};

/// Filesystem boundary for the projection-only enforcement journal reader.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementAuditHistoryPath(pub PathBuf);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnforcementAuditHistoryKind {
    RejectedIntent,
    AcceptedIntent,
    AdapterResult,
    TimerExpired,
    TimerRollback,
    TimerCancelled,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementAuditHistoryRow {
    pub sequence: u64,
    pub kind: EnforcementAuditHistoryKind,
    pub event: EnforcementAuditJournalEvent,
}

/// Reads the enforcement sidecar strictly as a projection. The returned rows
/// retain the typed, already-redacted journal contract and cannot dispatch an
/// adapter or replay an enforcement action.
pub async fn read_enforcement_audit_history(
    journal_path: EnforcementAuditHistoryPath,
) -> Result<Vec<EnforcementAuditHistoryRow>, EventingError> {
    let event_type = EventType::parse(enforcement::EVENT_AUDIT_JOURNAL_RECORDED)?;
    let journal =
        NdjsonEventJournal::with_options(journal_path.0, NdjsonJournalOptions::hash_chain());
    let replay = journal
        .replay_projection(ReplayFilter::for_event_type(event_type))
        .await?;

    replay
        .records
        .into_iter()
        .map(|record| {
            let decoded = record.envelope.decode::<EnforcementAuditJournalEvent>()?;
            Ok(EnforcementAuditHistoryRow {
                sequence: record.sequence,
                kind: history_kind(&decoded.payload),
                event: decoded.payload,
            })
        })
        .collect()
}

fn history_kind(event: &EnforcementAuditJournalEvent) -> EnforcementAuditHistoryKind {
    match event.audit_event_kind {
        EnforcementAuditEventKind::Expired => EnforcementAuditHistoryKind::TimerExpired,
        EnforcementAuditEventKind::RollbackRequested
        | EnforcementAuditEventKind::RollbackCompleted => {
            EnforcementAuditHistoryKind::TimerRollback
        }
        EnforcementAuditEventKind::Cancelled => EnforcementAuditHistoryKind::TimerCancelled,
        EnforcementAuditEventKind::Attempted
            if event.result_status == EnforcementResultStatus::WouldEnforce =>
        {
            EnforcementAuditHistoryKind::AcceptedIntent
        }
        EnforcementAuditEventKind::Failed
            if event
                .audit_event_id
                .starts_with(enforcement::JOURNAL_REJECTED_ID_PREFIX) =>
        {
            EnforcementAuditHistoryKind::RejectedIntent
        }
        _ => EnforcementAuditHistoryKind::AdapterResult,
    }
}

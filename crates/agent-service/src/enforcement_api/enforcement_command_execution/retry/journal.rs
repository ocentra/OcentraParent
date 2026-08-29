use std::io::ErrorKind;

use ocentra_eventing::{
    ids::EventType,
    journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions},
    replay::ReplayFilter,
};
use ocentra_parent_agent_protocol::{constants, enforcement::EnforcementAuditJournalEvent};

use crate::enforcement_payload::{EnforcementCommandPayload, EnforcementText};

use super::{recovery_store_error, EnforcementRetryRecoveryError};
use crate::enforcement_api::enforcement_command_execution::EnforcementJournalPaths;

#[derive(Clone)]
pub(super) struct AuditJournalRow {
    pub(super) sequence: u64,
    pub(super) envelope_event_id: String,
    pub(super) correlation_id: String,
    pub(super) event: EnforcementAuditJournalEvent,
}

pub(super) enum RetryJournalState {
    Empty,
    Incomplete,
    Complete {
        before: AuditJournalRow,
        completed: AuditJournalRow,
    },
}

pub(super) async fn read_retry_journal_state(
    paths: &EnforcementJournalPaths,
    request: &EnforcementCommandPayload,
) -> Result<RetryJournalState, EnforcementRetryRecoveryError> {
    let rows = read_audit_journal(paths).await?;
    let before_id = before_event_id(&EnforcementText(request.input.audit_event_id.clone()));
    let completed_id = EnforcementText(request.input.audit_event_id.clone());
    let before = matching_rows(&rows, &before_id);
    let completed = matching_rows(&rows, &completed_id);
    if before.len() > 1 || completed.len() > 1 {
        return Err(EnforcementRetryRecoveryError::ReconciliationRequired);
    }
    Ok(match (before.first(), completed.first()) {
        (None, None) => RetryJournalState::Empty,
        (Some(before), Some(completed)) if before.sequence < completed.sequence => {
            RetryJournalState::Complete {
                before: (*before).clone(),
                completed: (*completed).clone(),
            }
        }
        (Some(_), Some(_)) => return Err(EnforcementRetryRecoveryError::ReconciliationRequired),
        _ => RetryJournalState::Incomplete,
    })
}

fn matching_rows<'a>(
    rows: &'a [AuditJournalRow],
    event_id: &EnforcementText,
) -> Vec<&'a AuditJournalRow> {
    rows.iter()
        .filter(|row| row.envelope_event_id == event_id.0 || row.event.audit_event_id == event_id.0)
        .collect()
}

async fn read_audit_journal(
    paths: &EnforcementJournalPaths,
) -> Result<Vec<AuditJournalRow>, EnforcementRetryRecoveryError> {
    let mut path = paths.journal_path.clone();
    path.set_extension(constants::enforcement::EVENTING_JOURNAL_EXTENSION);
    match tokio::fs::metadata(&path).await {
        Ok(metadata) if metadata.is_file() => {}
        Ok(_) => return Err(EnforcementRetryRecoveryError::Store),
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(_) => return Err(EnforcementRetryRecoveryError::Store),
    }
    let event_type = EventType::parse(constants::enforcement::EVENT_AUDIT_JOURNAL_RECORDED)
        .map_err(recovery_store_error)?;
    let replay = NdjsonEventJournal::with_options(path, NdjsonJournalOptions::hash_chain())
        .replay_projection(ReplayFilter::for_event_type(event_type))
        .await
        .map_err(recovery_store_error)?;
    if replay.skipped_count > 0 {
        return Err(EnforcementRetryRecoveryError::ReconciliationRequired);
    }
    replay
        .records
        .into_iter()
        .map(|record| {
            let sequence = record.sequence;
            let envelope_event_id = record.envelope.event_id.as_str().to_string();
            let correlation_id = record.envelope.correlation_id.as_str().to_string();
            let event = record
                .envelope
                .decode::<EnforcementAuditJournalEvent>()
                .map_err(recovery_store_error)?
                .into_payload();
            Ok(AuditJournalRow {
                sequence,
                envelope_event_id,
                correlation_id,
                event,
            })
        })
        .collect()
}

pub(super) fn before_event_id(audit_event_id: &EnforcementText) -> EnforcementText {
    let mut value = constants::enforcement::JOURNAL_BEFORE_ACTION_ID_PREFIX.to_string();
    value.push_str(&audit_event_id.0);
    EnforcementText(value)
}

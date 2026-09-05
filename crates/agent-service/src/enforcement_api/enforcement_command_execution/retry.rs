#[path = "retry/identity.rs"]
mod identity;
#[path = "retry/journal.rs"]
mod journal;
#[path = "retry/report.rs"]
mod report;
#[path = "retry/store.rs"]
mod store;
#[path = "retry/stored_outcome.rs"]
mod stored_outcome;
#[path = "retry/timer_state.rs"]
mod timer_state;

use ocentra_parent_agent_protocol::logging::LogFields;

use crate::enforcement_payload::{EnforcementCommandPayload, EnforcementText};

use self::journal::RetryJournalState;
use super::{provenance::EnforcementAuditProvenance, EnforcementJournalPaths};

pub(super) struct RecoveredEnforcementCommand {
    pub(super) payload: LogFields,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum EnforcementRetryRecoveryError {
    IdentityMismatch,
    ReconciliationRequired,
    Store,
}

pub(super) async fn recover_completed_enforcement_command(
    command_correlation_id: &EnforcementText,
    command_sent_at: &EnforcementText,
    request: &EnforcementCommandPayload,
    paths: &EnforcementJournalPaths,
    provenance: Option<EnforcementAuditProvenance>,
) -> Result<Option<RecoveredEnforcementCommand>, EnforcementRetryRecoveryError> {
    let journal_state = journal::read_retry_journal_state(paths, request).await?;
    let audit_event_id = EnforcementText(request.input.audit_event_id.clone());
    let stored_audit = store::read_stored_audit(paths, &audit_event_id).await?;
    match (journal_state, stored_audit) {
        (RetryJournalState::Empty, None) => Ok(None),
        (RetryJournalState::Complete(completed_journal), Some(stored)) => {
            let journal::CompletedRetryJournal { before, completed } = *completed_journal;
            recover_completed_pair(RecoveredPairInput {
                command_correlation_id,
                command_sent_at,
                request,
                paths,
                before,
                completed,
                stored,
                provenance,
            })
            .await
            .map(Some)
        }
        _ => Err(EnforcementRetryRecoveryError::ReconciliationRequired),
    }
}

struct RecoveredPairInput<'a> {
    command_correlation_id: &'a EnforcementText,
    command_sent_at: &'a EnforcementText,
    request: &'a EnforcementCommandPayload,
    paths: &'a EnforcementJournalPaths,
    before: journal::AuditJournalRow,
    completed: journal::AuditJournalRow,
    stored: store::StoredAudit,
    provenance: Option<EnforcementAuditProvenance>,
}

async fn recover_completed_pair(
    input: RecoveredPairInput<'_>,
) -> Result<RecoveredEnforcementCommand, EnforcementRetryRecoveryError> {
    let RecoveredPairInput {
        command_correlation_id,
        command_sent_at,
        request,
        paths,
        before,
        completed,
        stored,
        provenance,
    } = input;
    if !identity::pair_matches_command(
        &before,
        &completed,
        command_correlation_id,
        command_sent_at,
        request,
    ) {
        return Err(EnforcementRetryRecoveryError::IdentityMismatch);
    }
    let outcome = stored_outcome::outcome_from_fields(&stored.fields)?;
    if !identity::outcome_matches_command(&outcome, request)
        || !identity::journal_matches_outcome(&completed, &outcome)
        || !super::provenance::audit_provenance_matches(&stored.fields, provenance)
    {
        return Err(EnforcementRetryRecoveryError::IdentityMismatch);
    }
    let active_state = timer_state::recovered_active_state(paths, &outcome).await?;
    report::validate_complete_report_payload(&stored.fields, &outcome, active_state.as_ref())?;
    Ok(RecoveredEnforcementCommand {
        payload: stored.fields,
    })
}

pub(super) fn recovery_store_error(_: impl std::fmt::Debug) -> EnforcementRetryRecoveryError {
    EnforcementRetryRecoveryError::Store
}

pub(super) fn recovery_reconciliation_error(_: serde_json::Error) -> EnforcementRetryRecoveryError {
    EnforcementRetryRecoveryError::ReconciliationRequired
}

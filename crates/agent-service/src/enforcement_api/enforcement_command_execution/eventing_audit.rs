use ocentra_eventing::{ids::CorrelationId, journal::policy::JournalDispatchPhase};
use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
use ocentra_parent_agent_protocol::{
    constants,
    enforcement::{EnforcementAuditJournalEvent, EnforcementAuditJournalProvenance},
};

use crate::enforcement_payload::{EnforcementCommandPayload, EnforcementText};

use super::super::enforcement_pre_action_journal::eventing_journal::{
    append_enforcement_audit_journal_event_phase, EnforcementEventingJournalPath,
};
use super::{EnforcementJournalBuildError, EnforcementJournalPaths};

pub(super) async fn record_eventing_enforcement_audit(
    command_correlation_id: &EnforcementText,
    command_sent_at: &EnforcementText,
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
    phase: JournalDispatchPhase,
) -> Result<ocentra_eventing::journal::JournalAppend, EnforcementJournalBuildError> {
    let mut eventing_journal_path = paths.journal_path.clone();
    eventing_journal_path.set_extension(constants::enforcement::EVENTING_JOURNAL_EXTENSION);
    let mut event = eventing_audit_event(request, outcome, command_sent_at);
    event.provenance = eventing_audit_provenance(phase);
    append_enforcement_audit_journal_event_phase(
        EnforcementEventingJournalPath {
            path: eventing_journal_path,
        },
        event,
        CorrelationId::parse(command_correlation_id.0.clone()).map_err(eventing_journal_error)?,
        phase,
    )
    .await
    .map_err(eventing_journal_error)
}

fn eventing_audit_provenance(phase: JournalDispatchPhase) -> EnforcementAuditJournalProvenance {
    match phase {
        JournalDispatchPhase::BeforeDispatch => EnforcementAuditJournalProvenance::AcceptedIntent,
        JournalDispatchPhase::AfterDispatch => EnforcementAuditJournalProvenance::AdapterResult,
    }
}

fn eventing_journal_error(_: impl std::fmt::Debug) -> EnforcementJournalBuildError {
    EnforcementJournalBuildError::Store
}

fn eventing_audit_event(
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    command_sent_at: &EnforcementText,
) -> EnforcementAuditJournalEvent {
    let mut event = EnforcementAuditJournalEvent::from(&outcome.audit_event);
    event.device_id = Some(request.device_id.0.clone());
    event.source_peer_id = Some(request.source_peer_id.0.clone());
    event.target_route = Some(request.target_route.0.clone());
    event.observed_at = command_sent_at.0.clone();
    event
}

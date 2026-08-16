use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAuditJournalEvent, EnforcementAuditJournalProvenance,
};

use crate::enforcement_payload::{EnforcementCommandPayload, EnforcementText};

use super::journal::{before_event_id, AuditJournalRow};

pub(super) fn pair_matches_command(
    before: &AuditJournalRow,
    completed: &AuditJournalRow,
    command_correlation_id: &EnforcementText,
    command_sent_at: &EnforcementText,
    request: &EnforcementCommandPayload,
) -> bool {
    before_matches_command(
        before,
        command_correlation_id,
        command_sent_at,
        request,
        &before_event_id(&EnforcementText(request.input.audit_event_id.clone())),
    ) && journal_matches_command(completed, command_correlation_id, command_sent_at, request)
}

fn before_matches_command(
    row: &AuditJournalRow,
    command_correlation_id: &EnforcementText,
    command_sent_at: &EnforcementText,
    request: &EnforcementCommandPayload,
    expected_before_event_id: &EnforcementText,
) -> bool {
    let input = &request.input;
    let event = &row.event;
    let before_result_id = before_event_id(&EnforcementText(input.result_id.clone()));
    row.envelope_event_id == expected_before_event_id.0
        && row.correlation_id == command_correlation_id.0
        && event.audit_event_id == expected_before_event_id.0
        && event.provenance == EnforcementAuditJournalProvenance::AcceptedIntent
        && event.action_id == input.action_id
        && event.intent_id == input.intent.intent_id
        && event.result_id == before_result_id.0
        && event.policy_decision_id == input.decision.decision_id
        && event.policy_decision_id == input.intent.policy_decision_id
        && event.policy_version == input.policy_version
        && event.policy_action == input.decision.action
        && event.target_id == input.intent.target.target_id
        && event.target_type == input.intent.target.target_type
        && event.adapter_kind == input.capability.adapter_kind
        && event.platform == input.capability.platform
        && event.capability_state == input.capability.capability_state
        && event.evidence_references == input.intent.evidence_references
        && event.actor == input.intent.actor
        && event.parent_override == input.intent.parent_approval
        && event.dry_run == input.decision.dry_run
        && event.reason_codes == input.decision.reason_codes
        && event.requested_at == input.requested_at
        && event.started_at.as_deref() == Some(input.requested_at.as_str())
        && event.completed_at.is_none()
        && event.device_id.as_deref() == Some(request.device_id.0.as_str())
        && event.source_peer_id.as_deref() == Some(request.source_peer_id.0.as_str())
        && event.target_route.as_deref() == Some(request.target_route.0.as_str())
        && event.observed_at == command_sent_at.0
}

fn journal_matches_command(
    row: &AuditJournalRow,
    command_correlation_id: &EnforcementText,
    command_sent_at: &EnforcementText,
    request: &EnforcementCommandPayload,
) -> bool {
    let input = &request.input;
    let event = &row.event;
    row.envelope_event_id == input.audit_event_id
        && row.correlation_id == command_correlation_id.0
        && event.audit_event_id == input.audit_event_id
        && event.provenance == EnforcementAuditJournalProvenance::AdapterResult
        && event.action_id == input.action_id
        && event.intent_id == input.intent.intent_id
        && event.result_id == input.result_id
        && event.policy_decision_id == input.decision.decision_id
        && event.policy_decision_id == input.intent.policy_decision_id
        && event.policy_version == input.policy_version
        && event.policy_action == input.decision.action
        && event.target_id == input.intent.target.target_id
        && event.target_type == input.intent.target.target_type
        && event.adapter_kind == input.capability.adapter_kind
        && event.platform == input.capability.platform
        && event.capability_state == input.capability.capability_state
        && event.evidence_references == input.intent.evidence_references
        && event.actor == input.intent.actor
        && event.parent_override == input.intent.parent_approval
        && event.dry_run == input.decision.dry_run
        && event.reason_codes == input.decision.reason_codes
        && event.requested_at == input.requested_at
        && event.device_id.as_deref() == Some(request.device_id.0.as_str())
        && event.source_peer_id.as_deref() == Some(request.source_peer_id.0.as_str())
        && event.target_route.as_deref() == Some(request.target_route.0.as_str())
        && event.observed_at == command_sent_at.0
}

pub(super) fn outcome_matches_command(
    outcome: &EnforcementBoundaryOutcome,
    request: &EnforcementCommandPayload,
) -> bool {
    let input = &request.input;
    let action = &outcome.action;
    let result = &outcome.result;
    let audit = &outcome.audit_event;
    action.action_id == input.action_id
        && action.intent_id == input.intent.intent_id
        && action.policy_decision_id == input.decision.decision_id
        && action.policy_action == input.decision.action
        && action.adapter_kind == input.capability.adapter_kind
        && action.platform == input.capability.platform
        && action.target == input.intent.target
        && action.capability == input.capability
        && action.reason_codes == input.decision.reason_codes
        && action.evidence_references == input.intent.evidence_references
        && action.parent_approval == input.intent.parent_approval
        && action.dry_run == input.decision.dry_run
        && action.requested_at == input.requested_at
        && action.expires_at == input.decision.expires_at
        && action.rollback_token == input.rollback_token
        && result.result_id == input.result_id
        && result.action_id == input.action_id
        && result.started_at == input.requested_at
        && result.rollback_token == input.rollback_token
        && result.capability == input.capability
        && audit.audit_event_id == input.audit_event_id
        && audit.action == *action
        && audit.result == *result
        && result.completed_at.as_deref() == Some(audit.observed_at.as_str())
        && audit.capability == input.capability
        && audit.policy_version == input.policy_version
        && audit.evidence_references == input.intent.evidence_references
        && audit.actor == input.intent.actor
        && audit.parent_override == input.intent.parent_approval
        && outcome.timer_event.as_ref().is_none_or(|timer| {
            timer.timer_event_id == input.timer_event_id
                && timer.action_id == input.action_id
                && timer.policy_decision_id == input.decision.decision_id
                && timer.evidence_references == input.intent.evidence_references
        })
}

pub(super) fn journal_matches_outcome(
    row: &AuditJournalRow,
    outcome: &EnforcementBoundaryOutcome,
) -> bool {
    let event: &EnforcementAuditJournalEvent = &row.event;
    let audit = &outcome.audit_event;
    let sequence = row.sequence.to_string();
    event.audit_event_id == audit.audit_event_id
        && event.action_id == outcome.action.action_id
        && event.intent_id == outcome.action.intent_id
        && event.result_id == outcome.result.result_id
        && event.policy_decision_id == outcome.action.policy_decision_id
        && event.policy_version == audit.policy_version
        && event.policy_action == outcome.action.policy_action
        && event.target_id == outcome.action.target.target_id
        && event.target_type == outcome.action.target.target_type
        && event.adapter_kind == outcome.action.adapter_kind
        && event.platform == outcome.action.platform
        && event.audit_event_kind == audit.audit_event_kind
        && event.result_status == outcome.result.status
        && event.adapter_result_code == outcome.result.adapter_result_code
        && event.capability_state == outcome.result.capability.capability_state
        && event.evidence_references == audit.evidence_references
        && event.actor == audit.actor
        && event.parent_override == audit.parent_override
        && event.unavailable_status == audit.unavailable_status
        && event.rollback_state == outcome.result.rollback_state
        && event.dry_run == outcome.action.dry_run
        && event.reason_codes == outcome.action.reason_codes
        && event.reason
            == outcome
                .result
                .failed_reason
                .clone()
                .or_else(|| outcome.result.unavailable_reason.clone())
        && event.requested_at == outcome.action.requested_at
        && event.started_at.as_deref() == Some(outcome.result.started_at.as_str())
        && event.completed_at == outcome.result.completed_at
        && audit.journal_sequence.as_deref() == Some(sequence.as_str())
        && event
            .journal_sequence
            .as_ref()
            .is_none_or(|event_sequence| event_sequence == &sequence)
}

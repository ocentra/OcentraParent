use std::fmt::Display;

use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementAction;
use ocentra_parent_agent_protocol::enforcement::EnforcementAdapterResultCode;
use ocentra_parent_agent_protocol::enforcement::EnforcementAuditEvent;
use ocentra_parent_agent_protocol::enforcement::EnforcementAuditEventKind;
use ocentra_parent_agent_protocol::enforcement::EnforcementResult;
use ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus;
use ocentra_parent_agent_protocol::enforcement::EnforcementRollbackState;

use crate::enforcement_payload::{EnforcementCommandPayload, EnforcementText};

#[path = "enforcement_pre_action_journal/eventing_journal.rs"]
pub(crate) mod eventing_journal;

pub(crate) fn journal_before_action_outcome(
    request: &EnforcementCommandPayload,
    action: &EnforcementAction,
    observed_at: impl Display,
) -> EnforcementBoundaryOutcome {
    let observed_at = EnforcementText(observed_at.to_string());
    let result = before_action_result(request, action);
    let audit_event = before_action_audit_event(request, action, &result, &observed_at);

    EnforcementBoundaryOutcome {
        action: action.clone(),
        result,
        audit_event,
        timer_event: None,
        adapter_request: None,
    }
}

fn before_action_result(
    request: &EnforcementCommandPayload,
    action: &EnforcementAction,
) -> EnforcementResult {
    EnforcementResult {
        schema_version: request.input.decision.schema_version.clone(),
        result_id: prefixed(
            constants::enforcement::JOURNAL_BEFORE_ACTION_ID_PREFIX,
            &request.input.result_id,
        )
        .0,
        action_id: action.action_id.clone(),
        status: EnforcementResultStatus::WouldEnforce,
        adapter_result_code: EnforcementAdapterResultCode::NoOp,
        started_at: request.input.requested_at.clone(),
        completed_at: None,
        rollback_token: action.rollback_token.clone(),
        rollback_state: EnforcementRollbackState::NotRequired,
        unavailable_reason: None,
        unavailable_status: None,
        failed_reason: None,
        next_check_at: action.expires_at.clone(),
        capability: action.capability.clone(),
    }
}

fn before_action_audit_event(
    request: &EnforcementCommandPayload,
    action: &EnforcementAction,
    result: &EnforcementResult,
    observed_at: &EnforcementText,
) -> EnforcementAuditEvent {
    EnforcementAuditEvent {
        schema_version: request.input.decision.schema_version.clone(),
        audit_event_id: prefixed(
            constants::enforcement::JOURNAL_BEFORE_ACTION_ID_PREFIX,
            &request.input.audit_event_id,
        )
        .0,
        audit_event_kind: EnforcementAuditEventKind::Attempted,
        action: action.clone(),
        result: result.clone(),
        capability: result.capability.clone(),
        unavailable_status: None,
        policy_version: request.input.policy_version.clone(),
        evidence_references: action.evidence_references.clone(),
        actor: request.input.intent.actor.clone(),
        parent_override: action.parent_approval.clone(),
        journal_sequence: Some(
            prefixed(
                constants::enforcement::JOURNAL_BEFORE_ACTION_ID_PREFIX,
                &request.input.audit_event_id,
            )
            .0,
        ),
        observed_at: observed_at.0.clone(),
    }
}

fn prefixed(prefix: impl Display, value: impl Display) -> EnforcementText {
    let mut output = prefix.to_string();
    output.push_str(&value.to_string());
    EnforcementText(output)
}

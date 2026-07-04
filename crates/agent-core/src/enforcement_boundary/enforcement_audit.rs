use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementAuditEvent, EnforcementAuditEventKind, EnforcementResult,
    EnforcementResultStatus,
};

use super::EnforcementBoundaryInput;

pub(super) fn enforcement_audit_event(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
    result: &EnforcementResult,
) -> EnforcementAuditEvent {
    EnforcementAuditEvent {
        schema_version: input.decision.schema_version.clone(),
        audit_event_id: input.audit_event_id.clone(),
        audit_event_kind: audit_kind(result.status),
        action: action.clone(),
        result: result.clone(),
        capability: result.capability.clone(),
        unavailable_status: result.unavailable_status.clone(),
        policy_version: input.policy_version.clone(),
        evidence_references: input.intent.evidence_references.clone(),
        actor: input.intent.actor.clone(),
        parent_override: input.intent.parent_approval.clone(),
        journal_sequence: None,
        observed_at: input
            .completed_at
            .clone()
            .unwrap_or_else(|| input.requested_at.clone()),
    }
}

fn audit_kind(status: EnforcementResultStatus) -> EnforcementAuditEventKind {
    match status {
        EnforcementResultStatus::WouldEnforce => EnforcementAuditEventKind::Attempted,
        EnforcementResultStatus::ActuallyEnforced => EnforcementAuditEventKind::Succeeded,
        EnforcementResultStatus::Unavailable => EnforcementAuditEventKind::Unavailable,
        EnforcementResultStatus::Failed => EnforcementAuditEventKind::Failed,
        EnforcementResultStatus::Expired => EnforcementAuditEventKind::Expired,
        EnforcementResultStatus::RolledBack => EnforcementAuditEventKind::RollbackCompleted,
        EnforcementResultStatus::Superseded => EnforcementAuditEventKind::Cancelled,
        EnforcementResultStatus::NoOp => EnforcementAuditEventKind::Attempted,
    }
}

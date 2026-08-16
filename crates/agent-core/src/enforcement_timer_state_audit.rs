use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementAuditEvent, EnforcementAuditEventKind, EnforcementResult,
    EnforcementResultStatus, ParentActionReference,
};

use super::EnforcementTimerTransitionIds;

pub(super) fn transition_audit_event(
    action: &EnforcementAction,
    result: &EnforcementResult,
    ids: &EnforcementTimerTransitionIds,
    previous_policy_version: &str,
    parent_override: Option<ParentActionReference>,
) -> EnforcementAuditEvent {
    EnforcementAuditEvent {
        schema_version: action.schema_version.clone(),
        audit_event_id: ids.audit_event_id.clone(),
        audit_event_kind: audit_kind(result.status),
        action: action.clone(),
        result: result.clone(),
        capability: result.capability.clone(),
        unavailable_status: result.unavailable_status.clone(),
        policy_version: parent_override
            .as_ref()
            .map(|reference| reference.policy_version.clone())
            .unwrap_or_else(|| previous_policy_version.to_string()),
        evidence_references: action.evidence_references.clone(),
        actor: parent_override
            .as_ref()
            .map(|reference| reference.actor.clone()),
        parent_override,
        journal_sequence: None,
        observed_at: ids.observed_at.clone(),
    }
}

fn audit_kind(status: EnforcementResultStatus) -> EnforcementAuditEventKind {
    match status {
        EnforcementResultStatus::Expired => EnforcementAuditEventKind::Expired,
        EnforcementResultStatus::Failed => EnforcementAuditEventKind::Failed,
        EnforcementResultStatus::RolledBack => EnforcementAuditEventKind::RollbackCompleted,
        EnforcementResultStatus::Superseded => EnforcementAuditEventKind::Cancelled,
        EnforcementResultStatus::Unavailable => EnforcementAuditEventKind::Unavailable,
        _ => EnforcementAuditEventKind::Attempted,
    }
}

use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementTimerEvent, EnforcementTimerEventKind,
    EnforcementUnavailableReason,
};

use super::enforcement_timer_state_helpers::timer_effective_at;
use super::EnforcementTimerTransitionIds;

pub(super) fn transition_timer_event(
    action: &EnforcementAction,
    previous_timer: &EnforcementTimerEvent,
    ids: &EnforcementTimerTransitionIds,
    timer_event_kind: EnforcementTimerEventKind,
    unavailable_reason: Option<EnforcementUnavailableReason>,
) -> EnforcementTimerEvent {
    EnforcementTimerEvent {
        schema_version: action.schema_version.clone(),
        timer_event_id: ids.timer_event_id.clone(),
        timer_event_kind,
        action_id: action.action_id.clone(),
        policy_decision_id: action.policy_decision_id.clone(),
        evidence_references: action.evidence_references.clone(),
        scheduled_at: previous_timer.scheduled_at.clone(),
        effective_at: timer_effective_at(action, timer_event_kind),
        rollback_token: action.rollback_token.clone(),
        recovered_after_restart: timer_event_kind == EnforcementTimerEventKind::RestartRecovered,
        unavailable_reason,
    }
}

pub(super) fn timer_event_kind_for_expiry(
    status: ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus,
) -> EnforcementTimerEventKind {
    match status {
        ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus::Expired => {
            EnforcementTimerEventKind::Expired
        }
        ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus::Failed => {
            EnforcementTimerEventKind::RecoveryNeeded
        }
        ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus::RolledBack => {
            EnforcementTimerEventKind::RollbackCompleted
        }
        ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus::Superseded => {
            EnforcementTimerEventKind::Cancelled
        }
        ocentra_parent_agent_protocol::enforcement::EnforcementResultStatus::Unavailable => {
            EnforcementTimerEventKind::Unavailable
        }
        _ => EnforcementTimerEventKind::RecoveryNeeded,
    }
}

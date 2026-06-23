use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementIntent, EnforcementIntentSource, EnforcementResult,
    EnforcementResultStatus, EnforcementRollbackState, EnforcementTimerEvent,
    EnforcementTimerEventKind, EnforcementUnavailableReason,
};

use super::EnforcementBoundaryInput;

pub(super) fn timer_event(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
    result: &EnforcementResult,
) -> Option<EnforcementTimerEvent> {
    action.expires_at.as_ref().map(|expires_at| {
        let timer_event_kind = input
            .timer_event_kind
            .unwrap_or_else(|| timer_event_kind(&input.intent, result));

        EnforcementTimerEvent {
            schema_version: input.decision.schema_version.clone(),
            timer_event_id: input.timer_event_id.clone(),
            timer_event_kind,
            action_id: action.action_id.clone(),
            policy_decision_id: action.policy_decision_id.clone(),
            evidence_references: action.evidence_references.clone(),
            scheduled_at: input.requested_at.clone(),
            effective_at: timer_effective_at(expires_at, timer_event_kind),
            rollback_token: action.rollback_token.clone(),
            recovered_after_restart: timer_event_kind
                == EnforcementTimerEventKind::RestartRecovered,
            unavailable_reason: timer_unavailable_reason(result, timer_event_kind),
        }
    })
}

fn timer_event_kind(
    intent: &EnforcementIntent,
    result: &EnforcementResult,
) -> EnforcementTimerEventKind {
    match result.rollback_state {
        EnforcementRollbackState::Requested => EnforcementTimerEventKind::RollbackRequested,
        EnforcementRollbackState::Completed => EnforcementTimerEventKind::RollbackCompleted,
        _ => match result.status {
            EnforcementResultStatus::Unavailable => EnforcementTimerEventKind::Unavailable,
            EnforcementResultStatus::Failed => EnforcementTimerEventKind::RecoveryNeeded,
            EnforcementResultStatus::Expired => EnforcementTimerEventKind::Expired,
            EnforcementResultStatus::RolledBack => EnforcementTimerEventKind::RollbackCompleted,
            EnforcementResultStatus::Superseded => EnforcementTimerEventKind::Cancelled,
            EnforcementResultStatus::NoOp
                if matches!(
                    intent.requested_action,
                    PolicyAction::AskParent | PolicyAction::TimeLimit
                ) =>
            {
                EnforcementTimerEventKind::Created
            }
            EnforcementResultStatus::NoOp => EnforcementTimerEventKind::Cancelled,
            EnforcementResultStatus::WouldEnforce | EnforcementResultStatus::ActuallyEnforced => {
                if intent.source == EnforcementIntentSource::SystemRecovery {
                    EnforcementTimerEventKind::RestartRecovered
                } else {
                    EnforcementTimerEventKind::Created
                }
            }
        },
    }
}

fn timer_effective_at(
    expires_at: &str,
    timer_event_kind: EnforcementTimerEventKind,
) -> Option<String> {
    match timer_event_kind {
        EnforcementTimerEventKind::Created
        | EnforcementTimerEventKind::Extended
        | EnforcementTimerEventKind::Expired
        | EnforcementTimerEventKind::RestartRecovered => Some(expires_at.to_string()),
        EnforcementTimerEventKind::Cancelled
        | EnforcementTimerEventKind::RollbackRequested
        | EnforcementTimerEventKind::RollbackCompleted
        | EnforcementTimerEventKind::RecoveryNeeded
        | EnforcementTimerEventKind::Unavailable => None,
    }
}

fn timer_unavailable_reason(
    result: &EnforcementResult,
    timer_event_kind: EnforcementTimerEventKind,
) -> Option<EnforcementUnavailableReason> {
    match timer_event_kind {
        EnforcementTimerEventKind::RecoveryNeeded
            if result.status == EnforcementResultStatus::Failed =>
        {
            Some(EnforcementUnavailableReason::AdapterError)
        }
        EnforcementTimerEventKind::RecoveryNeeded => result
            .unavailable_status
            .as_ref()
            .map(|status| status.unavailable_reason),
        EnforcementTimerEventKind::Unavailable => result
            .unavailable_status
            .as_ref()
            .map(|status| status.unavailable_reason),
        _ => None,
    }
}

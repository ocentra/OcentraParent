use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementResult, EnforcementTimerEvent, EnforcementTimerEventKind,
};

use super::EnforcementBoundaryInput;

#[path = "enforcement_timer_event_effective_at.rs"]
mod enforcement_timer_event_effective_at;
#[path = "enforcement_timer_event_kind.rs"]
mod enforcement_timer_event_kind;
#[path = "enforcement_timer_event_unavailable_reason.rs"]
mod enforcement_timer_event_unavailable_reason;

pub(super) fn timer_event(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
    result: &EnforcementResult,
) -> Option<EnforcementTimerEvent> {
    action.expires_at.as_ref().map(|expires_at| {
        let timer_event_kind = input.timer_event_kind.unwrap_or_else(|| {
            enforcement_timer_event_kind::timer_event_kind(&input.intent, result)
        });

        EnforcementTimerEvent {
            schema_version: input.decision.schema_version.clone(),
            timer_event_id: input.timer_event_id.clone(),
            timer_event_kind,
            action_id: action.action_id.clone(),
            policy_decision_id: action.policy_decision_id.clone(),
            evidence_references: action.evidence_references.clone(),
            scheduled_at: input.requested_at.clone(),
            effective_at: enforcement_timer_event_effective_at::timer_effective_at(
                expires_at,
                timer_event_kind,
            ),
            rollback_token: action.rollback_token.clone(),
            recovered_after_restart: timer_event_kind
                == EnforcementTimerEventKind::RestartRecovered,
            unavailable_reason:
                enforcement_timer_event_unavailable_reason::timer_unavailable_reason(
                    result,
                    timer_event_kind,
                ),
        }
    })
}

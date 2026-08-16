use ocentra_parent_agent_protocol::enforcement::{
    EnforcementIntent, EnforcementResult, EnforcementRollbackState, EnforcementTimerEventKind,
};

#[path = "enforcement_timer_event_kind_recovery.rs"]
mod enforcement_timer_event_kind_recovery;
#[path = "enforcement_timer_event_kind_status.rs"]
mod enforcement_timer_event_kind_status;

pub(super) fn timer_event_kind(
    intent: &EnforcementIntent,
    result: &EnforcementResult,
) -> EnforcementTimerEventKind {
    match result.rollback_state {
        EnforcementRollbackState::Requested => EnforcementTimerEventKind::RollbackRequested,
        EnforcementRollbackState::Completed => EnforcementTimerEventKind::RollbackCompleted,
        _ => {
            enforcement_timer_event_kind_status::timer_event_kind_for_status(intent, result.status)
        }
    }
}

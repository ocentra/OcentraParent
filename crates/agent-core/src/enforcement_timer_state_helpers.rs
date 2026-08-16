use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementResult, EnforcementResultStatus, EnforcementTimerEvent,
    EnforcementTimerEventKind,
};

pub(super) fn active_timer_event(
    timer_event: &EnforcementTimerEvent,
    result: &EnforcementResult,
) -> bool {
    matches!(
        timer_event.timer_event_kind,
        EnforcementTimerEventKind::Created
            | EnforcementTimerEventKind::Extended
            | EnforcementTimerEventKind::RestartRecovered
    ) && matches!(
        result.status,
        EnforcementResultStatus::WouldEnforce
            | EnforcementResultStatus::ActuallyEnforced
            | EnforcementResultStatus::NoOp
    )
}

pub(super) fn active_timer_state_id(action_id: &str) -> String {
    let mut value = String::from(enforcement_constants::TIMER_STATE_ID_PREFIX);
    value.push_str(action_id);
    value
}

pub(super) fn timer_effective_at(
    action: &EnforcementAction,
    timer_event_kind: EnforcementTimerEventKind,
) -> Option<String> {
    match timer_event_kind {
        EnforcementTimerEventKind::Expired | EnforcementTimerEventKind::RestartRecovered => {
            action.expires_at.clone()
        }
        EnforcementTimerEventKind::Cancelled => None,
        _ => None,
    }
}

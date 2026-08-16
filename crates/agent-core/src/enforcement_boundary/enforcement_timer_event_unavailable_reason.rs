use ocentra_parent_agent_protocol::enforcement::{
    EnforcementResult, EnforcementResultStatus, EnforcementTimerEventKind,
    EnforcementUnavailableReason,
};

pub(super) fn timer_unavailable_reason(
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

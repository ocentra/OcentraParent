use ocentra_parent_agent_protocol::enforcement::EnforcementTimerEventKind;

pub(super) fn timer_effective_at(
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

use ocentra_parent_agent_protocol::enforcement::{
    EnforcementIntent, EnforcementIntentSource, EnforcementTimerEventKind,
};

pub(super) fn timer_event_kind_for_recovery(
    intent: &EnforcementIntent,
) -> EnforcementTimerEventKind {
    if intent.source == EnforcementIntentSource::SystemRecovery {
        EnforcementTimerEventKind::RestartRecovered
    } else {
        EnforcementTimerEventKind::Created
    }
}

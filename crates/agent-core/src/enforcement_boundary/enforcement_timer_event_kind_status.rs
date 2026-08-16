use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementIntent, EnforcementResultStatus, EnforcementTimerEventKind,
};

use super::enforcement_timer_event_kind_recovery::timer_event_kind_for_recovery;

pub(super) fn timer_event_kind_for_status(
    intent: &EnforcementIntent,
    status: EnforcementResultStatus,
) -> EnforcementTimerEventKind {
    match status {
        EnforcementResultStatus::Unavailable => EnforcementTimerEventKind::Unavailable,
        EnforcementResultStatus::Failed => EnforcementTimerEventKind::RecoveryNeeded,
        EnforcementResultStatus::Expired => EnforcementTimerEventKind::Expired,
        EnforcementResultStatus::RolledBack => EnforcementTimerEventKind::RollbackCompleted,
        EnforcementResultStatus::Superseded => EnforcementTimerEventKind::Cancelled,
        EnforcementResultStatus::NoOp => timer_event_kind_for_no_op(intent),
        EnforcementResultStatus::WouldEnforce | EnforcementResultStatus::ActuallyEnforced => {
            timer_event_kind_for_recovery(intent)
        }
    }
}

fn timer_event_kind_for_no_op(intent: &EnforcementIntent) -> EnforcementTimerEventKind {
    if matches!(
        intent.requested_action,
        PolicyAction::AskParent | PolicyAction::TimeLimit
    ) {
        EnforcementTimerEventKind::Created
    } else {
        EnforcementTimerEventKind::Cancelled
    }
}

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimePhase;

use crate::screen_event_runtime_input::ScreenRuntimeInput;

pub(crate) fn summary_ref(phase: ScreenRuntimePhase) -> Option<String> {
    match phase {
        ScreenRuntimePhase::SummaryCommitted
        | ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => {
            Some(constants::screen_flow::SCREEN_SUMMARY_EVENT_REF.to_string())
        }
        _ => None,
    }
}

pub(crate) fn policy_decision_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    match phase {
        ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => Some(input.policy_decision_ref.clone()),
        _ => None,
    }
}

pub(crate) fn policy_action(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    match phase {
        ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => Some(input.policy_action.clone()),
        _ => None,
    }
}

pub(crate) fn parent_rule_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    match phase {
        ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => Some(input.parent_rule_ref.clone()),
        _ => None,
    }
}

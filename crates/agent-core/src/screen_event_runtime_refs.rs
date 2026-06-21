use ocentra_parent_agent_protocol::{constants, ScreenRuntimePhase};

use crate::screen_event_runtime_input::ScreenRuntimeInput;

pub(crate) fn screen_aggregate_key(queue_job_id: &str) -> String {
    let mut value = String::from(constants::screen_flow::AGGREGATE_SCREEN_QUEUE_PREFIX);
    value.push_str(queue_job_id);
    value
}

pub(crate) fn screen_correlation_id(queue_job_id: &str) -> String {
    let mut value = String::from(constants::screen_flow::CORRELATION_SCREEN_RUNTIME_PREFIX);
    value.push_str(queue_job_id);
    value
}

pub(crate) fn previous_phase_ref(phase: ScreenRuntimePhase) -> Option<String> {
    match phase {
        ScreenRuntimePhase::CaptureObserved => None,
        ScreenRuntimePhase::QueueEncrypted => {
            Some(constants::screen_flow::SCREEN_CAPTURE_EVENT_REF.to_string())
        }
        ScreenRuntimePhase::AiAnalysisRequested => {
            Some(constants::screen_flow::SCREEN_QUEUE_EVENT_REF.to_string())
        }
        ScreenRuntimePhase::AiAnalysisCompleted => {
            Some(constants::screen_flow::SCREEN_AI_REQUEST_EVENT_REF.to_string())
        }
        ScreenRuntimePhase::SummaryCommitted => {
            Some(constants::screen_flow::SCREEN_AI_RESULT_EVENT_REF.to_string())
        }
        ScreenRuntimePhase::PolicyDecisionCompleted => {
            Some(constants::screen_flow::SCREEN_SUMMARY_EVENT_REF.to_string())
        }
        ScreenRuntimePhase::ActionDryRunRecorded => {
            Some(constants::screen_flow::SCREEN_POLICY_EVENT_REF.to_string())
        }
        ScreenRuntimePhase::DeletionCommitted => {
            Some(constants::screen_flow::SCREEN_ACTION_EVENT_REF.to_string())
        }
        ScreenRuntimePhase::PortalReadModelUpdated => {
            Some(constants::screen_flow::SCREEN_DELETION_EVENT_REF.to_string())
        }
    }
}

pub(crate) fn queue_event_ref(phase: ScreenRuntimePhase) -> Option<String> {
    match phase {
        ScreenRuntimePhase::QueueEncrypted
        | ScreenRuntimePhase::AiAnalysisRequested
        | ScreenRuntimePhase::AiAnalysisCompleted
        | ScreenRuntimePhase::SummaryCommitted
        | ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => {
            Some(constants::screen_flow::SCREEN_QUEUE_EVENT_REF.to_string())
        }
        ScreenRuntimePhase::CaptureObserved => None,
    }
}

pub(crate) fn ai_request_ref(phase: ScreenRuntimePhase) -> Option<String> {
    match phase {
        ScreenRuntimePhase::AiAnalysisRequested
        | ScreenRuntimePhase::AiAnalysisCompleted
        | ScreenRuntimePhase::SummaryCommitted
        | ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => {
            Some(constants::screen_flow::SCREEN_AI_REQUEST_EVENT_REF.to_string())
        }
        _ => None,
    }
}

pub(crate) fn ai_result_ref(phase: ScreenRuntimePhase) -> Option<String> {
    match phase {
        ScreenRuntimePhase::AiAnalysisCompleted
        | ScreenRuntimePhase::SummaryCommitted
        | ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => {
            Some(constants::screen_flow::SCREEN_AI_RESULT_EVENT_REF.to_string())
        }
        _ => None,
    }
}

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

pub(crate) fn action_ref(phase: ScreenRuntimePhase, input: &ScreenRuntimeInput) -> Option<String> {
    match phase {
        ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => Some(input.action_ref.clone()),
        _ => None,
    }
}

pub(crate) fn deletion_proof_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    match phase {
        ScreenRuntimePhase::DeletionCommitted | ScreenRuntimePhase::PortalReadModelUpdated => {
            Some(input.deletion_proof_ref.clone())
        }
        _ => None,
    }
}

pub(crate) fn portal_read_model_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    match phase {
        ScreenRuntimePhase::PortalReadModelUpdated => Some(input.portal_read_model_ref.clone()),
        _ => None,
    }
}

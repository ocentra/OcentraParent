use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimePhase;

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

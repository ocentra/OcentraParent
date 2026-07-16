use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimePhase;

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

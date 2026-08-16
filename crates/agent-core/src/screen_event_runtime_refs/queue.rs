use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimePhase;

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

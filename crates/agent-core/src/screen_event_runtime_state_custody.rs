use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimePhase;

pub(crate) fn custody_state(phase: ScreenRuntimePhase) -> &'static str {
    match phase {
        ScreenRuntimePhase::CaptureObserved
        | ScreenRuntimePhase::QueueEncrypted
        | ScreenRuntimePhase::AiAnalysisRequested => constants::eventing_source::CUSTODY_LOCAL_ONLY,
        ScreenRuntimePhase::AiAnalysisCompleted
        | ScreenRuntimePhase::SummaryCommitted
        | ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted => {
            constants::eventing_source::CUSTODY_LOCAL_JOURNAL
        }
        ScreenRuntimePhase::PortalReadModelUpdated => {
            constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE
        }
    }
}

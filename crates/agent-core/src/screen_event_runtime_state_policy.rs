use ocentra_parent_agent_protocol::screen_evidence::{ScreenPolicyState, ScreenRuntimePhase};

pub(crate) fn policy_state(phase: ScreenRuntimePhase) -> ScreenPolicyState {
    match phase {
        ScreenRuntimePhase::AiAnalysisCompleted | ScreenRuntimePhase::SummaryCommitted => {
            ScreenPolicyState::ReadyForDryRun
        }
        ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => ScreenPolicyState::Completed,
        _ => ScreenPolicyState::NotReady,
    }
}

use ocentra_parent_agent_protocol::screen_evidence::{ScreenAiAuditState, ScreenRuntimePhase};

pub(crate) fn ai_audit_state(phase: ScreenRuntimePhase) -> ScreenAiAuditState {
    match phase {
        ScreenRuntimePhase::AiAnalysisRequested => ScreenAiAuditState::Requested,
        ScreenRuntimePhase::AiAnalysisCompleted
        | ScreenRuntimePhase::SummaryCommitted
        | ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => ScreenAiAuditState::Completed,
        _ => ScreenAiAuditState::NotRequested,
    }
}

use ocentra_parent_agent_protocol::screen_evidence::{ScreenEvidenceScope, ScreenRuntimePhase};

pub(crate) fn evidence_scope(phase: ScreenRuntimePhase) -> ScreenEvidenceScope {
    match phase {
        ScreenRuntimePhase::CaptureObserved
        | ScreenRuntimePhase::QueueEncrypted
        | ScreenRuntimePhase::AiAnalysisRequested => ScreenEvidenceScope::EncryptedLocalImage,
        ScreenRuntimePhase::AiAnalysisCompleted
        | ScreenRuntimePhase::SummaryCommitted
        | ScreenRuntimePhase::PolicyDecisionCompleted
        | ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => {
            ScreenEvidenceScope::DeletedQueryStoreSummary
        }
    }
}

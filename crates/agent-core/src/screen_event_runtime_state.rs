use ocentra_parent_agent_protocol::{constants, ScreenRuntimePhase};

pub(crate) type ScreenEvidenceScope = ocentra_parent_agent_protocol::ScreenEvidenceScope;
pub(crate) type ScreenAiAuditState = ocentra_parent_agent_protocol::ScreenAiAuditState;
pub(crate) type ScreenPolicyState = ocentra_parent_agent_protocol::ScreenPolicyState;
pub(crate) type ScreenActionState = ocentra_parent_agent_protocol::ScreenActionState;
pub(crate) type ScreenDeletionState = ocentra_parent_agent_protocol::ScreenDeletionState;
pub(crate) type ScreenRuntimeClaimBoundary =
    ocentra_parent_agent_protocol::ScreenRuntimeClaimBoundary;

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

pub(crate) fn action_state(phase: ScreenRuntimePhase) -> ScreenActionState {
    match phase {
        ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => ScreenActionState::DryRunRecorded,
        _ => ScreenActionState::NotReady,
    }
}

pub(crate) fn deletion_state(phase: ScreenRuntimePhase) -> ScreenDeletionState {
    match phase {
        ScreenRuntimePhase::DeletionCommitted | ScreenRuntimePhase::PortalReadModelUpdated => {
            ScreenDeletionState::Committed
        }
        _ => ScreenDeletionState::Pending,
    }
}

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

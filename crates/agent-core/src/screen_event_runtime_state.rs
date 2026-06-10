use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::screen_event_runtime_phase::ScreenRuntimePhase;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenEvidenceScope {
    EncryptedLocalImage,
    DeletedQueryStoreSummary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenAiAuditState {
    NotRequested,
    Requested,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenPolicyState {
    NotReady,
    ReadyForDryRun,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenActionState {
    NotReady,
    DryRunRecorded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenDeletionState {
    Pending,
    Committed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenRuntimeClaimBoundary {
    pub raw_image_available_to_ai_provider: bool,
    pub raw_image_available_to_policy: bool,
    pub raw_image_available_to_portal: bool,
    pub adapter_action_executed: bool,
}

impl ScreenRuntimeClaimBoundary {
    pub(crate) fn child_owned_no_raw_escape() -> Self {
        Self {
            raw_image_available_to_ai_provider: false,
            raw_image_available_to_policy: false,
            raw_image_available_to_portal: false,
            adapter_action_executed: false,
        }
    }
}

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

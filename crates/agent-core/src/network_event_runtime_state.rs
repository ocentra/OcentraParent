use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityProcessAttributionStatus,
};
use ocentra_parent_agent_protocol::network_flow::{
    NetworkAiAuditState, NetworkEvidenceScope, NetworkInterventionState, NetworkRiskBudgetState,
    NetworkRuntimeEvidenceGrade, NetworkRuntimePhase,
};

use crate::network_capture::NetworkObservation;

pub(crate) fn evidence_scope(observation: &NetworkObservation) -> NetworkEvidenceScope {
    if observation.status == ActivityCaptureCapabilityStatus::Available {
        NetworkEvidenceScope::MetadataOnly
    } else {
        NetworkEvidenceScope::AdapterUnavailable
    }
}

pub(crate) fn evidence_grade(observation: &NetworkObservation) -> NetworkRuntimeEvidenceGrade {
    if observation.status != ActivityCaptureCapabilityStatus::Available {
        return NetworkRuntimeEvidenceGrade::AdapterUnavailable;
    }
    if observation.domain_attribution_status() == ActivityDomainAttributionStatus::DomainObserved
        && observation.process_attribution_status()
            == ActivityProcessAttributionStatus::ProcessAttributed
    {
        return NetworkRuntimeEvidenceGrade::DomainAndProcessMetadata;
    }
    NetworkRuntimeEvidenceGrade::IpOrProcessPartialMetadata
}

pub(crate) fn ai_audit_state(phase: NetworkRuntimePhase) -> NetworkAiAuditState {
    match phase {
        NetworkRuntimePhase::AiAnalysisRequested => NetworkAiAuditState::Requested,
        NetworkRuntimePhase::AiAnalysisCompleted
        | NetworkRuntimePhase::PolicyEvaluationRequested
        | NetworkRuntimePhase::PolicyDecisionCompleted
        | NetworkRuntimePhase::EnforcementCommandIssued
        | NetworkRuntimePhase::EnforcementResultObserved
        | NetworkRuntimePhase::AuditEntryCommitted
        | NetworkRuntimePhase::PortalReadModelUpdated => NetworkAiAuditState::Completed,
        _ => NetworkAiAuditState::NotRequested,
    }
}

pub(crate) fn risk_budget_state(observation: &NetworkObservation) -> NetworkRiskBudgetState {
    if observation.status != ActivityCaptureCapabilityStatus::Available {
        return NetworkRiskBudgetState::Unavailable;
    }
    if evidence_grade(observation) == NetworkRuntimeEvidenceGrade::DomainAndProcessMetadata {
        return NetworkRiskBudgetState::ObserveOnly;
    }
    NetworkRiskBudgetState::ManualReviewRequired
}

pub(crate) fn intervention_state(observation: &NetworkObservation) -> NetworkInterventionState {
    match risk_budget_state(observation) {
        NetworkRiskBudgetState::ObserveOnly => NetworkInterventionState::DryRunOnly,
        NetworkRiskBudgetState::ManualReviewRequired => NetworkInterventionState::ManualRequired,
        NetworkRiskBudgetState::Unavailable => NetworkInterventionState::Unavailable,
    }
}

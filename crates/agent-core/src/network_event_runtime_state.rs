use ocentra_parent_agent_protocol::{
    ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityProcessAttributionStatus,
};
use serde::{Deserialize, Serialize};

use crate::{network_event_runtime_phase::NetworkRuntimePhase, NetworkObservation};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEvidenceScope {
    MetadataOnly,
    AdapterUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEvidenceGrade {
    DomainAndProcessMetadata,
    IpOrProcessPartialMetadata,
    AdapterUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiAuditState {
    NotRequested,
    Requested,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRiskBudgetState {
    ObserveOnly,
    ManualReviewRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkInterventionState {
    DryRunOnly,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRuntimeClaimBoundary {
    pub decrypted_https_payload_available: bool,
    pub exact_url_available: bool,
    pub page_content_available: bool,
    pub adapter_action_executed: bool,
}

impl NetworkRuntimeClaimBoundary {
    pub(crate) fn metadata_only() -> Self {
        Self {
            decrypted_https_payload_available: false,
            exact_url_available: false,
            page_content_available: false,
            adapter_action_executed: false,
        }
    }
}

pub(crate) fn evidence_scope(observation: &NetworkObservation) -> NetworkEvidenceScope {
    if observation.status == ActivityCaptureCapabilityStatus::Available {
        NetworkEvidenceScope::MetadataOnly
    } else {
        NetworkEvidenceScope::AdapterUnavailable
    }
}

pub(crate) fn evidence_grade(observation: &NetworkObservation) -> NetworkEvidenceGrade {
    if observation.status != ActivityCaptureCapabilityStatus::Available {
        return NetworkEvidenceGrade::AdapterUnavailable;
    }
    if observation.domain_attribution_status() == ActivityDomainAttributionStatus::DomainObserved
        && observation.process_attribution_status()
            == ActivityProcessAttributionStatus::ProcessAttributed
    {
        return NetworkEvidenceGrade::DomainAndProcessMetadata;
    }
    NetworkEvidenceGrade::IpOrProcessPartialMetadata
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
    if evidence_grade(observation) == NetworkEvidenceGrade::DomainAndProcessMetadata {
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

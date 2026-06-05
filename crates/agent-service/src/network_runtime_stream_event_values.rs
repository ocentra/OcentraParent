use serde::Serialize;
use serde_json::Value;

use ocentra_parent_agent_core::{
    NetworkEvidenceGrade as CoreNetworkEvidenceGrade, NetworkInterventionState,
    NetworkRuntimeEventPayload,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    NetworkAiAdvisoryState, NetworkClaimBoundary, NetworkDomainAttributionKind,
    NetworkEnforcementMode, NetworkEnforcementResultStatus, NetworkEvidenceGrade,
    NetworkPolicyDecisionAction, NetworkPortalUpdateKind,
};

pub(crate) fn no_claim_boundary() -> NetworkClaimBoundary {
    NetworkClaimBoundary {
        exact_url_available: false,
        decrypted_https_payload_available: false,
        message_content_available: false,
        search_query_available: false,
        adapter_action_executed: false,
    }
}

pub(crate) fn custody(payload: &NetworkRuntimeEventPayload) -> String {
    if payload.capability_status == ActivityCaptureCapabilityStatus::Available {
        return ocentra_parent_agent_protocol::NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE
            .to_string();
    }
    ocentra_parent_agent_protocol::NETWORK_FLOW_CUSTODY_UNAVAILABLE.to_string()
}

pub(crate) fn evidence_grade(payload: &NetworkRuntimeEventPayload) -> NetworkEvidenceGrade {
    match payload.evidence_grade {
        CoreNetworkEvidenceGrade::DomainAndProcessMetadata => NetworkEvidenceGrade::A,
        CoreNetworkEvidenceGrade::IpOrProcessPartialMetadata => NetworkEvidenceGrade::C,
        CoreNetworkEvidenceGrade::AdapterUnavailable => NetworkEvidenceGrade::D,
    }
}

pub(crate) fn domain_attribution(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkDomainAttributionKind {
    match payload.domain_attribution_status {
        ActivityDomainAttributionStatus::DomainObserved => NetworkDomainAttributionKind::DnsAnswer,
        ActivityDomainAttributionStatus::IpOnly => NetworkDomainAttributionKind::IpOnly,
        ActivityDomainAttributionStatus::Unavailable => NetworkDomainAttributionKind::Unavailable,
    }
}

pub(crate) fn ai_advisory_state(payload: &NetworkRuntimeEventPayload) -> NetworkAiAdvisoryState {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkAiAdvisoryState::Completed,
        NetworkInterventionState::ManualRequired => NetworkAiAdvisoryState::ManualReviewRequired,
        NetworkInterventionState::Unavailable => NetworkAiAdvisoryState::ProviderUnavailable,
    }
}

pub(crate) fn policy_decision_action(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkPolicyDecisionAction {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkPolicyDecisionAction::Observe,
        NetworkInterventionState::ManualRequired => NetworkPolicyDecisionAction::ManualReview,
        NetworkInterventionState::Unavailable => NetworkPolicyDecisionAction::Unknown,
    }
}

pub(crate) fn enforcement_mode(payload: &NetworkRuntimeEventPayload) -> NetworkEnforcementMode {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkEnforcementMode::DryRun,
        NetworkInterventionState::ManualRequired => NetworkEnforcementMode::ManualRequired,
        NetworkInterventionState::Unavailable => NetworkEnforcementMode::Unavailable,
    }
}

pub(crate) fn enforcement_result_status(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkEnforcementResultStatus {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkEnforcementResultStatus::DryRun,
        NetworkInterventionState::ManualRequired => NetworkEnforcementResultStatus::ManualRequired,
        NetworkInterventionState::Unavailable => NetworkEnforcementResultStatus::Unavailable,
    }
}

pub(crate) fn portal_update_kind(payload: &NetworkRuntimeEventPayload) -> NetworkPortalUpdateKind {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkPortalUpdateKind::NetworkReadModel,
        NetworkInterventionState::ManualRequired => NetworkPortalUpdateKind::ManualRequiredState,
        NetworkInterventionState::Unavailable => NetworkPortalUpdateKind::CapabilityState,
    }
}

pub(crate) fn unavailable_reason_code(payload: &NetworkRuntimeEventPayload) -> Option<String> {
    (payload.intervention_state != NetworkInterventionState::DryRunOnly)
        .then(|| constants::network_flow::UNAVAILABLE_REASON_MANUAL_REQUIRED.to_string())
}

pub(crate) fn confidence(payload: &NetworkRuntimeEventPayload) -> f32 {
    match payload.evidence_grade {
        CoreNetworkEvidenceGrade::DomainAndProcessMetadata => 1.0,
        CoreNetworkEvidenceGrade::IpOrProcessPartialMetadata => 0.5,
        CoreNetworkEvidenceGrade::AdapterUnavailable => 0.0,
    }
}

pub(crate) fn previous_event_ref(payload: &NetworkRuntimeEventPayload) -> String {
    payload
        .previous_phase_ref
        .clone()
        .unwrap_or_else(|| payload.evidence_ref.clone())
}

pub(crate) fn ref_or_current(value: &Option<String>, event_ref: &str) -> String {
    value.clone().unwrap_or_else(|| event_ref.to_string())
}

pub(crate) fn evidence_refs(payload: &NetworkRuntimeEventPayload) -> Vec<String> {
    vec![payload.evidence_ref.clone()]
}

pub(crate) fn parent_rule_refs() -> Vec<String> {
    vec![constants::network_flow::TEST_PARENT_RULE_REF.to_string()]
}

pub(crate) fn uncertainty_codes() -> Vec<String> {
    vec![constants::network_flow::UNCERTAINTY_NETWORK_ONLY_NO_EXACT_URL.to_string()]
}

pub(crate) fn json_value<T>(value: T) -> Value
where
    T: Serialize,
{
    serde_json::to_value(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

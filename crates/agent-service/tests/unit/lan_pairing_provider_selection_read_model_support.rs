#[cfg(test)]
use crate::app::{lan_pairing::LanPairingRuntime, time::TIMESTAMP_NOW};
use ocentra_parent_agent_protocol::constants;
#[cfg(test)]
use ocentra_parent_agent_protocol::lan_pairing::{
    LanAiProviderRoutingState, LanPairingDeviceReachability, LanPairingProductionDiscoveryState,
    LanPairingRejectionReason, LanPairingTrustState, LanSelectedRouteTarget,
};
#[cfg(test)]
use ocentra_parent_agent_protocol::lan_pairing_provider_selection::{
    LanProviderSelectionCandidateEvidence, LanProviderSelectionCloudRelayDecisionState,
    LanProviderSelectionCloudRelayImplementationState, LanProviderSelectionLifecycleState,
    LanProviderSelectionManualRequirement, LanProviderSelectionManualRequirementEvidence,
    LanProviderSelectionPolicyDecision, LanProviderSelectionProofState,
    LanProviderSelectionReadModel,
};
use std::primitive::str as TestStr;

#[path = "lan_pairing_provider_selection_read_model_support/candidate_state.rs"]
mod candidate_state;
#[path = "lan_pairing_provider_selection_read_model_support/evidence_label.rs"]
mod evidence_label;
#[path = "lan_pairing_provider_selection_read_model_support/policy_decision.rs"]
mod policy_decision;
#[path = "lan_pairing_provider_selection_read_model_support/rejection_reason.rs"]
mod rejection_reason;
#[path = "lan_pairing_provider_selection_read_model_support/routing_state.rs"]
mod routing_state;

use candidate_state::{
    discovery_state_for_selected, lifecycle_state_for_selected, provider_peer_id,
    reachability_for_selected, route_id_for_selected, trust_state_for_selected,
};
use evidence_label::evidence_label_for_selected;
use policy_decision::{policy_decision_for_selected, selected_provider_route_id};
use rejection_reason::selected_route_rejection_reason;
use routing_state::routing_state;

#[cfg(test)]
pub(crate) fn provider_selection_read_model(
    runtime: &LanPairingRuntime,
) -> LanProviderSelectionReadModel {
    let selected = runtime.selected_target();
    let routing_state = routing_state(runtime);
    let mut candidates = vec![selected_route_candidate(
        runtime,
        selected.as_ref(),
        routing_state.clone(),
    )];
    if runtime.lan_ai_provider_available() {
        candidates.push(unsupported_capability_candidate(runtime, selected.as_ref()));
    }
    candidates.push(physical_household_candidate(runtime));
    candidates.push(cloud_relay_candidate());

    LanProviderSelectionReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        checked_at: TIMESTAMP_NOW(),
        selected_provider_route_id: selected_provider_route_id(selected.as_ref(), &routing_state),
        authorized_provider_selection_state: LanProviderSelectionProofState::CiMechanicalProof,
        physical_household_provider_proof_state: LanProviderSelectionProofState::ManualRequired,
        cloud_relay_implementation_state:
            LanProviderSelectionCloudRelayImplementationState::NotImplemented,
        cloud_relay_decision_state:
            LanProviderSelectionCloudRelayDecisionState::ManualDecisionRequired,
        candidates,
        manual_requirements: manual_requirements(),
    }
}

#[cfg(test)]
fn selected_route_candidate(
    runtime: &LanPairingRuntime,
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: LanAiProviderRoutingState,
) -> LanProviderSelectionCandidateEvidence {
    let rejection_reason = selected_route_rejection_reason(selected, &routing_state);

    LanProviderSelectionCandidateEvidence {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        provider_peer_id: provider_peer_id(runtime),
        route_id: route_id_for_selected(selected),
        lifecycle_state: lifecycle_state_for_selected(selected, &routing_state),
        discovery_state: discovery_state_for_selected(selected),
        trust_state: trust_state_for_selected(selected),
        reachability: reachability_for_selected(selected),
        policy_decision: policy_decision_for_selected(selected, &routing_state),
        routing_state,
        rejection_reason: rejection_reason.clone(),
        proof_state: LanProviderSelectionProofState::CiMechanicalProof,
        evidence_label: evidence_label_for_selected(selected, rejection_reason.as_ref()),
    }
}

#[cfg(test)]
fn unsupported_capability_candidate(
    runtime: &LanPairingRuntime,
    selected: Option<&LanSelectedRouteTarget>,
) -> LanProviderSelectionCandidateEvidence {
    LanProviderSelectionCandidateEvidence {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        provider_peer_id: provider_peer_id(runtime),
        route_id: route_id_for_selected(selected),
        lifecycle_state: LanProviderSelectionLifecycleState::CandidateRejected,
        discovery_state: discovery_state_for_selected(selected),
        trust_state: trust_state_for_selected(selected),
        reachability: reachability_for_selected(selected),
        routing_state: LanAiProviderRoutingState::UnsupportedCapability,
        rejection_reason: Some(LanPairingRejectionReason::LanAiJobUnauthorized),
        policy_decision: LanProviderSelectionPolicyDecision::RefuseUnsupportedCapability,
        proof_state: LanProviderSelectionProofState::CiMechanicalProof,
        evidence_label: constants::value::LAN_REASON_LAN_AI_JOB_UNAUTHORIZED.to_string(),
    }
}

#[cfg(test)]
fn physical_household_candidate(
    runtime: &LanPairingRuntime,
) -> LanProviderSelectionCandidateEvidence {
    LanProviderSelectionCandidateEvidence {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        provider_peer_id: provider_peer_id(runtime),
        route_id: constants::lan_pairing::ROUTE_ID_UNSUPPORTED.to_string(),
        lifecycle_state: LanProviderSelectionLifecycleState::ManualRequired,
        discovery_state: LanPairingProductionDiscoveryState::Unavailable,
        trust_state: LanPairingTrustState::Unpaired,
        reachability: LanPairingDeviceReachability::Offline,
        routing_state: LanAiProviderRoutingState::Unavailable,
        rejection_reason: Some(LanPairingRejectionReason::LocalNetworkDisabled),
        policy_decision: LanProviderSelectionPolicyDecision::RequirePhysicalHouseholdProof,
        proof_state: LanProviderSelectionProofState::ManualRequired,
        evidence_label: constants::lan_pairing::MANUAL_PROOF_GAP_PHYSICAL_DEVICE.to_string(),
    }
}

#[cfg(test)]
fn cloud_relay_candidate() -> LanProviderSelectionCandidateEvidence {
    LanProviderSelectionCandidateEvidence {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        provider_peer_id: constants::value::UNKNOWN_HOST.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_UNSUPPORTED.to_string(),
        lifecycle_state: LanProviderSelectionLifecycleState::NotImplemented,
        discovery_state: LanPairingProductionDiscoveryState::Unavailable,
        trust_state: LanPairingTrustState::Unpaired,
        reachability: LanPairingDeviceReachability::Offline,
        routing_state: LanAiProviderRoutingState::Unavailable,
        rejection_reason: Some(LanPairingRejectionReason::LocalNetworkDisabled),
        policy_decision: LanProviderSelectionPolicyDecision::RequireCloudRelayDecision,
        proof_state: LanProviderSelectionProofState::NotImplemented,
        evidence_label: constants::lan_pairing::MANUAL_PROOF_GAP_LAN_BIND.to_string(),
    }
}

#[cfg(test)]
fn manual_requirements() -> Vec<LanProviderSelectionManualRequirementEvidence> {
    vec![
        manual_requirement(
            LanProviderSelectionManualRequirement::PhysicalHouseholdProviderHost,
            constants::lan_pairing::MANUAL_PROOF_GAP_PHYSICAL_DEVICE,
        ),
        manual_requirement(
            LanProviderSelectionManualRequirement::ProviderRouteOriginAllowlist,
            constants::lan_pairing::ROUTE_REQUIREMENT_ALLOWED_ORIGIN,
        ),
        manual_requirement(
            LanProviderSelectionManualRequirement::ProviderRouteStaleOfflineArtifact,
            constants::lan_pairing::ROUTE_REQUIREMENT_SELECTED_DEVICE_REACHABLE,
        ),
        manual_requirement(
            LanProviderSelectionManualRequirement::ProviderRevocationArtifact,
            constants::lan_pairing::ROUTE_REQUIREMENT_UNREVOKED_PAIRING,
        ),
        manual_requirement(
            LanProviderSelectionManualRequirement::CloudRelayProviderDecision,
            constants::lan_pairing::MANUAL_PROOF_GAP_LAN_BIND,
        ),
    ]
}

#[cfg(test)]
fn manual_requirement(
    requirement: LanProviderSelectionManualRequirement,
    required_artifact_summary: &'static TestStr,
) -> LanProviderSelectionManualRequirementEvidence {
    LanProviderSelectionManualRequirementEvidence {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        requirement,
        state: LanProviderSelectionProofState::ManualRequired,
        required_artifact_summary: required_artifact_summary.to_string(),
    }
}

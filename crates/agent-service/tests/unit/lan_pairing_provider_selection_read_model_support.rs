#[cfg(test)]
use crate::{app::lan_pairing::LanPairingRuntime, app::time::timestamp_now};
use ocentra_parent_agent_protocol::constants;
#[cfg(test)]
use ocentra_parent_agent_protocol::lan_pairing::{
    LanAiProviderRoutingState, LanPairingDeviceReachability,
    LanPairingProductionDiscoveryState, LanPairingRejectionReason, LanPairingTrustState,
    LanSelectedRouteTarget,
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
#[cfg(test)]
use std::string::String as TestString;

#[cfg(test)]
pub(crate) fn provider_selection_read_model(
    runtime: &LanPairingRuntime,
) -> LanProviderSelectionReadModel {
    let selected = runtime.selected_target();
    let routing_state = routing_state(runtime);
    let mut candidates = vec![selected_route_candidate(
        runtime,
        selected.as_ref(),
        routing_state,
    )];
    if runtime.lan_ai_provider_available() {
        candidates.push(unsupported_capability_candidate(runtime, selected.as_ref()));
    }
    candidates.push(physical_household_candidate(runtime));
    candidates.push(cloud_relay_candidate());

    LanProviderSelectionReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        checked_at: timestamp_now(),
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

#[cfg(test)]
fn selected_provider_route_id(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> Option<TestString> {
    selected
        .filter(|target| route_is_selectable(target, routing_state))
        .map(|target| target.route_id.clone())
}

#[cfg(test)]
fn route_is_selectable(
    selected: &LanSelectedRouteTarget,
    routing_state: &LanAiProviderRoutingState,
) -> bool {
    *routing_state == LanAiProviderRoutingState::AuthorizedResult
        && selected_route_rejection_reason(Some(selected), routing_state).is_none()
}

#[cfg(test)]
fn lifecycle_state_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> LanProviderSelectionLifecycleState {
    match (selected, routing_state) {
        (None, _) => LanProviderSelectionLifecycleState::CandidateUnavailable,
        (Some(_), LanAiProviderRoutingState::AuthorizedResult) => {
            LanProviderSelectionLifecycleState::CandidateSelected
        }
        (Some(_), LanAiProviderRoutingState::Busy | LanAiProviderRoutingState::Degraded) => {
            LanProviderSelectionLifecycleState::CandidateDegraded
        }
        (Some(_), LanAiProviderRoutingState::UnsupportedCapability) => {
            LanProviderSelectionLifecycleState::CandidateRejected
        }
        (Some(_), LanAiProviderRoutingState::Unavailable) => {
            LanProviderSelectionLifecycleState::CandidateUnavailable
        }
    }
}

#[cfg(test)]
fn policy_decision_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> LanProviderSelectionPolicyDecision {
    match (
        selected,
        selected_route_rejection_reason(selected, routing_state),
        routing_state,
    ) {
        (None, _, _) => LanProviderSelectionPolicyDecision::RefuseUnpairedProvider,
        (_, Some(_), _) => LanProviderSelectionPolicyDecision::RefuseRouteBlockedProvider,
        (_, None, LanAiProviderRoutingState::AuthorizedResult) => {
            LanProviderSelectionPolicyDecision::SelectAuthorizedProvider
        }
        (_, None, LanAiProviderRoutingState::Busy) => {
            LanProviderSelectionPolicyDecision::DegradeBusyProvider
        }
        (_, None, LanAiProviderRoutingState::Degraded | LanAiProviderRoutingState::Unavailable) => {
            LanProviderSelectionPolicyDecision::DegradeProviderUnavailable
        }
        (_, None, LanAiProviderRoutingState::UnsupportedCapability) => {
            LanProviderSelectionPolicyDecision::RefuseUnsupportedCapability
        }
    }
}

#[cfg(test)]
fn selected_route_rejection_reason(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> Option<LanPairingRejectionReason> {
    let selected = selected?;
    reachability_rejection(&selected.reachability)
        .or_else(|| trust_state_rejection(&selected.trust_state))
        .or_else(|| provider_unavailable_rejection(&selected.trust_state, routing_state))
}

#[cfg(test)]
fn reachability_rejection(
    reachability: &LanPairingDeviceReachability,
) -> Option<LanPairingRejectionReason> {
    match reachability {
        LanPairingDeviceReachability::Offline => Some(LanPairingRejectionReason::Offline),
        LanPairingDeviceReachability::Stale => Some(LanPairingRejectionReason::Stale),
        LanPairingDeviceReachability::Online => None,
    }
}

#[cfg(test)]
fn trust_state_rejection(
    trust_state: &LanPairingTrustState,
) -> Option<LanPairingRejectionReason> {
    match trust_state {
        LanPairingTrustState::Revoked => Some(LanPairingRejectionReason::Revoked),
        LanPairingTrustState::Expired => Some(LanPairingRejectionReason::Expired),
        LanPairingTrustState::Unpaired => Some(LanPairingRejectionReason::Anonymous),
        LanPairingTrustState::Pairing | LanPairingTrustState::Paired => None,
    }
}

#[cfg(test)]
fn provider_unavailable_rejection(
    trust_state: &LanPairingTrustState,
    routing_state: &LanAiProviderRoutingState,
) -> Option<LanPairingRejectionReason> {
    if *routing_state != LanAiProviderRoutingState::Unavailable {
        return None;
    }
    match trust_state {
        LanPairingTrustState::Pairing | LanPairingTrustState::Paired => {
            Some(LanPairingRejectionReason::LanAiProviderUnavailable)
        }
        LanPairingTrustState::Revoked
        | LanPairingTrustState::Expired
        | LanPairingTrustState::Unpaired => None,
    }
}

#[cfg(test)]
fn discovery_state_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
) -> LanPairingProductionDiscoveryState {
    selected.map_or(LanPairingProductionDiscoveryState::Unavailable, |target| {
        discovery_state_from_reachability(&target.reachability)
    })
}

#[cfg(test)]
fn discovery_state_from_reachability(
    reachability: &LanPairingDeviceReachability,
) -> LanPairingProductionDiscoveryState {
    match reachability {
        LanPairingDeviceReachability::Online => LanPairingProductionDiscoveryState::Paired,
        LanPairingDeviceReachability::Offline => LanPairingProductionDiscoveryState::Offline,
        LanPairingDeviceReachability::Stale => LanPairingProductionDiscoveryState::Stale,
    }
}

#[cfg(test)]
fn trust_state_for_selected(selected: Option<&LanSelectedRouteTarget>) -> LanPairingTrustState {
    selected
        .map(|target| target.trust_state.clone())
        .unwrap_or(LanPairingTrustState::Unpaired)
}

#[cfg(test)]
fn reachability_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
) -> LanPairingDeviceReachability {
    selected
        .map(|target| target.reachability.clone())
        .unwrap_or(LanPairingDeviceReachability::Offline)
}

#[cfg(test)]
fn evidence_label_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
    rejection_reason: Option<&LanPairingRejectionReason>,
) -> TestString {
    match (selected, rejection_reason) {
        (None, _) | (_, Some(LanPairingRejectionReason::Anonymous)) => {
            constants::value::LAN_REASON_ANONYMOUS
        }
        (_, Some(LanPairingRejectionReason::Offline)) => constants::value::LAN_REASON_OFFLINE,
        (_, Some(LanPairingRejectionReason::Stale)) => constants::value::LAN_REASON_STALE,
        (_, Some(LanPairingRejectionReason::Revoked)) => constants::value::LAN_REASON_REVOKED,
        (_, Some(LanPairingRejectionReason::Expired)) => constants::value::LAN_REASON_EXPIRED,
        (_, Some(LanPairingRejectionReason::LanAiProviderUnavailable)) => {
            constants::value::LAN_REASON_LAN_AI_PROVIDER_UNAVAILABLE
        }
        (_, Some(_)) => constants::value::LAN_CONTROL_REJECTED,
        (_, None) => constants::value::LAN_AUDIT_LAN_AI_JOB_COMPLETED,
    }
    .to_string()
}

#[cfg(test)]
fn route_id_for_selected(selected: Option<&LanSelectedRouteTarget>) -> TestString {
    selected
        .map(|target| target.route_id.clone())
        .unwrap_or_else(|| constants::lan_pairing::ROUTE_ID_UNSUPPORTED.to_string())
}

#[cfg(test)]
fn provider_peer_id(runtime: &LanPairingRuntime) -> TestString {
    runtime.device_role_read_model().physical_device_id
}

#[cfg(test)]
fn routing_state(runtime: &LanPairingRuntime) -> LanAiProviderRoutingState {
    match runtime.lan_ai_provider_routing_state() {
        constants::value::LAN_AI_PROVIDER_ROUTING_AUTHORIZED_RESULT => {
            LanAiProviderRoutingState::AuthorizedResult
        }
        constants::value::LAN_AI_PROVIDER_ROUTING_BUSY => LanAiProviderRoutingState::Busy,
        constants::value::LAN_AI_PROVIDER_ROUTING_DEGRADED => LanAiProviderRoutingState::Degraded,
        constants::value::LAN_AI_PROVIDER_ROUTING_UNSUPPORTED_CAPABILITY => {
            LanAiProviderRoutingState::UnsupportedCapability
        }
        _ => LanAiProviderRoutingState::Unavailable,
    }
}

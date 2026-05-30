use ocentra_parent_agent_protocol::{
    constants, LanAiProviderRoutingState, LanPairingDeviceReachability,
    LanPairingProductionDiscoveryState, LanPairingRejectionReason, LanPairingTrustState,
    LanProviderSelectionCandidateEvidence, LanProviderSelectionCloudRelayDecisionState,
    LanProviderSelectionCloudRelayImplementationState, LanProviderSelectionLifecycleState,
    LanProviderSelectionManualRequirement, LanProviderSelectionManualRequirementEvidence,
    LanProviderSelectionPolicyDecision, LanProviderSelectionProofState,
    LanProviderSelectionReadModel, LanSelectedRouteTarget,
};

use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

pub(crate) fn provider_selection_read_model(
    runtime: &LanPairingRuntime,
) -> LanProviderSelectionReadModel {
    let selected = runtime.selected_target();
    let routing_state = routing_state(runtime);
    let selected_provider_route_id = selected_provider_route_id(selected.as_ref(), &routing_state);
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
        selected_provider_route_id,
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

fn selected_route_candidate(
    runtime: &LanPairingRuntime,
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: LanAiProviderRoutingState,
) -> LanProviderSelectionCandidateEvidence {
    let reachability = reachability_for_selected(selected);
    let rejection_reason = selected_route_rejection_reason(selected, &routing_state);
    let evidence_label = evidence_label_for_selected(selected, rejection_reason.as_ref());
    LanProviderSelectionCandidateEvidence {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        provider_peer_id: provider_peer_id(runtime),
        route_id: route_id_for_selected(selected),
        lifecycle_state: lifecycle_state_for_selected(selected, &routing_state),
        discovery_state: discovery_state_for_selected(selected),
        trust_state: trust_state_for_selected(selected),
        reachability,
        policy_decision: policy_decision_for_selected(selected, &routing_state),
        routing_state,
        rejection_reason,
        proof_state: LanProviderSelectionProofState::CiMechanicalProof,
        evidence_label,
    }
}

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

fn manual_requirement(
    requirement: LanProviderSelectionManualRequirement,
    required_artifact_summary: &'static str,
) -> LanProviderSelectionManualRequirementEvidence {
    LanProviderSelectionManualRequirementEvidence {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        requirement,
        state: LanProviderSelectionProofState::ManualRequired,
        required_artifact_summary: required_artifact_summary.to_string(),
    }
}

fn selected_provider_route_id(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> Option<String> {
    if selected.is_some()
        && *routing_state == LanAiProviderRoutingState::AuthorizedResult
        && selected_route_rejection_reason(selected, routing_state).is_none()
    {
        return selected.map(|target| target.route_id.clone());
    }
    None
}

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

fn policy_decision_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> LanProviderSelectionPolicyDecision {
    if selected.is_none() {
        return LanProviderSelectionPolicyDecision::RefuseUnpairedProvider;
    }
    if selected_route_rejection_reason(selected, routing_state).is_some() {
        return LanProviderSelectionPolicyDecision::RefuseRouteBlockedProvider;
    }
    match routing_state {
        LanAiProviderRoutingState::AuthorizedResult => {
            LanProviderSelectionPolicyDecision::SelectAuthorizedProvider
        }
        LanAiProviderRoutingState::Busy => LanProviderSelectionPolicyDecision::DegradeBusyProvider,
        LanAiProviderRoutingState::Degraded | LanAiProviderRoutingState::Unavailable => {
            LanProviderSelectionPolicyDecision::DegradeProviderUnavailable
        }
        LanAiProviderRoutingState::UnsupportedCapability => {
            LanProviderSelectionPolicyDecision::RefuseUnsupportedCapability
        }
    }
}

fn selected_route_rejection_reason(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> Option<LanPairingRejectionReason> {
    let selected = selected?;
    match selected.reachability {
        LanPairingDeviceReachability::Offline => Some(LanPairingRejectionReason::Offline),
        LanPairingDeviceReachability::Stale => Some(LanPairingRejectionReason::Stale),
        LanPairingDeviceReachability::Online => match selected.trust_state {
            LanPairingTrustState::Revoked => Some(LanPairingRejectionReason::Revoked),
            LanPairingTrustState::Expired => Some(LanPairingRejectionReason::Expired),
            LanPairingTrustState::Unpaired => Some(LanPairingRejectionReason::Anonymous),
            LanPairingTrustState::Pairing | LanPairingTrustState::Paired => {
                if *routing_state == LanAiProviderRoutingState::Unavailable {
                    Some(LanPairingRejectionReason::LanAiProviderUnavailable)
                } else {
                    None
                }
            }
        },
    }
}

fn evidence_label_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
    rejection_reason: Option<&LanPairingRejectionReason>,
) -> String {
    if selected.is_none() {
        return constants::value::LAN_REASON_ANONYMOUS.to_string();
    }
    match rejection_reason {
        Some(LanPairingRejectionReason::Offline) => constants::value::LAN_REASON_OFFLINE,
        Some(LanPairingRejectionReason::Stale) => constants::value::LAN_REASON_STALE,
        Some(LanPairingRejectionReason::Revoked) => constants::value::LAN_REASON_REVOKED,
        Some(LanPairingRejectionReason::Expired) => constants::value::LAN_REASON_EXPIRED,
        Some(LanPairingRejectionReason::Anonymous) => constants::value::LAN_REASON_ANONYMOUS,
        Some(LanPairingRejectionReason::LanAiProviderUnavailable) => {
            constants::value::LAN_REASON_LAN_AI_PROVIDER_UNAVAILABLE
        }
        Some(_) => constants::value::LAN_CONTROL_REJECTED,
        None => constants::value::LAN_AUDIT_LAN_AI_JOB_COMPLETED,
    }
    .to_string()
}

fn discovery_state_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
) -> LanPairingProductionDiscoveryState {
    match selected {
        Some(target) => match target.reachability {
            LanPairingDeviceReachability::Online => LanPairingProductionDiscoveryState::Paired,
            LanPairingDeviceReachability::Offline => LanPairingProductionDiscoveryState::Offline,
            LanPairingDeviceReachability::Stale => LanPairingProductionDiscoveryState::Stale,
        },
        None => LanPairingProductionDiscoveryState::Unavailable,
    }
}

fn trust_state_for_selected(selected: Option<&LanSelectedRouteTarget>) -> LanPairingTrustState {
    selected
        .map(|target| target.trust_state.clone())
        .unwrap_or(LanPairingTrustState::Unpaired)
}

fn reachability_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
) -> LanPairingDeviceReachability {
    selected
        .map(|target| target.reachability.clone())
        .unwrap_or(LanPairingDeviceReachability::Offline)
}

fn route_id_for_selected(selected: Option<&LanSelectedRouteTarget>) -> String {
    selected
        .map(|target| target.route_id.clone())
        .unwrap_or_else(|| constants::lan_pairing::ROUTE_ID_UNSUPPORTED.to_string())
}

fn provider_peer_id(runtime: &LanPairingRuntime) -> String {
    runtime.device_role_read_model().physical_device_id
}

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

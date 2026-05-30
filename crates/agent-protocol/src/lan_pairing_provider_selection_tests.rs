use crate::{
    constants, LanAiProviderRoutingState, LanPairingDeviceReachability,
    LanPairingProductionDiscoveryState, LanPairingRejectionReason, LanPairingTrustState,
    LanProviderSelectionCandidateEvidence, LanProviderSelectionCloudRelayDecisionState,
    LanProviderSelectionCloudRelayImplementationState, LanProviderSelectionLifecycleState,
    LanProviderSelectionManualRequirement, LanProviderSelectionManualRequirementEvidence,
    LanProviderSelectionPolicyDecision, LanProviderSelectionProofState,
    LanProviderSelectionReadModel, LAN_PAIRING_SCHEMA_VERSION,
};

#[test]
fn provider_selection_read_model_serializes_honest_manual_and_cloud_states() {
    let model = LanProviderSelectionReadModel {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        checked_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        selected_provider_route_id: Some(
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        ),
        authorized_provider_selection_state: LanProviderSelectionProofState::CiMechanicalProof,
        physical_household_provider_proof_state: LanProviderSelectionProofState::ManualRequired,
        cloud_relay_implementation_state:
            LanProviderSelectionCloudRelayImplementationState::NotImplemented,
        cloud_relay_decision_state:
            LanProviderSelectionCloudRelayDecisionState::ManualDecisionRequired,
        candidates: vec![
            selected_candidate(),
            unsupported_capability_candidate(),
            cloud_relay_candidate(),
        ],
        manual_requirements: vec![
            manual_requirement(
                LanProviderSelectionManualRequirement::PhysicalHouseholdProviderHost,
            ),
            manual_requirement(LanProviderSelectionManualRequirement::CloudRelayProviderDecision),
        ],
    };

    let json = serde_json::to_string(&model).expect(constants::error::AGENT_EVENT_SERIALIZES);
    assert!(json.contains(constants::value::LAN_AI_PROVIDER_ROUTING_AUTHORIZED_RESULT));
    assert!(json.contains(constants::value::LAN_AI_PROVIDER_ROUTING_UNSUPPORTED_CAPABILITY));
    assert!(json.contains("manual-required"));
    assert!(json.contains("not-implemented"));
    assert!(!json.contains("product-ready"));
    assert!(!json.contains("cloud-relay-implemented"));
}

#[test]
fn provider_selection_candidate_serializes_route_blocked_rejection() {
    let candidate = LanProviderSelectionCandidateEvidence {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        provider_peer_id: constants::peer::PORTAL_DEV.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        lifecycle_state: LanProviderSelectionLifecycleState::CandidateUnavailable,
        discovery_state: LanPairingProductionDiscoveryState::Offline,
        trust_state: LanPairingTrustState::Paired,
        reachability: LanPairingDeviceReachability::Offline,
        routing_state: LanAiProviderRoutingState::Unavailable,
        rejection_reason: Some(LanPairingRejectionReason::Offline),
        policy_decision: LanProviderSelectionPolicyDecision::RefuseRouteBlockedProvider,
        proof_state: LanProviderSelectionProofState::CiMechanicalProof,
        evidence_label: constants::value::LAN_REASON_OFFLINE.to_string(),
    };

    let json = serde_json::to_string(&candidate).expect(constants::error::AGENT_EVENT_SERIALIZES);
    assert!(json.contains(constants::value::LAN_REASON_OFFLINE));
    assert!(json.contains(constants::value::LAN_AI_PROVIDER_ROUTING_UNAVAILABLE));
}

fn selected_candidate() -> LanProviderSelectionCandidateEvidence {
    LanProviderSelectionCandidateEvidence {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        provider_peer_id: constants::peer::PORTAL_DEV.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        lifecycle_state: LanProviderSelectionLifecycleState::CandidateSelected,
        discovery_state: LanPairingProductionDiscoveryState::Paired,
        trust_state: LanPairingTrustState::Paired,
        reachability: LanPairingDeviceReachability::Online,
        routing_state: LanAiProviderRoutingState::AuthorizedResult,
        rejection_reason: None,
        policy_decision: LanProviderSelectionPolicyDecision::SelectAuthorizedProvider,
        proof_state: LanProviderSelectionProofState::CiMechanicalProof,
        evidence_label: constants::value::LAN_AUDIT_LAN_AI_JOB_COMPLETED.to_string(),
    }
}

fn unsupported_capability_candidate() -> LanProviderSelectionCandidateEvidence {
    LanProviderSelectionCandidateEvidence {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        provider_peer_id: constants::peer::PORTAL_DEV.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        lifecycle_state: LanProviderSelectionLifecycleState::CandidateRejected,
        discovery_state: LanPairingProductionDiscoveryState::Paired,
        trust_state: LanPairingTrustState::Paired,
        reachability: LanPairingDeviceReachability::Online,
        routing_state: LanAiProviderRoutingState::UnsupportedCapability,
        rejection_reason: Some(LanPairingRejectionReason::LanAiJobUnauthorized),
        policy_decision: LanProviderSelectionPolicyDecision::RefuseUnsupportedCapability,
        proof_state: LanProviderSelectionProofState::CiMechanicalProof,
        evidence_label: constants::value::LAN_REASON_LAN_AI_JOB_UNAUTHORIZED.to_string(),
    }
}

fn cloud_relay_candidate() -> LanProviderSelectionCandidateEvidence {
    LanProviderSelectionCandidateEvidence {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
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

fn manual_requirement(
    requirement: LanProviderSelectionManualRequirement,
) -> LanProviderSelectionManualRequirementEvidence {
    LanProviderSelectionManualRequirementEvidence {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        requirement,
        state: LanProviderSelectionProofState::ManualRequired,
        required_artifact_summary: constants::lan_pairing::MANUAL_PROOF_GAP_PHYSICAL_DEVICE
            .to_string(),
    }
}

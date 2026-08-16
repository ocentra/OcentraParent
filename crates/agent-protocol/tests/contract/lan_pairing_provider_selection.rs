use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::LanAiProviderRoutingState;
use ocentra_parent_agent_protocol::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::LanPairingTrustState;
use ocentra_parent_agent_protocol::LanProviderSelectionCandidateEvidence;
use ocentra_parent_agent_protocol::LanProviderSelectionCloudRelayDecisionState;
use ocentra_parent_agent_protocol::LanProviderSelectionCloudRelayImplementationState;
use ocentra_parent_agent_protocol::LanProviderSelectionLifecycleState;
use ocentra_parent_agent_protocol::LanProviderSelectionManualRequirement;
use ocentra_parent_agent_protocol::LanProviderSelectionManualRequirementEvidence;
use ocentra_parent_agent_protocol::LanProviderSelectionPolicyDecision;
use ocentra_parent_agent_protocol::LanProviderSelectionProofState;
use ocentra_parent_agent_protocol::LanProviderSelectionReadModel;
use ocentra_parent_agent_protocol::LAN_PAIRING_SCHEMA_VERSION;

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

    let json = serde_json::to_value(&model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(
        json["candidates"][0]["routingState"],
        constants::value::LAN_AI_PROVIDER_ROUTING_AUTHORIZED_RESULT
    );
    assert_eq!(
        json["candidates"][1]["routingState"],
        constants::value::LAN_AI_PROVIDER_ROUTING_UNSUPPORTED_CAPABILITY
    );
    assert_eq!(
        json["physicalHouseholdProviderProofState"],
        "manual-required"
    );
    assert_eq!(json["cloudRelayImplementationState"], "not-implemented");
    assert_eq!(json["candidates"][2]["lifecycleState"], "not-implemented");
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

    let json =
        serde_json::to_value(&candidate).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(json["evidenceLabel"], constants::value::LAN_REASON_OFFLINE);
    assert_eq!(
        json["routingState"],
        constants::value::LAN_AI_PROVIDER_ROUTING_UNAVAILABLE
    );
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

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::LanPairingResponseState;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelayAdapterKind;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelayAdapterRow;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelayCacheCheck;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelayCacheRow;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelayCustodyLabel;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelayDecisionState;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelayRouteSafetyCheck;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelayRouteSafetyRow;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelaySignedProofCheck;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelaySignedProofRow;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelaySourceConfidence;
use ocentra_parent_agent_protocol::LanSignedDiscoveryRelaySpineSummary;
use ocentra_parent_agent_protocol::V09ProductionDiscoveryHouseholdProofState;
use ocentra_parent_agent_protocol::V09ProductionDiscoveryHouseholdRuntimeOwner;
use ocentra_parent_agent_protocol::LAN_PAIRING_SCHEMA_VERSION;

pub(super) fn signed_discovery_relay_spine_fixture() -> LanSignedDiscoveryRelaySpineSummary {
    LanSignedDiscoveryRelaySpineSummary {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        generated_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        adapter_rows: adapter_rows(),
        signed_proof_rows: signed_proof_rows(),
        route_safety_rows: route_safety_rows(),
        relay_cache_rows: relay_cache_rows(),
        manual_proof_required: vec![
            LanSignedDiscoveryRelayAdapterKind::MdnsName,
            LanSignedDiscoveryRelayAdapterKind::SsdpName,
            LanSignedDiscoveryRelayAdapterKind::RouterDhcpName,
            LanSignedDiscoveryRelayAdapterKind::ManualDirectAddress,
            LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHello,
            LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHeartbeat,
        ],
        not_implemented: vec![
            LanSignedDiscoveryRelayCacheCheck::RelayRouteUnavailable,
            LanSignedDiscoveryRelayCacheCheck::RelayRouteQueuedNotConfigured,
            LanSignedDiscoveryRelayCacheCheck::CacheRouteUnavailable,
            LanSignedDiscoveryRelayCacheCheck::ParentOwnedStorageUnavailable,
        ],
        claims_proved: vec![
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_PASSIVE_NEIGHBOR.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_REGISTRY_ROUTE.to_string(),
        ],
        claims_not_proved: vec![
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_PHYSICAL.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_CLOUD.to_string(),
            constants::lan_pairing::SIGNED_DISCOVERY_RELAY_NON_CLAIM_PARENT_STORAGE.to_string(),
        ],
    }
}

pub(super) fn assert_signed_discovery_relay_spine_json(value: &serde_json::Value) {
    assert_eq!(
        value["adapterRows"][0]["adapter"],
        serde_json::json!("passive-lan-neighbor")
    );
    assert_eq!(
        value["adapterRows"][2]["requiredArtifactSummary"],
        serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_MDNS)
    );
    assert_eq!(
        value["adapterRows"][7]["evidenceLabel"],
        serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT)
    );
    assert_eq!(
        value["adapterRows"][7]["requiredArtifactSummary"],
        serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HEARTBEAT)
    );
    assert_eq!(
        value["signedProofRows"][3]["check"],
        serde_json::json!("unauthenticated-caller-rejected")
    );
    assert_eq!(
        value["signedProofRows"][3]["rejectionReason"],
        serde_json::json!("anonymous")
    );
    assert_eq!(
        value["signedProofRows"][3]["evidenceLabel"],
        serde_json::json!(constants::value::LAN_REASON_ANONYMOUS)
    );
    assert_eq!(
        value["signedProofRows"][2]["evidenceLabel"],
        serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HELLO)
    );
    assert_eq!(
        value["routeSafetyRows"][0]["evidenceLabel"],
        serde_json::json!(constants::lan_pairing::ROUTE_REQUIREMENT_ROUTE_RECOVERY_PERSISTED)
    );
    assert_eq!(
        value["routeSafetyRows"][2]["discoveryState"],
        serde_json::json!("manual-required")
    );
    assert_eq!(
        value["routeSafetyRows"][10]["discoveryState"],
        serde_json::json!("discovered")
    );
    assert_eq!(
        value["relayCacheRows"][2]["evidenceLabel"],
        serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_LABEL_CACHE_ROUTE)
    );
    assert_eq!(
        value["relayCacheRows"][3]["evidenceLabel"],
        serde_json::json!(constants::lan_pairing::SIGNED_DISCOVERY_RELAY_NON_CLAIM_PARENT_STORAGE)
    );
}

fn adapter_rows() -> Vec<LanSignedDiscoveryRelayAdapterRow> {
    vec![
        adapter_row(
            LanSignedDiscoveryRelayAdapterKind::PassiveLanNeighbor,
            LanPairingProductionDiscoveryState::Discovered,
            V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            LanSignedDiscoveryRelaySourceConfidence::Strong,
            LanSignedDiscoveryRelayCustodyLabel::PassiveLanObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
            None,
        ),
        adapter_row(
            LanSignedDiscoveryRelayAdapterKind::RouterInfrastructure,
            LanPairingProductionDiscoveryState::Discovered,
            V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            LanSignedDiscoveryRelaySourceConfidence::Strong,
            LanSignedDiscoveryRelayCustodyLabel::RouterInfrastructureObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTER_NEIGHBOR,
            None,
        ),
        manual_adapter_row(LanSignedDiscoveryRelayAdapterKind::MdnsName),
        manual_adapter_row(LanSignedDiscoveryRelayAdapterKind::SsdpName),
        manual_adapter_row(LanSignedDiscoveryRelayAdapterKind::RouterDhcpName),
        manual_adapter_row(LanSignedDiscoveryRelayAdapterKind::ManualDirectAddress),
        manual_adapter_row(LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHello),
        manual_adapter_row(LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHeartbeat),
    ]
}

fn signed_proof_rows() -> Vec<LanSignedDiscoveryRelaySignedProofRow> {
    vec![
        manual_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::SignedHelloManualRequired,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO,
        ),
        manual_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::SignedHeartbeatManualRequired,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT,
        ),
        manual_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::AcceptedSignedChildAgentManualRequired,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HELLO,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::UnauthenticatedCallerRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::Anonymous,
            constants::value::LAN_REASON_ANONYMOUS,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::ExpiredSignedProofRejected,
            LanPairingProductionDiscoveryState::Expired,
            LanPairingRejectionReason::Expired,
            constants::value::LAN_REASON_EXPIRED,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::ReplayedSignedProofRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::Replayed,
            constants::value::LAN_REASON_REPLAYED,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::WrongOriginSignedProofRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::WrongOrigin,
            constants::value::LAN_REASON_WRONG_ORIGIN,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::WrongDeviceSignedProofRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::WrongDevice,
            constants::value::LAN_REASON_WRONG_DEVICE,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::RevokedSignedProofRejected,
            LanPairingProductionDiscoveryState::Revoked,
            LanPairingRejectionReason::Revoked,
            constants::value::LAN_REASON_REVOKED,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::StaleSignedProofRejected,
            LanPairingProductionDiscoveryState::Stale,
            LanPairingRejectionReason::Stale,
            constants::value::LAN_REASON_STALE,
        ),
    ]
}

fn route_safety_rows() -> Vec<LanSignedDiscoveryRelayRouteSafetyRow> {
    vec![
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::TrustedRegistryRestartRecovery,
            LanPairingProductionDiscoveryState::Paired,
            constants::lan_pairing::ROUTE_REQUIREMENT_ROUTE_RECOVERY_PERSISTED,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::SelectedRouteCustody,
            LanPairingProductionDiscoveryState::Paired,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTE_CUSTODY,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::StaleSelectedDeviceRejected,
            LanPairingProductionDiscoveryState::ManualRequired,
            LanPairingRejectionReason::Stale,
            constants::value::LAN_REASON_STALE,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::OfflineSelectedDeviceRejected,
            LanPairingProductionDiscoveryState::ManualRequired,
            LanPairingRejectionReason::Offline,
            constants::value::LAN_REASON_OFFLINE,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::WrongRouteRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::WrongDevice,
            constants::value::LAN_REASON_WRONG_DEVICE,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::RevokedRouteRejected,
            LanPairingProductionDiscoveryState::Revoked,
            LanPairingRejectionReason::Revoked,
            constants::value::LAN_REASON_REVOKED,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentAssignDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_ASSIGNMENT,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRenameDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_RENAME,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentIgnoreDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_IGNORE,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRestoreDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
            constants::lan_pairing::HOUSEHOLD_ACTION_RESTORE,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentTrustDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
            constants::lan_pairing::HOUSEHOLD_ACTION_TRUST,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRevokeDecisionAudited,
            LanPairingProductionDiscoveryState::Revoked,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_REVOCATION,
        ),
    ]
}

fn relay_cache_rows() -> Vec<LanSignedDiscoveryRelayCacheRow> {
    vec![
        relay_cache_row(
            LanSignedDiscoveryRelayCacheCheck::RelayRouteUnavailable,
            LanSignedDiscoveryRelayDecisionState::Unavailable,
            V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            LanSignedDiscoveryRelayCustodyLabel::NoOcentraChildDataCustody,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_RELAY_ROUTE,
        ),
        relay_cache_row(
            LanSignedDiscoveryRelayCacheCheck::RelayRouteQueuedNotConfigured,
            LanSignedDiscoveryRelayDecisionState::QueuedNotConfigured,
            V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            LanSignedDiscoveryRelayCustodyLabel::NoOcentraChildDataCustody,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_RELAY_ROUTE,
        ),
        relay_cache_row(
            LanSignedDiscoveryRelayCacheCheck::CacheRouteUnavailable,
            LanSignedDiscoveryRelayDecisionState::Unavailable,
            V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            LanSignedDiscoveryRelayCustodyLabel::NoOcentraChildDataCustody,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_CACHE_ROUTE,
        ),
        relay_cache_row(
            LanSignedDiscoveryRelayCacheCheck::ParentOwnedStorageUnavailable,
            LanSignedDiscoveryRelayDecisionState::Unavailable,
            V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            LanSignedDiscoveryRelayCustodyLabel::ParentOwnedStorageUnavailable,
            constants::lan_pairing::SIGNED_DISCOVERY_RELAY_NON_CLAIM_PARENT_STORAGE,
        ),
        relay_cache_row(
            LanSignedDiscoveryRelayCacheCheck::OcentraChildDataCustodyNotClaimed,
            LanSignedDiscoveryRelayDecisionState::LocalFirst,
            V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            LanSignedDiscoveryRelayCustodyLabel::NoOcentraChildDataCustody,
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_CLOUD,
        ),
    ]
}

fn manual_adapter_row(
    adapter: LanSignedDiscoveryRelayAdapterKind,
) -> LanSignedDiscoveryRelayAdapterRow {
    let (custody_label, evidence_label, required_artifact_summary) = match &adapter {
        LanSignedDiscoveryRelayAdapterKind::RouterDhcpName => (
            LanSignedDiscoveryRelayCustodyLabel::RouterInfrastructureObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTER_DHCP,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_ROUTER_DHCP,
        ),
        LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHello
        | LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHeartbeat => {
            let (label, artifact) = match adapter {
                LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHello => (
                    constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO,
                    constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HELLO,
                ),
                LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHeartbeat => (
                    constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT,
                    constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HEARTBEAT,
                ),
                _ => unreachable!(),
            };
            (
                LanSignedDiscoveryRelayCustodyLabel::SignedChildAgentArtifact,
                label,
                artifact,
            )
        }
        LanSignedDiscoveryRelayAdapterKind::ManualDirectAddress => (
            LanSignedDiscoveryRelayCustodyLabel::ManualParentEntry,
            constants::lan_pairing::MANUAL_PROOF_GAP_LAN_BIND,
            constants::lan_pairing::MANUAL_PROOF_GAP_LAN_BIND,
        ),
        LanSignedDiscoveryRelayAdapterKind::MdnsName => (
            LanSignedDiscoveryRelayCustodyLabel::PassiveLanObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_MDNS,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_MDNS,
        ),
        LanSignedDiscoveryRelayAdapterKind::SsdpName => (
            LanSignedDiscoveryRelayCustodyLabel::PassiveLanObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SSDP,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SSDP,
        ),
        LanSignedDiscoveryRelayAdapterKind::PassiveLanNeighbor
        | LanSignedDiscoveryRelayAdapterKind::RouterInfrastructure => unreachable!(),
    };
    adapter_row(
        adapter,
        LanPairingProductionDiscoveryState::ManualRequired,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        LanSignedDiscoveryRelaySourceConfidence::ManualRequired,
        custody_label,
        evidence_label,
        Some(required_artifact_summary.to_string()),
    )
}

fn adapter_row(
    adapter: LanSignedDiscoveryRelayAdapterKind,
    discovery_state: LanPairingProductionDiscoveryState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    source_confidence: LanSignedDiscoveryRelaySourceConfidence,
    custody_label: LanSignedDiscoveryRelayCustodyLabel,
    evidence_label: &str,
    required_artifact_summary: Option<String>,
) -> LanSignedDiscoveryRelayAdapterRow {
    let runtime_owner = runtime_owner_for(&proof_state);
    LanSignedDiscoveryRelayAdapterRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        adapter,
        discovery_state,
        proof_state,
        source_confidence,
        custody_label,
        runtime_owner,
        evidence_label: evidence_label.to_string(),
        required_artifact_summary,
    }
}

fn manual_signed_proof_row(
    check: LanSignedDiscoveryRelaySignedProofCheck,
    evidence_label: &str,
) -> LanSignedDiscoveryRelaySignedProofRow {
    signed_proof_row(
        check,
        LanPairingProductionDiscoveryState::ManualRequired,
        LanPairingResponseState::Queued,
        None,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        evidence_label,
    )
}

fn rejected_signed_proof_row(
    check: LanSignedDiscoveryRelaySignedProofCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    rejection_reason: LanPairingRejectionReason,
    evidence_label: &str,
) -> LanSignedDiscoveryRelaySignedProofRow {
    signed_proof_row(
        check,
        discovery_state,
        LanPairingResponseState::Rejected,
        Some(rejection_reason),
        V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        evidence_label,
    )
}

fn signed_proof_row(
    check: LanSignedDiscoveryRelaySignedProofCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    response_state: LanPairingResponseState,
    rejection_reason: Option<LanPairingRejectionReason>,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    evidence_label: &str,
) -> LanSignedDiscoveryRelaySignedProofRow {
    let runtime_owner = runtime_owner_for(&proof_state);
    LanSignedDiscoveryRelaySignedProofRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        check,
        discovery_state,
        response_state,
        rejection_reason,
        proof_state,
        runtime_owner,
        evidence_label: evidence_label.to_string(),
    }
}

fn accepted_route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    route_safety_row(
        check,
        discovery_state,
        LanPairingResponseState::Accepted,
        None,
        evidence_label,
    )
}

fn rejected_route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    rejection_reason: LanPairingRejectionReason,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    route_safety_row(
        check,
        discovery_state,
        LanPairingResponseState::Rejected,
        Some(rejection_reason),
        evidence_label,
    )
}

fn route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    response_state: LanPairingResponseState,
    rejection_reason: Option<LanPairingRejectionReason>,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    LanSignedDiscoveryRelayRouteSafetyRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        check,
        route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        discovery_state,
        response_state,
        rejection_reason,
        proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        custody_label: LanSignedDiscoveryRelayCustodyLabel::ParentLocalService,
        evidence_label: evidence_label.to_string(),
    }
}

fn relay_cache_row(
    check: LanSignedDiscoveryRelayCacheCheck,
    decision_state: LanSignedDiscoveryRelayDecisionState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    custody_label: LanSignedDiscoveryRelayCustodyLabel,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayCacheRow {
    let runtime_owner = runtime_owner_for(&proof_state);
    LanSignedDiscoveryRelayCacheRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        check,
        decision_state,
        discovery_state: LanPairingProductionDiscoveryState::Unavailable,
        proof_state,
        runtime_owner,
        custody_label,
        evidence_label: evidence_label.to_string(),
    }
}

fn runtime_owner_for(
    proof_state: &V09ProductionDiscoveryHouseholdProofState,
) -> V09ProductionDiscoveryHouseholdRuntimeOwner {
    if *proof_state == V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof {
        V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel
    } else {
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof
    }
}

use ocentra_eventing::expect_value::ExpectValue;
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

#[derive(Clone)]
struct AdapterRowMetadata {
    evidence_label: &'static str,
    required_artifact_summary: Option<&'static str>,
}

#[derive(Clone, Copy)]
struct SignedProofMetadata {
    evidence_label: &'static str,
}

#[derive(Clone, Copy)]
struct RouteSafetyMetadata {
    evidence_label: &'static str,
}

#[derive(Clone, Copy)]
struct RelayCacheMetadata {
    evidence_label: &'static str,
}

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
            &adapter_row_metadata(&LanSignedDiscoveryRelayAdapterKind::PassiveLanNeighbor),
        ),
        adapter_row(
            LanSignedDiscoveryRelayAdapterKind::RouterInfrastructure,
            LanPairingProductionDiscoveryState::Discovered,
            V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            LanSignedDiscoveryRelaySourceConfidence::Strong,
            LanSignedDiscoveryRelayCustodyLabel::RouterInfrastructureObservation,
            &adapter_row_metadata(&LanSignedDiscoveryRelayAdapterKind::RouterInfrastructure),
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
        manual_signed_proof_row(LanSignedDiscoveryRelaySignedProofCheck::SignedHelloManualRequired),
        manual_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::SignedHeartbeatManualRequired,
        ),
        manual_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::AcceptedSignedChildAgentManualRequired,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::UnauthenticatedCallerRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::Anonymous,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::ExpiredSignedProofRejected,
            LanPairingProductionDiscoveryState::Expired,
            LanPairingRejectionReason::Expired,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::ReplayedSignedProofRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::Replayed,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::WrongOriginSignedProofRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::WrongOrigin,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::WrongDeviceSignedProofRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::WrongDevice,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::RevokedSignedProofRejected,
            LanPairingProductionDiscoveryState::Revoked,
            LanPairingRejectionReason::Revoked,
        ),
        rejected_signed_proof_row(
            LanSignedDiscoveryRelaySignedProofCheck::StaleSignedProofRejected,
            LanPairingProductionDiscoveryState::Stale,
            LanPairingRejectionReason::Stale,
        ),
    ]
}

fn route_safety_rows() -> Vec<LanSignedDiscoveryRelayRouteSafetyRow> {
    vec![
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::TrustedRegistryRestartRecovery,
            LanPairingProductionDiscoveryState::Paired,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::SelectedRouteCustody,
            LanPairingProductionDiscoveryState::Paired,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::StaleSelectedDeviceRejected,
            LanPairingProductionDiscoveryState::ManualRequired,
            LanPairingRejectionReason::Stale,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::OfflineSelectedDeviceRejected,
            LanPairingProductionDiscoveryState::ManualRequired,
            LanPairingRejectionReason::Offline,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::WrongRouteRejected,
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::WrongDevice,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::RevokedRouteRejected,
            LanPairingProductionDiscoveryState::Revoked,
            LanPairingRejectionReason::Revoked,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentAssignDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRenameDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentIgnoreDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRestoreDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentTrustDecisionAudited,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRevokeDecisionAudited,
            LanPairingProductionDiscoveryState::Revoked,
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
        ),
        relay_cache_row(
            LanSignedDiscoveryRelayCacheCheck::RelayRouteQueuedNotConfigured,
            LanSignedDiscoveryRelayDecisionState::QueuedNotConfigured,
            V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            LanSignedDiscoveryRelayCustodyLabel::NoOcentraChildDataCustody,
        ),
        relay_cache_row(
            LanSignedDiscoveryRelayCacheCheck::CacheRouteUnavailable,
            LanSignedDiscoveryRelayDecisionState::Unavailable,
            V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            LanSignedDiscoveryRelayCustodyLabel::NoOcentraChildDataCustody,
        ),
        relay_cache_row(
            LanSignedDiscoveryRelayCacheCheck::ParentOwnedStorageUnavailable,
            LanSignedDiscoveryRelayDecisionState::Unavailable,
            V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            LanSignedDiscoveryRelayCustodyLabel::ParentOwnedStorageUnavailable,
        ),
        relay_cache_row(
            LanSignedDiscoveryRelayCacheCheck::OcentraChildDataCustodyNotClaimed,
            LanSignedDiscoveryRelayDecisionState::LocalFirst,
            V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            LanSignedDiscoveryRelayCustodyLabel::NoOcentraChildDataCustody,
        ),
    ]
}

fn manual_adapter_row(
    adapter: LanSignedDiscoveryRelayAdapterKind,
) -> LanSignedDiscoveryRelayAdapterRow {
    let metadata = adapter_row_metadata(&adapter);
    let custody_label = match adapter {
        LanSignedDiscoveryRelayAdapterKind::PassiveLanNeighbor => {
            LanSignedDiscoveryRelayCustodyLabel::PassiveLanObservation
        }
        LanSignedDiscoveryRelayAdapterKind::RouterInfrastructure => {
            LanSignedDiscoveryRelayCustodyLabel::RouterInfrastructureObservation
        }
        LanSignedDiscoveryRelayAdapterKind::MdnsName
        | LanSignedDiscoveryRelayAdapterKind::SsdpName => {
            LanSignedDiscoveryRelayCustodyLabel::PassiveLanObservation
        }
        LanSignedDiscoveryRelayAdapterKind::RouterDhcpName => {
            LanSignedDiscoveryRelayCustodyLabel::RouterInfrastructureObservation
        }
        LanSignedDiscoveryRelayAdapterKind::ManualDirectAddress => {
            LanSignedDiscoveryRelayCustodyLabel::ManualParentEntry
        }
        LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHello
        | LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHeartbeat => {
            LanSignedDiscoveryRelayCustodyLabel::SignedChildAgentArtifact
        }
    };
    adapter_row(
        adapter,
        LanPairingProductionDiscoveryState::ManualRequired,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        LanSignedDiscoveryRelaySourceConfidence::ManualRequired,
        custody_label,
        &metadata,
    )
}

fn adapter_row(
    adapter: LanSignedDiscoveryRelayAdapterKind,
    discovery_state: LanPairingProductionDiscoveryState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    source_confidence: LanSignedDiscoveryRelaySourceConfidence,
    custody_label: LanSignedDiscoveryRelayCustodyLabel,
    metadata: &AdapterRowMetadata,
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
        evidence_label: metadata.evidence_label.to_string(),
        required_artifact_summary: metadata.required_artifact_summary.map(str::to_string),
    }
}

fn manual_signed_proof_row(
    check: LanSignedDiscoveryRelaySignedProofCheck,
) -> LanSignedDiscoveryRelaySignedProofRow {
    signed_proof_row(
        check,
        LanPairingProductionDiscoveryState::ManualRequired,
        LanPairingResponseState::Queued,
        None,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
    )
}

fn rejected_signed_proof_row(
    check: LanSignedDiscoveryRelaySignedProofCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    rejection_reason: LanPairingRejectionReason,
) -> LanSignedDiscoveryRelaySignedProofRow {
    signed_proof_row(
        check,
        discovery_state,
        LanPairingResponseState::Rejected,
        Some(rejection_reason),
        V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
    )
}

fn signed_proof_row(
    check: LanSignedDiscoveryRelaySignedProofCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    response_state: LanPairingResponseState,
    rejection_reason: Option<LanPairingRejectionReason>,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
) -> LanSignedDiscoveryRelaySignedProofRow {
    let runtime_owner = runtime_owner_for(&proof_state);
    let metadata = signed_proof_metadata(&check);
    LanSignedDiscoveryRelaySignedProofRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        check,
        discovery_state,
        response_state,
        rejection_reason,
        proof_state,
        runtime_owner,
        evidence_label: metadata.evidence_label.to_string(),
    }
}

fn accepted_route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    discovery_state: LanPairingProductionDiscoveryState,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    route_safety_row(
        check,
        discovery_state,
        LanPairingResponseState::Accepted,
        None,
    )
}

fn rejected_route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    rejection_reason: LanPairingRejectionReason,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    route_safety_row(
        check,
        discovery_state,
        LanPairingResponseState::Rejected,
        Some(rejection_reason),
    )
}

fn route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    response_state: LanPairingResponseState,
    rejection_reason: Option<LanPairingRejectionReason>,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    let metadata = route_safety_metadata(&check);
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
        evidence_label: metadata.evidence_label.to_string(),
    }
}

fn relay_cache_row(
    check: LanSignedDiscoveryRelayCacheCheck,
    decision_state: LanSignedDiscoveryRelayDecisionState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    custody_label: LanSignedDiscoveryRelayCustodyLabel,
) -> LanSignedDiscoveryRelayCacheRow {
    let runtime_owner = runtime_owner_for(&proof_state);
    let metadata = relay_cache_metadata(&check);
    LanSignedDiscoveryRelayCacheRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        check,
        decision_state,
        discovery_state: LanPairingProductionDiscoveryState::Unavailable,
        proof_state,
        runtime_owner,
        custody_label,
        evidence_label: metadata.evidence_label.to_string(),
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

fn adapter_row_metadata(adapter: &LanSignedDiscoveryRelayAdapterKind) -> AdapterRowMetadata {
    ADAPTER_ROW_METADATA
        .iter()
        .find(|(kind, _)| kind == adapter)
        .map(|(_, metadata)| metadata.clone())
        .expect_value("adapter metadata exists")
}

fn signed_proof_metadata(check: &LanSignedDiscoveryRelaySignedProofCheck) -> SignedProofMetadata {
    SIGNED_PROOF_METADATA
        .iter()
        .find(|(kind, _)| kind == check)
        .map(|(_, metadata)| *metadata)
        .expect_value("signed proof metadata exists")
}

fn route_safety_metadata(check: &LanSignedDiscoveryRelayRouteSafetyCheck) -> RouteSafetyMetadata {
    ROUTE_SAFETY_METADATA
        .iter()
        .find(|(kind, _)| kind == check)
        .map(|(_, metadata)| *metadata)
        .expect_value("route safety metadata exists")
}

fn relay_cache_metadata(check: &LanSignedDiscoveryRelayCacheCheck) -> RelayCacheMetadata {
    RELAY_CACHE_METADATA
        .iter()
        .find(|(kind, _)| kind == check)
        .map(|(_, metadata)| *metadata)
        .expect_value("relay cache metadata exists")
}

const ADAPTER_ROW_METADATA: &[(LanSignedDiscoveryRelayAdapterKind, AdapterRowMetadata)] = &[
    (
        LanSignedDiscoveryRelayAdapterKind::PassiveLanNeighbor,
        AdapterRowMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
            required_artifact_summary: None,
        },
    ),
    (
        LanSignedDiscoveryRelayAdapterKind::RouterInfrastructure,
        AdapterRowMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTER_NEIGHBOR,
            required_artifact_summary: None,
        },
    ),
    (
        LanSignedDiscoveryRelayAdapterKind::MdnsName,
        AdapterRowMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_MDNS,
            required_artifact_summary: Some(constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_MDNS),
        },
    ),
    (
        LanSignedDiscoveryRelayAdapterKind::SsdpName,
        AdapterRowMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_SSDP,
            required_artifact_summary: Some(constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SSDP),
        },
    ),
    (
        LanSignedDiscoveryRelayAdapterKind::RouterDhcpName,
        AdapterRowMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTER_DHCP,
            required_artifact_summary: Some(
                constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_ROUTER_DHCP,
            ),
        },
    ),
    (
        LanSignedDiscoveryRelayAdapterKind::ManualDirectAddress,
        AdapterRowMetadata {
            evidence_label: constants::lan_pairing::MANUAL_PROOF_GAP_LAN_BIND,
            required_artifact_summary: Some(constants::lan_pairing::MANUAL_PROOF_GAP_LAN_BIND),
        },
    ),
    (
        LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHello,
        AdapterRowMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO,
            required_artifact_summary: Some(
                constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HELLO,
            ),
        },
    ),
    (
        LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHeartbeat,
        AdapterRowMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT,
            required_artifact_summary: Some(
                constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HEARTBEAT,
            ),
        },
    ),
];

const SIGNED_PROOF_METADATA: &[(LanSignedDiscoveryRelaySignedProofCheck, SignedProofMetadata)] = &[
    (
        LanSignedDiscoveryRelaySignedProofCheck::SignedHelloManualRequired,
        SignedProofMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO,
        },
    ),
    (
        LanSignedDiscoveryRelaySignedProofCheck::SignedHeartbeatManualRequired,
        SignedProofMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT,
        },
    ),
    (
        LanSignedDiscoveryRelaySignedProofCheck::AcceptedSignedChildAgentManualRequired,
        SignedProofMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HELLO,
        },
    ),
    (
        LanSignedDiscoveryRelaySignedProofCheck::UnauthenticatedCallerRejected,
        SignedProofMetadata {
            evidence_label: constants::value::LAN_REASON_ANONYMOUS,
        },
    ),
    (
        LanSignedDiscoveryRelaySignedProofCheck::ExpiredSignedProofRejected,
        SignedProofMetadata {
            evidence_label: constants::value::LAN_REASON_EXPIRED,
        },
    ),
    (
        LanSignedDiscoveryRelaySignedProofCheck::ReplayedSignedProofRejected,
        SignedProofMetadata {
            evidence_label: constants::value::LAN_REASON_REPLAYED,
        },
    ),
    (
        LanSignedDiscoveryRelaySignedProofCheck::WrongOriginSignedProofRejected,
        SignedProofMetadata {
            evidence_label: constants::value::LAN_REASON_WRONG_ORIGIN,
        },
    ),
    (
        LanSignedDiscoveryRelaySignedProofCheck::WrongDeviceSignedProofRejected,
        SignedProofMetadata {
            evidence_label: constants::value::LAN_REASON_WRONG_DEVICE,
        },
    ),
    (
        LanSignedDiscoveryRelaySignedProofCheck::RevokedSignedProofRejected,
        SignedProofMetadata {
            evidence_label: constants::value::LAN_REASON_REVOKED,
        },
    ),
    (
        LanSignedDiscoveryRelaySignedProofCheck::StaleSignedProofRejected,
        SignedProofMetadata {
            evidence_label: constants::value::LAN_REASON_STALE,
        },
    ),
];

const ROUTE_SAFETY_METADATA: &[(LanSignedDiscoveryRelayRouteSafetyCheck, RouteSafetyMetadata)] = &[
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::TrustedRegistryRestartRecovery,
        RouteSafetyMetadata {
            evidence_label: constants::lan_pairing::ROUTE_REQUIREMENT_ROUTE_RECOVERY_PERSISTED,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::SelectedRouteCustody,
        RouteSafetyMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTE_CUSTODY,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::StaleSelectedDeviceRejected,
        RouteSafetyMetadata {
            evidence_label: constants::value::LAN_REASON_STALE,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::OfflineSelectedDeviceRejected,
        RouteSafetyMetadata {
            evidence_label: constants::value::LAN_REASON_OFFLINE,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::WrongRouteRejected,
        RouteSafetyMetadata {
            evidence_label: constants::value::LAN_REASON_WRONG_DEVICE,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::RevokedRouteRejected,
        RouteSafetyMetadata {
            evidence_label: constants::value::LAN_REASON_REVOKED,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::ParentAssignDecisionAudited,
        RouteSafetyMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_ASSIGNMENT,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::ParentRenameDecisionAudited,
        RouteSafetyMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_RENAME,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::ParentIgnoreDecisionAudited,
        RouteSafetyMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_IGNORE,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::ParentRestoreDecisionAudited,
        RouteSafetyMetadata {
            evidence_label: constants::lan_pairing::HOUSEHOLD_ACTION_RESTORE,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::ParentTrustDecisionAudited,
        RouteSafetyMetadata {
            evidence_label: constants::lan_pairing::HOUSEHOLD_ACTION_TRUST,
        },
    ),
    (
        LanSignedDiscoveryRelayRouteSafetyCheck::ParentRevokeDecisionAudited,
        RouteSafetyMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_REVOCATION,
        },
    ),
];

const RELAY_CACHE_METADATA: &[(LanSignedDiscoveryRelayCacheCheck, RelayCacheMetadata)] = &[
    (
        LanSignedDiscoveryRelayCacheCheck::RelayRouteUnavailable,
        RelayCacheMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_RELAY_ROUTE,
        },
    ),
    (
        LanSignedDiscoveryRelayCacheCheck::RelayRouteQueuedNotConfigured,
        RelayCacheMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_RELAY_ROUTE,
        },
    ),
    (
        LanSignedDiscoveryRelayCacheCheck::CacheRouteUnavailable,
        RelayCacheMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_CACHE_ROUTE,
        },
    ),
    (
        LanSignedDiscoveryRelayCacheCheck::ParentOwnedStorageUnavailable,
        RelayCacheMetadata {
            evidence_label: constants::lan_pairing::SIGNED_DISCOVERY_RELAY_NON_CLAIM_PARENT_STORAGE,
        },
    ),
    (
        LanSignedDiscoveryRelayCacheCheck::OcentraChildDataCustodyNotClaimed,
        RelayCacheMetadata {
            evidence_label: constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_CLOUD,
        },
    ),
];

use ocentra_parent_agent_protocol::{
    constants, LanPairingProductionDiscoveryState, LanPairingRejectionReason,
    LanPairingResponseState, LanSignedDiscoveryRelayAdapterKind, LanSignedDiscoveryRelayAdapterRow,
    LanSignedDiscoveryRelayCacheCheck, LanSignedDiscoveryRelayCacheRow,
    LanSignedDiscoveryRelayCustodyLabel, LanSignedDiscoveryRelayDecisionState,
    LanSignedDiscoveryRelayRouteSafetyCheck, LanSignedDiscoveryRelayRouteSafetyRow,
    LanSignedDiscoveryRelaySignedProofCheck, LanSignedDiscoveryRelaySignedProofRow,
    LanSignedDiscoveryRelaySourceConfidence, LanSignedDiscoveryRelaySpineSummary,
    V09ProductionDiscoveryHouseholdProofState, V09ProductionDiscoveryHouseholdRuntimeOwner,
    LAN_PAIRING_SCHEMA_VERSION,
};

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
        value["adapterRows"][6]["custodyLabel"],
        serde_json::json!("signed-child-agent-artifact")
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
        value["routeSafetyRows"][4]["check"],
        serde_json::json!("wrong-route-rejected")
    );
    assert_eq!(
        value["routeSafetyRows"][5]["rejectionReason"],
        serde_json::json!("revoked")
    );
    assert_eq!(
        value["relayCacheRows"][4]["custodyLabel"],
        serde_json::json!("no-ocentra-child-data-custody")
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
            LanPairingProductionDiscoveryState::Stale,
            LanPairingRejectionReason::Stale,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::OfflineSelectedDeviceRejected,
            LanPairingProductionDiscoveryState::Offline,
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
            LanPairingProductionDiscoveryState::Paired,
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
    let custody_label = match &adapter {
        LanSignedDiscoveryRelayAdapterKind::RouterDhcpName => {
            LanSignedDiscoveryRelayCustodyLabel::RouterInfrastructureObservation
        }
        LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHello
        | LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHeartbeat => {
            LanSignedDiscoveryRelayCustodyLabel::SignedChildAgentArtifact
        }
        LanSignedDiscoveryRelayAdapterKind::ManualDirectAddress => {
            LanSignedDiscoveryRelayCustodyLabel::ManualParentEntry
        }
        _ => LanSignedDiscoveryRelayCustodyLabel::PassiveLanObservation,
    };
    adapter_row(
        adapter,
        LanPairingProductionDiscoveryState::ManualRequired,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        LanSignedDiscoveryRelaySourceConfidence::ManualRequired,
        custody_label,
        constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED,
        Some(constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED.to_string()),
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
    LanSignedDiscoveryRelaySignedProofRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        check,
        discovery_state,
        response_state,
        rejection_reason,
        proof_state,
        runtime_owner,
        evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO.to_string(),
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
        evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTE_CUSTODY.to_string(),
    }
}

fn relay_cache_row(
    check: LanSignedDiscoveryRelayCacheCheck,
    decision_state: LanSignedDiscoveryRelayDecisionState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    custody_label: LanSignedDiscoveryRelayCustodyLabel,
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
        evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_RELAY_ROUTE.to_string(),
    }
}

fn runtime_owner_for(
    proof_state: &V09ProductionDiscoveryHouseholdProofState,
) -> V09ProductionDiscoveryHouseholdRuntimeOwner {
    if *proof_state == V09ProductionDiscoveryHouseholdProofState::ManualRequired {
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof
    } else {
        V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel
    }
}

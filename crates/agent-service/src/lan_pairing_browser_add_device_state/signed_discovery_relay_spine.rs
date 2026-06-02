use ocentra_parent_agent_protocol::{
    constants, LanBrowserAddDeviceScanSummary, LanHouseholdDeviceActionKind,
    LanHouseholdDeviceDecision, LanPairingDeviceReachability, LanPairingProductionDiscoveryState,
    LanPairingRejectionReason, LanPairingResponseState, LanPairingTrustState,
    LanSelectedDeviceReadiness, LanSignedDiscoveryRelayAdapterKind,
    LanSignedDiscoveryRelayAdapterRow, LanSignedDiscoveryRelayCacheCheck,
    LanSignedDiscoveryRelayCacheRow, LanSignedDiscoveryRelayCustodyLabel,
    LanSignedDiscoveryRelayDecisionState, LanSignedDiscoveryRelayRouteSafetyCheck,
    LanSignedDiscoveryRelayRouteSafetyRow, LanSignedDiscoveryRelaySignedProofCheck,
    LanSignedDiscoveryRelaySignedProofRow, LanSignedDiscoveryRelaySourceConfidence,
    LanSignedDiscoveryRelaySpineSummary, LanTrustedDeviceRegistryEntry,
    V09ProductionDiscoveryHouseholdProofState, V09ProductionDiscoveryHouseholdRuntimeOwner,
};

pub(super) fn signed_discovery_relay_spine_summary(
    generated_at: &str,
    physical_household_lan_state: LanPairingProductionDiscoveryState,
    scan_summary: &LanBrowserAddDeviceScanSummary,
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> LanSignedDiscoveryRelaySpineSummary {
    LanSignedDiscoveryRelaySpineSummary {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        adapter_rows: adapter_rows(physical_household_lan_state, scan_summary),
        signed_proof_rows: signed_proof_rows(),
        route_safety_rows: route_safety_rows(
            trusted_device_registry,
            household_device_decisions,
            selected_device_readiness,
        ),
        relay_cache_rows: relay_cache_rows(),
        manual_proof_required: manual_proof_required(),
        not_implemented: not_implemented(),
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

fn adapter_rows(
    physical_household_lan_state: LanPairingProductionDiscoveryState,
    scan_summary: &LanBrowserAddDeviceScanSummary,
) -> Vec<LanSignedDiscoveryRelayAdapterRow> {
    vec![
        ci_adapter_row(
            LanSignedDiscoveryRelayAdapterKind::PassiveLanNeighbor,
            physical_household_lan_state,
            LanSignedDiscoveryRelaySourceConfidence::Strong,
            LanSignedDiscoveryRelayCustodyLabel::PassiveLanObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
        ),
        ci_adapter_row(
            LanSignedDiscoveryRelayAdapterKind::RouterInfrastructure,
            router_neighbor_state(scan_summary),
            LanSignedDiscoveryRelaySourceConfidence::Strong,
            LanSignedDiscoveryRelayCustodyLabel::RouterInfrastructureObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTER_NEIGHBOR,
        ),
        manual_adapter_row(
            LanSignedDiscoveryRelayAdapterKind::MdnsName,
            LanSignedDiscoveryRelayCustodyLabel::PassiveLanObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_MDNS,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_MDNS,
        ),
        manual_adapter_row(
            LanSignedDiscoveryRelayAdapterKind::SsdpName,
            LanSignedDiscoveryRelayCustodyLabel::PassiveLanObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SSDP,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SSDP,
        ),
        manual_adapter_row(
            LanSignedDiscoveryRelayAdapterKind::RouterDhcpName,
            LanSignedDiscoveryRelayCustodyLabel::RouterInfrastructureObservation,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTER_DHCP,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_ROUTER_DHCP,
        ),
        manual_adapter_row(
            LanSignedDiscoveryRelayAdapterKind::ManualDirectAddress,
            LanSignedDiscoveryRelayCustodyLabel::ManualParentEntry,
            constants::lan_pairing::MANUAL_PROOF_GAP_LAN_BIND,
            constants::lan_pairing::MANUAL_PROOF_GAP_LAN_BIND,
        ),
        manual_adapter_row(
            LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHello,
            LanSignedDiscoveryRelayCustodyLabel::SignedChildAgentArtifact,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HELLO,
        ),
        manual_adapter_row(
            LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHeartbeat,
            LanSignedDiscoveryRelayCustodyLabel::SignedChildAgentArtifact,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HEARTBEAT,
        ),
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

fn route_safety_rows(
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> Vec<LanSignedDiscoveryRelayRouteSafetyRow> {
    let route_id = selected_device_readiness
        .route_id
        .clone()
        .unwrap_or_else(|| constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string());
    vec![
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::TrustedRegistryRestartRecovery,
            Some(route_id.clone()),
            registry_state(trusted_device_registry),
            constants::lan_pairing::ROUTE_REQUIREMENT_ROUTE_RECOVERY_PERSISTED,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::SelectedRouteCustody,
            Some(route_id.clone()),
            route_custody_state(selected_device_readiness),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTE_CUSTODY,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::StaleSelectedDeviceRejected,
            Some(route_id.clone()),
            stale_selected_state(selected_device_readiness),
            LanPairingRejectionReason::Stale,
            constants::value::LAN_REASON_STALE,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::OfflineSelectedDeviceRejected,
            Some(route_id.clone()),
            offline_selected_state(selected_device_readiness),
            LanPairingRejectionReason::Offline,
            constants::value::LAN_REASON_OFFLINE,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::WrongRouteRejected,
            Some(route_id.clone()),
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::WrongDevice,
            constants::value::LAN_REASON_WRONG_DEVICE,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::RevokedRouteRejected,
            Some(route_id.clone()),
            LanPairingProductionDiscoveryState::Revoked,
            LanPairingRejectionReason::Revoked,
            constants::value::LAN_REASON_REVOKED,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentAssignDecisionAudited,
            Some(route_id.clone()),
            decision_state(household_device_decisions, LanHouseholdDeviceActionKind::Assign),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_ASSIGNMENT,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRenameDecisionAudited,
            Some(route_id.clone()),
            decision_state(household_device_decisions, LanHouseholdDeviceActionKind::Rename),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_RENAME,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentIgnoreDecisionAudited,
            Some(route_id.clone()),
            decision_state(household_device_decisions, LanHouseholdDeviceActionKind::Ignore),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_IGNORE,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRestoreDecisionAudited,
            Some(route_id.clone()),
            decision_state(household_device_decisions, LanHouseholdDeviceActionKind::Restore),
            constants::lan_pairing::HOUSEHOLD_ACTION_RESTORE,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentTrustDecisionAudited,
            Some(route_id.clone()),
            decision_state(household_device_decisions, LanHouseholdDeviceActionKind::Trust),
            constants::lan_pairing::HOUSEHOLD_ACTION_TRUST,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRevokeDecisionAudited,
            Some(route_id),
            revocation_state(trusted_device_registry),
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

fn manual_proof_required() -> Vec<LanSignedDiscoveryRelayAdapterKind> {
    vec![
        LanSignedDiscoveryRelayAdapterKind::MdnsName,
        LanSignedDiscoveryRelayAdapterKind::SsdpName,
        LanSignedDiscoveryRelayAdapterKind::RouterDhcpName,
        LanSignedDiscoveryRelayAdapterKind::ManualDirectAddress,
        LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHello,
        LanSignedDiscoveryRelayAdapterKind::SignedChildAgentHeartbeat,
    ]
}

fn not_implemented() -> Vec<LanSignedDiscoveryRelayCacheCheck> {
    vec![
        LanSignedDiscoveryRelayCacheCheck::RelayRouteUnavailable,
        LanSignedDiscoveryRelayCacheCheck::RelayRouteQueuedNotConfigured,
        LanSignedDiscoveryRelayCacheCheck::CacheRouteUnavailable,
        LanSignedDiscoveryRelayCacheCheck::ParentOwnedStorageUnavailable,
    ]
}

fn ci_adapter_row(
    adapter: LanSignedDiscoveryRelayAdapterKind,
    discovery_state: LanPairingProductionDiscoveryState,
    source_confidence: LanSignedDiscoveryRelaySourceConfidence,
    custody_label: LanSignedDiscoveryRelayCustodyLabel,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayAdapterRow {
    adapter_row(
        adapter,
        discovery_state,
        V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        source_confidence,
        custody_label,
        V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        evidence_label,
        None,
    )
}

fn manual_adapter_row(
    adapter: LanSignedDiscoveryRelayAdapterKind,
    custody_label: LanSignedDiscoveryRelayCustodyLabel,
    evidence_label: &str,
    required_artifact_summary: &str,
) -> LanSignedDiscoveryRelayAdapterRow {
    adapter_row(
        adapter,
        LanPairingProductionDiscoveryState::ManualRequired,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        LanSignedDiscoveryRelaySourceConfidence::ManualRequired,
        custody_label,
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
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
    runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    evidence_label: &str,
    required_artifact_summary: Option<String>,
) -> LanSignedDiscoveryRelayAdapterRow {
    LanSignedDiscoveryRelayAdapterRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
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
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
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
        V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        evidence_label,
    )
}

fn signed_proof_row(
    check: LanSignedDiscoveryRelaySignedProofCheck,
    discovery_state: LanPairingProductionDiscoveryState,
    response_state: LanPairingResponseState,
    rejection_reason: Option<LanPairingRejectionReason>,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    evidence_label: &str,
) -> LanSignedDiscoveryRelaySignedProofRow {
    LanSignedDiscoveryRelaySignedProofRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
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
    route_id: Option<String>,
    discovery_state: LanPairingProductionDiscoveryState,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    route_safety_row(
        check,
        route_id,
        discovery_state,
        LanPairingResponseState::Accepted,
        None,
        evidence_label,
    )
}

fn rejected_route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    route_id: Option<String>,
    discovery_state: LanPairingProductionDiscoveryState,
    rejection_reason: LanPairingRejectionReason,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    route_safety_row(
        check,
        route_id,
        discovery_state,
        LanPairingResponseState::Rejected,
        Some(rejection_reason),
        evidence_label,
    )
}

fn parent_decision_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    route_id: Option<String>,
    discovery_state: LanPairingProductionDiscoveryState,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    accepted_route_safety_row(check, route_id, discovery_state, evidence_label)
}

fn route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    route_id: Option<String>,
    discovery_state: LanPairingProductionDiscoveryState,
    response_state: LanPairingResponseState,
    rejection_reason: Option<LanPairingRejectionReason>,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    LanSignedDiscoveryRelayRouteSafetyRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        check,
        route_id,
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
    let runtime_owner = if proof_state == V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof
    {
        V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel
    } else {
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof
    };
    LanSignedDiscoveryRelayCacheRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        check,
        decision_state,
        discovery_state: LanPairingProductionDiscoveryState::Unavailable,
        proof_state,
        runtime_owner,
        custody_label,
        evidence_label: evidence_label.to_string(),
    }
}

fn router_neighbor_state(
    scan_summary: &LanBrowserAddDeviceScanSummary,
) -> LanPairingProductionDiscoveryState {
    if scan_summary.infrastructure_device_count > 0 {
        LanPairingProductionDiscoveryState::Discovered
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn registry_state(
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
) -> LanPairingProductionDiscoveryState {
    if trusted_device_registry.is_empty() {
        LanPairingProductionDiscoveryState::Pending
    } else {
        LanPairingProductionDiscoveryState::Paired
    }
}

fn decision_state(
    household_device_decisions: &[LanHouseholdDeviceDecision],
    action_kind: LanHouseholdDeviceActionKind,
) -> LanPairingProductionDiscoveryState {
    if household_device_decisions
        .iter()
        .any(|decision| decision.action_kind == action_kind && decision.revoked_at.is_none())
    {
        LanPairingProductionDiscoveryState::Discovered
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn revocation_state(
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
) -> LanPairingProductionDiscoveryState {
    if trusted_device_registry.iter().any(|entry| {
        entry.revoked_at.is_some() || entry.trust_state == LanPairingTrustState::Revoked
    }) {
        LanPairingProductionDiscoveryState::Revoked
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn route_custody_state(
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> LanPairingProductionDiscoveryState {
    if selected_device_readiness.ready_for_control {
        LanPairingProductionDiscoveryState::Paired
    } else if selected_device_readiness.route_id.is_some() {
        LanPairingProductionDiscoveryState::Pending
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn stale_selected_state(
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> LanPairingProductionDiscoveryState {
    if selected_device_readiness.reachability == LanPairingDeviceReachability::Stale {
        LanPairingProductionDiscoveryState::Stale
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn offline_selected_state(
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> LanPairingProductionDiscoveryState {
    if selected_device_readiness.reachability == LanPairingDeviceReachability::Offline {
        LanPairingProductionDiscoveryState::Offline
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

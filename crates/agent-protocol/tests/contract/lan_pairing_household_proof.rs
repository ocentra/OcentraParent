use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingRejectionReason,
    LanPairingTrustState, V09ProductionDiscoveryHouseholdCheck,
    V09ProductionDiscoveryHouseholdManualChecklistItem,
    V09ProductionDiscoveryHouseholdManualProofGate, V09ProductionDiscoveryHouseholdProofBoundary,
    V09ProductionDiscoveryHouseholdProofReadModel, V09ProductionDiscoveryHouseholdProofState,
    V09ProductionDiscoveryHouseholdReadinessDecision,
    V09ProductionDiscoveryHouseholdRouteRecoveryState, V09ProductionDiscoveryHouseholdRuntimeOwner,
    V09ProductionDiscoveryHouseholdSourceState, V09ProductionDiscoveryHouseholdStateEvidence,
};

#[test]
fn v09_production_discovery_household_proof_read_model_serializes_honest_route_states() {
    let serialized = serde_json::to_value(read_model()).unwrap_or_else(|error| {
        unreachable!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
    });
    let reparsed =
        serde_json::from_value::<V09ProductionDiscoveryHouseholdProofReadModel>(serialized.clone())
            .unwrap_or_else(|error| {
                unreachable!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
            });
    assert_eq!(
        serialized["proofBoundary"],
        "local-real-service-not-physical-household-lan"
    );
    assert_eq!(
        serialized["productReadinessDecision"],
        "not-ready-for-product-ready-household-lan-claim"
    );
    assert_eq!(
        serialized["routeChecks"][1]["sourceState"],
        "failed-unpaired"
    );
    assert_eq!(
        serialized["routeChecks"][2]["rejectionReason"],
        constants::value::LAN_REASON_WRONG_ORIGIN
    );
    assert_eq!(
        serialized["routeChecks"][3]["rejectionReason"],
        constants::value::LAN_REASON_WRONG_DEVICE
    );
    assert_eq!(
        serialized["restartRecovery"][0]["routeRecoveryState"],
        "registry-restored-after-restart"
    );
    assert_eq!(
        serialized["sourceDeviceStates"][3]["proofState"],
        "manual-required"
    );
    assert_eq!(reparsed.manual_household_proof_checklist.len(), 11);
    assert!(
        !json_surface_contains_marker(&serialized, constants::lan_pairing::RAW_MARKER_RAW_EVIDENCE)
    );
    assert!(!json_surface_contains_marker(
        &serialized,
        constants::lan_pairing::RAW_MARKER_RAW_TOKEN
    ));
    assert!(!json_surface_contains_marker(
        &serialized,
        constants::lan_pairing::RAW_MARKER_ACTIVITY_SQLITE
    ));
}

fn json_surface_contains_marker(value: &serde_json::Value, marker: &str) -> bool {
    match value {
        serde_json::Value::String(text) => text.contains(marker),
        serde_json::Value::Array(values) => values
            .iter()
            .any(|nested_value| json_surface_contains_marker(nested_value, marker)),
        serde_json::Value::Object(entries) => entries.iter().any(|(key, nested_value)| {
            key.contains(marker) || json_surface_contains_marker(nested_value, marker)
        }),
        _ => false,
    }
}

fn read_model() -> V09ProductionDiscoveryHouseholdProofReadModel {
    V09ProductionDiscoveryHouseholdProofReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
        checked_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        proof_boundary:
            V09ProductionDiscoveryHouseholdProofBoundary::LocalRealServiceNotPhysicalHouseholdLan,
        product_readiness_decision:
            V09ProductionDiscoveryHouseholdReadinessDecision::NotReadyForProductReadyHouseholdLanClaim,
        production_discovery_states: production_states(),
        route_checks: route_checks(),
        restart_recovery: restart_recovery(),
        source_device_states: source_device_states(),
        manual_household_proof_checklist: manual_checklist(),
        claims_proved: vec![constants::lan_pairing::ROUTE_REQUIREMENT_ROUTE_RECOVERY_PERSISTED
            .to_string()],
        claims_not_proved: vec![constants::lan_pairing::MANUAL_PROOF_GAP_PHYSICAL_DEVICE
            .to_string()],
    }
}

fn production_states() -> Vec<V09ProductionDiscoveryHouseholdStateEvidence> {
    vec![
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::ProductionDiscoveryStates,
            source_state: V09ProductionDiscoveryHouseholdSourceState::Discovered,
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::FailClosedUnpaired,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::AgentProtocol,
        }),
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::ProductionDiscoveryStates,
            source_state: V09ProductionDiscoveryHouseholdSourceState::Pending,
            discovery_state: LanPairingProductionDiscoveryState::Pending,
            trust_state: LanPairingTrustState::Pairing,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::FailClosedUnpaired,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::AgentProtocol,
        }),
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::ProductionDiscoveryStates,
            source_state: V09ProductionDiscoveryHouseholdSourceState::Paired,
            discovery_state: LanPairingProductionDiscoveryState::Paired,
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::SelectedRoutePersisted,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        }),
    ]
}

fn route_checks() -> Vec<V09ProductionDiscoveryHouseholdStateEvidence> {
    vec![
        route_check(
            V09ProductionDiscoveryHouseholdCheck::PairedRouteAccepted,
            None,
        ),
        route_check(
            V09ProductionDiscoveryHouseholdCheck::FailedUnpairedRejected,
            Some(LanPairingRejectionReason::Anonymous),
        ),
        route_check(
            V09ProductionDiscoveryHouseholdCheck::WrongOriginRejected,
            Some(LanPairingRejectionReason::WrongOrigin),
        ),
        route_check(
            V09ProductionDiscoveryHouseholdCheck::WrongDeviceRejected,
            Some(LanPairingRejectionReason::WrongDevice),
        ),
    ]
}

fn restart_recovery() -> Vec<V09ProductionDiscoveryHouseholdStateEvidence> {
    vec![
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::RestartSelectedRouteRecovered,
            source_state: V09ProductionDiscoveryHouseholdSourceState::RestartRecovered,
            discovery_state: LanPairingProductionDiscoveryState::Paired,
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::RegistryRestoredAfterRestart,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        }),
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::RestartRegistryStateRecovered,
            source_state: V09ProductionDiscoveryHouseholdSourceState::RestartRecovered,
            discovery_state: LanPairingProductionDiscoveryState::Paired,
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::SelectedRoutePersisted,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        }),
    ]
}

fn source_device_states() -> Vec<V09ProductionDiscoveryHouseholdStateEvidence> {
    vec![
        source_state(
            V09ProductionDiscoveryHouseholdCheck::StaleSourceRejected,
            V09ProductionDiscoveryHouseholdSourceState::Stale,
            LanPairingProductionDiscoveryState::Stale,
            LanPairingDeviceReachability::Stale,
            LanPairingRejectionReason::Stale,
        ),
        source_state(
            V09ProductionDiscoveryHouseholdCheck::OfflineDeviceRejected,
            V09ProductionDiscoveryHouseholdSourceState::Offline,
            LanPairingProductionDiscoveryState::Offline,
            LanPairingDeviceReachability::Offline,
            LanPairingRejectionReason::Offline,
        ),
        source_state(
            V09ProductionDiscoveryHouseholdCheck::RevokedPairingRejected,
            V09ProductionDiscoveryHouseholdSourceState::Revoked,
            LanPairingProductionDiscoveryState::Revoked,
            LanPairingDeviceReachability::Online,
            LanPairingRejectionReason::Revoked,
        ),
        manual_source_state(),
    ]
}

fn route_check(
    check: V09ProductionDiscoveryHouseholdCheck,
    rejection_reason: Option<LanPairingRejectionReason>,
) -> V09ProductionDiscoveryHouseholdStateEvidence {
    let source_state = match check {
        V09ProductionDiscoveryHouseholdCheck::PairedRouteAccepted => {
            V09ProductionDiscoveryHouseholdSourceState::Paired
        }
        V09ProductionDiscoveryHouseholdCheck::FailedUnpairedRejected => {
            V09ProductionDiscoveryHouseholdSourceState::FailedUnpaired
        }
        V09ProductionDiscoveryHouseholdCheck::WrongOriginRejected => {
            V09ProductionDiscoveryHouseholdSourceState::WrongOrigin
        }
        V09ProductionDiscoveryHouseholdCheck::WrongDeviceRejected => {
            V09ProductionDiscoveryHouseholdSourceState::WrongDevice
        }
        _ => V09ProductionDiscoveryHouseholdSourceState::Unavailable,
    };
    let trust_state = if rejection_reason.is_none() {
        LanPairingTrustState::Paired
    } else {
        LanPairingTrustState::Unpaired
    };
    household_evidence(HouseholdEvidenceCase {
        check,
        source_state,
        discovery_state: LanPairingProductionDiscoveryState::Unavailable,
        trust_state,
        reachability: LanPairingDeviceReachability::Online,
        rejection_reason,
        route_recovery_state: V09ProductionDiscoveryHouseholdRouteRecoveryState::FailClosedUnpaired,
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ProofHarness,
    })
}

fn source_state(
    check: V09ProductionDiscoveryHouseholdCheck,
    source_state: V09ProductionDiscoveryHouseholdSourceState,
    discovery_state: LanPairingProductionDiscoveryState,
    reachability: LanPairingDeviceReachability,
    rejection_reason: LanPairingRejectionReason,
) -> V09ProductionDiscoveryHouseholdStateEvidence {
    household_evidence(HouseholdEvidenceCase {
        check,
        source_state,
        discovery_state,
        trust_state: LanPairingTrustState::Paired,
        reachability,
        rejection_reason: Some(rejection_reason),
        route_recovery_state: V09ProductionDiscoveryHouseholdRouteRecoveryState::FailClosedUnpaired,
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
    })
}

fn manual_source_state() -> V09ProductionDiscoveryHouseholdStateEvidence {
    V09ProductionDiscoveryHouseholdStateEvidence {
        proof_state: V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        ..household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::ManualPhysicalHouseholdChecklist,
            source_state: V09ProductionDiscoveryHouseholdSourceState::ManualRequired,
            discovery_state: LanPairingProductionDiscoveryState::Unavailable,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            rejection_reason: Some(LanPairingRejectionReason::LocalNetworkDisabled),
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::ManualRequiredPhysicalRouteRecovery,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        })
    }
}

struct HouseholdEvidenceCase {
    check: V09ProductionDiscoveryHouseholdCheck,
    source_state: V09ProductionDiscoveryHouseholdSourceState,
    discovery_state: LanPairingProductionDiscoveryState,
    trust_state: LanPairingTrustState,
    reachability: LanPairingDeviceReachability,
    rejection_reason: Option<LanPairingRejectionReason>,
    route_recovery_state: V09ProductionDiscoveryHouseholdRouteRecoveryState,
    runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
}

fn household_evidence(case: HouseholdEvidenceCase) -> V09ProductionDiscoveryHouseholdStateEvidence {
    V09ProductionDiscoveryHouseholdStateEvidence {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
        check: case.check,
        source_state: case.source_state,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        discovery_state: case.discovery_state,
        trust_state: case.trust_state,
        reachability: case.reachability,
        rejection_reason: case.rejection_reason,
        route_recovery_state: case.route_recovery_state,
        proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        runtime_owner: case.runtime_owner,
        evidence_label: constants::lan_pairing::ROUTE_REQUIREMENT_DISCOVERY_STATE_EXPLICIT
            .to_string(),
    }
}

fn manual_checklist() -> Vec<V09ProductionDiscoveryHouseholdManualChecklistItem> {
    vec![
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::TwoPhysicalHosts),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::HouseholdRouterReachability),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::OsFirewallOrLocalNetworkPermission,
        ),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::AllowedOriginOnPhysicalController,
        ),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::PhysicalRouteSelectionAndTakeover,
        ),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::PhysicalRevocationAndRejection,
        ),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::PhysicalStaleOfflineSelectedDevice,
        ),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::RealMobileControllerPackage),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::RealMobileObserverPackage),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::RealLanAiProviderHost),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::CloudRelaySeparateProof),
    ]
}

fn household_gate(
    gate: V09ProductionDiscoveryHouseholdManualProofGate,
) -> V09ProductionDiscoveryHouseholdManualChecklistItem {
    V09ProductionDiscoveryHouseholdManualChecklistItem {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
        gate,
        state: V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        required_artifact_summary: constants::lan_pairing::MANUAL_PROOF_GAP_PHYSICAL_DEVICE
            .to_string(),
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
    }
}

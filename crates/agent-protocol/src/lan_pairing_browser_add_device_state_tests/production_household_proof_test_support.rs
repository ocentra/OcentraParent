use crate::{
    constants, LanBrowserAddDeviceReadModel, LanHouseholdDeviceActionKind,
    LanHouseholdDeviceDecision, LanPairingDeviceReachability, LanPairingDiscoverySource,
    LanPairingParentAuthority, LanPairingProductionDiscoveryState, LanPairingTrustState,
    LanProductionHouseholdProofCapability, LanProductionHouseholdProofStatus,
    LanProductionHouseholdProofSummary, LanSelectedDeviceReadiness,
    V09ProductionDiscoveryHouseholdProofState, V09ProductionDiscoveryHouseholdRuntimeOwner,
    LAN_PAIRING_SCHEMA_VERSION,
};

pub(super) fn browser_add_device_read_model_fixture() -> LanBrowserAddDeviceReadModel {
    LanBrowserAddDeviceReadModel {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        generated_at: "2026-06-01T15:20:00.000Z".to_string(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        add_device_state: LanPairingProductionDiscoveryState::Pending,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Pending,
        physical_household_lan_state: LanPairingProductionDiscoveryState::ManualRequired,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        scan_summary: super::scan_summary(),
        discovered_devices: Vec::new(),
        canonical_household_devices: vec![super::canonical_child_agent_device()],
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: vec![household_decision()],
        production_household_proof: Some(production_household_proof()),
        signed_discovery_relay_spine: Some(
            super::signed_discovery_relay_spine_test_support::signed_discovery_relay_spine_fixture(
            ),
        ),
        lan_discovery_source_matrix: Some(
            super::source_matrix_test_support::source_matrix_fixture(),
        ),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: selected_device_readiness_fixture(),
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
        route_requirement_labels: vec![
            constants::lan_pairing::ROUTE_REQUIREMENT_ALLOWED_ORIGIN.to_string()
        ],
        audit_check_labels: vec![
            constants::value::LAN_REASON_WRONG_ORIGIN.to_string(),
            constants::value::LAN_REASON_REPLAYED.to_string(),
        ],
        honest_non_claims: vec![
            constants::value::LAN_NON_CLAIM_CLOUD_RELAY_NOT_IMPLEMENTED.to_string()
        ],
    }
}

pub(super) fn assert_browser_add_device_read_model_json(value: &serde_json::Value) {
    assert_eq!(
        value[constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED)
    );
    assert_eq!(
        value[constants::field::LAN_CLOUD_RELAY_STATE],
        serde_json::json!(constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE)
    );
    assert_eq!(
        value["selectedDeviceReadiness"]["readyForControl"],
        serde_json::json!(false)
    );
    assert_eq!(
        value[constants::field::LAN_SCAN_SUMMARY][constants::field::SOURCE_LABELS],
        serde_json::json!([constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE])
    );
    assert_eq!(value["trustedDeviceRegistry"], serde_json::json!([]));
    assert_eq!(
        value["householdDeviceDecisions"][0]["actionKind"],
        serde_json::json!("rename")
    );
    assert_eq!(
        value["productionHouseholdProof"]["manualProofRequired"][0],
        serde_json::json!("signed-lan-hello")
    );
    assert_eq!(
        value["productionHouseholdProof"]["notImplemented"],
        serde_json::json!(["relay-route", "cache-route"])
    );
    super::signed_discovery_relay_spine_test_support::assert_signed_discovery_relay_spine_json(
        &value["signedDiscoveryRelaySpine"],
    );
    super::source_matrix_test_support::assert_source_matrix_json(
        &value[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SUMMARY],
    );
    assert_eq!(
        value["canonicalHouseholdDevices"][0]["policyTargetSurfaces"],
        serde_json::json!([
            "devices", "policy", "browser", "app", "screen", "network", "activity", "tracking",
            "ai"
        ])
    );
    assert_eq!(
        value["canonicalHouseholdDevices"][0]["networkIdentity"]["evidenceRecords"][0]["source"],
        serde_json::json!("local-service")
    );
}

fn selected_device_readiness_fixture() -> LanSelectedDeviceReadiness {
    LanSelectedDeviceReadiness {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        selected_child_device_id: None,
        route_id: None,
        pairing_id: None,
        trust_state: LanPairingTrustState::Unpaired,
        reachability: LanPairingDeviceReachability::Offline,
        ready_for_control: false,
        stale_at: None,
        offline_at: None,
    }
}

fn household_decision() -> LanHouseholdDeviceDecision {
    LanHouseholdDeviceDecision {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        action_id: "lan-action-rename-1".to_string(),
        action_kind: LanHouseholdDeviceActionKind::Rename,
        canonical_device_id: "lan-physical-mac-54271e97c331".to_string(),
        child_profile_id: None,
        display_name: Some("GAMEDEV Study PC".to_string()),
        parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
        decided_at: "2026-06-01T15:20:00.000Z".to_string(),
        revoked_at: None,
    }
}

fn production_household_proof() -> LanProductionHouseholdProofSummary {
    LanProductionHouseholdProofSummary {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        generated_at: "2026-06-01T15:20:00.000Z".to_string(),
        status_rows: production_status_rows(),
        manual_proof_required: vec![
            LanProductionHouseholdProofCapability::SignedLanHello,
            LanProductionHouseholdProofCapability::SignedLanHeartbeat,
            LanProductionHouseholdProofCapability::MdnsNameDiscovery,
            LanProductionHouseholdProofCapability::SsdpNameDiscovery,
            LanProductionHouseholdProofCapability::RouterDhcpNameDiscovery,
            LanProductionHouseholdProofCapability::SecondPhysicalChildAgent,
            LanProductionHouseholdProofCapability::AndroidChildAgentParity,
            LanProductionHouseholdProofCapability::IosChildAgentParity,
            LanProductionHouseholdProofCapability::StoreSigning,
        ],
        not_implemented: vec![
            LanProductionHouseholdProofCapability::RelayRoute,
            LanProductionHouseholdProofCapability::CacheRoute,
        ],
        claims_proved: vec![
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_PASSIVE_NEIGHBOR.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_REGISTRY_ROUTE.to_string(),
        ],
        claims_not_proved: vec![
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_PHYSICAL.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_CLOUD.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_ANDROID.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_IOS.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_STORE.to_string(),
        ],
    }
}

fn production_status_rows() -> Vec<LanProductionHouseholdProofStatus> {
    [
        production_discovery_status_rows(),
        production_decision_status_rows(),
        production_route_status_rows(),
        production_manual_platform_status_rows(),
    ]
    .concat()
}

fn production_discovery_status_rows() -> Vec<LanProductionHouseholdProofStatus> {
    vec![
        manual_production_status(
            LanProductionHouseholdProofCapability::SignedLanHello,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HELLO,
        ),
        manual_production_status(
            LanProductionHouseholdProofCapability::SignedLanHeartbeat,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HEARTBEAT,
        ),
        ci_production_status(
            LanProductionHouseholdProofCapability::PassiveNeighborDiscovery,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        ci_production_status(
            LanProductionHouseholdProofCapability::RouterNeighborDiscovery,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        manual_production_status(
            LanProductionHouseholdProofCapability::MdnsNameDiscovery,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_MDNS,
        ),
        manual_production_status(
            LanProductionHouseholdProofCapability::SsdpNameDiscovery,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SSDP,
        ),
        manual_production_status(
            LanProductionHouseholdProofCapability::RouterDhcpNameDiscovery,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_ROUTER_DHCP,
        ),
        ci_production_status(
            LanProductionHouseholdProofCapability::TrustedRegistry,
            LanPairingProductionDiscoveryState::Paired,
        ),
    ]
}

fn production_decision_status_rows() -> Vec<LanProductionHouseholdProofStatus> {
    vec![
        ci_production_status(
            LanProductionHouseholdProofCapability::ParentAssignment,
            LanPairingProductionDiscoveryState::ManualRequired,
        ),
        ci_production_status(
            LanProductionHouseholdProofCapability::ParentRename,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        ci_production_status(
            LanProductionHouseholdProofCapability::ParentIgnore,
            LanPairingProductionDiscoveryState::ManualRequired,
        ),
        ci_production_status(
            LanProductionHouseholdProofCapability::ParentRevocation,
            LanPairingProductionDiscoveryState::ManualRequired,
        ),
    ]
}

fn production_route_status_rows() -> Vec<LanProductionHouseholdProofStatus> {
    vec![
        ci_production_status(
            LanProductionHouseholdProofCapability::RouteCustody,
            LanPairingProductionDiscoveryState::Paired,
        ),
        ci_production_status(
            LanProductionHouseholdProofCapability::StaleSelectedDevice,
            LanPairingProductionDiscoveryState::ManualRequired,
        ),
        ci_production_status(
            LanProductionHouseholdProofCapability::OfflineSelectedDevice,
            LanPairingProductionDiscoveryState::Offline,
        ),
        not_implemented_status(LanProductionHouseholdProofCapability::RelayRoute),
        not_implemented_status(LanProductionHouseholdProofCapability::CacheRoute),
    ]
}

fn production_manual_platform_status_rows() -> Vec<LanProductionHouseholdProofStatus> {
    vec![
        manual_production_status(
            LanProductionHouseholdProofCapability::SecondPhysicalChildAgent,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SECOND_PHYSICAL_AGENT,
        ),
        manual_production_status(
            LanProductionHouseholdProofCapability::AndroidChildAgentParity,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_ANDROID_PARITY,
        ),
        manual_production_status(
            LanProductionHouseholdProofCapability::IosChildAgentParity,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_IOS_PARITY,
        ),
        manual_production_status(
            LanProductionHouseholdProofCapability::StoreSigning,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_STORE_SIGNING,
        ),
    ]
}

fn manual_production_status(
    capability: LanProductionHouseholdProofCapability,
    required_artifact_summary: &str,
) -> LanProductionHouseholdProofStatus {
    production_status(
        capability,
        LanPairingProductionDiscoveryState::ManualRequired,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        required_artifact_summary,
    )
}

fn ci_production_status(
    capability: LanProductionHouseholdProofCapability,
    discovery_state: LanPairingProductionDiscoveryState,
) -> LanProductionHouseholdProofStatus {
    production_status(
        capability,
        discovery_state,
        V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
    )
}

fn not_implemented_status(
    capability: LanProductionHouseholdProofCapability,
) -> LanProductionHouseholdProofStatus {
    production_status(
        capability,
        LanPairingProductionDiscoveryState::Unavailable,
        V09ProductionDiscoveryHouseholdProofState::NotImplemented,
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        constants::lan_pairing::PRODUCTION_PROOF_LABEL_RELAY_ROUTE,
    )
}

fn production_status(
    capability: LanProductionHouseholdProofCapability,
    discovery_state: LanPairingProductionDiscoveryState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    required_artifact_summary: &str,
) -> LanProductionHouseholdProofStatus {
    LanProductionHouseholdProofStatus {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        capability,
        discovery_state,
        proof_state,
        runtime_owner,
        evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR.to_string(),
        required_artifact_summary: Some(required_artifact_summary.to_string()),
    }
}

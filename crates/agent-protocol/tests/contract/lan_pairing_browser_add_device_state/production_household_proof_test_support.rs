use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistory, LanDiscoveryEventHistoryState, LanDiscoveryEventKind,
    LanDiscoveryEventRow,
};
use ocentra_parent_agent_protocol::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::LanHouseholdDeviceActionKind;
use ocentra_parent_agent_protocol::LanHouseholdDeviceDecision;
use ocentra_parent_agent_protocol::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::LanPairingDiscoverySource;
use ocentra_parent_agent_protocol::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::LanPairingTrustState;
use ocentra_parent_agent_protocol::LanProductionHouseholdProofCapability;
use ocentra_parent_agent_protocol::LanProductionHouseholdProofStatus;
use ocentra_parent_agent_protocol::LanProductionHouseholdProofSummary;
use ocentra_parent_agent_protocol::LanSelectedDeviceReadiness;
use ocentra_parent_agent_protocol::V09ProductionDiscoveryHouseholdProofState;
use ocentra_parent_agent_protocol::V09ProductionDiscoveryHouseholdRuntimeOwner;
use ocentra_parent_agent_protocol::LAN_PAIRING_SCHEMA_VERSION;

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
        discovery_event_history: discovery_event_history_fixture(),
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
    assert_eq!(
        value["discoveryEventHistory"]["state"],
        serde_json::json!("ready")
    );
    assert_eq!(
        value["discoveryEventHistory"]["latestEventId"],
        serde_json::json!("lan-discovery-scan-finished-lan-scan-1717255200000")
    );
    assert_eq!(
        value["discoveryEventHistory"]["rows"][0]["eventKind"],
        serde_json::json!("scan-started")
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
    assert_production_household_proof_json(&value["productionHouseholdProof"]);
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
        value["canonicalHouseholdDevices"]
            .as_array()
            .expect_value("canonical household devices serializes as an array")
            .len(),
        1
    );
    assert!(
        !value["canonicalHouseholdDevices"][0]["networkIdentity"]["evidenceRecords"]
            .as_array()
            .expect_value("network identity evidence records serializes as an array")
            .is_empty()
    );
    assert_eq!(
        value["canonicalHouseholdDevices"][0]["networkIdentity"]["evidenceRecords"][0]["source"],
        serde_json::json!("local-service")
    );
}

fn assert_production_household_proof_json(value: &serde_json::Value) {
    let status_rows = value["statusRows"]
        .as_array()
        .expect_value("production household proof status rows serialize as an array");
    let manual_proof_required = value["manualProofRequired"]
        .as_array()
        .expect_value("production household manual proof list serializes as an array");
    let not_implemented = value["notImplemented"]
        .as_array()
        .expect_value("production household not-implemented list serializes as an array");
    let claims_not_proved = value["claimsNotProved"]
        .as_array()
        .expect_value("production household non-claims serialize as an array");

    for capability in [
        "signed-lan-hello",
        "signed-lan-heartbeat",
        "passive-neighbor-discovery",
        "router-neighbor-discovery",
        "mdns-name-discovery",
        "ssdp-name-discovery",
        "router-dhcp-name-discovery",
        "trusted-registry",
        "parent-assignment",
        "parent-rename",
        "parent-ignore",
        "parent-revocation",
        "route-custody",
        "stale-selected-device",
        "offline-selected-device",
        "relay-route",
        "cache-route",
        "second-physical-child-agent",
        "android-child-agent-parity",
        "ios-child-agent-parity",
        "store-signing",
    ] {
        assert!(
            status_rows
                .iter()
                .any(|row| row["capability"] == serde_json::json!(capability)),
            "missing production LAN proof capability row: {capability}"
        );
    }

    for capability in [
        "signed-lan-hello",
        "signed-lan-heartbeat",
        "mdns-name-discovery",
        "ssdp-name-discovery",
        "router-dhcp-name-discovery",
        "second-physical-child-agent",
        "android-child-agent-parity",
        "ios-child-agent-parity",
        "store-signing",
    ] {
        assert!(
            manual_proof_required
                .iter()
                .any(|entry| entry == capability)
                && status_rows.iter().any(|row| {
                    row["capability"] == serde_json::json!(capability)
                        && row["proofState"] == serde_json::json!("manual-required")
                }),
            "manual LAN proof capability must be listed and statused manual-required: {capability}"
        );
    }

    for capability in ["relay-route", "cache-route"] {
        assert!(
            not_implemented.iter().any(|entry| entry == capability)
                && status_rows.iter().any(|row| {
                    row["capability"] == serde_json::json!(capability)
                        && row["proofState"] == serde_json::json!("not-implemented")
                }),
            "not-implemented LAN proof capability must be listed and statused not-implemented: {capability}"
        );
    }

    for (capability, proof_state) in [
        ("passive-neighbor-discovery", "ci-mechanical-proof"),
        ("route-custody", "ci-mechanical-proof"),
    ] {
        assert!(
            status_rows.iter().any(|row| {
                row["capability"] == serde_json::json!(capability)
                    && row["proofState"] == serde_json::json!(proof_state)
            }),
            "production LAN proof capability must retain Rust-owned proof state: {capability}"
        );
    }

    assert_eq!(
        claims_not_proved,
        &[
            serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_PHYSICAL),
            serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED),
            serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_CLOUD),
            serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_ANDROID),
            serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_IOS),
            serde_json::json!(constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_STORE),
        ]
    );
}

fn discovery_event_history_fixture() -> LanDiscoveryEventHistory {
    LanDiscoveryEventHistory {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        generated_at: "2026-06-01T15:20:00.000Z".to_string(),
        state: LanDiscoveryEventHistoryState::Ready,
        latest_event_id: Some("lan-discovery-scan-finished-lan-scan-1717255200000".to_string()),
        latest_observed_at: Some("2026-06-01T15:20:00.000Z".to_string()),
        rows: vec![
            LanDiscoveryEventRow {
                schema_version: LAN_PAIRING_SCHEMA_VERSION,
                event_id: "lan-discovery-scan-started-lan-scan-1717255200000".to_string(),
                event_kind: LanDiscoveryEventKind::ScanStarted,
                occurred_at: "2026-06-01T15:19:58.000Z".to_string(),
                previous_event_id: None,
                scan_session_id: Some("lan-scan-1717255200000".to_string()),
                affected_device_id: None,
                evidence_id: None,
                summary: "LAN scan started".to_string(),
            },
            LanDiscoveryEventRow {
                schema_version: LAN_PAIRING_SCHEMA_VERSION,
                event_id: "lan-discovery-scan-finished-lan-scan-1717255200000".to_string(),
                event_kind: LanDiscoveryEventKind::ScanFinished,
                occurred_at: "2026-06-01T15:20:00.000Z".to_string(),
                previous_event_id: Some(
                    "lan-discovery-scan-started-lan-scan-1717255200000".to_string(),
                ),
                scan_session_id: Some("lan-scan-1717255200000".to_string()),
                affected_device_id: None,
                evidence_id: None,
                summary: "LAN scan finished with 1 visible devices".to_string(),
            },
        ],
    }
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
        device_kind: Some(constants::lan_pairing::HOUSEHOLD_DEVICE_KIND_DESKTOP.to_string()),
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
        manual_production_status(LanProductionHouseholdProofCapability::SignedLanHello),
        manual_production_status(LanProductionHouseholdProofCapability::SignedLanHeartbeat),
        ci_production_status(
            LanProductionHouseholdProofCapability::PassiveNeighborDiscovery,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        ci_production_status(
            LanProductionHouseholdProofCapability::RouterNeighborDiscovery,
            LanPairingProductionDiscoveryState::Discovered,
        ),
        manual_production_status(LanProductionHouseholdProofCapability::MdnsNameDiscovery),
        manual_production_status(LanProductionHouseholdProofCapability::SsdpNameDiscovery),
        manual_production_status(LanProductionHouseholdProofCapability::RouterDhcpNameDiscovery),
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
        manual_production_status(LanProductionHouseholdProofCapability::SecondPhysicalChildAgent),
        manual_production_status(LanProductionHouseholdProofCapability::AndroidChildAgentParity),
        manual_production_status(LanProductionHouseholdProofCapability::IosChildAgentParity),
        manual_production_status(LanProductionHouseholdProofCapability::StoreSigning),
    ]
}

fn manual_production_status(
    capability: LanProductionHouseholdProofCapability,
) -> LanProductionHouseholdProofStatus {
    production_status(
        capability,
        LanPairingProductionDiscoveryState::ManualRequired,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
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
    )
}

fn production_status(
    capability: LanProductionHouseholdProofCapability,
    discovery_state: LanPairingProductionDiscoveryState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
) -> LanProductionHouseholdProofStatus {
    let required_artifact_summary = match (&capability, &proof_state) {
        (
            LanProductionHouseholdProofCapability::SecondPhysicalChildAgent,
            V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        ) => constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SECOND_PHYSICAL_AGENT,
        (
            LanProductionHouseholdProofCapability::AndroidChildAgentParity,
            V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        ) => constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_ANDROID_PARITY,
        (
            LanProductionHouseholdProofCapability::IosChildAgentParity,
            V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        ) => constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_IOS_PARITY,
        (
            LanProductionHouseholdProofCapability::StoreSigning,
            V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        ) => constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_STORE_SIGNING,
        (_, V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof) => {
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR
        }
        (_, V09ProductionDiscoveryHouseholdProofState::NotImplemented) => {
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_RELAY_ROUTE
        }
        _ => constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
    };
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

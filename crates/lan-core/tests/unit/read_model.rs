use crate::support::OptionTestExt as _;
use ocentra_lan_core::network_inventory::api::discovered_devices_from_network_inventory;
use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
use ocentra_lan_core::read_model::{
    lan_add_device_read_model_from_inventory,
    lan_add_device_read_model_from_inventory_with_platform_data,
    platform_data_available_for_identity_with_manual_required_override,
};
use ocentra_lan_core::read_model_builder::{build_lan_add_device_read_model, LanAddDeviceReadModelInput};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingProductionDiscoveryState,
    LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::{
    LanDiscoverySourceAuthority, LanDiscoverySourceKind, LanDiscoverySourceRuntimePath,
    LanDiscoverySourceStatus, LanPlanWorkpackId,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::signed_discovery_relay_spine::{
    LanSignedDiscoveryRelayRouteSafetyCheck, LanSignedDiscoveryRelaySignedProofCheck,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistoryState, LanDiscoveryEventKind, LanHouseholdDeviceActionKind,
    LanHouseholdDeviceDecision, LanPairingDiscoverySource, LanSelectedDeviceReadiness,
};
use ocentra_parent_agent_protocol::LanTrustedDeviceRegistryEntry;

#[path = "../property-based/lan_plan.rs"]
mod lan_plan_property;
#[path = "read_model_relay.rs"]
mod read_model_relay;
#[path = "read_model_source_matrix.rs"]
mod read_model_source_matrix;
#[path = "read_model_spine.rs"]
mod read_model_spine;

fn neighbor(
    label: impl std::fmt::Display,
    hostname: Option<impl std::fmt::Display>,
    reachability: LanPairingDeviceReachability,
) -> LanNetworkInventoryDevice {
    neighbor_with_mac(
        label,
        hostname,
        reachability,
        constants::lan_pairing::TEST_LAN_MAC,
    )
}

fn neighbor_with_mac(
    label: impl std::fmt::Display,
    hostname: Option<impl std::fmt::Display>,
    reachability: LanPairingDeviceReachability,
    mac_address: impl std::fmt::Display,
) -> LanNetworkInventoryDevice {
    let label = label.to_string();
    LanNetworkInventoryDevice {
        device_id: format!("network-neighbor-{label}"),
        label,
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: mac_address.to_string(),
        hostname: hostname.map(|value| value.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }
}

#[test]
fn empty_inventory_stays_honest_about_manual_required_lan_state() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(
        model.physical_household_lan_state,
        LanPairingProductionDiscoveryState::ManualRequired
    );
    assert_eq!(
        model.discovery_event_history.state,
        LanDiscoveryEventHistoryState::ManualRequired
    );
    assert!(model.canonical_household_devices.is_empty());
    let production_household_proof = model.production_household_proof.value_or_unreachable();
    assert_eq!(
        production_household_proof.schema_version,
        constants::lan_pairing::SCHEMA_VERSION
    );
    assert_eq!(
        production_household_proof.generated_at,
        "2026-06-23T00:00:00Z"
    );
    assert_eq!(
        production_household_proof.claims_proved,
        vec![
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_PASSIVE_NEIGHBOR.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_REGISTRY_ROUTE.to_string(),
        ]
    );
    assert_eq!(
        production_household_proof.claims_not_proved,
        vec![
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_PHYSICAL.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_CLOUD.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_ANDROID.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_IOS.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_STORE.to_string(),
        ]
    );
    let signed_discovery_relay_spine = model.signed_discovery_relay_spine.value_or_unreachable();
    assert_eq!(
        signed_discovery_relay_spine.schema_version,
        constants::lan_pairing::SCHEMA_VERSION
    );
    assert_eq!(
        signed_discovery_relay_spine.generated_at,
        "2026-06-23T00:00:00Z"
    );
    assert_eq!(signed_discovery_relay_spine.adapter_rows.len(), 8);
    assert_eq!(signed_discovery_relay_spine.signed_proof_rows.len(), 10);
    assert_eq!(
        signed_discovery_relay_spine.claims_proved,
        vec![
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_PASSIVE_NEIGHBOR.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_REGISTRY_ROUTE.to_string(),
        ]
    );
    assert_eq!(
        signed_discovery_relay_spine.claims_not_proved,
        vec![
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_PHYSICAL.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_CLOUD.to_string(),
            constants::lan_pairing::SIGNED_DISCOVERY_RELAY_NON_CLAIM_PARENT_STORAGE.to_string(),
        ]
    );
    let lan_discovery_source_matrix = model.lan_discovery_source_matrix.value_or_unreachable();
    assert_eq!(
        lan_discovery_source_matrix.schema_version,
        constants::lan_pairing::SCHEMA_VERSION
    );
    assert_eq!(
        lan_discovery_source_matrix.generated_at,
        "2026-06-23T00:00:00Z"
    );
    assert_eq!(lan_discovery_source_matrix.workpack_rows.len(), 25);
    assert_eq!(lan_discovery_source_matrix.source_rows.len(), 35);
    assert_eq!(
        lan_discovery_source_matrix.claims_proved,
        vec![
            constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_READ_MODEL.to_string(),
            constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_WEAK_SOURCES.to_string(),
        ]
    );
    assert_eq!(
        lan_discovery_source_matrix.claims_not_proved,
        vec![
            constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PACKET_MODE.to_string(),
            constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PHYSICAL.to_string(),
            constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_MDNS_SSDP.to_string(),
        ]
    );
    assert!(model
        .honest_non_claims
        .iter()
        .any(|claim| claim == constants::value::LAN_NON_CLAIM_REMOTE_DESKTOP_NOT_IMPLEMENTED));
}

#[test]
fn unavailable_platform_data_is_reported_as_unavailable_not_ready() {
    let model = lan_add_device_read_model_from_inventory_with_platform_data(
        &[],
        "2026-06-23T00:00:00Z".to_string(),
        false,
    );

    assert_eq!(
        model.add_device_state,
        LanPairingProductionDiscoveryState::Unavailable
    );
    assert_eq!(
        model.local_service_discovery_state,
        LanPairingProductionDiscoveryState::Unavailable
    );
    assert_eq!(
        model.physical_household_lan_state,
        LanPairingProductionDiscoveryState::Unavailable
    );
    assert_eq!(
        model.discovery_event_history.state,
        LanDiscoveryEventHistoryState::Unavailable
    );
}

#[test]
fn discovery_event_history_uses_canonical_registry_and_decision_timestamps() {
    let canonical_device_id = "lan-child-profile-childprofile1".to_string();
    let mut child_device = LanPairingDeviceRef::new(
        "trusted-child-1".to_string(),
        Some("child-profile-1".to_string()),
        "Study Laptop".to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    child_device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    child_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
    child_device.hostname = Some("study-laptop".to_string());
    child_device.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T10:30:00Z".to_string(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: Vec::new(),
        pairing_requests: Vec::new(),
        trusted_device_registry: vec![LanTrustedDeviceRegistryEntry {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            pairing_id: "pairing-child-profile-1".to_string(),
            child_device,
            parent_device: LanPairingDeviceRef::new(
                constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
                None,
                "Parent".to_string(),
                constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ),
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            origin: "test-trusted-registry".to_string(),
            proof_digest: "sha256:test-proof".to_string(),
            trust_state: LanPairingTrustState::Paired,
            trusted_at: "2026-06-26T09:59:00Z".to_string(),
            expires_at: "2026-06-27T09:59:00Z".to_string(),
            revoked_at: None,
        }],
        household_device_decisions: vec![LanHouseholdDeviceDecision {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            action_id: "household-action-trust-1".to_string(),
            action_kind: LanHouseholdDeviceActionKind::Trust,
            canonical_device_id: canonical_device_id.clone(),
            child_profile_id: Some("child-profile-1".to_string()),
            display_name: Some("Study Laptop".to_string()),
            device_kind: None,
            parent_actor_id: "parent-1".to_string(),
            decided_at: "2026-06-26T10:01:00Z".to_string(),
            revoked_at: None,
        }],
        trusted_device_ids: vec!["trusted-child-1".to_string()],
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    });

    let history = &model.discovery_event_history;
    let model_canonical_device_id = model
        .canonical_household_devices
        .first()
        .value_or_unreachable()
        .canonical_device_id
        .clone();

    assert_eq!(history.state, LanDiscoveryEventHistoryState::Ready);
    assert_eq!(model_canonical_device_id, canonical_device_id);
    assert_eq!(
        history.rows.first().map(|row| row.occurred_at.as_str()),
        Some("2026-06-26T10:01:00Z")
    );
    assert_eq!(
        history.rows.last().map(|row| row.occurred_at.as_str()),
        Some("2026-06-26T10:30:00Z")
    );
    assert!(history
        .rows
        .iter()
        .all(|row| row.scan_session_id.as_deref() == Some("lan-scan-20260626t103000z")));
    assert!(history.rows.iter().any(|row| {
        row.event_kind == LanDiscoveryEventKind::EvidenceFound
            && row.occurred_at == "2026-06-26T10:01:00Z"
            && row.affected_device_id.as_deref() == Some(model_canonical_device_id.as_str())
            && row.evidence_id.is_some()
    }));
    assert!(history.rows.iter().any(|row| {
        row.event_kind == LanDiscoveryEventKind::AgentConfirmed
            && row.affected_device_id.as_deref() == Some(model_canonical_device_id.as_str())
            && row.scan_session_id.as_deref() == Some("lan-scan-20260626t103000z")
    }));
}

#[test]
fn discovery_event_history_marks_selected_paired_online_child_without_route_as_agent_offline() {
    let canonical_device_id = "lan-child-profile-childprofile1".to_string();
    let mut child_device = LanPairingDeviceRef::new(
        "trusted-child-1".to_string(),
        Some("child-profile-1".to_string()),
        "Study Laptop".to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    child_device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    child_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
    child_device.hostname = Some("study-laptop".to_string());
    child_device.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T10:30:00Z".to_string(),
        discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: Vec::new(),
        pairing_requests: Vec::new(),
        trusted_device_registry: vec![LanTrustedDeviceRegistryEntry {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            pairing_id: "pairing-child-profile-1".to_string(),
            child_device,
            parent_device: LanPairingDeviceRef::new(
                constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
                None,
                "Parent".to_string(),
                constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ),
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            origin: "test-trusted-registry".to_string(),
            proof_digest: "sha256:test-proof".to_string(),
            trust_state: LanPairingTrustState::Paired,
            trusted_at: "2026-06-26T09:59:00Z".to_string(),
            expires_at: "2026-06-27T09:59:00Z".to_string(),
            revoked_at: None,
        }],
        household_device_decisions: vec![LanHouseholdDeviceDecision {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            action_id: "household-action-trust-1".to_string(),
            action_kind: LanHouseholdDeviceActionKind::Trust,
            canonical_device_id,
            child_profile_id: Some("child-profile-1".to_string()),
            display_name: Some("Study Laptop".to_string()),
            device_kind: None,
            parent_actor_id: "parent-1".to_string(),
            decided_at: "2026-06-26T10:01:00Z".to_string(),
            revoked_at: None,
        }],
        trusted_device_ids: vec!["trusted-child-1".to_string()],
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: Some("trusted-child-1".to_string()),
            route_id: None,
            pairing_id: Some("pairing-child-profile-1".to_string()),
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Online,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    });

    assert_eq!(
        model.discovery_event_history.state,
        LanDiscoveryEventHistoryState::AgentOffline
    );
    assert_eq!(
        model
            .discovery_event_history
            .rows
            .iter()
            .filter(|row| row.event_kind == LanDiscoveryEventKind::AgentConfirmed)
            .count(),
        1
    );
    assert!(model.discovery_event_history.rows.iter().any(|row| {
        row.event_kind == LanDiscoveryEventKind::AgentConfirmed && row.affected_device_id.is_some()
    }));
}

#[test]
fn unknown_neighbor_history_emits_unknown_detected_without_agent_confirmation() {
    let model = lan_add_device_read_model_from_inventory(
        &[neighbor(
            "mystery-box",
            Some("mystery-box.local"),
            LanPairingDeviceReachability::Online,
        )],
        "2026-06-23T00:00:00Z".to_string(),
    );

    let canonical = &model.canonical_household_devices[0];
    let history = &model.discovery_event_history;

    assert_eq!(
        canonical.classification,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::UnknownLanDevice
    );
    assert!(history.rows.iter().any(|row| {
        row.event_kind == LanDiscoveryEventKind::UnknownDetected
            && row.affected_device_id.as_deref() == Some(canonical.canonical_device_id.as_str())
            && row.scan_session_id.as_deref() == Some("lan-scan-20260623t000000z")
    }));
    assert!(history.rows.iter().any(|row| {
        row.event_kind == LanDiscoveryEventKind::EvidenceFound
            && row.affected_device_id.as_deref() == Some(canonical.canonical_device_id.as_str())
    }));
    assert!(!history
        .rows
        .iter()
        .any(|row| row.event_kind == LanDiscoveryEventKind::AgentConfirmed));
    assert!(history.rows.iter().all(|row| !row.event_id.is_empty()));
    assert!(history.rows.iter().all(|row| !row.occurred_at.is_empty()));
}

#[test]
fn apple_manual_required_platform_keeps_manual_state_available_for_projection() {
    assert!(platform_data_available_for_identity_with_manual_required_override(false, true,));
    assert!(!platform_data_available_for_identity_with_manual_required_override(false, false,));
}

#[test]
fn inventory_backed_read_model_preserves_real_neighbor_rows() {
    let model = lan_add_device_read_model_from_inventory(
        &[neighbor(
            constants::lan_pairing::TEST_HOSTNAME,
            Some(constants::lan_pairing::TEST_HOSTNAME),
            LanPairingDeviceReachability::Online,
        )],
        "2026-06-23T00:00:00Z".to_string(),
    );

    assert_eq!(model.scan_summary.scanned_device_count, 1);
    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
        ]
    );
    assert_eq!(
        model.canonical_household_devices[0].display_name,
        constants::lan_pairing::TEST_HOSTNAME
    );
    assert_eq!(
        model.canonical_household_devices[0].source_labels,
        vec![
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::NetworkNeighbor
        ]
    );
}

#[test]
fn service_probe_presence_stays_non_enrollable_but_records_probe_source() {
    let mut discovered = neighbor(
        constants::lan_pairing::TEST_HOSTNAME,
        Some(constants::lan_pairing::TEST_HOSTNAME),
        LanPairingDeviceReachability::Online,
    );
    let expected_device_id = discovered.device_id.clone();
    discovered.agent_status =
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS.to_string());
    discovered.service_identity_probe_evidence = vec![
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidence {
            evidence_kind:
                ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::HttpStatus,
            value: "200".to_string(),
            selected_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        },
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidence {
            evidence_kind:
                ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::HtmlTitle,
            value: "Printer Admin".to_string(),
            selected_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        },
    ];

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert!(model.discovery_event_history.rows.iter().any(|row| {
        row.event_kind == LanDiscoveryEventKind::AgentDiscovered
            && row.affected_device_id.as_deref() == Some(expected_device_id.as_str())
    }));
    assert!(!model
        .discovery_event_history
        .rows
        .iter()
        .any(|row| row.event_kind == LanDiscoveryEventKind::AgentConfirmed));
    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            "service-identity-probe".to_string(),
        ]
    );
    assert_eq!(model.scan_summary.agent_device_count, 0);
    assert!(model.discovered_devices[0]
        .evidence_sources
        .contains(
            &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::ServiceIdentityProbe
        ));
    assert_eq!(
        model.discovered_devices[0]
            .service_identity_probe_evidence
            .len(),
        2
    );
    let canonical = &model.canonical_household_devices[0];
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.evidence_kind
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::ChildAgentPresence
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Weak
        }));
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::ServiceIdentityProbe
                && record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::ServiceProbeHint
                && record.value == "html-title:Printer Admin"
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Weak
                && record.note.as_deref()
                    == Some(constants::lan_pairing::LAN_SERVICE_PROBE_HINT_NOTE)
        }));
}

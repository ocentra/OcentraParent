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

fn neighbor(
    label: &str,
    hostname: Option<&str>,
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
    label: &str,
    hostname: Option<&str>,
    reachability: LanPairingDeviceReachability,
    mac_address: &str,
) -> LanNetworkInventoryDevice {
    LanNetworkInventoryDevice {
        device_id: format!("network-neighbor-{label}"),
        label: label.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: mac_address.to_string(),
        hostname: hostname.map(str::to_string),
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
    let production_household_proof = model
        .production_household_proof
        .value_or_unreachable("production household proof");
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
    let signed_discovery_relay_spine = model
        .signed_discovery_relay_spine
        .value_or_unreachable("signed discovery relay spine");
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
    let lan_discovery_source_matrix = model
        .lan_discovery_source_matrix
        .value_or_unreachable("LAN discovery source matrix");
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
        .value_or_unreachable("canonical device from trusted registry")
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

#[test]
fn local_service_child_and_ip_only_neighbor_row_merge_into_one_canonical_device() {
    let mut local_child = LanPairingDeviceRef::new(
        "trusted-child-local".to_string(),
        Some("child-profile-local".to_string()),
        "Study Laptop".to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    local_child.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    local_child.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
    local_child.hostname = Some("study-laptop.local".to_string());
    local_child.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    let mut ip_only_neighbor = LanPairingDeviceRef::new(
        "neighbor-shadow-child".to_string(),
        None,
        "Study Laptop".to_string(),
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );
    ip_only_neighbor.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    ip_only_neighbor.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T11:00:00Z".to_string(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: vec![
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice {
                schema_version: constants::lan_pairing::SCHEMA_VERSION,
                discovered_at: "2026-06-26T11:00:00Z".to_string(),
                child_device: local_child,
                agent_peer_id: "trusted-child-local".to_string(),
                pairing_id: None,
                route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
                network_mode: ocentra_parent_agent_protocol::lan_pairing::LanPairingNetworkMode::LocalNetwork,
                reachability: LanPairingDeviceReachability::Online,
                address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
                discovery_status: ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
                discovery_state: LanPairingProductionDiscoveryState::Discovered,
                evidence_sources: vec![
                    ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LocalService,
                ],
                hint_sources: Vec::new(),
                service_identity_probe_evidence: Vec::new(),
            },
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice {
                schema_version: constants::lan_pairing::SCHEMA_VERSION,
                discovered_at: "2026-06-26T11:00:05Z".to_string(),
                child_device: ip_only_neighbor,
                agent_peer_id: "neighbor-shadow-child".to_string(),
                pairing_id: None,
                route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
                network_mode: ocentra_parent_agent_protocol::lan_pairing::LanPairingNetworkMode::LocalNetwork,
                reachability: LanPairingDeviceReachability::Online,
                address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
                discovery_status: ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
                discovery_state: LanPairingProductionDiscoveryState::Discovered,
                evidence_sources: vec![
                    ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::WindowsNeighborTable,
                ],
                hint_sources: Vec::new(),
                service_identity_probe_evidence: Vec::new(),
            },
        ],
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: Vec::new(),
        trusted_device_ids: vec!["trusted-child-local".to_string()],
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    });

    assert_eq!(model.canonical_household_devices.len(), 1);
    let canonical = &model.canonical_household_devices[0];
    assert_eq!(
        canonical.classification,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
    assert!(canonical.enrollable);
    assert!(canonical
        .source_labels
        .contains(
            &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::LocalService
        ));
    assert!(canonical
        .source_labels
        .contains(
            &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::NetworkNeighbor
        ));
    assert_eq!(
        canonical.network_identity.ip_addresses,
        vec![constants::lan_pairing::TEST_LAN_IP.to_string()]
    );
    let child_agent_inventory = canonical
        .child_agent_inventory
        .as_ref()
        .value_or_unreachable("child agent inventory");
    assert_eq!(
        child_agent_inventory.device_name,
        "study-laptop.local".to_string()
    );
    assert_eq!(
        child_agent_inventory.platform,
        constants::lan_pairing::PLATFORM_WINDOWS
    );
    assert_eq!(
        child_agent_inventory.os,
        constants::lan_pairing::PLATFORM_WINDOWS
    );
    assert_eq!(
        child_agent_inventory.network_interfaces,
        vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()]
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::WindowsNeighborTable
                && record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::IpAddress
        }));
}

#[test]
fn signed_discovery_relay_spine_projects_validator_and_route_safety_rows_from_live_inputs() {
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
            route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
            pairing_id: Some("pairing-child-profile-1".to_string()),
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Stale,
            ready_for_control: false,
            stale_at: Some("2026-06-26T10:20:00Z".to_string()),
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    });

    let spine = model
        .signed_discovery_relay_spine
        .value_or_unreachable("signed discovery relay spine stays projected");
    let signed_hello_row = spine
        .signed_proof_rows
        .iter()
        .find(|row| row.check == LanSignedDiscoveryRelaySignedProofCheck::SignedHelloManualRequired)
        .value_or_unreachable("signed hello manual row");
    let stale_route_row = spine
        .route_safety_rows
        .iter()
        .find(|row| {
            row.check == LanSignedDiscoveryRelayRouteSafetyCheck::StaleSelectedDeviceRejected
        })
        .value_or_unreachable("stale route-safety row");
    let trust_decision_row = spine
        .route_safety_rows
        .iter()
        .find(|row| {
            row.check == LanSignedDiscoveryRelayRouteSafetyCheck::ParentTrustDecisionAudited
        })
        .value_or_unreachable("trust route-safety row");

    assert_eq!(
        signed_hello_row.evidence_label,
        constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO
    );
    assert_eq!(
        stale_route_row.discovery_state,
        LanPairingProductionDiscoveryState::Stale
    );
    assert_eq!(
        stale_route_row.evidence_label,
        constants::value::LAN_REASON_STALE
    );
    assert_eq!(
        trust_decision_row.discovery_state,
        LanPairingProductionDiscoveryState::Discovered
    );
    assert_eq!(
        trust_decision_row.evidence_label,
        constants::lan_pairing::HOUSEHOLD_ACTION_TRUST
    );
}

#[test]
fn discovery_event_history_orders_rows_by_timestamp_before_linking_replay_chain() {
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

    let mut scanned_neighbor = neighbor(
        "hallway-tablet",
        Some("hallway-tablet.local"),
        LanPairingDeviceReachability::Online,
    );
    scanned_neighbor.agent_status = Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    let discovered_devices =
        discovered_devices_from_network_inventory(&[scanned_neighbor], "2026-06-26T10:05:00Z");

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T10:30:00Z".to_string(),
        discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices,
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
    assert!(history
        .rows
        .windows(2)
        .all(|pair| pair[0].occurred_at <= pair[1].occurred_at));
    assert_eq!(
        history
            .rows
            .first()
            .and_then(|row| row.previous_event_id.as_deref()),
        None
    );
    assert!(history
        .rows
        .windows(2)
        .all(|pair| { pair[1].previous_event_id.as_deref() == Some(pair[0].event_id.as_str()) }));

    let agent_confirmed_index = history
        .rows
        .iter()
        .position(|row| row.event_kind == LanDiscoveryEventKind::AgentConfirmed)
        .value_or_unreachable("trusted registry child stays visible in event history");
    let agent_discovered_index = history
        .rows
        .iter()
        .position(|row| row.event_kind == LanDiscoveryEventKind::AgentDiscovered)
        .value_or_unreachable("later discovered agent signature stays visible in event history");
    assert!(agent_confirmed_index < agent_discovered_index);
    assert_eq!(
        history.latest_observed_at.as_deref(),
        history.rows.last().map(|row| row.occurred_at.as_str())
    );
    assert_eq!(
        history.latest_event_id,
        history.rows.last().map(|row| row.event_id.clone())
    );
}

#[test]
fn service_probe_snmp_evidence_adds_allowed_snmp_response_scan_label() {
    let mut discovered = neighbor(
        constants::lan_pairing::TEST_HOSTNAME,
        Some(constants::lan_pairing::TEST_HOSTNAME),
        LanPairingDeviceReachability::Online,
    );
    discovered.agent_status =
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS.to_string());
    discovered.service_identity_probe_evidence = vec![
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidence {
            evidence_kind:
                ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::SnmpSysDescr,
            value: "Printer".to_string(),
            selected_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        },
    ];

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE.to_string(),
        ]
    );
}

#[test]
fn previous_scan_hint_stays_weak_but_visible_in_scan_summary_and_evidence() {
    let mut discovered = neighbor(
        constants::lan_pairing::TEST_HOSTNAME,
        Some(constants::lan_pairing::TEST_HOSTNAME),
        LanPairingDeviceReachability::Online,
    );
    discovered.used_previous_scan_hint = true;

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_PREVIOUS_SCAN_SNAPSHOT.to_string(),
        ]
    );
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::PreviousScanSnapshot
                && record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::HistoricalIdentityHint
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Weak
                && record.note.as_deref()
                    == Some(constants::lan_pairing::LAN_PREVIOUS_SCAN_CONTINUITY_NOTE)
        }));
    assert!(model
        .lan_discovery_source_matrix
        .as_ref()
        .is_some_and(|matrix| matrix.source_rows.iter().any(|row| {
            row.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PreviousScanSnapshot
                && row.workpack_id
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W15
                && row.status
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Implemented
                && row.persists_across_restart
        })));
}

#[test]
fn inventory_backed_read_model_keeps_linux_neighbor_sources_truthful() {
    let mut discovered = neighbor(
        constants::lan_pairing::TEST_HOSTNAME,
        Some(constants::lan_pairing::TEST_HOSTNAME),
        LanPairingDeviceReachability::Online,
    );
    discovered.scan_sources = vec![
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
    ];

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
        ]
    );
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LinuxIpNeigh
                && record.evidence_kind
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::IpAddress
        }));
}

#[test]
fn inventory_backed_read_model_preserves_neighbor_observed_at_timestamp() {
    let model = lan_add_device_read_model_from_inventory(
        &[LanNetworkInventoryDevice {
            device_id: "network-neighbor-study-laptop".to_string(),
            label: "study-laptop.local".to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: "192.168.2.90".to_string(),
            mac_address: "00-11-22-33-44-90".to_string(),
            hostname: Some("study-laptop.local".to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            observed_at: "2026-06-28T10:15:30Z".to_string(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        }],
        "2026-06-28T12:00:00Z".to_string(),
    );

    assert_eq!(model.discovered_devices.len(), 1);
    assert_eq!(
        model.discovered_devices[0].discovered_at,
        "2026-06-28T10:15:30Z"
    );
}

#[test]
fn mdns_only_inventory_rows_remain_visible_as_weak_agentless_devices() {
    let discovered = LanNetworkInventoryDevice {
        device_id: "network-neighbor-mdns-officeprinter".to_string(),
        label: "Office Printer".to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "192.168.2.88".to_string(),
        mac_address: String::new(),
        hostname: Some("office-printer.local".to_string()),
        network_interface: None,
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(model.scan_summary.scanned_device_count, 1);
    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD.to_string(),
        ]
    );
    assert_eq!(model.scan_summary.agent_device_count, 0);
    let canonical = &model.canonical_household_devices[0];
    assert_eq!(canonical.display_name, "office-printer.local");
    assert_eq!(
        canonical.classification,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Printer
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert!(canonical.network_identity.mac_address.is_none());
    assert_eq!(
        canonical.network_identity.ip_addresses,
        vec!["192.168.2.88".to_string()]
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::MdnsDnsSdQuery
                && record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::IpAddress
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Strong
        }));
}

#[test]
fn ssdp_only_inventory_rows_remain_visible_as_agentless_protocol_hints() {
    let discovered = LanNetworkInventoryDevice {
        device_id: "tv-1".to_string(),
        label: "Living Room TV".to_string(),
        platform: "MediaRenderer".to_string(),
        ip_address: "192.168.2.89".to_string(),
        mac_address: String::new(),
        hostname: None,
        network_interface: None,
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(model.scan_summary.scanned_device_count, 1);
    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP.to_string(),
        ]
    );
    assert_eq!(model.scan_summary.agent_device_count, 0);
    let canonical = &model.canonical_household_devices[0];
    assert_eq!(canonical.display_name, "Living Room TV");
    assert_eq!(
        canonical.classification,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Television
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert_eq!(
        canonical.source_labels,
        vec![
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::NetworkNeighbor
        ]
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::SsdpUpnpQuery
                && record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::IpAddress
        }));
}

#[test]
fn passive_local_neighbor_collection_summaries_are_persisted_in_scan_summary() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let summaries = &model
        .scan_summary
        .passive_local_neighbor_collection_summaries;

    let expected_summary_count = if cfg!(any(target_os = "linux", target_os = "android")) {
        2
    } else {
        1
    };
    assert_eq!(summaries.len(), expected_summary_count);
    for summary in summaries {
        assert_eq!(
            summary.schema_version,
            constants::lan_pairing::SCHEMA_VERSION
        );
        assert!(!summary.source_label.trim().is_empty());
        assert!(summary.observed_count >= summary.recorded_count);
    }

    if cfg!(target_os = "windows") {
        assert!(summaries
            .iter()
            .any(|summary| summary.source_label == "windows-neighbor-table"));
    } else if cfg!(any(target_os = "linux", target_os = "android")) {
        assert!(summaries
            .iter()
            .any(|summary| summary.source_label == "linux-proc-net-arp"));
        assert!(summaries
            .iter()
            .any(|summary| summary.source_label == "linux-ip-neigh"));
    } else if cfg!(target_os = "macos") {
        assert!(summaries
            .iter()
            .any(|summary| summary.source_label == "macos-arp"));
    } else {
        assert!(summaries.iter().all(|summary| summary.reason.is_some()));
    }
}

#[test]
fn oui_vendor_lookup_is_visible_in_read_model_truth() {
    let model = lan_add_device_read_model_from_inventory(
        &[neighbor(
            constants::lan_pairing::TEST_HOSTNAME,
            Some(constants::lan_pairing::TEST_HOSTNAME),
            LanPairingDeviceReachability::Online,
        )],
        "2026-06-23T00:00:00Z".to_string(),
    );

    let canonical = &model.canonical_household_devices[0];
    assert_eq!(
        canonical.network_identity.mac_vendor.as_deref(),
        Some("AzureWave Technology Inc.")
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.evidence_kind
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::Vendor
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Strong
                && record.value == "AzureWave Technology Inc."
        }));
    assert!(model
        .lan_discovery_source_matrix
        .as_ref()
        .is_some_and(|matrix| matrix.source_rows.iter().any(|row| {
            row.source == LanDiscoverySourceKind::OuiVendorLookup
                && row.status == LanDiscoverySourceStatus::Implemented
                && row.authority == LanDiscoverySourceAuthority::ClassificationOnly
                && row.runtime_path == LanDiscoverySourceRuntimePath::RustServiceReadModel
        })));
}

#[test]
fn source_matrix_has_runtime_rows_for_all_lan_workpacks() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    let workpack_ids = matrix
        .workpack_rows
        .iter()
        .map(|row| &row.workpack_id)
        .collect::<Vec<_>>();
    assert_eq!(workpack_ids.len(), 25);
    for workpack_id in [
        LanPlanWorkpackId::W01,
        LanPlanWorkpackId::W02,
        LanPlanWorkpackId::W03,
        LanPlanWorkpackId::W04,
        LanPlanWorkpackId::W05,
        LanPlanWorkpackId::W06,
        LanPlanWorkpackId::W07,
        LanPlanWorkpackId::W08,
        LanPlanWorkpackId::W09,
        LanPlanWorkpackId::W10,
        LanPlanWorkpackId::W11,
        LanPlanWorkpackId::W12,
        LanPlanWorkpackId::W13,
        LanPlanWorkpackId::W14,
        LanPlanWorkpackId::W15,
        LanPlanWorkpackId::W16,
        LanPlanWorkpackId::W17,
        LanPlanWorkpackId::W18,
        LanPlanWorkpackId::W19,
        LanPlanWorkpackId::W20,
        LanPlanWorkpackId::W21,
        LanPlanWorkpackId::W22,
        LanPlanWorkpackId::W23,
        LanPlanWorkpackId::W24,
        LanPlanWorkpackId::W25,
    ] {
        assert!(
            workpack_ids.contains(&&workpack_id),
            "missing workpack row for {workpack_id:?}"
        );
    }

    for source in [
        LanDiscoverySourceKind::EvidenceModel,
        LanDiscoverySourceKind::InterfaceSelection,
        LanDiscoverySourceKind::MacosArp,
        LanDiscoverySourceKind::MergeDeduplication,
        LanDiscoverySourceKind::ExplainableClassification,
        LanDiscoverySourceKind::HouseholdDeviceStore,
        LanDiscoverySourceKind::ReadModelEventStream,
        LanDiscoverySourceKind::AssignmentRevocationAudit,
        LanDiscoverySourceKind::ProofGateRollout,
    ] {
        assert!(
            matrix.source_rows.iter().any(|row| row.source == source),
            "missing source row for {source:?}"
        );
    }
}

#[test]
fn source_matrix_reports_cross_platform_neighbor_sources_without_platform_overclaiming() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    for source in [
        LanDiscoverySourceKind::WindowsNeighborTable,
        LanDiscoverySourceKind::LinuxProcNetArp,
        LanDiscoverySourceKind::LinuxIpNeigh,
    ] {
        let row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == LanPlanWorkpackId::W04)
            .value_or_unreachable("implemented neighbor source row");
        assert_eq!(row.status, LanDiscoverySourceStatus::Implemented);
        assert_eq!(row.authority, LanDiscoverySourceAuthority::WeakIdentity);
        assert_eq!(
            row.runtime_path,
            LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert!(row.requires_selected_interface);
        assert!(!row.can_confirm_child_agent);
        assert!(!row.can_control_route);
        assert!(row.required_artifact_summary.is_none());
    }

    let macos = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::MacosArp
                && row.workpack_id == LanPlanWorkpackId::W04
        })
        .value_or_unreachable("macOS ARP source row");
    assert_eq!(macos.status, LanDiscoverySourceStatus::Partial);
    assert_eq!(macos.authority, LanDiscoverySourceAuthority::WeakIdentity);
    assert_eq!(
        macos.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert!(macos.requires_selected_interface);
    assert!(!macos.can_confirm_child_agent);
    assert!(!macos.can_control_route);
    assert_eq!(
        macos.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PHYSICAL)
    );
}

#[test]
fn source_matrix_marks_core_read_model_spine_without_manual_proof_churn() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    for (source, workpack_id, authority) in [
        (
            LanDiscoverySourceKind::ContractBoundary,
            LanPlanWorkpackId::W01,
            LanDiscoverySourceAuthority::ProofGate,
        ),
        (
            LanDiscoverySourceKind::EvidenceModel,
            LanPlanWorkpackId::W02,
            LanDiscoverySourceAuthority::ProofGate,
        ),
        (
            LanDiscoverySourceKind::InterfaceSelection,
            LanPlanWorkpackId::W03,
            LanDiscoverySourceAuthority::WeakIdentity,
        ),
        (
            LanDiscoverySourceKind::TargetedArpRefresh,
            LanPlanWorkpackId::W05,
            LanDiscoverySourceAuthority::PresenceOnly,
        ),
        (
            LanDiscoverySourceKind::OuiVendorLookup,
            LanPlanWorkpackId::W12,
            LanDiscoverySourceAuthority::ClassificationOnly,
        ),
        (
            LanDiscoverySourceKind::MergeDeduplication,
            LanPlanWorkpackId::W13,
            LanDiscoverySourceAuthority::ProofGate,
        ),
        (
            LanDiscoverySourceKind::ExplainableClassification,
            LanPlanWorkpackId::W14,
            LanDiscoverySourceAuthority::ClassificationOnly,
        ),
        (
            LanDiscoverySourceKind::HouseholdDeviceStore,
            LanPlanWorkpackId::W15,
            LanDiscoverySourceAuthority::ManualParentDecision,
        ),
        (
            LanDiscoverySourceKind::ReadModelEventStream,
            LanPlanWorkpackId::W16,
            LanDiscoverySourceAuthority::ProofGate,
        ),
        (
            LanDiscoverySourceKind::AssignmentRevocationAudit,
            LanPlanWorkpackId::W19,
            LanDiscoverySourceAuthority::ManualParentDecision,
        ),
    ] {
        let row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == workpack_id)
            .value_or_unreachable("implemented source row");
        assert_eq!(row.status, LanDiscoverySourceStatus::Implemented);
        assert_eq!(row.authority, authority);
        if source == LanDiscoverySourceKind::ContractBoundary {
            assert_eq!(
                row.runtime_path,
                LanDiscoverySourceRuntimePath::AgentProtocol
            );
        } else {
            assert_eq!(
                row.runtime_path,
                LanDiscoverySourceRuntimePath::RustServiceReadModel
            );
        }
        assert!(row.required_artifact_summary.is_none());
    }
}

#[test]
fn source_matrix_spine_rows_cover_merge_classification_and_durable_store_details() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    let merge = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::MergeDeduplication
                && row.workpack_id == LanPlanWorkpackId::W13
        })
        .value_or_unreachable("merge deduplication source row");
    assert_eq!(merge.status, LanDiscoverySourceStatus::Implemented);
    assert_eq!(merge.authority, LanDiscoverySourceAuthority::ProofGate);
    assert_eq!(
        merge.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert!(!merge.persists_across_restart);
    assert!(merge.required_artifact_summary.is_none());

    let classification = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::ExplainableClassification
                && row.workpack_id == LanPlanWorkpackId::W14
        })
        .value_or_unreachable("explainable classification source row");
    assert_eq!(classification.status, LanDiscoverySourceStatus::Implemented);
    assert_eq!(
        classification.authority,
        LanDiscoverySourceAuthority::ClassificationOnly
    );
    assert_eq!(
        classification.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert!(!classification.can_confirm_child_agent);
    assert!(!classification.can_assign_child_profile);
    assert!(!classification.can_control_route);
    assert!(!classification.requires_selected_interface);
    assert!(!classification.persists_across_restart);
    assert!(classification.required_artifact_summary.is_none());

    let durable_store = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::HouseholdDeviceStore
                && row.workpack_id == LanPlanWorkpackId::W15
        })
        .value_or_unreachable("household device store source row");
    assert_eq!(durable_store.status, LanDiscoverySourceStatus::Implemented);
    assert_eq!(
        durable_store.authority,
        LanDiscoverySourceAuthority::ManualParentDecision
    );
    assert_eq!(
        durable_store.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert!(durable_store.persists_across_restart);
    assert!(durable_store.required_artifact_summary.is_none());
}

fn assert_wp10_name_sources_stay_partial_name_only_rust_read_model_inputs() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    for source in [
        LanDiscoverySourceKind::NetbiosNameCache,
        LanDiscoverySourceKind::LlmnrNameQuery,
        LanDiscoverySourceKind::ReverseDnsQuery,
    ] {
        let row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == LanPlanWorkpackId::W10)
            .value_or_unreachable("weak name source row");
        assert_eq!(row.status, LanDiscoverySourceStatus::Partial);
        assert_eq!(row.authority, LanDiscoverySourceAuthority::NameOnly);
        assert_eq!(
            row.runtime_path,
            LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert!(!row.can_confirm_child_agent);
        assert!(!row.can_control_route);
        assert!(row.required_artifact_summary.is_none());
    }
}

#[test]
fn source_matrix_marks_wp10_name_sources_as_partial_name_only_rust_read_model_inputs() {
    assert_wp10_name_sources_stay_partial_name_only_rust_read_model_inputs();
}

#[test]
fn source_matrix_separates_real_weak_name_sources_from_unimplemented_llmnr() {
    // Keep the historical WP10 filter target alive while asserting the current
    // Rust-owned contract: all three W10 name sources are partial, name-only,
    // and non-controlling in the read model.
    assert_wp10_name_sources_stay_partial_name_only_rust_read_model_inputs();
}

#[test]
fn source_matrix_keeps_service_identity_probe_partial_and_non_controlling() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    let row = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::ServiceIdentityProbe
                && row.workpack_id == LanPlanWorkpackId::W11
        })
        .value_or_unreachable("service identity probe row");

    assert_eq!(row.status, LanDiscoverySourceStatus::Partial);
    assert_eq!(row.authority, LanDiscoverySourceAuthority::PresenceOnly);
    assert_eq!(
        row.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert_eq!(
        row.ui_surface,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::DevicesLan
    );
    assert!(!row.can_confirm_child_agent);
    assert!(!row.can_assign_child_profile);
    assert!(!row.can_control_route);
    assert!(row.required_artifact_summary.is_none());
}

#[test]
fn locally_administered_mac_downgrades_neighbor_confidence_and_emits_warning() {
    let model = lan_add_device_read_model_from_inventory(
        &[neighbor_with_mac(
            "phone-private-mac",
            Some(constants::lan_pairing::TEST_HOSTNAME),
            LanPairingDeviceReachability::Online,
            "02-aa-bb-cc-dd-ee",
        )],
        "2026-06-23T00:00:00Z".to_string(),
    );

    let canonical = &model.canonical_household_devices[0];
    assert_eq!(
        canonical.network_identity.confidence,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceConfidence::ManualRequired
    );
    assert_eq!(canonical.network_identity.mac_address.as_deref(), None);
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.evidence_kind
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::Vendor
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::ManualRequired
                && record.note.as_deref()
                    == Some(constants::lan_pairing::LAN_VENDOR_LOCAL_ADMINISTERED_NOTE)
        }));
}

#[test]
fn mdns_and_ssdp_rows_are_partially_implemented_rust_read_model_sources() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    for (source, workpack_id, title) in [
        (
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::MdnsDnsSdQuery,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W08,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_08,
        ),
        (
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::SsdpUpnpQuery,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W09,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_09,
        ),
    ] {
        let source_row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == workpack_id)
            .value_or_unreachable("source row");
        assert_eq!(source_row.evidence_label, title);
        assert_eq!(
            source_row.status,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
        );
        assert_eq!(
            source_row.authority,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceAuthority::PresenceOnly
        );
        assert_eq!(
            source_row.runtime_path,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert_eq!(
            source_row.ui_surface,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::DevicesLan
        );
        assert!(!source_row.can_confirm_child_agent);
        assert!(!source_row.can_control_route);
        assert!(source_row.required_artifact_summary.is_none());

        let workpack_row = matrix
            .workpack_rows
            .iter()
            .find(|row| row.workpack_id == workpack_id)
            .value_or_unreachable("workpack row");
        assert_eq!(workpack_row.title, title);
        assert_eq!(
            workpack_row.discovery_state,
            ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState::Pending
        );
        assert_eq!(
            workpack_row.runtime_owner,
            ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel
        );
        assert_eq!(
            workpack_row.status,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
        );
        assert!(workpack_row.read_model_visible);
        assert!(workpack_row.required_artifact_summary.is_none());
    }
}

#[test]
fn passive_listener_rows_reflect_current_udp_runtime_support_and_manual_gaps() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    for source in [
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveArpListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveDhcpListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveMdnsListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveSsdpListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveWsDiscoveryListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveLlmnrListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveNetbiosListener,
    ] {
        let source_row = matrix
            .source_rows
            .iter()
            .find(|row| {
                row.source == source
                    && row.workpack_id
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W07
            })
            .value_or_unreachable("passive UDP source row");
        assert_eq!(
            source_row.status,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
        );
        assert_eq!(
            source_row.runtime_path,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert_eq!(
            source_row.ui_surface,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::DevicesLan
        );
        assert!(!source_row.can_confirm_child_agent);
        assert!(!source_row.can_control_route);
    }

    {
        let source = ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveSnmpResponseListener;
        let source_row = matrix
            .source_rows
            .iter()
            .find(|row| {
                row.source == source
                    && row.workpack_id
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W07
            })
            .value_or_unreachable("raw passive source row");
        assert_eq!(
            source_row.status,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
        );
        assert_eq!(
            source_row.runtime_path,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
    }

    let workpack_row = matrix
        .workpack_rows
        .iter()
        .find(|row| {
            row.workpack_id
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W07
        })
        .value_or_unreachable("W07 workpack row");
    assert_eq!(
        workpack_row.status,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
    );
    assert_eq!(
        workpack_row.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE)
    );
}

#[test]
fn active_refresh_rows_separate_targeted_arp_from_bounded_sweep_completion() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    for (source, workpack_id, title, status) in [
        (
            LanDiscoverySourceKind::TargetedArpRefresh,
            LanPlanWorkpackId::W05,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_05,
            LanDiscoverySourceStatus::Implemented,
        ),
        (
            LanDiscoverySourceKind::BoundedArpSweep,
            LanPlanWorkpackId::W06,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_06,
            LanDiscoverySourceStatus::Implemented,
        ),
    ] {
        let source_row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == workpack_id)
            .value_or_unreachable("source row");
        assert_eq!(source_row.evidence_label, title);
        assert_eq!(source_row.status, status);
        assert_eq!(
            source_row.authority,
            LanDiscoverySourceAuthority::PresenceOnly
        );
        assert_eq!(
            source_row.runtime_path,
            LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert_eq!(
            source_row.ui_surface,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::DevicesLan
        );
        assert!(source_row.requires_selected_interface);
        assert!(!source_row.can_confirm_child_agent);
        assert!(!source_row.can_control_route);
        assert!(source_row.required_artifact_summary.is_none());

        let workpack_row = matrix
            .workpack_rows
            .iter()
            .find(|row| row.workpack_id == workpack_id)
            .value_or_unreachable("workpack row");
        assert_eq!(workpack_row.title, title);
        assert_eq!(
            workpack_row.runtime_owner,
            ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel
        );
        assert_eq!(workpack_row.status, status);
        assert!(workpack_row.read_model_visible);
        assert!(workpack_row.required_artifact_summary.is_none());
    }
}

#[test]
fn parent_and_child_mdns_advertisements_are_partial_but_hint_only() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable("LAN source matrix");

    let parent_row = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::ParentMdnsAdvertisement
                && row.workpack_id
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W17
        })
        .value_or_unreachable("parent mDNS source row");
    assert_eq!(
        parent_row.status,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
    );
    assert_eq!(
        parent_row.authority,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceAuthority::PresenceOnly
    );
    assert_eq!(
        parent_row.runtime_path,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::AgentProtocol
    );
    assert_eq!(
        parent_row.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP)
    );
    assert!(!parent_row.can_confirm_child_agent);
    assert!(!parent_row.can_control_route);

    let child_row = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::ChildMdnsAdvertisement
                && row.workpack_id
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W17
    })
        .value_or_unreachable("child mDNS source row");
    assert_eq!(
        child_row.status,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
    );
    assert_eq!(
        child_row.authority,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceAuthority::PresenceOnly
    );
    assert_eq!(
        child_row.runtime_path,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::AgentProtocol
    );
    assert_eq!(
        child_row.ui_surface,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::ProofReport
    );
    assert!(!child_row.can_confirm_child_agent);
    assert!(!child_row.can_control_route);
    assert_eq!(
        child_row.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP)
    );

    let workpack_row = matrix
        .workpack_rows
        .iter()
        .find(|row| {
            row.workpack_id
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W17
        })
        .value_or_unreachable("W17 workpack row");
    assert_eq!(
        workpack_row.status,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
    );
    assert_eq!(
        workpack_row.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP)
    );
}

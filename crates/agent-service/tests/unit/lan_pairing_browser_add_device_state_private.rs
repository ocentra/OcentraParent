use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
use ocentra_lan_core::network_inventory::{LanDiscoveryRefreshMode, LanDiscoveryScanPlan};
use ocentra_lan_core::read_model_builder::{
    build_lan_add_device_read_model, LanAddDeviceReadModelInput,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingNetworkMode, LanPairingProductionDiscoveryState,
    LanPairingTrustState, LanSelectedRouteTarget,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdNetworkIdentity, LanCanonicalHouseholdRoleState,
    LanCanonicalHouseholdRouteState, LanCanonicalHouseholdSurface, LanChildAgentInventoryPacket,
    LanDiscoveryEventHistoryState, LanDiscoveryEventKind, LanDiscoveryEvidenceConfidence,
    LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord, LanDiscoveryEvidenceSource,
    LanPairingDiscoverySource, LanSelectedDeviceReadiness,
};

use super::scan_history::{LanScanHistoryMetadata, LanScanHistorySnapshot};
use super::{
    discovery_event_history_state, network_neighbor_child_device, ordered_discovery_event_rows,
    platform_data_available_for_scan_result_with_manual_required_override,
    LanNetworkDeviceScanResult,
};

#[test]
fn apple_manual_required_platform_keeps_physical_lan_state_manual_not_unavailable() {
    assert!(
        platform_data_available_for_scan_result_with_manual_required_override(
            &LanNetworkDeviceScanResult::default(),
            true,
        )
    );
    assert!(
        !platform_data_available_for_scan_result_with_manual_required_override(
            &LanNetworkDeviceScanResult::default(),
            false,
        )
    );
}

#[test]
fn network_neighbor_child_device_omits_empty_mac_address() {
    let device = LanNetworkInventoryDevice {
        device_id: "mdns-only-device".to_string(),
        label: "Office Printer".to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "192.168.2.88".to_string(),
        mac_address: String::new(),
        hostname: Some("office-printer.local".to_string()),
        network_interface: None,
        observed_at: "2026-06-26T20:45:47.000Z".to_string(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    let child_device = network_neighbor_child_device(&device);

    assert_eq!(child_device.mac_address, None);
    assert_eq!(child_device.ip_address.as_deref(), Some("192.168.2.88"));
}

#[test]
fn selected_device_readiness_requires_non_empty_route_id_for_control() {
    let readiness = super::selected_device_readiness(Some(LanSelectedRouteTarget {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: "   ".to_string(),
        pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
        trust_state: LanPairingTrustState::Paired,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        stale_at: None,
        offline_at: None,
    }));

    assert_eq!(readiness.route_id, None);
    assert!(!readiness.ready_for_control);
}

#[test]
fn metadata_only_scan_history_stays_empty_not_ready() {
    let scan_result = LanNetworkDeviceScanResult {
        current_scan_snapshot: Some(scan_history_snapshot(Some(sample_scan_metadata()))),
        ..LanNetworkDeviceScanResult::default()
    };
    let read_model = sample_read_model();

    let rows = ordered_discovery_event_rows(&scan_result, &read_model);

    assert!(rows
        .iter()
        .all(|row| matches!(
            row.event_kind,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventKind::ScanStarted
                | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventKind::ScanFinished
        )));
    assert_eq!(
        discovery_event_history_state(&scan_result, &rows, &read_model),
        LanDiscoveryEventHistoryState::Empty
    );
}

#[test]
fn legacy_snapshot_without_metadata_marks_history_degraded() {
    let scan_result = LanNetworkDeviceScanResult {
        current_scan_snapshot: Some(scan_history_snapshot(None)),
        ..LanNetworkDeviceScanResult::default()
    };
    let read_model = sample_read_model();
    let rows = ordered_discovery_event_rows(&scan_result, &read_model);

    assert_eq!(
        discovery_event_history_state(&scan_result, &rows, &read_model),
        LanDiscoveryEventHistoryState::Degraded
    );
}

#[test]
fn no_scan_history_with_manual_required_physical_state_keeps_history_manual_required() {
    let scan_result = LanNetworkDeviceScanResult::default();
    let read_model = sample_read_model();
    let rows = ordered_discovery_event_rows(&scan_result, &read_model);

    assert!(rows.is_empty());
    assert_eq!(
        discovery_event_history_state(&scan_result, &rows, &read_model),
        LanDiscoveryEventHistoryState::ManualRequired
    );
}

#[test]
fn available_scan_history_with_selected_paired_online_child_without_route_keeps_history_agent_offline(
) {
    let scan_result = LanNetworkDeviceScanResult {
        current_scan_snapshot: Some(scan_history_snapshot(Some(sample_scan_metadata()))),
        ..LanNetworkDeviceScanResult::default()
    };
    let mut read_model = sample_read_model();
    read_model.selected_device_readiness = LanSelectedDeviceReadiness {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        selected_child_device_id: Some(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
        route_id: None,
        pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
        trust_state: LanPairingTrustState::Paired,
        reachability: LanPairingDeviceReachability::Online,
        ready_for_control: false,
        stale_at: None,
        offline_at: None,
    };
    let rows = ordered_discovery_event_rows(&scan_result, &read_model);

    assert_eq!(
        discovery_event_history_state(&scan_result, &rows, &read_model),
        LanDiscoveryEventHistoryState::AgentOffline
    );
}

#[test]
fn ordered_discovery_event_rows_emit_interface_device_and_agent_rows_with_contract_fields() {
    let mut previous_metadata = sample_scan_metadata();
    previous_metadata.scan_plan.selected_interface = Some("Wi-Fi".to_string());
    previous_metadata.scan_plan.ipv4_cidr = Some("192.168.0.42/24".to_string());

    let mut current_metadata = sample_scan_metadata();
    current_metadata.scan_id = "lan-scan-1719434747999".to_string();
    current_metadata.scan_plan.selected_interface = Some("Ethernet".to_string());
    current_metadata.scan_plan.ipv4_cidr = Some("10.0.0.7/24".to_string());

    let discovered_device = inventory_device(
        "lan-new-agent",
        "office-agent.local",
        "10.0.0.88",
        "00-11-22-33-44-77",
        Some(constants::lan_pairing::LOCAL_AGENT_STATUS),
    );
    let scan_result = LanNetworkDeviceScanResult {
        devices: vec![discovered_device.clone()],
        previous_scan_snapshot: Some(scan_history_snapshot(Some(previous_metadata))),
        current_scan_snapshot: Some(scan_history_snapshot_with_devices(
            Some(current_metadata.clone()),
            vec![discovered_device.clone()],
        )),
        reused_recent_snapshot: false,
    };
    let read_model = sample_read_model();

    let rows = ordered_discovery_event_rows(&scan_result, &read_model);

    let scan_started = rows
        .iter()
        .find(|row| row.event_kind == LanDiscoveryEventKind::ScanStarted)
        .unwrap_or_else(|| unreachable!("scan started row should be emitted"));
    assert_eq!(
        scan_started.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );
    assert!(scan_started.affected_device_id.is_none());
    assert!(!scan_started.event_id.is_empty());
    assert!(!scan_started.occurred_at.is_empty());

    let scan_finished = rows
        .iter()
        .find(|row| row.event_kind == LanDiscoveryEventKind::ScanFinished)
        .unwrap_or_else(|| unreachable!("scan finished row should be emitted"));
    assert_eq!(
        scan_finished.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );
    assert!(scan_finished.affected_device_id.is_none());
    assert!(!scan_finished.event_id.is_empty());
    assert!(!scan_finished.occurred_at.is_empty());

    let interface_changed = rows
        .iter()
        .find(|row| row.event_kind == LanDiscoveryEventKind::InterfaceChanged)
        .unwrap_or_else(|| unreachable!("interface change row should be emitted"));
    assert_eq!(
        interface_changed.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );
    assert!(interface_changed.affected_device_id.is_none());

    let device_found = rows
        .iter()
        .find(|row| row.event_kind == LanDiscoveryEventKind::DeviceFound)
        .unwrap_or_else(|| unreachable!("device found row should be emitted"));
    assert_eq!(
        device_found.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );
    assert_eq!(
        device_found.affected_device_id.as_deref(),
        Some(discovered_device.device_id.as_str())
    );
    assert!(device_found.evidence_id.is_none());

    let agent_discovered = rows
        .iter()
        .find(|row| row.event_kind == LanDiscoveryEventKind::AgentDiscovered)
        .unwrap_or_else(|| unreachable!("agent discovered row should be emitted"));
    assert_eq!(
        agent_discovered.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );
    assert_eq!(
        agent_discovered.affected_device_id.as_deref(),
        Some(discovered_device.device_id.as_str())
    );
    assert!(!agent_discovered.event_id.is_empty());
    assert!(!agent_discovered.occurred_at.is_empty());
}

#[test]
fn ordered_discovery_event_rows_emit_update_and_reachability_rows_with_contract_fields() {
    let mut previous_metadata = sample_scan_metadata();
    previous_metadata.scan_id = "lan-scan-1719434747001".to_string();

    let mut current_metadata = sample_scan_metadata();
    current_metadata.scan_id = "lan-scan-1719434747002".to_string();

    let previously_online_now_offline = inventory_device_with_reachability(
        "lan-device-down",
        "shared-host.local",
        "10.0.0.41",
        "00-11-22-33-44-41",
        LanPairingDeviceReachability::Online,
        None,
    );
    let currently_offline = inventory_device_with_reachability(
        "lan-device-down",
        "shared-host.local",
        "10.0.0.41",
        "00-11-22-33-44-41",
        LanPairingDeviceReachability::Offline,
        None,
    );

    let previously_offline = inventory_device_with_reachability(
        "lan-device-up",
        "old-host.local",
        "10.0.0.42",
        "00-11-22-33-44-42",
        LanPairingDeviceReachability::Offline,
        None,
    );
    let currently_online_with_update = inventory_device_with_reachability(
        "lan-device-up",
        "new-host.local",
        "10.0.0.42",
        "00-11-22-33-44-42",
        LanPairingDeviceReachability::Online,
        None,
    );

    let scan_result = LanNetworkDeviceScanResult {
        devices: vec![
            currently_offline.clone(),
            currently_online_with_update.clone(),
        ],
        previous_scan_snapshot: Some(scan_history_snapshot_with_devices(
            Some(previous_metadata),
            vec![previously_online_now_offline, previously_offline],
        )),
        current_scan_snapshot: Some(scan_history_snapshot_with_devices(
            Some(current_metadata.clone()),
            vec![
                currently_offline.clone(),
                currently_online_with_update.clone(),
            ],
        )),
        reused_recent_snapshot: false,
    };
    let read_model = sample_read_model();

    let rows = ordered_discovery_event_rows(&scan_result, &read_model);

    let device_updated = rows
        .iter()
        .find(|row| row.event_kind == LanDiscoveryEventKind::DeviceUpdated)
        .unwrap_or_else(|| unreachable!("device updated row should be emitted"));
    assert_eq!(
        device_updated.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );
    assert_eq!(
        device_updated.affected_device_id.as_deref(),
        Some(currently_online_with_update.device_id.as_str())
    );

    let device_online = rows
        .iter()
        .find(|row| {
            row.event_kind == LanDiscoveryEventKind::DeviceOnline
                && row.affected_device_id.as_deref()
                    == Some(currently_online_with_update.device_id.as_str())
        })
        .unwrap_or_else(|| unreachable!("device online row should be emitted"));
    assert_eq!(
        device_online.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );

    let device_offline = rows
        .iter()
        .find(|row| {
            row.event_kind == LanDiscoveryEventKind::DeviceOffline
                && row.affected_device_id.as_deref() == Some(currently_offline.device_id.as_str())
        })
        .unwrap_or_else(|| unreachable!("device offline row should be emitted"));
    assert_eq!(
        device_offline.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );
}

#[test]
fn ordered_discovery_event_rows_emit_canonical_household_rows_and_previous_event_chain() {
    let mut current_metadata = sample_scan_metadata();
    current_metadata.scan_id = "lan-scan-1719434747003".to_string();

    let scan_result = LanNetworkDeviceScanResult {
        current_scan_snapshot: Some(scan_history_snapshot(Some(current_metadata.clone()))),
        ..LanNetworkDeviceScanResult::default()
    };
    let mut read_model = sample_read_model();
    read_model.generated_at = "2026-06-26T20:45:49.000Z".to_string();
    read_model.canonical_household_devices = vec![
        canonical_unknown_household_device(),
        canonical_child_agent_household_device(),
        canonical_offline_router_household_device(),
    ];

    let rows = ordered_discovery_event_rows(&scan_result, &read_model);

    assert_eq!(
        rows.first().and_then(|row| row.previous_event_id.as_ref()),
        None
    );

    let evidence_found = rows
        .iter()
        .find(|row| {
            row.event_kind == LanDiscoveryEventKind::EvidenceFound
                && row.affected_device_id.as_deref() == Some("lan-canonical-child-agent")
                && row.evidence_id.as_deref() == Some("lan-child-agent-evidence")
        })
        .unwrap_or_else(|| unreachable!("canonical evidence row should be emitted"));
    assert_eq!(
        evidence_found.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );

    let unknown_detected = rows
        .iter()
        .find(|row| {
            row.event_kind == LanDiscoveryEventKind::UnknownDetected
                && row.affected_device_id.as_deref() == Some("lan-canonical-unknown")
        })
        .unwrap_or_else(|| unreachable!("unknown device row should be emitted"));
    assert_eq!(
        unknown_detected.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );

    let agent_confirmed = rows
        .iter()
        .find(|row| {
            row.event_kind == LanDiscoveryEventKind::AgentConfirmed
                && row.affected_device_id.as_deref() == Some("lan-canonical-child-agent")
        })
        .unwrap_or_else(|| unreachable!("child agent confirmation row should be emitted"));
    assert_eq!(
        agent_confirmed.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );

    let device_offline = rows
        .iter()
        .find(|row| {
            row.event_kind == LanDiscoveryEventKind::DeviceOffline
                && row.affected_device_id.as_deref() == Some("lan-canonical-router")
        })
        .unwrap_or_else(|| unreachable!("canonical offline device row should be emitted"));
    assert_eq!(
        device_offline.scan_session_id.as_deref(),
        Some(current_metadata.scan_id.as_str())
    );
    assert_eq!(device_offline.occurred_at, "2026-06-26T20:45:46.000Z");

    for row_pair in rows.windows(2) {
        assert_eq!(
            row_pair[1].previous_event_id.as_deref(),
            Some(row_pair[0].event_id.as_str())
        );
    }
}

fn sample_read_model(
) -> ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel
{
    build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T20:45:47.000Z".to_string(),
        discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::ManualRequired,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: Vec::new(),
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: Vec::new(),
        trusted_device_ids: Vec::new(),
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
    })
}

fn scan_history_snapshot(metadata: Option<LanScanHistoryMetadata>) -> LanScanHistorySnapshot {
    LanScanHistorySnapshot {
        schema_version: 2,
        updated_at: "2026-06-26T20:45:47.000Z".to_string(),
        metadata,
        devices: Vec::new(),
    }
}

fn scan_history_snapshot_with_devices(
    metadata: Option<LanScanHistoryMetadata>,
    devices: Vec<LanNetworkInventoryDevice>,
) -> LanScanHistorySnapshot {
    LanScanHistorySnapshot {
        schema_version: 2,
        updated_at: "2026-06-26T20:45:47.000Z".to_string(),
        metadata,
        devices,
    }
}

fn inventory_device(
    device_id: &str,
    hostname: &str,
    ip_address: &str,
    mac_address: &str,
    agent_status: Option<&str>,
) -> LanNetworkInventoryDevice {
    inventory_device_with_reachability(
        device_id,
        hostname,
        ip_address,
        mac_address,
        LanPairingDeviceReachability::Online,
        agent_status,
    )
}

fn inventory_device_with_reachability(
    device_id: &str,
    hostname: &str,
    ip_address: &str,
    mac_address: &str,
    reachability: LanPairingDeviceReachability,
    agent_status: Option<&str>,
) -> LanNetworkInventoryDevice {
    LanNetworkInventoryDevice {
        device_id: device_id.to_string(),
        label: hostname.to_string(),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        ip_address: ip_address.to_string(),
        mac_address: mac_address.to_string(),
        hostname: Some(hostname.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        observed_at: "2026-06-26T20:45:47.000Z".to_string(),
        reachability,
        agent_status: agent_status.map(str::to_string),
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }
}

fn sample_scan_metadata() -> LanScanHistoryMetadata {
    LanScanHistoryMetadata {
        scan_id: "lan-scan-1719434747000".to_string(),
        paired_registry_truth_count: 0,
        recent_previous_agent_truth_count: 0,
        durable_household_truth_count: 0,
        scan_plan: LanDiscoveryScanPlan {
            refresh_mode: LanDiscoveryRefreshMode::ActiveSubnetRefresh,
            selected_interface: Some("Wi-Fi".to_string()),
            local_ip_address: Some("192.168.0.42".to_string()),
            ipv4_cidr: Some("192.168.0.42/24".to_string()),
            default_gateway: Some("192.168.0.1".to_string()),
            dns_servers: vec!["192.168.0.1".to_string()],
            dhcp_server: Some("192.168.0.1".to_string()),
            broadcast_address: Some("192.168.0.255".to_string()),
            ipv6_prefixes: vec!["2001:db8::42/64".to_string()],
            trusted_truth_device_count: 0,
            previous_device_count: 0,
            active_ipv4_candidate_count: 0,
            active_ipv4_target_count: 0,
            prioritized_previous_target_count: 0,
            active_ipv4_target_timeout_ms: None,
            allow_wsd_identity_query: false,
            allow_snmp_identity_query: false,
            allow_os_fingerprint: false,
            suppressed_active_ipv4_targets: Vec::new(),
            targeted_arp_refresh_evidence: Vec::new(),
        },
    }
}

fn canonical_unknown_household_device() -> LanCanonicalHouseholdDevice {
    LanCanonicalHouseholdDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        canonical_device_id: "lan-canonical-unknown".to_string(),
        display_name: "Mystery Speaker".to_string(),
        classification: LanCanonicalHouseholdDeviceClassification::UnknownLanDevice,
        role_badges: Vec::new(),
        enrollable: false,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        trust_state: LanPairingTrustState::Unpaired,
        route_id: None,
        route_state: LanCanonicalHouseholdRouteState::Unavailable,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![LanCanonicalHouseholdDeviceSource::NetworkNeighbor],
        network_identity: canonical_network_identity(
            "mystery-speaker.local",
            "10.0.0.90",
            "00-11-22-33-44-90",
            LanPairingDeviceReachability::Online,
            LanCanonicalHouseholdDeviceConfidence::ManualRequired,
            None,
            Vec::new(),
        ),
        child_agent_inventory: None,
        policy_target_surfaces: vec![LanCanonicalHouseholdSurface::Devices],
    }
}

fn canonical_child_agent_household_device() -> LanCanonicalHouseholdDevice {
    LanCanonicalHouseholdDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        canonical_device_id: "lan-canonical-child-agent".to_string(),
        display_name: "Family Laptop".to_string(),
        classification: LanCanonicalHouseholdDeviceClassification::ChildAgent,
        role_badges: Vec::new(),
        enrollable: true,
        discovery_state: LanPairingProductionDiscoveryState::Paired,
        trust_state: LanPairingTrustState::Paired,
        route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![
            LanCanonicalHouseholdDeviceSource::LocalService,
            LanCanonicalHouseholdDeviceSource::TrustedRegistry,
        ],
        network_identity: canonical_network_identity(
            "family-laptop.local",
            "10.0.0.91",
            "00-11-22-33-44-91",
            LanPairingDeviceReachability::Online,
            LanCanonicalHouseholdDeviceConfidence::AgentConfirmed,
            None,
            vec![canonical_evidence_record()],
        ),
        child_agent_inventory: Some(LanChildAgentInventoryPacket {
            device_name: "Family Laptop".to_string(),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            os: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            cpu_model: None,
            cpu_cores: None,
            memory_total: None,
            gpu_model: None,
            gpu_driver: None,
            gpu_memory: None,
            nvidia_smi: None,
            network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
            capabilities: vec![constants::lan_pairing::SURFACE_SCREEN.to_string()],
            role_state: LanCanonicalHouseholdRoleState::Implemented,
            route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
            pairing_trust_state: LanPairingTrustState::Paired,
        }),
        policy_target_surfaces: vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Screen,
        ],
    }
}

fn canonical_offline_router_household_device() -> LanCanonicalHouseholdDevice {
    LanCanonicalHouseholdDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        canonical_device_id: "lan-canonical-router".to_string(),
        display_name: "Main Router".to_string(),
        classification: LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure,
        role_badges: Vec::new(),
        enrollable: false,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        trust_state: LanPairingTrustState::Unpaired,
        route_id: None,
        route_state: LanCanonicalHouseholdRouteState::Unavailable,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![LanCanonicalHouseholdDeviceSource::NetworkNeighbor],
        network_identity: canonical_network_identity(
            "router.local",
            "10.0.0.1",
            "00-11-22-33-44-01",
            LanPairingDeviceReachability::Offline,
            LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
            Some("2026-06-26T20:45:46.000Z"),
            Vec::new(),
        ),
        child_agent_inventory: None,
        policy_target_surfaces: vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Network,
        ],
    }
}

fn canonical_network_identity(
    hostname: &str,
    ip_address: &str,
    mac_address: &str,
    reachability: LanPairingDeviceReachability,
    confidence: LanCanonicalHouseholdDeviceConfidence,
    offline_at: Option<&str>,
    evidence_records: Vec<LanDiscoveryEvidenceRecord>,
) -> LanCanonicalHouseholdNetworkIdentity {
    LanCanonicalHouseholdNetworkIdentity {
        hostname: Some(hostname.to_string()),
        ip_addresses: vec![ip_address.to_string()],
        mac_address: Some(mac_address.to_string()),
        mac_vendor: None,
        network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
        reachability,
        confidence,
        stale_at: None,
        offline_at: offline_at.map(str::to_string),
        evidence_records,
    }
}

fn canonical_evidence_record() -> LanDiscoveryEvidenceRecord {
    LanDiscoveryEvidenceRecord {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        evidence_id: "lan-child-agent-evidence".to_string(),
        source: LanDiscoveryEvidenceSource::LocalService,
        evidence_kind: LanDiscoveryEvidenceKind::ChildAgentPresence,
        device_id: "lan-canonical-child-agent".to_string(),
        value: constants::lan_pairing::LOCAL_AGENT_STATUS.to_string(),
        normalized_value: constants::lan_pairing::LOCAL_AGENT_STATUS.to_ascii_lowercase(),
        first_seen_at: "2026-06-26T20:45:44.000Z".to_string(),
        last_seen_at: "2026-06-26T20:45:44.000Z".to_string(),
        expires_at: None,
        confidence: LanDiscoveryEvidenceConfidence::Confirmed,
        merge_key: "agent:lan-canonical-child-agent".to_string(),
        note: None,
    }
}

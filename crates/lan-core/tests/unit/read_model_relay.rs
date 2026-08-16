use super::*;

fn local_child_for_ip_merge() -> LanPairingDeviceRef {
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
    local_child
}

fn ip_only_neighbor_for_ip_merge() -> LanPairingDeviceRef {
    let mut ip_only_neighbor = LanPairingDeviceRef::new(
        "neighbor-shadow-child".to_string(),
        None,
        "Study Laptop".to_string(),
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );
    ip_only_neighbor.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    ip_only_neighbor.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
    ip_only_neighbor
}

fn assert_merged_local_child_canonical(
    canonical: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice,
) {
    assert_eq!(
        canonical.classification,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
    assert!(canonical.enrollable);
    assert!(canonical.source_labels.contains(
        &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::LocalService
    ));
    assert!(canonical.source_labels.contains(
        &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::NetworkNeighbor
    ));
    assert_eq!(
        canonical.network_identity.ip_addresses,
        vec![constants::lan_pairing::TEST_LAN_IP.to_string()]
    );
    let child_agent_inventory = canonical
        .child_agent_inventory
        .as_ref()
        .value_or_unreachable();
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

fn trusted_child_for_relay_tests() -> LanPairingDeviceRef {
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
    child_device
}

fn trusted_registry_entry_for_relay_tests() -> LanTrustedDeviceRegistryEntry {
    LanTrustedDeviceRegistryEntry {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: "pairing-child-profile-1".to_string(),
        child_device: trusted_child_for_relay_tests(),
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
    }
}

fn relay_trust_decision() -> LanHouseholdDeviceDecision {
    LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: "household-action-trust-1".to_string(),
        action_kind: LanHouseholdDeviceActionKind::Trust,
        canonical_device_id: "lan-child-profile-childprofile1".to_string(),
        child_profile_id: Some("child-profile-1".to_string()),
        display_name: Some("Study Laptop".to_string()),
        device_kind: None,
        parent_actor_id: "parent-1".to_string(),
        decided_at: "2026-06-26T10:01:00Z".to_string(),
        revoked_at: None,
    }
}

fn relay_read_model_input(
    discovered_devices: Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice>,
    selected_device_readiness: LanSelectedDeviceReadiness,
) -> LanAddDeviceReadModelInput {
    LanAddDeviceReadModelInput {
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
        trusted_device_registry: vec![trusted_registry_entry_for_relay_tests()],
        household_device_decisions: vec![relay_trust_decision()],
        trusted_device_ids: vec!["trusted-child-1".to_string()],
        revoked_device_ids: Vec::new(),
        selected_device_readiness,
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    }
}

fn stale_selected_device_readiness() -> LanSelectedDeviceReadiness {
    LanSelectedDeviceReadiness {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        selected_child_device_id: Some("trusted-child-1".to_string()),
        route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        pairing_id: Some("pairing-child-profile-1".to_string()),
        trust_state: LanPairingTrustState::Paired,
        reachability: LanPairingDeviceReachability::Stale,
        ready_for_control: false,
        stale_at: Some("2026-06-26T10:20:00Z".to_string()),
        offline_at: None,
    }
}

fn offline_paired_selected_device_readiness() -> LanSelectedDeviceReadiness {
    LanSelectedDeviceReadiness {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        selected_child_device_id: None,
        route_id: None,
        pairing_id: None,
        trust_state: LanPairingTrustState::Paired,
        reachability: LanPairingDeviceReachability::Offline,
        ready_for_control: false,
        stale_at: None,
        offline_at: None,
    }
}

#[test]
fn local_service_child_and_ip_only_neighbor_row_merge_into_one_canonical_device() {
    let local_child = local_child_for_ip_merge();
    let ip_only_neighbor = ip_only_neighbor_for_ip_merge();

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
    assert_merged_local_child_canonical(&model.canonical_household_devices[0]);
}

#[test]
fn signed_discovery_relay_spine_projects_validator_and_route_safety_rows_from_live_inputs() {
    let model = build_lan_add_device_read_model(relay_read_model_input(
        Vec::new(),
        stale_selected_device_readiness(),
    ));

    let spine = model.signed_discovery_relay_spine.value_or_unreachable();
    let signed_hello_row = spine
        .signed_proof_rows
        .iter()
        .find(|row| row.check == LanSignedDiscoveryRelaySignedProofCheck::SignedHelloManualRequired)
        .value_or_unreachable();
    let stale_route_row = spine
        .route_safety_rows
        .iter()
        .find(|row| {
            row.check == LanSignedDiscoveryRelayRouteSafetyCheck::StaleSelectedDeviceRejected
        })
        .value_or_unreachable();
    let trust_decision_row = spine
        .route_safety_rows
        .iter()
        .find(|row| {
            row.check == LanSignedDiscoveryRelayRouteSafetyCheck::ParentTrustDecisionAudited
        })
        .value_or_unreachable();

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
    let mut scanned_neighbor = neighbor(
        "hallway-tablet",
        Some("hallway-tablet.local"),
        LanPairingDeviceReachability::Online,
    );
    scanned_neighbor.agent_status = Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    let discovered_devices =
        discovered_devices_from_network_inventory(&[scanned_neighbor], "2026-06-26T10:05:00Z");
    let model = build_lan_add_device_read_model(relay_read_model_input(
        discovered_devices,
        offline_paired_selected_device_readiness(),
    ));

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
        .value_or_unreachable();
    let agent_discovered_index = history
        .rows
        .iter()
        .position(|row| row.event_kind == LanDiscoveryEventKind::AgentDiscovered)
        .value_or_unreachable();
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

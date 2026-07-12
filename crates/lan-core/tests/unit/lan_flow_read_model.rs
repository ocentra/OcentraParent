use super::*;

#[test]
fn lan_read_model_projects_snapshot_events_for_visible_unknown_devices() {
    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        unknown_network_device_with_sources(vec![
            LanDiscoveryEvidenceSource::WindowsNeighborTable,
            LanDiscoveryEvidenceSource::DnsCache,
        ]),
    ]));

    assert_eq!(
        model.discovery_event_history.state,
        LanDiscoveryEventHistoryState::Ready
    );
    assert!(model.discovery_event_history.rows.len() >= 4);
    assert_eq!(
        model.discovery_event_history.latest_event_id,
        model
            .discovery_event_history
            .rows
            .last()
            .map(|row| row.event_id.clone())
    );
    assert!(model.discovery_event_history.rows.iter().all(|row| {
        row.scan_session_id
            .as_deref()
            .map(|value| value.starts_with("lan-scan-"))
            .unwrap_or(false)
    }));
    assert_eq!(
        model.discovery_event_history.rows[0].event_kind,
        LanDiscoveryEventKind::ScanStarted
    );
    assert!(model
        .discovery_event_history
        .rows
        .iter()
        .any(|row| row.event_kind == LanDiscoveryEventKind::UnknownDetected));
    assert!(model
        .discovery_event_history
        .rows
        .iter()
        .any(|row| row.event_kind == LanDiscoveryEventKind::DeviceOnline));
    assert_eq!(
        model
            .discovery_event_history
            .rows
            .last()
            .map(|row| &row.event_kind),
        Some(&LanDiscoveryEventKind::ScanFinished)
    );
    assert!(model
        .discovery_event_history
        .rows
        .iter()
        .skip(1)
        .all(|row| row.previous_event_id.is_some()));
    assert!(model.discovery_event_history.rows.iter().any(|row| {
        row.event_kind == LanDiscoveryEventKind::EvidenceFound
            && row.evidence_id.is_some()
            && row.affected_device_id.is_some()
    }));
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable();
    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::UnknownLanDevice
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
}

#[test]
fn locally_administered_mac_does_not_merge_distinct_neighbor_devices() {
    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        unknown_network_device(
            "lan-randomized-one",
            "Printer One",
            "192.168.1.31",
            "02-aa-bb-cc-dd-ee",
            "printer-one.local",
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
        unknown_network_device(
            "lan-randomized-two",
            "Printer Two",
            "192.168.1.32",
            "02-aa-bb-cc-dd-ee",
            "printer-two.local",
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(model.canonical_household_devices.iter().all(|device| {
        !device.enrollable
            && device.network_identity.confidence
                == LanCanonicalHouseholdDeviceConfidence::ManualRequired
    }));
    assert!(model.canonical_household_devices.iter().all(|device| {
        device
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind == LanDiscoveryEvidenceKind::Vendor
                    && record.confidence == LanDiscoveryEvidenceConfidence::ManualRequired
            })
    }));
}

#[test]
fn service_probe_hints_make_device_visible_without_control_authority() {
    let model =
        build_lan_add_device_read_model(lan_read_model_input(vec![service_probe_only_device()]));
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable();

    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::Printer
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert_eq!(
        canonical.route_state,
        LanCanonicalHouseholdRouteState::Unavailable
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::ServiceIdentityProbe
                && record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Weak
        }));
}

#[test]
fn service_probe_hints_stay_weak_when_previous_scan_hints_exist() {
    let mut device = service_probe_only_device();
    device.hint_sources = vec![LanDiscoveryEvidenceSource::PreviousScanSnapshot];

    let model = build_lan_add_device_read_model(lan_read_model_input(vec![device]));
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable();

    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::Printer
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::ServiceIdentityProbe
                && record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Weak
        }));
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::PreviousScanSnapshot
                && record.evidence_kind == LanDiscoveryEvidenceKind::HistoricalIdentityHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Weak
        }));
}

#[test]
fn dns_cache_and_netbios_hostname_evidence_stays_weak() {
    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        unknown_network_device_with_sources(vec![
            LanDiscoveryEvidenceSource::WindowsNeighborTable,
            LanDiscoveryEvidenceSource::DnsCache,
            LanDiscoveryEvidenceSource::Netbios,
            LanDiscoveryEvidenceSource::Llmnr,
        ]),
    ]));
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable();
    let hostname_records = canonical
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind == LanDiscoveryEvidenceKind::Hostname)
        .collect::<Vec<_>>();

    assert!(hostname_records.iter().any(|record| {
        record.source == LanDiscoveryEvidenceSource::DnsCache
            && record.confidence == LanDiscoveryEvidenceConfidence::Weak
    }));
    assert!(hostname_records.iter().any(|record| {
        record.source == LanDiscoveryEvidenceSource::Netbios
            && record.confidence == LanDiscoveryEvidenceConfidence::Weak
    }));
    assert!(hostname_records.iter().any(|record| {
        record.source == LanDiscoveryEvidenceSource::Llmnr
            && record.confidence == LanDiscoveryEvidenceConfidence::Weak
    }));
    assert!(!hostname_records.iter().any(|record| {
        record.source == LanDiscoveryEvidenceSource::WindowsNeighborTable
            && record.confidence == LanDiscoveryEvidenceConfidence::Strong
    }));
}

#[test]
fn household_revoke_decision_records_audit_evidence_and_blocks_control() {
    let mut input = lan_read_model_input(vec![unknown_network_device_with_sources(vec![
        LanDiscoveryEvidenceSource::WindowsNeighborTable,
    ])]);
    input.household_device_decisions = vec![LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: "household-action-revoke-1".to_string(),
        action_kind: LanHouseholdDeviceActionKind::Revoke,
        canonical_device_id: canonical_test_mac_device_id().to_string(),
        child_profile_id: Some("child-profile-1".to_string()),
        display_name: Some("Kitchen Printer".to_string()),
        device_kind: Some(constants::lan_pairing::HOUSEHOLD_DEVICE_KIND_UNKNOWN.to_string()),
        parent_actor_id: "parent-1".to_string(),
        decided_at: "2026-06-26T10:01:00Z".to_string(),
        revoked_at: None,
    }];

    let model = build_lan_add_device_read_model(input);
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable();

    assert_eq!(canonical.display_name, "Kitchen Printer");
    assert_eq!(
        canonical.discovery_state,
        LanPairingProductionDiscoveryState::Revoked
    );
    assert_eq!(canonical.trust_state, LanPairingTrustState::Revoked);
    assert_eq!(canonical.route_id, None);
    assert_eq!(
        canonical.route_state,
        LanCanonicalHouseholdRouteState::Unavailable
    );
    assert!(!canonical.enrollable);
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::ParentAssignment
                && record.evidence_kind == LanDiscoveryEvidenceKind::ParentDecision
                && record.value == constants::lan_pairing::HOUSEHOLD_ACTION_REVOKE
                && record.confidence == LanDiscoveryEvidenceConfidence::Rejected
        }));
}

#[test]
fn conflicting_child_profile_assignments_do_not_merge_even_with_matching_mac() {
    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        child_profile_device(
            "lan-device-child-profile-one",
            "child-profile-1",
            "Alpha Tablet",
        ),
        child_profile_device(
            "lan-device-child-profile-two",
            "child-profile-2",
            "Alpha Tablet",
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(model.canonical_household_devices.iter().all(
        |device| device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
    ));
    assert!(model.canonical_household_devices.iter().any(|device| {
        device.canonical_device_id.starts_with("lan-child-profile-")
            && device
                .source_labels
                .contains(&LanCanonicalHouseholdDeviceSource::NetworkNeighbor)
    }));
}

#[test]
fn paired_registry_truth_outweighs_previous_scan_hint() {
    let discovered = hinted_child_profile_device(
        "lan-device-child-profile-hint",
        "child-profile-3",
        "Kitchen Tablet",
    );
    let registry_entry = trusted_registry_entry_for_child_profile(
        "lan-device-child-profile-hint",
        "child-profile-3",
        "Kitchen Tablet",
    );

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T10:00:30Z".to_string(),
        discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: vec![discovered],
        pairing_requests: Vec::new(),
        trusted_device_registry: vec![registry_entry],
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
    });

    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable();
    assert_eq!(model.canonical_household_devices.len(), 1);
    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
    assert_eq!(canonical.trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        canonical.network_identity.confidence,
        LanCanonicalHouseholdDeviceConfidence::AgentConfirmed
    );
    let inventory = canonical
        .child_agent_inventory
        .as_ref()
        .value_or_unreachable();
    assert_eq!(inventory.device_name, "tablet.local");
    assert_eq!(inventory.pairing_trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        inventory.route_state,
        LanCanonicalHouseholdRouteState::LocalNetwork
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::PreviousScanSnapshot
                && record.evidence_kind == LanDiscoveryEvidenceKind::HistoricalIdentityHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Weak
        }));
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::TrustedRegistry
                && record.evidence_kind == LanDiscoveryEvidenceKind::TrustedRegistry
                && record.confidence == LanDiscoveryEvidenceConfidence::ManualRequired
        }));
}

#[test]
fn router_classification_is_visible_but_not_controllable() {
    let mut child_device = LanPairingDeviceRef::new(
        "lan-device-router".to_string(),
        None,
        "Gateway".to_string(),
        constants::lan_pairing::PLATFORM_ROUTER.to_string(),
    );
    child_device.ip_address = Some(constants::lan_pairing::TEST_ROUTER_IP.to_string());
    child_device.mac_address = Some(constants::lan_pairing::TEST_ROUTER_MAC.to_string());
    child_device.hostname = Some("gateway.local".to_string());
    child_device.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        LanBrowserAddDeviceDiscoveryDevice {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            discovered_at: "2026-06-26T10:00:30Z".to_string(),
            child_device,
            agent_peer_id: "lan-device-router".to_string(),
            pairing_id: None,
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability: LanPairingDeviceReachability::Online,
            address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
            discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            evidence_sources: vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
            hint_sources: Vec::new(),
            service_identity_probe_evidence: Vec::new(),
        },
    ]));

    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable();
    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert_eq!(
        canonical.route_state,
        LanCanonicalHouseholdRouteState::Unavailable
    );
}

use std::collections::HashSet;

use ocentra_lan_core::read_model_builder::{
    build_lan_add_device_read_model, LanAddDeviceReadModelInput,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDiscoveryRuntimeStatus, LanPairingNetworkMode,
    LanPairingProductionDiscoveryState, LanPairingTrustState, LanTrustedDeviceRegistryEntry,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceSource, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceSource, LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
    LanPairingDiscoverySource, LanSelectedDeviceReadiness, LanServiceIdentityProbeEvidence,
    LanServiceIdentityProbeEvidenceKind,
};

#[test]
fn different_ocentra_device_ids_do_not_auto_merge_even_with_same_stable_mac() {
    let model = build_lan_add_device_read_model(lan_input(vec![
        discovery_device(
            "lan-device-alpha",
            None,
            "Printer Alpha",
            Some("printer-alpha.local"),
            Some(constants::lan_pairing::TEST_LAN_IP),
            Some(constants::lan_pairing::TEST_LAN_MAC),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
        discovery_device(
            "lan-device-bravo",
            None,
            "Printer Bravo",
            Some("printer-bravo.local"),
            Some("192.168.1.53"),
            Some(constants::lan_pairing::TEST_LAN_MAC),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        &[
            "dedupe-decision=forbidden",
            "conflicting-ocentra-device-id",
            "shared-stable-mac",
        ],
    );
    assert!(model.canonical_household_devices.iter().all(|device| {
        device
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind == LanDiscoveryEvidenceKind::MacAddress
                    && record.source == LanDiscoveryEvidenceSource::WindowsNeighborTable
            })
    }));
}

#[test]
fn different_manually_assigned_child_ids_do_not_auto_merge_even_with_same_mac() {
    let model = build_lan_add_device_read_model(lan_input(vec![
        discovery_device(
            "lan-device-child-alpha",
            Some("child-profile-alpha"),
            "Alpha Tablet",
            Some("tablet-alpha.local"),
            Some(constants::lan_pairing::TEST_LAN_IP),
            Some(constants::lan_pairing::TEST_LAN_MAC),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
        discovery_device(
            "lan-device-child-bravo",
            Some("child-profile-bravo"),
            "Bravo Tablet",
            Some("tablet-bravo.local"),
            Some("192.168.1.54"),
            Some(constants::lan_pairing::TEST_LAN_MAC),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        &[
            "dedupe-decision=forbidden",
            "conflicting-child-profile-id",
            "shared-stable-mac",
        ],
    );
    assert!(model.canonical_household_devices.iter().all(|device| {
        device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
            && device
                .network_identity
                .evidence_records
                .iter()
                .any(|record| {
                    record.evidence_kind == LanDiscoveryEvidenceKind::MacAddress
                        && record.source == LanDiscoveryEvidenceSource::WindowsNeighborTable
                })
    }));
}

#[test]
fn different_parent_assigned_child_ids_do_not_auto_merge_even_when_mdns_instance_matches() {
    let mut alpha = discovery_device(
        "lan-device-mdns-alpha",
        None,
        "Living Room TV",
        Some("living-room-tv.local"),
        Some("192.168.1.90"),
        None,
        vec![LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
    );
    alpha.service_identity_probe_evidence = vec![service_hint(
        LanServiceIdentityProbeEvidenceKind::MdnsInstanceName,
        "Living Room TV._airplay._tcp.local",
    )];

    let mut bravo = discovery_device(
        "lan-device-mdns-bravo",
        None,
        "Living Room TV",
        Some("living-room-tv.local"),
        Some("192.168.1.91"),
        None,
        vec![LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
    );
    bravo.service_identity_probe_evidence = alpha.service_identity_probe_evidence.clone();

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        household_device_decisions: vec![
            household_assignment_decision(
                "household-action-assign-alpha",
                &canonical_device_id_from_device_id("lan-device-mdns-alpha"),
                "child-profile-alpha",
            ),
            household_assignment_decision(
                "household-action-assign-bravo",
                &canonical_device_id_from_device_id("lan-device-mdns-bravo"),
                "child-profile-bravo",
            ),
        ],
        discovered_devices: vec![alpha, bravo],
        trusted_device_ids: vec![
            "lan-device-mdns-alpha".to_string(),
            "lan-device-mdns-bravo".to_string(),
        ],
        ..lan_input(Vec::new())
    });

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        &[
            "dedupe-decision=forbidden",
            "conflicting-child-profile-id",
            "shared-mdns-instance-name",
        ],
    );
    assert!(model.canonical_household_devices.iter().all(|device| {
        device.trust_state == LanPairingTrustState::Paired
            && device
                .network_identity
                .evidence_records
                .iter()
                .any(|record| {
                    record.evidence_kind == LanDiscoveryEvidenceKind::ParentDecision
                        && record.value == constants::lan_pairing::HOUSEHOLD_ACTION_ASSIGN
                })
    }));
}

#[test]
fn weak_hostname_overlap_stays_separate_and_keeps_weak_evidence() {
    let model = build_lan_add_device_read_model(lan_input(vec![
        discovery_device(
            "lan-device-hostname-alpha",
            None,
            "Speaker Alpha",
            Some("speaker.local"),
            Some("192.168.1.61"),
            None,
            vec![LanDiscoveryEvidenceSource::DnsCache],
        ),
        discovery_device(
            "lan-device-hostname-bravo",
            None,
            "Speaker Bravo",
            Some("speaker.local"),
            Some("192.168.1.62"),
            None,
            vec![LanDiscoveryEvidenceSource::DnsCache],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        &["dedupe-decision=manual-required", "shared-hostname"],
    );
    assert!(model.canonical_household_devices.iter().all(|device| {
        device
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind == LanDiscoveryEvidenceKind::Hostname
                    && record.source == LanDiscoveryEvidenceSource::DnsCache
                    && record.confidence == LanDiscoveryEvidenceConfidence::Weak
            })
    }));
}

#[test]
fn vendor_only_overlap_stays_separate() {
    let model = build_lan_add_device_read_model(lan_input(vec![
        discovery_device(
            "lan-device-vendor-alpha",
            None,
            "Speaker Alpha",
            None,
            Some("192.168.1.70"),
            Some("54-27-1e-97-c3-31"),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
        discovery_device(
            "lan-device-vendor-bravo",
            None,
            "Speaker Bravo",
            None,
            Some("192.168.1.71"),
            Some("54-27-1e-97-c3-32"),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        &["dedupe-decision=manual-required", "shared-vendor"],
    );
    assert!(model.canonical_household_devices.iter().all(|device| {
        device
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind == LanDiscoveryEvidenceKind::Vendor
                    && record.value == "AzureWave Technology Inc."
            })
    }));
}

#[test]
fn weak_device_type_only_overlap_stays_separate_without_ssdp_udn() {
    let mut alpha = discovery_device(
        "lan-device-type-alpha",
        None,
        "Renderer Alpha",
        None,
        Some("192.168.1.72"),
        None,
        vec![LanDiscoveryEvidenceSource::SsdpUpnpQuery],
    );
    alpha.service_identity_probe_evidence = vec![service_hint(
        LanServiceIdentityProbeEvidenceKind::SsdpDeviceType,
        "urn:schemas-upnp-org:device:MediaRenderer:1",
    )];

    let mut bravo = discovery_device(
        "lan-device-type-bravo",
        None,
        "Renderer Bravo",
        None,
        Some("192.168.1.73"),
        None,
        vec![LanDiscoveryEvidenceSource::SsdpUpnpQuery],
    );
    bravo.service_identity_probe_evidence = alpha.service_identity_probe_evidence.clone();

    let model = build_lan_add_device_read_model(lan_input(vec![alpha, bravo]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        &["dedupe-decision=manual-required", "shared-device-type"],
    );
    assert!(model.canonical_household_devices.iter().all(|device| {
        device
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
                    && record.value
                        == "ssdp-device-type:urn:schemas-upnp-org:device:MediaRenderer:1"
            })
    }));
}

#[test]
fn shared_install_id_merges_local_service_and_registry_device_when_canonical_ids_differ() {
    let mut local_service = discovery_device(
        "lan-device-install-merge",
        None,
        "Merge Tablet",
        Some("merge-tablet.local"),
        Some(constants::lan_pairing::TEST_LAN_IP),
        None,
        vec![LanDiscoveryEvidenceSource::LocalService],
    );
    local_service.agent_peer_id = constants::lan_pairing::PARENT_PEER_ID.to_string();
    local_service.address_ref = constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string();
    local_service.discovery_status = LanPairingDiscoveryRuntimeStatus::WebsocketDirect;
    local_service.child_device.agent_status =
        Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    local_service.child_device.install_id = Some("install-merge-1".to_string());

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        discovered_devices: vec![local_service],
        trusted_device_registry: vec![LanTrustedDeviceRegistryEntry {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            pairing_id: "pairing-install-merge-registry".to_string(),
            child_device: {
                let mut child_device =
                    ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
                        "lan-device-install-merge".to_string(),
                        Some("child-profile-install-merge".to_string()),
                        "Merge Tablet".to_string(),
                        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                    );
                child_device.hostname = Some("merge-tablet.local".to_string());
                child_device.network_interface =
                    Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
                child_device.install_id = Some("install-merge-1".to_string());
                child_device
            },
            parent_device: ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
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
        ..lan_input(Vec::new())
    });

    assert_eq!(model.canonical_household_devices.len(), 1);
    assert_model_has_dedupe_note(&model, &["dedupe-decision=automatic", "shared-install-id"]);
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind == LanDiscoveryEvidenceKind::InstallId)
        .any(|record| record.source == LanDiscoveryEvidenceSource::LocalService));
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind == LanDiscoveryEvidenceKind::InstallId)
        .any(|record| record.source == LanDiscoveryEvidenceSource::TrustedRegistry));
}

#[test]
fn shared_pairing_id_merges_local_service_and_registry_device_when_canonical_ids_differ() {
    let mut local_service = discovery_device(
        "lan-device-pairing-merge",
        None,
        "Merge Laptop",
        Some("merge-laptop.local"),
        Some(constants::lan_pairing::TEST_LAN_IP),
        None,
        vec![LanDiscoveryEvidenceSource::LocalService],
    );
    local_service.agent_peer_id = constants::lan_pairing::PARENT_PEER_ID.to_string();
    local_service.address_ref = constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string();
    local_service.discovery_status = LanPairingDiscoveryRuntimeStatus::WebsocketDirect;
    local_service.child_device.agent_status =
        Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    local_service.pairing_id = Some("pairing-merge-shared".to_string());

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        discovered_devices: vec![local_service],
        trusted_device_registry: vec![LanTrustedDeviceRegistryEntry {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            pairing_id: "pairing-merge-shared".to_string(),
            child_device: {
                let mut child_device =
                    ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
                        "lan-device-pairing-merge".to_string(),
                        Some("child-profile-pairing-merge".to_string()),
                        "Merge Laptop".to_string(),
                        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                    );
                child_device.hostname = Some("merge-laptop.local".to_string());
                child_device.network_interface =
                    Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
                child_device
            },
            parent_device: ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
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
        ..lan_input(Vec::new())
    });

    assert_eq!(model.canonical_household_devices.len(), 1);
    assert_model_has_dedupe_note(&model, &["dedupe-decision=automatic", "shared-pairing-id"]);
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind == LanDiscoveryEvidenceKind::PairingId)
        .any(|record| record.source == LanDiscoveryEvidenceSource::LocalService));
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind == LanDiscoveryEvidenceKind::PairingId)
        .any(|record| record.source == LanDiscoveryEvidenceSource::TrustedRegistry));
}

#[test]
fn shared_child_profile_id_merges_and_preserves_discovery_and_registry_evidence() {
    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T10:00:30Z".to_string(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: vec![discovery_device(
            "lan-device-merge",
            Some("child-profile-merge"),
            "Merge Tablet",
            Some("merge-tablet.local"),
            Some(constants::lan_pairing::TEST_LAN_IP),
            Some(constants::lan_pairing::TEST_LAN_MAC),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        )],
        pairing_requests: Vec::new(),
        trusted_device_registry: vec![LanTrustedDeviceRegistryEntry {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            pairing_id: "pairing-merge-child-profile".to_string(),
            child_device: {
                let mut child_device =
                    ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
                        "lan-device-merge".to_string(),
                        Some("child-profile-merge".to_string()),
                        "Merge Tablet".to_string(),
                        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                    );
                child_device.ip_address = Some("192.168.1.62".to_string());
                child_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
                child_device.hostname = Some("merge-tablet.local".to_string());
                child_device.network_interface =
                    Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
                child_device
            },
            parent_device: ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
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
            action_id: "household-action-rename-merge".to_string(),
            action_kind: LanHouseholdDeviceActionKind::Rename,
            canonical_device_id: "lan-child-profile-childprofilemerge".to_string(),
            child_profile_id: Some("child-profile-merge".to_string()),
            display_name: Some("Merge Tablet Renamed".to_string()),
            device_kind: None,
            parent_actor_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
            decided_at: "2026-06-26T10:00:10Z".to_string(),
            revoked_at: None,
        }],
        trusted_device_ids: vec!["lan-device-merge".to_string()],
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
    let device = &model.canonical_household_devices[0];
    assert!(dedupe_notes(device)
        .iter()
        .any(|note| note.contains("dedupe-decision=automatic")));
    assert_eq!(
        device.classification,
        LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
    assert_eq!(device.trust_state, LanPairingTrustState::Paired);
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::NetworkNeighbor));
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::TrustedRegistry));
    assert!(device
        .network_identity
        .ip_addresses
        .contains(&"192.168.1.62".to_string()));
    assert!(device
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.evidence_kind == LanDiscoveryEvidenceKind::ParentDecision
                && record.value == constants::lan_pairing::HOUSEHOLD_ACTION_RENAME
                && record.note.as_deref() == Some("Merge Tablet Renamed")
        }));
    assert!(device
        .network_identity
        .evidence_records
        .iter()
        .any(
            |record| record.source == LanDiscoveryEvidenceSource::WindowsNeighborTable
                && record.evidence_kind == LanDiscoveryEvidenceKind::MacAddress
        ));
    assert!(device
        .network_identity
        .evidence_records
        .iter()
        .any(
            |record| record.source == LanDiscoveryEvidenceSource::TrustedRegistry
                && record.evidence_kind == LanDiscoveryEvidenceKind::TrustedRegistry
        ));
}

#[test]
fn discovery_and_registry_with_different_device_ids_do_not_merge_even_with_same_mac() {
    let mut model_input = lan_input(vec![discovery_device(
        "lan-device-alpha",
        None,
        "Printer Alpha",
        Some("printer-alpha.local"),
        Some(constants::lan_pairing::TEST_LAN_IP),
        Some(constants::lan_pairing::TEST_LAN_MAC),
        vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
    )]);
    model_input.trusted_device_registry = vec![LanTrustedDeviceRegistryEntry {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: "pairing-merge-conflict".to_string(),
        child_device: {
            let mut child_device =
                ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
                    "lan-device-bravo".to_string(),
                    None,
                    "Printer Bravo".to_string(),
                    constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                );
            child_device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
            child_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
            child_device.hostname = Some("printer-alpha.local".to_string());
            child_device.network_interface =
                Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
            child_device
        },
        parent_device: ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
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
    }];

    let model = build_lan_add_device_read_model(model_input);

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        &[
            "dedupe-decision=forbidden",
            "conflicting-ocentra-device-id",
            "shared-stable-mac",
        ],
    );
    assert!(model.canonical_household_devices.iter().any(|device| {
        device
            .source_labels
            .contains(&LanCanonicalHouseholdDeviceSource::TrustedRegistry)
            && device.trust_state == LanPairingTrustState::Paired
    }));
}

#[test]
fn trusted_registry_ip_reuse_by_different_device_stays_separate() {
    let mut model_input = lan_input(vec![discovery_device(
        "lan-device-ip-reuse",
        None,
        "Camera Reuse",
        Some("camera-reuse.local"),
        Some(constants::lan_pairing::TEST_LAN_IP),
        Some("02:11:22:33:44:55"),
        vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
    )]);
    model_input.trusted_device_registry = vec![LanTrustedDeviceRegistryEntry {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: "pairing-ip-reuse".to_string(),
        child_device: {
            let mut child_device =
                ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
                    "lan-device-paired".to_string(),
                    None,
                    "Paired Console".to_string(),
                    constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                );
            child_device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
            child_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
            child_device.hostname = Some("paired-console.local".to_string());
            child_device.network_interface =
                Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
            child_device
        },
        parent_device: ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
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
    }];

    let model = build_lan_add_device_read_model(model_input);

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        &[
            "dedupe-decision=forbidden",
            "conflicting-ocentra-device-id",
            "shared-ip-address",
        ],
    );
    assert!(model.canonical_household_devices.iter().any(|device| {
        device
            .source_labels
            .contains(&LanCanonicalHouseholdDeviceSource::TrustedRegistry)
            && device.network_identity.mac_address.as_deref()
                == Some(constants::lan_pairing::TEST_LAN_MAC)
    }));
}

#[test]
fn weak_ip_only_overlap_stays_separate() {
    let model = build_lan_add_device_read_model(lan_input(vec![
        discovery_device(
            "lan-device-ip-alpha",
            None,
            "Sensor Alpha",
            None,
            Some("192.168.1.80"),
            None,
            vec![LanDiscoveryEvidenceSource::DnsCache],
        ),
        discovery_device(
            "lan-device-ip-bravo",
            None,
            "Sensor Bravo",
            None,
            Some("192.168.1.80"),
            None,
            vec![LanDiscoveryEvidenceSource::DnsCache],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert_model_has_dedupe_note(
        &model,
        &["dedupe-decision=manual-required", "shared-ip-address"],
    );
    assert!(model.canonical_household_devices.iter().all(|device| {
        device
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind == LanDiscoveryEvidenceKind::IpAddress
                    && record.confidence == LanDiscoveryEvidenceConfidence::Strong
            })
    }));
}

#[test]
fn mdns_instance_name_merges_same_device_across_dhcp_renewal() {
    let mut alpha = discovery_device(
        "lan-device-mdns-alpha",
        None,
        "Living Room TV",
        Some("living-room-tv.local"),
        Some("192.168.1.90"),
        None,
        vec![LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
    );
    alpha.service_identity_probe_evidence = vec![
        service_hint(
            LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
            "_airplay._tcp.local",
        ),
        service_hint(
            LanServiceIdentityProbeEvidenceKind::MdnsInstanceName,
            "Living Room TV._airplay._tcp.local",
        ),
    ];

    let mut bravo = discovery_device(
        "lan-device-mdns-bravo",
        None,
        "Living Room TV",
        Some("living-room-tv.local"),
        Some("192.168.1.91"),
        None,
        vec![LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
    );
    bravo.service_identity_probe_evidence = alpha.service_identity_probe_evidence.clone();

    let model = build_lan_add_device_read_model(lan_input(vec![alpha, bravo]));

    assert_eq!(model.canonical_household_devices.len(), 1);
    assert_model_has_dedupe_note(
        &model,
        &["dedupe-decision=automatic", "shared-mdns-instance-name"],
    );
    let device = &model.canonical_household_devices[0];
    assert!(device
        .network_identity
        .ip_addresses
        .contains(&"192.168.1.90".to_string()));
    assert!(device
        .network_identity
        .ip_addresses
        .contains(&"192.168.1.91".to_string()));
    assert!(device
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Strong
                && record.value == "mdns-instance-name:Living Room TV._airplay._tcp.local"
        }));
}

#[test]
fn ssdp_udn_merges_same_device_even_when_neighbor_device_ids_differ() {
    let mut alpha = discovery_device(
        "lan-device-ssdp-alpha",
        None,
        "Media Renderer",
        Some("renderer.local"),
        Some("192.168.1.100"),
        None,
        vec![LanDiscoveryEvidenceSource::SsdpUpnpQuery],
    );
    alpha.service_identity_probe_evidence = vec![
        service_hint(
            LanServiceIdentityProbeEvidenceKind::SsdpDeviceType,
            "urn:schemas-upnp-org:device:MediaRenderer:1",
        ),
        service_hint(
            LanServiceIdentityProbeEvidenceKind::SsdpUdn,
            "uuid:media-renderer-1",
        ),
    ];

    let mut bravo = discovery_device(
        "lan-device-ssdp-bravo",
        None,
        "Media Renderer",
        Some("renderer.local"),
        Some("192.168.1.101"),
        None,
        vec![LanDiscoveryEvidenceSource::SsdpUpnpQuery],
    );
    bravo.service_identity_probe_evidence = alpha.service_identity_probe_evidence.clone();

    let model = build_lan_add_device_read_model(lan_input(vec![alpha, bravo]));

    assert_eq!(model.canonical_household_devices.len(), 1);
    assert_model_has_dedupe_note(&model, &["dedupe-decision=automatic", "shared-ssdp-udn"]);
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Strong
                && record.value == "ssdp-udn:uuid:media-renderer-1"
        }));
}

fn canonical_ids_are_unique(
    model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
) -> bool {
    let ids = model
        .canonical_household_devices
        .iter()
        .map(|device| device.canonical_device_id.as_str())
        .collect::<HashSet<_>>();
    ids.len() == model.canonical_household_devices.len()
}

fn assert_model_has_dedupe_note(
    model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
    fragments: &[&str],
) {
    assert!(
        model.canonical_household_devices.iter().any(|device| {
            dedupe_notes(device)
                .iter()
                .any(|note| fragments.iter().all(|fragment| note.contains(fragment)))
        }),
        "expected a dedupe decision note containing {:?}",
        fragments
    );
}

fn dedupe_notes(
    device: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice,
) -> Vec<&str> {
    device
        .network_identity
        .evidence_records
        .iter()
        .filter_map(|record| record.note.as_deref())
        .filter(|note| note.contains("dedupe-decision="))
        .collect()
}

fn lan_input(
    discovered_devices: Vec<LanBrowserAddDeviceDiscoveryDevice>,
) -> LanAddDeviceReadModelInput {
    LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T10:00:30Z".to_string(),
        discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices,
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
    }
}

fn discovery_device(
    device_id: &str,
    child_profile_id: Option<&str>,
    label: &str,
    hostname: Option<&str>,
    ip_address: Option<&str>,
    mac_address: Option<&str>,
    evidence_sources: Vec<LanDiscoveryEvidenceSource>,
) -> LanBrowserAddDeviceDiscoveryDevice {
    let mut child_device = ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
        device_id.to_string(),
        child_profile_id.map(ToString::to_string),
        label.to_string(),
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );
    child_device.hostname = hostname.map(ToString::to_string);
    child_device.ip_address = ip_address.map(ToString::to_string);
    child_device.mac_address = mac_address.map(ToString::to_string);
    child_device.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: "2026-06-26T10:00:30Z".to_string(),
        child_device,
        agent_peer_id: device_id.to_string(),
        pairing_id: None,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
        discovery_status: ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        evidence_sources,
        hint_sources: Vec::new(),
        service_identity_probe_evidence: Vec::new(),
    }
}

fn service_hint(
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: &str,
) -> LanServiceIdentityProbeEvidence {
    LanServiceIdentityProbeEvidence {
        evidence_kind,
        value: value.to_string(),
        selected_interface: None,
    }
}

fn household_assignment_decision(
    action_id: &str,
    canonical_device_id: &str,
    child_profile_id: &str,
) -> LanHouseholdDeviceDecision {
    LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: action_id.to_string(),
        action_kind: LanHouseholdDeviceActionKind::Assign,
        canonical_device_id: canonical_device_id.to_string(),
        child_profile_id: Some(child_profile_id.to_string()),
        display_name: None,
        device_kind: None,
        parent_actor_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        decided_at: "2026-06-26T10:00:10Z".to_string(),
        revoked_at: None,
    }
}

fn canonical_device_id_from_device_id(device_id: &str) -> String {
    let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_ID_PREFIX);
    id.push_str(
        &device_id
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<String>(),
    );
    id
}

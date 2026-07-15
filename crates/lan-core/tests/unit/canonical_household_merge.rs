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

#[path = "canonical_household_merge_registry.rs"]
mod canonical_household_merge_registry;

#[test]
fn different_ocentra_device_ids_do_not_auto_merge_even_with_same_stable_mac() {
    let model = build_lan_add_device_read_model(lan_input(vec![
        discovery_device(
            "lan-device-alpha",
            None::<&str>,
            "Printer Alpha",
            Some("printer-alpha.local"),
            Some(constants::lan_pairing::TEST_LAN_IP),
            Some(constants::lan_pairing::TEST_LAN_MAC),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
        discovery_device(
            "lan-device-bravo",
            None::<&str>,
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
        [
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
        [
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
    let mut canonical_alpha = String::from(constants::lan_pairing::CANONICAL_DEVICE_ID_PREFIX);
    canonical_alpha.push_str(
        &"lan-device-mdns-alpha"
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<String>(),
    );
    let mut canonical_bravo = String::from(constants::lan_pairing::CANONICAL_DEVICE_ID_PREFIX);
    canonical_bravo.push_str(
        &"lan-device-mdns-bravo"
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<String>(),
    );
    let mut alpha = discovery_device(
        "lan-device-mdns-alpha",
        None::<&str>,
        "Living Room TV",
        Some("living-room-tv.local"),
        Some("192.168.1.90"),
        None::<&str>,
        vec![LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
    );
    alpha.service_identity_probe_evidence = vec![service_hint(
        LanServiceIdentityProbeEvidenceKind::MdnsInstanceName,
        "Living Room TV._airplay._tcp.local",
    )];

    let mut bravo = discovery_device(
        "lan-device-mdns-bravo",
        None::<&str>,
        "Living Room TV",
        Some("living-room-tv.local"),
        Some("192.168.1.91"),
        None::<&str>,
        vec![LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
    );
    bravo.service_identity_probe_evidence = alpha.service_identity_probe_evidence.clone();

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        household_device_decisions: vec![
            household_assignment_decision(
                "household-action-assign-alpha",
                canonical_alpha,
                "child-profile-alpha",
            ),
            household_assignment_decision(
                "household-action-assign-bravo",
                canonical_bravo,
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
        [
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
            None::<&str>,
            "Speaker Alpha",
            Some("speaker.local"),
            Some("192.168.1.61"),
            None::<&str>,
            vec![LanDiscoveryEvidenceSource::DnsCache],
        ),
        discovery_device(
            "lan-device-hostname-bravo",
            None::<&str>,
            "Speaker Bravo",
            Some("speaker.local"),
            Some("192.168.1.62"),
            None::<&str>,
            vec![LanDiscoveryEvidenceSource::DnsCache],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        ["dedupe-decision=manual-required", "shared-hostname"],
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
            None::<&str>,
            "Speaker Alpha",
            None::<&str>,
            Some("192.168.1.70"),
            Some("54-27-1e-97-c3-31"),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
        discovery_device(
            "lan-device-vendor-bravo",
            None::<&str>,
            "Speaker Bravo",
            None::<&str>,
            Some("192.168.1.71"),
            Some("54-27-1e-97-c3-32"),
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(&model, ["dedupe-decision=manual-required", "shared-vendor"]);
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
        None::<&str>,
        "Renderer Alpha",
        None::<&str>,
        Some("192.168.1.72"),
        None::<&str>,
        vec![LanDiscoveryEvidenceSource::SsdpUpnpQuery],
    );
    alpha.service_identity_probe_evidence = vec![service_hint(
        LanServiceIdentityProbeEvidenceKind::SsdpDeviceType,
        "urn:schemas-upnp-org:device:MediaRenderer:1",
    )];

    let mut bravo = discovery_device(
        "lan-device-type-bravo",
        None::<&str>,
        "Renderer Bravo",
        None::<&str>,
        Some("192.168.1.73"),
        None::<&str>,
        vec![LanDiscoveryEvidenceSource::SsdpUpnpQuery],
    );
    bravo.service_identity_probe_evidence = alpha.service_identity_probe_evidence.clone();

    let model = build_lan_add_device_read_model(lan_input(vec![alpha, bravo]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(canonical_ids_are_unique(&model));
    assert_model_has_dedupe_note(
        &model,
        ["dedupe-decision=manual-required", "shared-device-type"],
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
        None::<&str>,
        "Merge Tablet",
        Some("merge-tablet.local"),
        Some(constants::lan_pairing::TEST_LAN_IP),
        None::<&str>,
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
    assert_model_has_dedupe_note(&model, ["dedupe-decision=automatic", "shared-install-id"]);
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
        None::<&str>,
        "Merge Laptop",
        Some("merge-laptop.local"),
        Some(constants::lan_pairing::TEST_LAN_IP),
        None::<&str>,
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
                None::<String>,
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
    assert_model_has_dedupe_note(&model, ["dedupe-decision=automatic", "shared-pairing-id"]);
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
fn discovery_and_registry_with_different_device_ids_do_not_merge_even_with_same_mac() {
    let mut model_input = lan_input(vec![discovery_device(
        "lan-device-alpha",
        None::<&str>,
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
                    None::<String>,
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
            None::<String>,
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
        [
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
        None::<&str>,
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
            None::<String>,
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
        [
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
            None::<&str>,
            "Sensor Alpha",
            None::<&str>,
            Some("192.168.1.80"),
            None::<&str>,
            vec![LanDiscoveryEvidenceSource::DnsCache],
        ),
        discovery_device(
            "lan-device-ip-bravo",
            None::<&str>,
            "Sensor Bravo",
            None::<&str>,
            Some("192.168.1.80"),
            None::<&str>,
            vec![LanDiscoveryEvidenceSource::DnsCache],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert_model_has_dedupe_note(
        &model,
        ["dedupe-decision=manual-required", "shared-ip-address"],
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
        None::<&str>,
        "Living Room TV",
        Some("living-room-tv.local"),
        Some("192.168.1.90"),
        None::<&str>,
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
        None::<&str>,
        "Living Room TV",
        Some("living-room-tv.local"),
        Some("192.168.1.91"),
        None::<&str>,
        vec![LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
    );
    bravo.service_identity_probe_evidence = alpha.service_identity_probe_evidence.clone();

    let model = build_lan_add_device_read_model(lan_input(vec![alpha, bravo]));

    assert_eq!(model.canonical_household_devices.len(), 1);
    assert_model_has_dedupe_note(
        &model,
        ["dedupe-decision=automatic", "shared-mdns-instance-name"],
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
        None::<&str>,
        "Media Renderer",
        Some("renderer.local"),
        Some("192.168.1.100"),
        None::<&str>,
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
        None::<&str>,
        "Media Renderer",
        Some("renderer.local"),
        Some("192.168.1.101"),
        None::<&str>,
        vec![LanDiscoveryEvidenceSource::SsdpUpnpQuery],
    );
    bravo.service_identity_probe_evidence = alpha.service_identity_probe_evidence.clone();

    let model = build_lan_add_device_read_model(lan_input(vec![alpha, bravo]));

    assert_eq!(model.canonical_household_devices.len(), 1);
    assert_model_has_dedupe_note(&model, ["dedupe-decision=automatic", "shared-ssdp-udn"]);
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
    fragments: impl IntoIterator<Item = impl std::fmt::Display>,
) {
    let fragments = fragments
        .into_iter()
        .map(|fragment| fragment.to_string())
        .collect::<Vec<_>>();
    assert!(
        model
            .canonical_household_devices
            .iter()
            .any(|device| has_dedupe_note(device, &fragments,)),
        "expected a dedupe decision note containing {:?}",
        fragments
    );
}

fn has_dedupe_note(
    device: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice,
    fragments: impl IntoIterator<Item = impl std::fmt::Display>,
) -> bool {
    let fragments = fragments
        .into_iter()
        .map(|fragment| fragment.to_string())
        .collect::<Vec<_>>();
    device
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.note.as_ref().is_some_and(|note| {
                note.contains("dedupe-decision=")
                    && fragments.iter().all(|fragment| note.contains(fragment))
            })
        })
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
    device_id: impl std::fmt::Display,
    child_profile_id: Option<impl std::fmt::Display>,
    label: impl std::fmt::Display,
    hostname: Option<impl std::fmt::Display>,
    ip_address: Option<impl std::fmt::Display>,
    mac_address: Option<impl std::fmt::Display>,
    evidence_sources: Vec<LanDiscoveryEvidenceSource>,
) -> LanBrowserAddDeviceDiscoveryDevice {
    let device_id = device_id.to_string();
    let label = label.to_string();
    let mut child_device = ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
        device_id.clone(),
        child_profile_id.map(|value| value.to_string()),
        label,
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );
    child_device.hostname = hostname.map(|value| value.to_string());
    child_device.ip_address = ip_address.map(|value| value.to_string());
    child_device.mac_address = mac_address.map(|value| value.to_string());
    child_device.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: "2026-06-26T10:00:30Z".to_string(),
        child_device,
        agent_peer_id: device_id,
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
    value: impl std::fmt::Display,
) -> LanServiceIdentityProbeEvidence {
    LanServiceIdentityProbeEvidence {
        evidence_kind,
        value: value.to_string(),
        selected_interface: None,
    }
}

fn household_assignment_decision(
    action_id: impl std::fmt::Display,
    canonical_device_id: impl std::fmt::Display,
    child_profile_id: impl std::fmt::Display,
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

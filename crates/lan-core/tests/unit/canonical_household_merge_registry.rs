use super::*;

fn merged_child_profile_input() -> LanAddDeviceReadModelInput {
    LanAddDeviceReadModelInput {
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
    }
}

fn assert_merged_child_profile_evidence(
    device: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice,
) {
    assert!(has_dedupe_note(device, ["dedupe-decision=automatic"]));
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
}

fn assert_merged_child_profile_records(
    device: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice,
) {
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
fn shared_child_profile_id_merges_and_preserves_discovery_and_registry_evidence() {
    let model = build_lan_add_device_read_model(merged_child_profile_input());
    assert_eq!(model.canonical_household_devices.len(), 1);
    let device = &model.canonical_household_devices[0];
    assert_merged_child_profile_evidence(device);
    assert_merged_child_profile_records(device);
}

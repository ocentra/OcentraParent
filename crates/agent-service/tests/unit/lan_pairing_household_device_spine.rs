use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdDeviceConfidence,
    LanCanonicalHouseholdDeviceRole, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdSurface, LanHouseholdDeviceActionKind,
};

use crate::app::lan_pairing_household_device_spine::canonical_household_devices;
use crate::lan_pairing_household_device_spine_test_fixtures::{
    expected_test_mac_canonical_id, household_decision, household_restore_decision,
    ip_only_neighbor, local_agent_discovery_device, router_neighbor, same_host_network_neighbor,
    trusted_registry_entry,
};
use crate::test_require_some::require_some;

#[test]
fn local_agent_and_neighbor_merge_into_one_canonical_physical_device() {
    let devices = canonical_household_devices(
        &[local_agent_discovery_device(), same_host_network_neighbor()],
        &[],
        &[],
    );

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(device.canonical_device_id, expected_test_mac_canonical_id());
    assert_eq!(
        device.classification,
        LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
    assert_eq!(
        device.network_identity.confidence,
        LanCanonicalHouseholdDeviceConfidence::MacIpMatch
    );
    assert_eq!(
        device.network_identity.ip_addresses,
        vec![constants::lan_pairing::TEST_LAN_IP.to_string()]
    );
    assert_eq!(
        device.source_labels.as_slice(),
        vec![
            LanCanonicalHouseholdDeviceSource::LocalService,
            LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        ]
        .as_slice()
    );
    assert_eq!(
        device.role_badges.as_slice(),
        vec![
            LanCanonicalHouseholdDeviceRole::ChildAgent,
            LanCanonicalHouseholdDeviceRole::Portal,
            LanCanonicalHouseholdDeviceRole::ParentController,
        ]
        .as_slice()
    );
    assert_eq!(
        device
            .child_agent_inventory
            .as_ref()
            .map(|inventory| inventory.device_name.as_str()),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
}

#[test]
fn router_neighbor_stays_visible_but_not_enrollable() {
    let devices = canonical_household_devices(&[router_neighbor()], &[], &[]);

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(
        device.classification,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
    );
    assert!(!device.enrollable);
    assert_eq!(device.role_badges.as_slice(), [].as_slice());
    assert!(device.child_agent_inventory.as_ref().is_none());
    assert_eq!(
        device.policy_target_surfaces,
        vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Network
        ]
    );
}

#[test]
fn passive_neighbors_do_not_merge_on_ip_only_identity() {
    let devices = canonical_household_devices(
        &[
            ip_only_neighbor(
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME,
                constants::lan_pairing::PLATFORM_UNKNOWN,
                constants::lan_pairing::LOCAL_AGENT_DEVICE_ID,
            ),
            ip_only_neighbor(
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_UNKNOWN,
                constants::lan_pairing::CHILD_DEVICE_ID,
            ),
        ],
        &[],
        &[],
    );

    assert_eq!(devices.len(), 2);
    assert!(devices
        .iter()
        .all(|device| device.network_identity.mac_address.is_none()));
    assert!(devices
        .iter()
        .any(|device| device.network_identity.confidence
            == LanCanonicalHouseholdDeviceConfidence::ManualRequired));
    assert!(devices.iter().any(|device| {
        device
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.note.as_deref().is_some_and(|note| {
                    note.contains("dedupe-decision=manual-required")
                        && note.contains("shared-ip-address")
                })
            })
    }));
}

#[test]
fn child_agent_and_neighbor_may_merge_on_ip_when_agent_evidence_exists() {
    let mut child_agent = local_agent_discovery_device();
    child_agent.child_device.mac_address = None;
    let mut neighbor = ip_only_neighbor(
        constants::lan_pairing::TEST_LAN_IP,
        constants::lan_pairing::TEST_HOSTNAME,
        constants::lan_pairing::PLATFORM_UNKNOWN,
        constants::lan_pairing::CHILD_DEVICE_ID,
    );
    neighbor.child_device.mac_address = None;

    let devices = canonical_household_devices(&[child_agent, neighbor], &[], &[]);

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(
        device.classification,
        LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
    assert_eq!(
        device.network_identity.confidence,
        LanCanonicalHouseholdDeviceConfidence::MacIpMatch
    );
    assert_eq!(
        device.source_labels.as_slice(),
        vec![
            LanCanonicalHouseholdDeviceSource::LocalService,
            LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        ]
        .as_slice()
    );
}

#[test]
fn local_agent_and_registry_merge_on_explicit_install_and_pairing_keys() {
    let mut local_agent = local_agent_discovery_device();
    local_agent.child_device.mac_address = None;
    local_agent.child_device.child_profile_id = None;
    local_agent.child_device.install_id = Some("fixture-install-merge".to_string());
    local_agent.pairing_id = Some("fixture-pairing-merge".to_string());

    let mut trusted = trusted_registry_entry();
    trusted.child_device.device_id = local_agent.child_device.device_id.clone();
    trusted.child_device.mac_address = None;
    trusted.child_device.child_profile_id = Some("fixture-child-profile-merge".to_string());
    trusted.child_device.install_id = Some("fixture-install-merge".to_string());
    trusted.pairing_id = "fixture-pairing-merge".to_string();

    let devices = canonical_household_devices(&[local_agent], &[trusted], &[]);

    assert_eq!(devices.len(), 1);
    let evidence = &devices[0].network_identity.evidence_records;
    assert!(evidence.iter().any(|record| {
        record.evidence_kind
            == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::InstallId
    }));
    assert!(evidence.iter().any(|record| {
        record.evidence_kind
            == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::PairingId
    }));
}

#[test]
fn trusted_registry_device_remains_available_to_product_target_surfaces() {
    let devices = canonical_household_devices(&[], &[trusted_registry_entry()], &[]);

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(device.trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        device.discovery_state,
        LanPairingProductionDiscoveryState::Paired
    );
    assert_eq!(
        device.policy_target_surfaces.as_slice(),
        vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Policy,
            LanCanonicalHouseholdSurface::Browser,
            LanCanonicalHouseholdSurface::App,
            LanCanonicalHouseholdSurface::Screen,
            LanCanonicalHouseholdSurface::Network,
            LanCanonicalHouseholdSurface::Activity,
            LanCanonicalHouseholdSurface::Tracking,
            LanCanonicalHouseholdSurface::Ai,
        ]
        .as_slice()
    );
    let inventory = require_some(
        device.child_agent_inventory.as_ref(),
        "paired trusted registry child agent inventory",
    );
    assert_eq!(inventory.pairing_trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        inventory.route_state,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState::LocalNetwork
    );
}

#[test]
fn parent_rename_decision_updates_canonical_display_name_with_evidence() {
    let canonical_device_id = expected_test_mac_canonical_id();
    let devices = canonical_household_devices(
        &[local_agent_discovery_device()],
        &[],
        &[household_decision(
            LanHouseholdDeviceActionKind::Rename,
            &canonical_device_id,
            Some(constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL.to_string()),
        )],
    );

    let device = &devices[0];
    assert_eq!(
        device.display_name,
        constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL
    );
    let expected_merge_key = format!(
        "{}{}",
        constants::lan_pairing::LAN_EVIDENCE_KEY_PARENT_DECISION_PREFIX,
        constants::lan_pairing::HOUSEHOLD_ACTION_ID
    );
    assert_eq!(
        device
            .network_identity
            .evidence_records
            .last()
            .map(|record| record.merge_key.as_str()),
        Some(expected_merge_key.as_str())
    );
}

#[test]
fn parent_rename_decision_survives_same_mac_rescan_with_new_ip_and_weaker_neighbor_label() {
    let renamed_label = constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL;
    let initial_devices = canonical_household_devices(&[same_host_network_neighbor()], &[], &[]);
    let canonical_device_id = initial_devices[0].canonical_device_id.clone();
    let mut rescanned_neighbor = same_host_network_neighbor();
    rescanned_neighbor.child_device.ip_address = Some("192.168.0.77".to_string());
    rescanned_neighbor.child_device.hostname =
        Some(constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME.to_string());
    rescanned_neighbor.child_device.label = "LAN 192.168.0.77".to_string();
    rescanned_neighbor.child_device.platform = constants::lan_pairing::PLATFORM_UNKNOWN.to_string();

    let devices = canonical_household_devices(
        &[rescanned_neighbor],
        &[],
        &[household_decision(
            LanHouseholdDeviceActionKind::Rename,
            &canonical_device_id,
            Some(renamed_label.to_string()),
        )],
    );

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(device.canonical_device_id, canonical_device_id);
    assert_eq!(device.display_name, renamed_label);
    assert_eq!(
        device.network_identity.ip_addresses,
        vec!["192.168.0.77".to_string()]
    );
    assert_eq!(
        device.network_identity.mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert_eq!(
        device.discovery_state,
        LanPairingProductionDiscoveryState::Discovered
    );
}

#[test]
fn parent_ignore_and_restore_decisions_change_enrollment_state() {
    let canonical_device_id = expected_test_mac_canonical_id();
    let ignored = canonical_household_devices(
        &[local_agent_discovery_device()],
        &[],
        &[household_decision(
            LanHouseholdDeviceActionKind::Ignore,
            &canonical_device_id,
            None,
        )],
    );

    assert_eq!(
        ignored[0].discovery_state,
        LanPairingProductionDiscoveryState::Revoked
    );
    assert_eq!(ignored[0].trust_state, LanPairingTrustState::Revoked);
    assert!(!ignored[0].enrollable);

    let restored = canonical_household_devices(
        &[local_agent_discovery_device()],
        &[],
        &[
            household_decision(
                LanHouseholdDeviceActionKind::Ignore,
                &canonical_device_id,
                None,
            ),
            household_restore_decision(&canonical_device_id),
        ],
    );

    assert_eq!(
        restored[0].discovery_state,
        LanPairingProductionDiscoveryState::Discovered
    );
    assert_eq!(restored[0].trust_state, LanPairingTrustState::Unpaired);
    assert!(restored[0].enrollable);
}

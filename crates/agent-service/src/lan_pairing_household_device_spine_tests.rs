use ocentra_parent_agent_protocol::{
    constants, LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdDeviceConfidence,
    LanCanonicalHouseholdDeviceRole, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdSurface, LanHouseholdDeviceActionKind, LanPairingProductionDiscoveryState,
    LanPairingTrustState,
};

use crate::lan_pairing_household_device_spine::canonical_household_devices;
use crate::lan_pairing_household_device_spine_test_fixtures::{
    expected_test_mac_canonical_id, household_decision, household_restore_decision,
    ip_only_neighbor, local_agent_discovery_device, router_neighbor, same_host_network_neighbor,
    trusted_registry_entry,
};

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
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::LocalService));
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::NetworkNeighbor));
    assert!(device
        .role_badges
        .contains(&LanCanonicalHouseholdDeviceRole::Portal));
    assert!(device
        .role_badges
        .contains(&LanCanonicalHouseholdDeviceRole::ParentController));
    assert!(device.child_agent_inventory.is_some());
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
    assert!(device.role_badges.is_empty());
    assert!(device.child_agent_inventory.is_none());
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
        .all(|device| device.network_identity.confidence
            == LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor));
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
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::LocalService));
    assert!(device
        .source_labels
        .contains(&LanCanonicalHouseholdDeviceSource::NetworkNeighbor));
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
    for surface in [
        LanCanonicalHouseholdSurface::Policy,
        LanCanonicalHouseholdSurface::Activity,
        LanCanonicalHouseholdSurface::Network,
        LanCanonicalHouseholdSurface::Tracking,
        LanCanonicalHouseholdSurface::Ai,
    ] {
        assert!(device.policy_target_surfaces.contains(&surface));
    }
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
            Some(constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL),
        )],
    );

    let device = &devices[0];
    assert_eq!(
        device.display_name,
        constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL
    );
    assert!(device
        .network_identity
        .evidence_records
        .iter()
        .any(|record| record
            .merge_key
            .contains(constants::lan_pairing::HOUSEHOLD_ACTION_ID)));
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

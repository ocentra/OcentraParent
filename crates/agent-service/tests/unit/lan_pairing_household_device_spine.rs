use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdDeviceConfidence,
    LanCanonicalHouseholdDeviceRole, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdSurface, LanHouseholdDeviceActionKind,
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
    assert!(matches!(device.child_agent_inventory.as_ref(), Some(_)));
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
    assert!(matches!(device.child_agent_inventory.as_ref(), None));
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

use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::fs;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use ocentra_lan_core::network_inventory::macos_neighbors::*;
use ocentra_lan_core::network_inventory::neighbor_support::filter_neighbor_observations_for_selected_interface;
use ocentra_lan_core::network_inventory::{
    LanIdentityHintInventory, LanNeighborObservation, LanNetworkInventoryDevice,
    LanPreviousNetworkInventory,
};

const TEST_NEIGHBOR_OBSERVED_AT: &str = "2026-06-28T12:00:00Z";

#[test]
fn macos_basic_fixture_preserves_named_and_unnamed_neighbors() {
    let observations = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/macos_arp_a_basic.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .filter_map(|line| macos_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT))
    .collect::<Vec<_>>();

    assert_eq!(observations.len(), 2);
    assert_eq!(observations[0].ip_address, "192.168.2.1");
    assert_eq!(observations[0].mac_address, "00-11-22-33-44-55");
    assert_eq!(observations[0].hostname, None);
    assert_eq!(observations[0].observed_at, TEST_NEIGHBOR_OBSERVED_AT);
    assert_eq!(observations[1].hostname.as_deref(), Some("iphone.local"));
    assert_eq!(
        observations[1].scan_sources,
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()]
    );
}

#[test]
fn macos_empty_incomplete_and_malformed_fixtures_produce_no_rows() {
    let empty = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/macos_arp_a_empty.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .filter_map(|line| macos_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT))
    .collect::<Vec<_>>();
    let incomplete = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/macos_arp_a_incomplete.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .filter_map(|line| macos_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT))
    .collect::<Vec<_>>();
    let malformed = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/macos_arp_a_malformed.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .filter_map(|line| macos_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT))
    .collect::<Vec<_>>();

    assert!(empty.is_empty());
    assert!(incomplete.is_empty());
    assert!(malformed.is_empty());
}

#[test]
fn macos_duplicate_fixture_keeps_duplicate_candidates_until_merge() {
    let parsed = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/macos_arp_a_duplicate.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .filter_map(|line| macos_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT))
    .collect::<Vec<_>>();

    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].mac_address, parsed[1].mac_address);
    assert_ne!(parsed[0].ip_address, parsed[1].ip_address);

    let merged = ocentra_lan_core::network_inventory::merge_neighbor_observations_by_mac(parsed);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].ip_address, "192.168.2.46");
    assert_eq!(merged[0].hostname.as_deref(), Some("iphone.local"));
}

#[test]
fn macos_arp_parser_accepts_named_private_neighbor() {
    let observation = macos_arp_observation_with_observed_at(
        "iphone.local (192.168.2.45) at a4:5e:60:11:22:33 on en0 ifscope [ethernet]",
        TEST_NEIGHBOR_OBSERVED_AT,
    )
    .value_or_unreachable();

    assert_eq!(observation.ip_address, "192.168.2.45");
    assert_eq!(observation.mac_address, "a4-5e-60-11-22-33");
    assert_eq!(observation.network_interface.as_deref(), Some("en0"));
    assert_eq!(observation.hostname.as_deref(), Some("iphone.local"));
    assert_eq!(observation.observed_at, TEST_NEIGHBOR_OBSERVED_AT);
    assert_eq!(
        observation.reachability,
        LanPairingDeviceReachability::Stale
    );
    assert_eq!(
        observation.scan_sources,
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()]
    );
}

#[test]
fn macos_arp_parser_rejects_incomplete_public_and_bad_rows() {
    assert!(macos_arp_observation_with_observed_at(
        "? (192.168.2.50) at (incomplete) on en0 ifscope [ethernet]",
        TEST_NEIGHBOR_OBSERVED_AT,
    )
    .is_none());
    assert!(macos_arp_observation_with_observed_at(
        "? (8.8.8.8) at 00:11:22:33:44:55 on en0 ifscope [ethernet]",
        TEST_NEIGHBOR_OBSERVED_AT,
    )
    .is_none());
    assert!(
        macos_arp_observation_with_observed_at("not an arp row", TEST_NEIGHBOR_OBSERVED_AT)
            .is_none()
    );
}

#[test]
fn macos_neighbor_device_reuses_previous_identity_as_weak_hint() {
    let previous = LanNetworkInventoryDevice {
        device_id: "previous-device".to_string(),
        label: "Kitchen iPad".to_string(),
        platform: constants::lan_pairing::PLATFORM_MACOS.to_string(),
        ip_address: "192.168.2.45".to_string(),
        mac_address: "a4-5e-60-11-22-33".to_string(),
        hostname: Some("kitchen-ipad.local".to_string()),
        network_interface: Some("en0".to_string()),
        observed_at: String::new(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };
    let observation =
        macos_arp_observation("? (192.168.2.45) at a4:5e:60:11:22:33 on en0 ifscope [ethernet]")
            .value_or_unreachable();
    let previous_inventory = LanPreviousNetworkInventory::from_devices(&[previous]);
    let identity_hint_inventory = LanIdentityHintInventory::from_devices(&[]);

    let device = network_device_from_macos_observation(
        observation,
        &identity_hint_inventory,
        &previous_inventory,
    )
    .value_or_unreachable();

    assert_eq!(device.label, "kitchen-ipad.local");
    assert_eq!(device.hostname.as_deref(), Some("kitchen-ipad.local"));
    assert_eq!(device.platform, constants::lan_pairing::PLATFORM_MACOS);
    assert!(device.used_previous_scan_hint);
}

#[test]
fn current_macos_neighbor_ipv4_observations_keep_first_normalized_mapping_for_duplicate_ips() {
    let observations = vec![
        LanNeighborObservation {
            ip_address: "192.168.2.91".to_string(),
            mac_address: "a4-5e-60-11-22-33".to_string(),
            network_interface: Some("en0".to_string()),
            hostname: Some("first.local".to_string()),
            reachability: LanPairingDeviceReachability::Stale,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
        LanNeighborObservation {
            ip_address: "192.168.2.91".to_string(),
            mac_address: "a4-5e-60-11-22-34".to_string(),
            network_interface: Some("en1".to_string()),
            hostname: Some("second.local".to_string()),
            reachability: LanPairingDeviceReachability::Online,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
    ];

    let observations = current_macos_neighbor_ipv4_observations_from_observations(observations);
    let expected_mac = "a4-5e-60-11-22-33".to_string();

    assert_eq!(observations.get("192.168.2.91"), Some(&expected_mac));
    assert_eq!(observations.len(), 1);
}

#[test]
fn current_macos_neighbor_ipv4_observations_prefer_more_reachable_ip_for_same_mac() {
    let observations = vec![
        LanNeighborObservation {
            ip_address: "192.168.2.92".to_string(),
            mac_address: "a4-5e-60-11-22-35".to_string(),
            network_interface: Some("en0".to_string()),
            hostname: Some("old.local".to_string()),
            reachability: LanPairingDeviceReachability::Stale,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
        LanNeighborObservation {
            ip_address: "192.168.2.93".to_string(),
            mac_address: "a4-5e-60-11-22-35".to_string(),
            network_interface: Some("en0".to_string()),
            hostname: Some("new.local".to_string()),
            reachability: LanPairingDeviceReachability::Online,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
    ];

    let observations = current_macos_neighbor_ipv4_observations_from_observations(observations);
    let expected_mac = "a4-5e-60-11-22-35".to_string();

    assert_eq!(observations.get("192.168.2.93"), Some(&expected_mac));
    assert!(!observations.contains_key("192.168.2.92"));
    assert_eq!(observations.len(), 1);
}

#[test]
fn selected_interface_filter_keeps_only_matching_macos_observations() {
    let observations = filter_neighbor_observations_for_selected_interface(
        vec![
            LanNeighborObservation {
                ip_address: "192.168.2.94".to_string(),
                mac_address: "a4-5e-60-11-22-94".to_string(),
                network_interface: Some(" en0 ".to_string()),
                hostname: Some("camera.local".to_string()),
                reachability: LanPairingDeviceReachability::Stale,
                scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()],
                observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
            },
            LanNeighborObservation {
                ip_address: "192.168.2.95".to_string(),
                mac_address: "a4-5e-60-11-22-95".to_string(),
                network_interface: Some("en1".to_string()),
                hostname: Some("printer.local".to_string()),
                reachability: LanPairingDeviceReachability::Online,
                scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()],
                observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
            },
        ],
        Some("EN0"),
    );

    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].ip_address, "192.168.2.94");
}

use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::fs;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use ocentra_lan_core::network_inventory::linux_neighbors::identity::{
    network_device_from_neighbor_observation, reverse_dns_hostname_from_getent_line,
};
use ocentra_lan_core::network_inventory::linux_neighbors::merge::merge_neighbor_observations;
use ocentra_lan_core::network_inventory::linux_neighbors::observations::{
    current_linux_neighbor_ipv4_observations_from_observations,
    linux_ip_neigh_observation_with_observed_at, linux_proc_net_arp_observation_with_observed_at,
};
use ocentra_lan_core::network_inventory::neighbor_support::filter_neighbor_observations_for_selected_interface;
use ocentra_lan_core::network_inventory::{
    LanIdentityHintInventory, LanNeighborObservation, LanPreviousNetworkInventory,
};

const TEST_NEIGHBOR_OBSERVED_AT: &str = "2026-06-28T12:00:00Z";

#[test]
fn linux_proc_net_arp_basic_fixture_preserves_neighbor_rows() {
    let observations = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/linux_proc_net_arp_basic.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .skip(1)
    .filter_map(|line| {
        linux_proc_net_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();

    assert_eq!(observations.len(), 2);
    assert_eq!(observations[0].ip_address, "192.168.2.1");
    assert_eq!(observations[0].mac_address, "00-11-22-33-44-55");
    assert_eq!(observations[0].network_interface.as_deref(), Some("eth0"));
    assert_eq!(
        observations[0].reachability,
        LanPairingDeviceReachability::Stale
    );
    assert_eq!(
        observations[0].scan_sources,
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string()]
    );
    assert_eq!(observations[0].observed_at, TEST_NEIGHBOR_OBSERVED_AT);
    assert_eq!(observations[1].ip_address, "192.168.2.45");
    assert_eq!(observations[1].mac_address, "a4-5e-60-11-22-33");
}

#[test]
fn linux_proc_net_arp_empty_incomplete_and_malformed_fixtures_produce_no_rows() {
    let empty = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/linux_proc_net_arp_empty.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .skip(1)
    .filter_map(|line| {
        linux_proc_net_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();
    let incomplete = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/linux_proc_net_arp_incomplete.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .skip(1)
    .filter_map(|line| {
        linux_proc_net_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();
    let malformed = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/linux_proc_net_arp_malformed.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .skip(1)
    .filter_map(|line| {
        linux_proc_net_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();

    assert!(empty.is_empty());
    assert!(incomplete.is_empty());
    assert!(malformed.is_empty());
}

#[test]
fn linux_proc_net_arp_duplicate_fixture_keeps_duplicate_candidates_until_merge() {
    let parsed = fs::read_to_string(format!(
        "{}/tests/fixtures/lan-plan/linux_proc_net_arp_duplicate.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .value_or_unreachable()
    .lines()
    .skip(1)
    .filter_map(|line| {
        linux_proc_net_arp_observation_with_observed_at(line, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();

    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].mac_address, parsed[1].mac_address);
    assert_ne!(parsed[0].ip_address, parsed[1].ip_address);

    let merged = merge_neighbor_observations(parsed);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].ip_address, "192.168.2.61");
}

#[test]
fn linux_ip_neigh_basic_fixture_preserves_neighbor_rows() {
    let parsed = serde_json::from_str::<Vec<serde_json::Value>>(
        &fs::read_to_string(format!(
            "{}/tests/fixtures/lan-plan/linux_ip_neigh_basic.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .value_or_unreachable(),
    )
    .value_or_unreachable()
    .iter()
    .filter_map(|record| {
        linux_ip_neigh_observation_with_observed_at(record, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();

    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].ip_address, "192.168.2.45");
    assert_eq!(parsed[0].mac_address, "a4-5e-60-11-22-33");
    assert_eq!(parsed[0].network_interface.as_deref(), Some("eth0"));
    assert_eq!(parsed[0].observed_at, TEST_NEIGHBOR_OBSERVED_AT);
    assert_eq!(parsed[1].ip_address, "fe80::abcd");
}

#[test]
fn linux_ip_neigh_empty_incomplete_and_malformed_fixtures_produce_no_rows() {
    let empty = serde_json::from_str::<Vec<serde_json::Value>>(
        &fs::read_to_string(format!(
            "{}/tests/fixtures/lan-plan/linux_ip_neigh_empty.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .value_or_unreachable(),
    )
    .value_or_unreachable()
    .iter()
    .filter_map(|record| {
        linux_ip_neigh_observation_with_observed_at(record, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();
    let incomplete = serde_json::from_str::<Vec<serde_json::Value>>(
        &fs::read_to_string(format!(
            "{}/tests/fixtures/lan-plan/linux_ip_neigh_incomplete.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .value_or_unreachable(),
    )
    .value_or_unreachable()
    .iter()
    .filter_map(|record| {
        linux_ip_neigh_observation_with_observed_at(record, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();
    let malformed = serde_json::from_str::<Vec<serde_json::Value>>(
        &fs::read_to_string(format!(
            "{}/tests/fixtures/lan-plan/linux_ip_neigh_malformed.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .value_or_unreachable(),
    )
    .value_or_unreachable()
    .iter()
    .filter_map(|record| {
        linux_ip_neigh_observation_with_observed_at(record, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();

    assert!(empty.is_empty());
    assert!(incomplete.is_empty());
    assert!(malformed.is_empty());
}

#[test]
fn linux_ip_neigh_duplicate_fixture_keeps_duplicate_candidates_until_merge() {
    let parsed = serde_json::from_str::<Vec<serde_json::Value>>(
        &fs::read_to_string(format!(
            "{}/tests/fixtures/lan-plan/linux_ip_neigh_duplicate.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .value_or_unreachable(),
    )
    .value_or_unreachable()
    .iter()
    .filter_map(|record| {
        linux_ip_neigh_observation_with_observed_at(record, TEST_NEIGHBOR_OBSERVED_AT)
    })
    .collect::<Vec<_>>();

    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].mac_address, parsed[1].mac_address);
    assert_ne!(parsed[0].ip_address, parsed[1].ip_address);

    let merged = merge_neighbor_observations(parsed);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].ip_address, "192.168.2.80");
}

#[test]
fn linux_ip_neigh_parser_accepts_ipv6_neighbors() {
    let parsed = linux_ip_neigh_observation_with_observed_at(
        &serde_json::json!({
            constants::lan_pairing::JSON_KEY_DST: "fe80::abcd",
            constants::lan_pairing::JSON_KEY_LLADDR: "00:11:22:33:44:77",
            constants::lan_pairing::JSON_KEY_LOWER_STATE: constants::lan_pairing::LINUX_NEIGHBOR_STATE_REACHABLE,
            constants::lan_pairing::JSON_KEY_DEV: "eth0",
        }),
        TEST_NEIGHBOR_OBSERVED_AT,
    );
    let parsed = parsed.value_or_unreachable();

    assert_eq!(parsed.ip_address, "fe80::abcd");
    assert_eq!(parsed.mac_address, "00-11-22-33-44-77");
    assert_eq!(parsed.network_interface.as_deref(), Some("eth0"));
    assert_eq!(parsed.reachability, LanPairingDeviceReachability::Online);
    assert_eq!(parsed.observed_at, TEST_NEIGHBOR_OBSERVED_AT);
    assert_eq!(
        parsed.scan_sources,
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()]
    );
}

#[test]
fn linux_ip_neigh_parser_rejects_incomplete_and_malformed_rows() {
    assert!(linux_ip_neigh_observation_with_observed_at(&serde_json::json!({
        constants::lan_pairing::JSON_KEY_DST: "192.168.2.50",
        constants::lan_pairing::JSON_KEY_LLADDR: "00:11:22:33:44:55",
        constants::lan_pairing::JSON_KEY_LOWER_STATE: constants::lan_pairing::LINUX_NEIGHBOR_STATE_INCOMPLETE,
        constants::lan_pairing::JSON_KEY_DEV: "eth0",
    }), TEST_NEIGHBOR_OBSERVED_AT)
    .is_none());

    assert!(linux_ip_neigh_observation_with_observed_at(&serde_json::json!({
        constants::lan_pairing::JSON_KEY_DST: "192.168.2.50",
        constants::lan_pairing::JSON_KEY_LLADDR: "00:00:00:00:00:00",
        constants::lan_pairing::JSON_KEY_LOWER_STATE: constants::lan_pairing::LINUX_NEIGHBOR_STATE_REACHABLE,
        constants::lan_pairing::JSON_KEY_DEV: "eth0",
    }), TEST_NEIGHBOR_OBSERVED_AT)
    .is_none());
}

#[test]
fn reverse_dns_parser_rejects_invalid_display_values() {
    assert_eq!(
        reverse_dns_hostname_from_getent_line("192.168.2.50 bad<script>"),
        None
    );
    assert_eq!(
        reverse_dns_hostname_from_getent_line(&format!("192.168.2.50 {}", "a".repeat(256))),
        None
    );
}

#[test]
fn reverse_dns_parser_accepts_trimmed_dns_style_names() {
    assert_eq!(
        reverse_dns_hostname_from_getent_line("192.168.2.50 printer-1.example.local"),
        Some("printer-1.example.local".to_string())
    );
    assert_eq!(
        reverse_dns_hostname_from_getent_line("192.168.2.50 printer-1.example.local."),
        Some("printer-1.example.local".to_string())
    );
}

#[test]
fn duplicate_hostname_fixtures_stay_separate_by_mac() {
    let first = network_device_from_neighbor_observation(
        LanNeighborObservation {
            ip_address: "192.168.2.81".to_string(),
            mac_address: "00-11-22-33-44-80".to_string(),
            network_interface: Some("eth0".to_string()),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
    )
    .value_or_unreachable();
    let second = network_device_from_neighbor_observation(
        LanNeighborObservation {
            ip_address: "192.168.2.82".to_string(),
            mac_address: "00-11-22-33-44-81".to_string(),
            network_interface: Some("eth0".to_string()),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
    )
    .value_or_unreachable();

    assert_ne!(first.device_id, second.device_id);
    assert_eq!(
        first.hostname.as_deref(),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
    assert_eq!(
        second.hostname.as_deref(),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
    assert_eq!(first.label, constants::lan_pairing::TEST_HOSTNAME);
    assert_eq!(second.label, constants::lan_pairing::TEST_HOSTNAME);
}

#[test]
fn hostname_only_neighbor_evidence_stays_below_previous_scan_trust() {
    let device = network_device_from_neighbor_observation(
        LanNeighborObservation {
            ip_address: "192.168.2.83".to_string(),
            mac_address: "00-11-22-33-44-82".to_string(),
            network_interface: Some("eth0".to_string()),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
    )
    .value_or_unreachable();

    assert_eq!(
        device.hostname.as_deref(),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
    assert_eq!(device.label, constants::lan_pairing::TEST_HOSTNAME);
    assert!(!device.used_previous_scan_hint);
    assert_eq!(
        device.scan_sources,
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()]
    );
}

#[test]
fn duplicate_neighbor_rows_merge_sources_and_prefer_private_ipv4_identity() {
    let merged = merge_neighbor_observations(vec![
        LanNeighborObservation {
            ip_address: "fe80::abcd".to_string(),
            mac_address: "00-11-22-33-44-88".to_string(),
            network_interface: Some("eth0".to_string()),
            hostname: None,
            reachability: LanPairingDeviceReachability::Online,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()],
            observed_at: "2026-06-28T12:05:00Z".to_string(),
        },
        LanNeighborObservation {
            ip_address: "192.168.2.80".to_string(),
            mac_address: "00-11-22-33-44-88".to_string(),
            network_interface: Some("eth0".to_string()),
            hostname: None,
            reachability: LanPairingDeviceReachability::Stale,
            scan_sources: vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string()
            ],
            observed_at: "2026-06-28T12:00:00Z".to_string(),
        },
    ]);

    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].ip_address, "192.168.2.80");
    assert_eq!(merged[0].reachability, LanPairingDeviceReachability::Online);
    assert_eq!(
        merged[0].scan_sources,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
        ]
    );
    assert_eq!(merged[0].observed_at, "2026-06-28T12:00:00Z");
}

#[test]
fn current_linux_neighbor_ipv4_observations_keep_first_normalized_mapping_for_duplicate_ips() {
    let observations = vec![
        LanNeighborObservation {
            ip_address: "192.168.2.80".to_string(),
            mac_address: "00-11-22-33-44-88".to_string(),
            network_interface: Some("eth0".to_string()),
            hostname: None,
            reachability: LanPairingDeviceReachability::Online,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
        LanNeighborObservation {
            ip_address: "192.168.2.80".to_string(),
            mac_address: "00-11-22-33-44-99".to_string(),
            network_interface: Some("eth1".to_string()),
            hostname: None,
            reachability: LanPairingDeviceReachability::Stale,
            scan_sources: vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string()
            ],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
    ];

    let observations = current_linux_neighbor_ipv4_observations_from_observations(observations);
    let expected_mac = "00-11-22-33-44-88".to_ascii_lowercase();

    assert_eq!(observations.get("192.168.2.80"), Some(&expected_mac));
    assert_eq!(observations.len(), 1);
}

#[test]
fn current_linux_neighbor_ipv4_observations_prefer_more_reachable_ip_for_same_mac() {
    let observations = vec![
        LanNeighborObservation {
            ip_address: "192.168.2.82".to_string(),
            mac_address: "00-11-22-33-44-AA".to_string(),
            network_interface: Some("eth0".to_string()),
            hostname: None,
            reachability: LanPairingDeviceReachability::Stale,
            scan_sources: vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string()
            ],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
        LanNeighborObservation {
            ip_address: "192.168.2.83".to_string(),
            mac_address: "00-11-22-33-44-AA".to_string(),
            network_interface: Some("eth0".to_string()),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()],
            observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
        },
    ];

    let observations = current_linux_neighbor_ipv4_observations_from_observations(observations);
    let expected_mac = "00-11-22-33-44-AA".to_ascii_lowercase();

    assert_eq!(observations.get("192.168.2.83"), Some(&expected_mac));
    assert!(!observations.contains_key("192.168.2.82"));
    assert_eq!(observations.len(), 1);
}

#[test]
fn selected_interface_filter_keeps_only_matching_linux_observations_before_merge() {
    let observations = filter_neighbor_observations_for_selected_interface(
        vec![
            LanNeighborObservation {
                ip_address: "192.168.2.84".to_string(),
                mac_address: "00-11-22-33-44-84".to_string(),
                network_interface: Some(" eth0 ".to_string()),
                hostname: None,
                reachability: LanPairingDeviceReachability::Online,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()
                ],
                observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
            },
            LanNeighborObservation {
                ip_address: "192.168.2.85".to_string(),
                mac_address: "00-11-22-33-44-85".to_string(),
                network_interface: Some("wlan0".to_string()),
                hostname: None,
                reachability: LanPairingDeviceReachability::Online,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string()
                ],
                observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
            },
            LanNeighborObservation {
                ip_address: "192.168.2.86".to_string(),
                mac_address: "00-11-22-33-44-86".to_string(),
                network_interface: None,
                hostname: None,
                reachability: LanPairingDeviceReachability::Online,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string()
                ],
                observed_at: TEST_NEIGHBOR_OBSERVED_AT.to_string(),
            },
        ],
        Some("ETH0"),
    );

    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].ip_address, "192.168.2.84");
    assert_eq!(observations[0].network_interface.as_deref(), Some(" eth0 "));
}

use super::*;

use std::collections::HashSet;

fn target_ip(value: impl std::fmt::Display) -> std::net::Ipv4Addr {
    value.to_string().parse().value_or_unreachable()
}
use ocentra_lan_core::network_inventory::active_refresh::evidence::{
    observations_by_ip, probe_targeted_arp_refresh_target_until,
    targeted_arp_refresh_evidence_from_observation,
    targeted_arp_refresh_targets_with_packet_io_until,
};
use ocentra_lan_core::network_inventory::active_refresh::suppression::current_observation_confirms_ip_and_mac;
use ocentra_lan_core::network_inventory::active_refresh::target_builders::targeted_arp_refresh_targets;
use ocentra_lan_core::network_inventory::active_refresh::throttle::{
    clear_targeted_arp_refresh_attempts, targeted_arp_refresh_throttled_at,
    TARGETED_ARP_REFRESH_ATTEMPTS, TARGETED_ARP_REFRESH_THROTTLE_MS,
};
use ocentra_lan_core::network_inventory::active_refresh::{
    CommandTargetedArpRefreshPacketIo, TargetedArpRefreshPacketIo,
};

#[test]
fn targeted_arp_refresh_targets_only_selected_hosts_on_the_selected_interface() {
    let targets = targeted_arp_refresh_targets(
        Some("192.168.2.42"),
        Some("192.168.2.42/24"),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE),
        &[
            trusted_device(
                "54:27:1e:97:c3:31",
                Some("192.168.2.20"),
                Some(constants::lan_pairing::TEST_HOSTNAME),
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_WINDOWS,
            ),
            trusted_device(
                constants::lan_pairing::TEST_ROUTER_MAC,
                Some("192.168.3.20"),
                Some(constants::lan_pairing::TEST_HOSTNAME),
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_WINDOWS,
            ),
        ],
        &[
            LanNetworkInventoryDevice {
                device_id: "lan-device-previous".to_string(),
                label: "Previous LAN device".to_string(),
                platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                ip_address: "192.168.2.21".to_string(),
                mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
                hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
                network_interface: Some("Wi-Fi".to_string()),
                reachability: LanPairingDeviceReachability::Online,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                observed_at: String::new(),
                used_previous_scan_hint: false,
                service_identity_probe_evidence: Vec::new(),
            },
            LanNetworkInventoryDevice {
                device_id: "lan-device-selected".to_string(),
                label: "Selected LAN device".to_string(),
                platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                ip_address: "192.168.2.22".to_string(),
                mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
                hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                reachability: LanPairingDeviceReachability::Online,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                observed_at: String::new(),
                used_previous_scan_hint: false,
                service_identity_probe_evidence: Vec::new(),
            },
        ],
    );

    assert_eq!(targets.len(), 2);
    assert_eq!(
        targets
            .iter()
            .map(|target| target.ip_address.to_string())
            .collect::<Vec<_>>(),
        vec!["192.168.2.20".to_string(), "192.168.2.22".to_string()]
    );
    assert!(targets
        .iter()
        .all(|target| target.network_interface.as_deref()
            == Some(constants::lan_pairing::TEST_NETWORK_INTERFACE)));
    assert_eq!(
        targets
            .iter()
            .map(|target| target.expected_mac_address.as_deref())
            .collect::<Vec<_>>(),
        vec![
            Some(constants::lan_pairing::TEST_LAN_MAC),
            Some(constants::lan_pairing::TEST_LAN_MAC)
        ]
    );
}

#[test]
fn targeted_arp_refresh_targets_require_a_selected_interface_and_local_subnet() {
    assert!(targeted_arp_refresh_targets(
        Some("192.168.2.42"),
        Some("192.168.2.42/24"),
        Some(""),
        &[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some("192.168.2.20"),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        )],
        &[]
    )
    .is_empty());

    assert!(targeted_arp_refresh_targets(
        Some("192.168.2.42"),
        Some("192.168.3.42/24"),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE),
        &[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some("192.168.2.20"),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        )],
        &[]
    )
    .is_empty());
}

#[test]
fn targeted_arp_refresh_response_records_mac_and_strong_identity_match_only_when_macs_align() {
    let target_ip_address = "192.168.2.23";
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip(target_ip_address),
        expected_mac_address: Some("54:27:1e:97:c3:31".to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };

    let evidence = targeted_arp_refresh_evidence_from_observation(
        &target,
        Some(constants::lan_pairing::TEST_LAN_MAC.to_ascii_lowercase()),
        1_717_000_000_000,
        false,
    );

    assert_eq!(evidence.target_ip_address, target_ip_address);
    assert_eq!(
        evidence.selected_interface.as_deref(),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE)
    );
    assert_eq!(
        evidence.expected_mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert_eq!(
        evidence.observed_mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert_eq!(evidence.observed_at_unix_ms, 1_717_000_000_000);
    assert_eq!(
        evidence.source,
        if cfg!(target_os = "windows") {
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
        } else if cfg!(target_os = "macos") {
            constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()
        } else {
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()
        }
    );
    assert_eq!(
        evidence.outcome,
        Some(LanTargetedArpRefreshOutcome::Response)
    );
    assert!(evidence.strong_identity_match);
    assert!(!evidence.throttled);
}

#[test]
fn targeted_arp_refresh_suppression_normalizes_expected_mac_before_matching() {
    let current_observations = HashMap::from([(
        target_ip("192.168.2.20"),
        constants::lan_pairing::TEST_LAN_MAC.to_ascii_lowercase(),
    )]);

    assert!(current_observation_confirms_ip_and_mac(
        &current_observations,
        target_ip("192.168.2.20"),
        Some("54:27:1e:97:c3:31"),
    ));
}

#[test]
fn targeted_arp_refresh_duplicate_target_keeps_later_known_mac_and_normalizes_it() {
    let target_ip_address = target_ip("192.168.2.23");
    let interface = constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string();
    let mut targets = Vec::new();
    let mut seen_targets = HashSet::new();

    ocentra_lan_core::network_inventory::active_refresh::target_builders::push_targeted_arp_target(
        &mut targets,
        &mut seen_targets,
        Some(
            ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
                ip_address: target_ip_address,
                expected_mac_address: None,
                network_interface: Some(interface.clone()),
            },
        ),
    );
    ocentra_lan_core::network_inventory::active_refresh::target_builders::push_targeted_arp_target(
        &mut targets,
        &mut seen_targets,
        Some(
            ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
                ip_address: target_ip_address,
                expected_mac_address: Some("54:27:1e:97:c3:31".to_string()),
                network_interface: Some(interface),
            },
        ),
    );

    assert_eq!(targets.len(), 1);
    assert_eq!(
        targets[0].expected_mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
}

#[test]
fn targeted_arp_refresh_no_response_is_stale_presence_evidence_not_deletion() {
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };

    let evidence =
        targeted_arp_refresh_evidence_from_observation(&target, None, 1_717_000_000_000, false);

    assert_eq!(
        evidence.outcome,
        Some(LanTargetedArpRefreshOutcome::NoResponse)
    );
    assert!(evidence.observed_mac_address.is_none());
    assert!(!evidence.strong_identity_match);
    assert!(!evidence.throttled);
    assert_eq!(
        evidence.source,
        if cfg!(target_os = "windows") {
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
        } else if cfg!(target_os = "macos") {
            constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()
        } else {
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()
        }
    );
}

#[test]
fn targeted_arp_refresh_malformed_response_is_rejected_without_identity_match() {
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.20"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };

    let evidence = targeted_arp_refresh_evidence_from_observation(
        &target,
        Some("not-a-mac-address".to_string()),
        1_717_000_000_000,
        false,
    );

    assert_eq!(
        evidence.outcome,
        Some(LanTargetedArpRefreshOutcome::NoResponse)
    );
    assert!(evidence.observed_mac_address.is_none());
    assert!(!evidence.strong_identity_match);
}

#[test]
fn targeted_arp_refresh_throttles_repeated_checks_per_host_and_interface() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let first = Instant::now();
    assert!(targeted_arp_refresh_throttled_at(&target, first).is_none());
    assert_eq!(
        targeted_arp_refresh_throttled_at(
            &target,
            first + Duration::from_millis(TARGETED_ARP_REFRESH_THROTTLE_MS - 1)
        ),
        Some(first)
    );
    assert!(targeted_arp_refresh_throttled_at(
        &target,
        first + Duration::from_millis(TARGETED_ARP_REFRESH_THROTTLE_MS + 1)
    )
    .is_none());

    let same_ip_other_interface =
        ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
            ip_address: target_ip("192.168.2.250"),
            expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
            network_interface: Some("Ethernet 3".to_string()),
        };
    assert!(targeted_arp_refresh_throttled_at(
        &same_ip_other_interface,
        first + Duration::from_millis(1)
    )
    .is_none());
}

#[test]
fn targeted_arp_refresh_recovers_from_poisoned_attempt_state() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    clear_targeted_arp_refresh_attempts();

    let attempts = TARGETED_ARP_REFRESH_ATTEMPTS.get_or_init(|| Mutex::new(HashMap::new()));
    let _ = std::panic::catch_unwind(|| {
        let _lock = attempts.lock().value_or_unreachable();
        assert_eq!(0, 1, "poison targeted arp refresh attempt state");
    });

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let first = Instant::now();

    assert!(targeted_arp_refresh_throttled_at(&target, first).is_none());
    assert_eq!(
        targeted_arp_refresh_throttled_at(
            &target,
            first + Duration::from_millis(TARGETED_ARP_REFRESH_THROTTLE_MS - 1)
        ),
        Some(first)
    );

    clear_targeted_arp_refresh_attempts();
}

#[test]
fn targeted_arp_refresh_reply_observations_normalize_and_dedupe_real_rows() {
    let observations = observations_by_ip(vec![
        ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation {
            ip_address: target_ip("192.168.2.24"),
            mac_address: "54:27:1E:97:C3:31".to_string(),
        },
        ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation {
            ip_address: target_ip("192.168.2.24"),
            mac_address: "not-a-mac".to_string(),
        },
        ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation {
            ip_address: target_ip("192.168.2.25"),
            mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
        },
    ]);
    let expected_lan_mac = constants::lan_pairing::TEST_LAN_MAC.to_ascii_lowercase();
    let expected_router_mac = constants::lan_pairing::TEST_ROUTER_MAC.to_ascii_lowercase();

    assert_eq!(
        observations
            .get(&target_ip("192.168.2.24"))
            .map(String::as_str),
        Some(expected_lan_mac.as_str())
    );
    assert_eq!(
        observations
            .get(&target_ip("192.168.2.25"))
            .map(String::as_str),
        Some(expected_router_mac.as_str())
    );
}

#[test]
fn targeted_arp_refresh_budget_skips_without_recording_false_no_response() {
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let mut packet_io = CommandTargetedArpRefreshPacketIo;
    let evidence = targeted_arp_refresh_targets_with_packet_io_until(
        &[target],
        Instant::now() - Duration::from_millis(1),
        &mut packet_io,
    );

    assert!(evidence.is_empty());
}

#[test]
fn targeted_arp_refresh_without_probe_budget_records_no_false_no_response() {
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.251"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    assert!(!probe_targeted_arp_refresh_target_until(
        &target,
        Instant::now() - Duration::from_millis(1),
    ));
}

#[test]
fn targeted_arp_refresh_command_adapter_is_real_and_budget_bounded() {
    let mut packet_io = CommandTargetedArpRefreshPacketIo;
    assert!(!packet_io.has_observation_budget(Instant::now() - Duration::from_millis(1)));
}

use crate::support::ResultTestExt as _;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::network_inventory::trusted_device;
use ocentra_lan_core::network_inventory::active_refresh::*;
use ocentra_lan_core::network_inventory::{
    LanDiscoveryRefreshMode, LanNetworkInventoryDevice, LanTargetedArpRefreshOutcome,
};
use ocentra_lan_core::network_inventory_hardware::LocalNetworkIdentity;

fn target_ip(value: &str) -> std::net::Ipv4Addr {
    value.parse().value_or_unreachable("test target ip parses")
}

fn targeted_arp_refresh_attempt_state_guard() -> MutexGuard<'static, ()> {
    static TARGETED_ARP_REFRESH_ATTEMPT_STATE_GUARD: OnceLock<Mutex<()>> = OnceLock::new();

    TARGETED_ARP_REFRESH_ATTEMPT_STATE_GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .value_or_unreachable("targeted arp refresh attempt state test guard")
}

#[test]
fn bounded_active_targets_exclude_network_broadcast_and_local_host() {
    assert_eq!(
        bounded_active_ipv4_targets(
            Some("192.168.2.42"),
            Some("192.168.2.42/30"),
            None,
            &[],
            &[],
        ),
        vec!["192.168.2.41".to_string()]
    );
}

#[test]
fn bounded_active_targets_limit_large_subnet_to_local_24_window() {
    let targets =
        bounded_active_ipv4_targets(Some("10.1.2.42"), Some("10.1.2.42/16"), None, &[], &[]);

    assert_eq!(
        targets.len(),
        (constants::lan_pairing::LAN_ACTIVE_IPV4_SWEEP_MAX_HOSTS - 1) as usize
    );
    assert_eq!(targets.first().map(String::as_str), Some("10.1.2.1"));
    assert_eq!(targets.last().map(String::as_str), Some("10.1.2.254"));
    assert!(!targets.iter().any(|target| target == "10.1.1.1"));
    assert!(!targets.iter().any(|target| target == "10.1.2.42"));
}

#[test]
fn bounded_active_targets_require_ipv4_identity_and_cidr() {
    assert!(bounded_active_ipv4_targets(None, Some("192.168.2.42/24"), None, &[], &[]).is_empty());
    assert!(bounded_active_ipv4_targets(Some("192.168.2.42"), None, None, &[], &[]).is_empty());
    assert!(bounded_active_ipv4_targets(
        Some("192.168.2.42"),
        Some("192.168.2.42/31"),
        None,
        &[],
        &[],
    )
    .is_empty());
    assert!(bounded_active_ipv4_targets(
        Some("192.168.2.42"),
        Some("192.168.2.42/not-a-prefix"),
        None,
        &[],
        &[],
    )
    .is_empty());
}

#[test]
fn active_refresh_always_skips_router_truth_but_not_unconfirmed_child_ip_truth() {
    let targets = bounded_active_ipv4_targets(
        Some("192.168.2.42"),
        Some("192.168.2.42/24"),
        Some("192.168.2.1"),
        &[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some("192.168.2.20"),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        )],
        &[LanNetworkInventoryDevice {
            device_id: "lan-router-1".to_string(),
            label: "Home Router".to_string(),
            platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
            ip_address: "192.168.2.1".to_string(),
            mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
            hostname: Some("home-router".to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            observed_at: String::new(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        }],
    );

    assert!(targets.iter().any(|target| target == "192.168.2.20"));
    assert!(!targets.iter().any(|target| target == "192.168.2.1"));
}

#[test]
fn active_refresh_prioritizes_previous_unresolved_devices_before_unknown_space() {
    let targets = bounded_active_ipv4_targets(
        Some("192.168.2.42"),
        Some("192.168.2.42/24"),
        Some("192.168.2.1"),
        &[],
        &[LanNetworkInventoryDevice {
            device_id: "lan-device-previous".to_string(),
            label: format!(
                "{}{}",
                constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
                "192.168.2.77"
            ),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: "192.168.2.77".to_string(),
            mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
            hostname: None,
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Offline,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            observed_at: String::new(),
            used_previous_scan_hint: true,
            service_identity_probe_evidence: Vec::new(),
        }],
    );

    assert_eq!(targets.first().map(String::as_str), Some("192.168.2.77"));
}

#[test]
fn active_refresh_suppression_skips_currently_confirmed_known_child_ip() {
    let suppressed_targets = suppressed_active_ipv4_targets_for_current_observations(
        Some("192.168.2.1"),
        &[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some("192.168.2.20"),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        )],
        &[],
        &HashMap::from([(
            target_ip("192.168.2.20"),
            constants::lan_pairing::TEST_LAN_MAC.to_ascii_lowercase(),
        )]),
    );

    assert_eq!(
        suppressed_targets,
        std::collections::HashSet::from([target_ip("192.168.2.20")])
    );
}

#[test]
fn active_refresh_suppression_does_not_skip_reused_ip_with_different_mac() {
    let suppressed_targets = suppressed_active_ipv4_targets_for_current_observations(
        Some("192.168.2.1"),
        &[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some("192.168.2.20"),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        )],
        &[],
        &HashMap::from([(
            target_ip("192.168.2.20"),
            constants::lan_pairing::TEST_ROUTER_MAC.to_ascii_lowercase(),
        )]),
    );

    assert_eq!(suppressed_targets, std::collections::HashSet::new());
}

#[test]
fn active_scan_plan_records_selected_interface_and_suppressed_targets() {
    let plan = scan_plan_for_identity(
        Some(&LocalNetworkIdentity {
            ip_address: Some("192.168.2.42".to_string()),
            mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            wifi_ssid: None,
            default_gateway: Some("192.168.2.1".to_string()),
            ipv4_cidr: Some("192.168.2.42/24".to_string()),
            dns_servers: vec!["192.168.2.1".to_string(), "1.1.1.1".to_string()],
            dhcp_server: Some("192.168.2.1".to_string()),
            broadcast_address: Some("192.168.2.255".to_string()),
            ipv6_prefixes: vec!["2001:db8::42/64".to_string()],
        }),
        &[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some("192.168.2.20"),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        )],
        &[LanNetworkInventoryDevice {
            device_id: "lan-router-1".to_string(),
            label: "Home Router".to_string(),
            platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
            ip_address: "192.168.2.1".to_string(),
            mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
            hostname: Some("home-router".to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            observed_at: String::new(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        }],
        LanDiscoveryRefreshMode::ActiveSubnetRefresh,
        &[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some("192.168.2.20"),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        )],
    );

    assert_eq!(
        plan.selected_interface.as_deref(),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE)
    );
    assert_eq!(plan.default_gateway.as_deref(), Some("192.168.2.1"));
    assert_eq!(
        plan.dns_servers,
        vec!["192.168.2.1".to_string(), "1.1.1.1".to_string()]
    );
    assert_eq!(plan.dhcp_server.as_deref(), Some("192.168.2.1"));
    assert_eq!(plan.broadcast_address.as_deref(), Some("192.168.2.255"));
    assert_eq!(plan.ipv6_prefixes, vec!["2001:db8::42/64".to_string()]);
    assert_eq!(plan.trusted_truth_device_count, 1);
    assert_eq!(plan.previous_device_count, 1);
    assert_eq!(
        plan.active_ipv4_target_timeout_ms,
        active_ipv4_target_timeout_ms()
    );
    assert!(plan
        .suppressed_active_ipv4_targets
        .iter()
        .any(|target| target == "192.168.2.1"));
    assert!(!plan
        .suppressed_active_ipv4_targets
        .iter()
        .any(|target| target == "192.168.2.20"));
    assert_eq!(plan.active_ipv4_candidate_count, 253);
    assert_eq!(plan.active_ipv4_target_count, 252);
}

#[test]
fn passive_scan_plan_keeps_identity_metadata_without_active_targets() {
    let plan = scan_plan_for_identity(
        Some(&LocalNetworkIdentity {
            ip_address: Some("192.168.2.42".to_string()),
            mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            wifi_ssid: None,
            default_gateway: Some("192.168.2.1".to_string()),
            ipv4_cidr: Some("192.168.2.42/24".to_string()),
            dns_servers: vec!["192.168.2.1".to_string()],
            dhcp_server: Some("192.168.2.1".to_string()),
            broadcast_address: Some("192.168.2.255".to_string()),
            ipv6_prefixes: vec!["2001:db8::42/64".to_string()],
        }),
        &[],
        &[],
        LanDiscoveryRefreshMode::Passive,
        &[],
    );

    assert_eq!(plan.refresh_mode, LanDiscoveryRefreshMode::Passive);
    assert_eq!(plan.active_ipv4_candidate_count, 0);
    assert_eq!(plan.active_ipv4_target_count, 0);
    assert_eq!(plan.prioritized_previous_target_count, 0);
    assert_eq!(plan.active_ipv4_target_timeout_ms, None);
    assert_eq!(plan.dns_servers, vec!["192.168.2.1".to_string()]);
    assert_eq!(plan.dhcp_server.as_deref(), Some("192.168.2.1"));
    assert_eq!(plan.broadcast_address.as_deref(), Some("192.168.2.255"));
    assert_eq!(plan.ipv6_prefixes, vec!["2001:db8::42/64".to_string()]);
    assert!(plan.suppressed_active_ipv4_targets.is_empty());
}

#[test]
fn targeted_arp_refresh_targets_only_selected_hosts_on_the_selected_interface() {
    let targets = ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_targets(
        Some("192.168.2.42"),
        Some("192.168.2.42/24"),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE),
        &[
            trusted_device(
                constants::lan_pairing::TEST_LAN_MAC,
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
    assert!(targets
        .iter()
        .all(|target| target.expected_mac_address.is_some()));
}

#[test]
fn targeted_arp_refresh_targets_require_a_selected_interface_and_local_subnet() {
    assert!(
        ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_targets(
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
        .is_empty()
    );

    assert!(
        ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_targets(
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
        .is_empty()
    );
}

#[test]
fn targeted_arp_refresh_response_records_mac_and_strong_identity_match_only_when_macs_align() {
    let target_ip_address = "192.168.2.23";
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip(target_ip_address),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };

    let evidence = ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_evidence_from_observation(
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
fn targeted_arp_refresh_no_response_is_stale_presence_evidence_not_deletion() {
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };

    let evidence = ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_evidence_from_observation(
        &target,
        None,
        1_717_000_000_000,
        false,
    );

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

    let evidence = ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_evidence_from_observation(
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
    ocentra_lan_core::network_inventory::active_refresh::clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let first = Instant::now();
    assert!(
        ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_throttled_at(
            &target, first
        )
        .is_none()
    );
    assert!(ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_throttled_at(
        &target,
        first + Duration::from_millis(ocentra_lan_core::network_inventory::active_refresh::TARGETED_ARP_REFRESH_THROTTLE_MS - 1)
    )
    .is_some());
    assert!(ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_throttled_at(
        &target,
        first + Duration::from_millis(ocentra_lan_core::network_inventory::active_refresh::TARGETED_ARP_REFRESH_THROTTLE_MS + 1)
    )
    .is_none());

    let same_ip_other_interface =
        ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
            ip_address: target_ip("192.168.2.250"),
            expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
            network_interface: Some("Ethernet 3".to_string()),
        };
    assert!(
        ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_throttled_at(
            &same_ip_other_interface,
            first + Duration::from_millis(1)
        )
        .is_none()
    );
}

#[test]
fn targeted_arp_refresh_recovers_from_poisoned_attempt_state() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    ocentra_lan_core::network_inventory::active_refresh::clear_targeted_arp_refresh_attempts();

    let attempts =
        ocentra_lan_core::network_inventory::active_refresh::TARGETED_ARP_REFRESH_ATTEMPTS
            .get_or_init(|| Mutex::new(HashMap::new()));
    let _ = std::panic::catch_unwind(|| {
        let _lock = attempts
            .lock()
            .value_or_unreachable("poison target attempts lock");
        unreachable!("poison targeted arp refresh attempt state");
    });

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let first = Instant::now();

    assert!(
        ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_throttled_at(
            &target, first
        )
        .is_none()
    );
    assert!(ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_throttled_at(
        &target,
        first + Duration::from_millis(ocentra_lan_core::network_inventory::active_refresh::TARGETED_ARP_REFRESH_THROTTLE_MS - 1)
    )
    .is_some());

    ocentra_lan_core::network_inventory::active_refresh::clear_targeted_arp_refresh_attempts();
}

#[derive(Default)]
struct FakePacketIo {
    probed_targets: Vec<std::net::Ipv4Addr>,
    observations:
        Vec<ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation>,
}

impl ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshPacketIo
    for FakePacketIo
{
    fn probe_target(
        &mut self,
        target: &ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget,
        _deadline: Instant,
    ) -> bool {
        self.probed_targets.push(target.ip_address);
        true
    }

    fn observations(
        &mut self,
        _deadline: Instant,
    ) -> Vec<ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation>
    {
        self.observations.clone()
    }
}

#[test]
fn targeted_arp_refresh_uses_packet_io_abstraction_and_dedupes_replies() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    ocentra_lan_core::network_inventory::active_refresh::clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.24"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let mut packet_io = FakePacketIo {
        probed_targets: Vec::new(),
        observations: vec![
            ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation {
                ip_address: target_ip("192.168.2.24"),
                mac_address: constants::lan_pairing::TEST_LAN_MAC.to_ascii_lowercase(),
            },
            ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation {
                ip_address: target_ip("192.168.2.24"),
                mac_address: "not-a-mac".to_string(),
            },
            ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation {
                ip_address: target_ip("192.168.2.25"),
                mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
            },
        ],
    };

    let evidence = ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_targets_with_packet_io_until(
        &[target],
        Instant::now() + Duration::from_secs(1),
        &mut packet_io,
    );

    assert_eq!(packet_io.probed_targets, vec![target_ip("192.168.2.24")]);
    assert_eq!(evidence.len(), 1);
    assert_eq!(evidence[0].target_ip_address, "192.168.2.24");
    assert_eq!(
        evidence[0].selected_interface.as_deref(),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE)
    );
    assert_eq!(
        evidence[0].observed_mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert!(evidence[0].observed_at_unix_ms > 0);
    assert_eq!(
        evidence[0].source,
        if cfg!(target_os = "windows") {
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
        } else if cfg!(target_os = "macos") {
            constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()
        } else {
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()
        }
    );
    assert!(evidence[0].strong_identity_match);
}

#[test]
fn targeted_arp_refresh_budget_skips_without_recording_false_no_response() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    ocentra_lan_core::network_inventory::active_refresh::clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let mut packet_io = FakePacketIo {
        probed_targets: Vec::new(),
        observations: Vec::new(),
    };
    let evidence = ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_targets_with_packet_io_until(
        &[target],
        Instant::now() - Duration::from_millis(1),
        &mut packet_io,
    );

    assert!(evidence.is_empty());
    assert!(packet_io.probed_targets.is_empty());
}

struct NoAttemptPacketIo;

impl ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshPacketIo
    for NoAttemptPacketIo
{
    fn probe_target(
        &mut self,
        _target: &ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget,
        _deadline: Instant,
    ) -> bool {
        false
    }

    fn observations(
        &mut self,
        _deadline: Instant,
    ) -> Vec<ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation>
    {
        unreachable!("observation reads require at least one attempted probe")
    }
}

struct ExhaustedObservationBudgetPacketIo {
    probed_targets: Vec<std::net::Ipv4Addr>,
}

impl ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshPacketIo
    for ExhaustedObservationBudgetPacketIo
{
    fn probe_target(
        &mut self,
        target: &ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget,
        _deadline: Instant,
    ) -> bool {
        self.probed_targets.push(target.ip_address);
        true
    }

    fn observations(
        &mut self,
        _deadline: Instant,
    ) -> Vec<ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation>
    {
        unreachable!("expired observation budget must not read neighbor observations")
    }

    fn has_observation_budget(&mut self, _deadline: Instant) -> bool {
        false
    }
}

#[test]
fn targeted_arp_refresh_without_probe_attempt_records_no_false_no_response() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    ocentra_lan_core::network_inventory::active_refresh::clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.251"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let mut packet_io = NoAttemptPacketIo;
    let evidence = ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_targets_with_packet_io_until(
        &[target],
        Instant::now() + Duration::from_secs(1),
        &mut packet_io,
    );

    assert!(evidence.is_empty());
}

#[test]
fn targeted_arp_refresh_skips_no_response_when_observation_budget_is_exhausted_after_probe() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    ocentra_lan_core::network_inventory::active_refresh::clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.252"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let mut packet_io = ExhaustedObservationBudgetPacketIo {
        probed_targets: Vec::new(),
    };
    let evidence = ocentra_lan_core::network_inventory::active_refresh::targeted_arp_refresh_targets_with_packet_io_until(
        &[target],
        Instant::now() + Duration::from_secs(1),
        &mut packet_io,
    );

    assert_eq!(packet_io.probed_targets, vec![target_ip("192.168.2.252")]);
    assert!(evidence.is_empty());
}

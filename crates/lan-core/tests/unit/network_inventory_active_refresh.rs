use crate::support::ResultTestExt as _;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::network_inventory::trusted_device;
use ocentra_lan_core::network_inventory::active_refresh::suppression::suppressed_active_ipv4_targets_for_current_observations;
use ocentra_lan_core::network_inventory::active_refresh::targets::{
    active_ipv4_target_timeout_ms, bounded_active_ipv4_targets,
};
use ocentra_lan_core::network_inventory::active_refresh::*;
use ocentra_lan_core::network_inventory::{
    LanDiscoveryRefreshMode, LanNetworkInventoryDevice, LanTargetedArpRefreshOutcome,
};
use ocentra_lan_core::network_inventory_hardware::LocalNetworkIdentity;

fn target_ip(octets: [u8; 4]) -> std::net::Ipv4Addr {
    octets.into()
}

fn targeted_arp_refresh_attempt_state_guard() -> MutexGuard<'static, ()> {
    static TARGETED_ARP_REFRESH_ATTEMPT_STATE_GUARD: OnceLock<Mutex<()>> = OnceLock::new();

    TARGETED_ARP_REFRESH_ATTEMPT_STATE_GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .value_or_unreachable()
}

#[path = "network_inventory_active_refresh_targeted.rs"]
mod network_inventory_active_refresh_targeted;

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
            target_ip([192, 168, 2, 20]),
            constants::lan_pairing::TEST_LAN_MAC.to_ascii_lowercase(),
        )]),
    );

    assert_eq!(
        suppressed_targets,
        std::collections::HashSet::from([target_ip([192, 168, 2, 20])])
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
            target_ip([192, 168, 2, 20]),
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

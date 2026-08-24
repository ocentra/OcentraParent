use std::net::Ipv4Addr;
use std::time::Duration;
use std::{collections::HashMap, sync::atomic::AtomicBool};

use chrono::Utc;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use crate::network_inventory_command::{
    command_stdout, command_stdout_with_timeout, normalize_mac_address,
};

use super::neighbor_support::{
    cached_neighbor_identity, filter_neighbor_observations_for_selected_interface,
    is_household_unicast, is_supported_neighbor_ip, likely_router_address_text,
    network_neighbor_label, normalize_neighbor_hostname, normalized_optional_interface_name,
    previous_inventory_label, remember_neighbor_identity, trusted_device_hostname,
    trusted_device_label, trusted_device_platform,
};
use super::service_identity::{
    enrich_service_identity_probes_with_cancellation, AllowedSnmpResponseObserver,
};
use super::{
    merge_neighbor_observations_by_mac, LanIdentityHintInventory, LanNeighborObservation,
    LanNetworkInventoryDevice, LanPreviousNetworkInventory,
};

pub struct ResolvedMacosNeighborIdentity {
    platform: String,
    hostname: Option<String>,
    label: String,
    used_previous_scan_hint: bool,
}

pub fn macos_lan_neighbors(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) -> Vec<LanNetworkInventoryDevice> {
    macos_lan_neighbors_with_cancellation(
        identity_hint_devices,
        previous_devices,
        probe_suppression_devices,
        selected_interface,
        allowed_snmp_response_observer,
        None,
    )
}

pub fn macos_lan_neighbors_with_cancellation(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
    cancellation: Option<&AtomicBool>,
) -> Vec<LanNetworkInventoryDevice> {
    let identity_hint_inventory = LanIdentityHintInventory::from_devices(identity_hint_devices);
    let previous_inventory = LanPreviousNetworkInventory::from_devices(previous_devices);
    let observed_at = Utc::now().to_rfc3339();
    let mut devices = filter_neighbor_observations_for_selected_interface(
        macos_arp_observations_with_observed_at(&observed_at),
        selected_interface,
    )
    .into_iter()
    .filter_map(|observation| {
        network_device_from_macos_observation(
            observation,
            &identity_hint_inventory,
            &previous_inventory,
        )
    })
    .collect::<Vec<_>>();
    enrich_service_identity_probes_with_cancellation(
        &mut devices,
        probe_suppression_devices,
        selected_interface,
        allowed_snmp_response_observer,
        cancellation,
    );
    devices
}

pub fn current_macos_neighbor_ipv4_observations_with_timeout(
    timeout: Duration,
) -> HashMap<String, String> {
    current_macos_neighbor_ipv4_observations_from_observations(
        macos_arp_observations_with_timeout_and_observed_at(timeout, &Utc::now().to_rfc3339()),
    )
}

pub fn current_macos_neighbor_ipv4_observations_from_observations(
    observations: Vec<LanNeighborObservation>,
) -> HashMap<String, String> {
    let mut by_ip = HashMap::new();
    for observation in merge_neighbor_observations_by_mac(observations) {
        let Some(ip_address) = observation.ip_address.parse::<Ipv4Addr>().ok() else {
            continue;
        };
        if !is_household_unicast(ip_address) {
            continue;
        }
        by_ip
            .entry(ip_address.to_string().to_ascii_lowercase())
            .or_insert(observation.mac_address.to_ascii_lowercase());
    }
    by_ip
}

pub fn macos_arp_observations() -> Vec<LanNeighborObservation> {
    macos_arp_observations_with_observed_at(&Utc::now().to_rfc3339())
}

pub fn macos_arp_observations_with_observed_at(observed_at: &str) -> Vec<LanNeighborObservation> {
    command_stdout("arp", &["-a"])
        .map(|output| {
            output
                .lines()
                .filter_map(|line| macos_arp_observation_with_observed_at(line, observed_at))
                .collect()
        })
        .unwrap_or_default()
}

pub fn macos_arp_observations_with_timeout(timeout: Duration) -> Vec<LanNeighborObservation> {
    macos_arp_observations_with_timeout_and_observed_at(timeout, &Utc::now().to_rfc3339())
}

pub fn macos_arp_observations_with_timeout_and_observed_at(
    timeout: Duration,
    observed_at: &str,
) -> Vec<LanNeighborObservation> {
    command_stdout_with_timeout("arp", &["-a"], timeout)
        .map(|output| {
            output
                .lines()
                .filter_map(|line| macos_arp_observation_with_observed_at(line, observed_at))
                .collect()
        })
        .unwrap_or_default()
}

pub fn macos_arp_observation(line: &str) -> Option<LanNeighborObservation> {
    macos_arp_observation_with_observed_at(line, &Utc::now().to_rfc3339())
}

pub fn macos_arp_observation_with_observed_at(
    line: &str,
    observed_at: &str,
) -> Option<LanNeighborObservation> {
    let line = line.trim();
    let (host_text, after_host) = line.split_once(" (")?;
    let (ip_address, after_ip) = after_host.split_once(") at ")?;
    if !is_supported_neighbor_ip(ip_address) {
        return None;
    }
    let (mac_text, after_mac) = after_ip.split_once(" on ")?;
    if mac_text.trim().eq_ignore_ascii_case("(incomplete)") {
        return None;
    }
    let mac_address = normalize_mac_address(mac_text.trim())?;
    let network_interface =
        normalized_optional_interface_name(after_mac.split_whitespace().next().map(str::to_string));
    let hostname = normalize_macos_arp_hostname(host_text, ip_address);

    Some(LanNeighborObservation {
        ip_address: ip_address.to_string(),
        mac_address,
        network_interface,
        hostname,
        observed_at: observed_at.to_string(),
        reachability: LanPairingDeviceReachability::Stale,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()],
    })
}

pub fn normalize_macos_arp_hostname(host_text: &str, ip_address: &str) -> Option<String> {
    let host_text = host_text.trim();
    if host_text == "?" || host_text.eq_ignore_ascii_case(ip_address) {
        return None;
    }
    normalize_neighbor_hostname(host_text)
}

pub fn network_device_from_macos_observation(
    observation: LanNeighborObservation,
    identity_hint_inventory: &LanIdentityHintInventory,
    previous_inventory: &LanPreviousNetworkInventory,
) -> Option<LanNetworkInventoryDevice> {
    let LanNeighborObservation {
        ip_address,
        mac_address,
        network_interface,
        hostname,
        observed_at,
        reachability,
        scan_sources,
    } = observation;
    let trusted_device = identity_hint_inventory.find(&mac_address, &ip_address);
    let previous_device = previous_inventory.find(&mac_address, &ip_address);
    let resolved_identity = resolved_macos_neighbor_identity(
        &ip_address,
        &mac_address,
        hostname,
        trusted_device,
        previous_device,
    );
    let mut device_id = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_DEVICE_PREFIX);
    device_id.push_str(
        &mac_address
            .chars()
            .filter(|character| *character != '-')
            .collect::<String>(),
    );

    Some(LanNetworkInventoryDevice {
        device_id,
        label: resolved_identity.label,
        platform: resolved_identity.platform,
        ip_address,
        mac_address,
        hostname: resolved_identity.hostname,
        network_interface,
        observed_at,
        reachability,
        agent_status: None,
        scan_sources,
        used_previous_scan_hint: resolved_identity.used_previous_scan_hint,
        service_identity_probe_evidence: Vec::new(),
    })
}

pub fn resolved_macos_neighbor_identity(
    ip_address: &str,
    mac_address: &str,
    hostname_hint: Option<String>,
    trusted_device: Option<&LanPairingDeviceRef>,
    previous_device: Option<&LanNetworkInventoryDevice>,
) -> ResolvedMacosNeighborIdentity {
    let mut platform = if likely_router_address_text(ip_address) {
        constants::lan_pairing::PLATFORM_ROUTER
    } else {
        constants::lan_pairing::PLATFORM_UNKNOWN
    }
    .to_string();
    let trusted_hostname = trusted_device_hostname(trusted_device);
    let trusted_label = trusted_device_label(trusted_device);
    let trusted_platform = trusted_device_platform(trusted_device);
    let cached_identity = cached_neighbor_identity(mac_address);
    let hostname_hint = hostname_hint.and_then(|value| normalize_neighbor_hostname(&value));
    let previous_hostname = previous_device
        .and_then(|device| device.hostname.clone())
        .and_then(|value| normalize_neighbor_hostname(&value));
    let previous_label = previous_inventory_label(previous_device);
    let previous_platform = previous_device
        .map(|device| device.platform.clone())
        .filter(|value| !value.is_empty());
    let cached_platform = cached_identity
        .as_ref()
        .and_then(|identity| identity.platform.clone());
    let used_previous_scan_hostname = hostname_hint.is_none()
        && trusted_hostname.is_none()
        && cached_identity.is_none()
        && previous_hostname.is_some();

    if platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        if let Some(trusted_platform) = trusted_platform.clone() {
            platform = trusted_platform;
        } else if let Some(cached_platform) = cached_platform {
            platform = cached_platform;
        } else if let Some(previous_platform) = previous_platform.clone() {
            platform = previous_platform;
        }
    }

    let hostname = hostname_hint
        .or(trusted_hostname)
        .or_else(|| cached_identity.map(|identity| identity.hostname))
        .or(previous_hostname)
        .filter(|value| !value.is_empty());
    if let Some(hostname) = hostname.as_deref() {
        remember_neighbor_identity(mac_address, hostname, &platform);
    }

    let used_previous_scan_label =
        hostname.is_none() && trusted_label.is_none() && previous_label.is_some();
    let used_previous_scan_platform = platform != constants::lan_pairing::PLATFORM_ROUTER
        && trusted_platform.is_none()
        && previous_platform.is_some();
    let used_previous_scan_hint =
        used_previous_scan_hostname || used_previous_scan_label || used_previous_scan_platform;
    let label = hostname
        .clone()
        .or(trusted_label)
        .or(previous_label)
        .unwrap_or_else(|| network_neighbor_label(ip_address));

    ResolvedMacosNeighborIdentity {
        platform,
        hostname,
        label,
        used_previous_scan_hint,
    }
}

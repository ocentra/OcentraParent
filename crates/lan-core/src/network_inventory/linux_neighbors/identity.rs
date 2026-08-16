use chrono::Utc;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use crate::network_inventory_command::command_stdout;

use super::super::name_evidence::reverse_dns_name_evidence;
use super::super::neighbor_support::{
    cached_neighbor_identity, likely_router_address_text, network_neighbor_label,
    normalize_neighbor_hostname, previous_inventory_label, remember_neighbor_identity,
    trusted_device_hostname, trusted_device_label, trusted_device_platform,
};
use super::super::{
    LanIdentityHintInventory, LanNeighborObservation, LanNetworkInventoryDevice,
    LanPreviousNetworkInventory,
};

pub struct ResolvedLinuxNeighborIdentity {
    platform: String,
    hostname: Option<String>,
    label: String,
    used_previous_scan_hint: bool,
    name_scan_sources: Vec<String>,
}

pub fn network_device_from_neighbor_observation(
    observation: LanNeighborObservation,
    trusted_inventory: &LanIdentityHintInventory,
    previous_inventory: &LanPreviousNetworkInventory,
) -> Option<LanNetworkInventoryDevice> {
    let LanNeighborObservation {
        ip_address,
        mac_address,
        network_interface,
        hostname: hostname_hint,
        observed_at,
        reachability,
        scan_sources,
    } = observation;

    let trusted_device = trusted_inventory.find(&mac_address, &ip_address);
    let previous_device = previous_inventory.find(&mac_address, &ip_address);
    let resolved_identity = resolved_linux_neighbor_identity(
        &ip_address,
        &mac_address,
        hostname_hint,
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

    let mut scan_sources = scan_sources;
    for source in &resolved_identity.name_scan_sources {
        push_unique_scan_source(&mut scan_sources, source);
    }

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

pub fn resolved_linux_neighbor_identity(
    ip_address: &str,
    mac_address: &str,
    hostname_hint: Option<String>,
    trusted_device: Option<&LanPairingDeviceRef>,
    previous_device: Option<&LanNetworkInventoryDevice>,
) -> ResolvedLinuxNeighborIdentity {
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
    let reverse_dns_hostname = if hostname_hint.is_none() {
        reverse_dns_hostname(ip_address)
    } else {
        None
    };
    let reverse_dns_hostname_present = reverse_dns_hostname.is_some();
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
        } else if let Some(cached_platform) = cached_platform.clone() {
            platform = cached_platform;
        } else if let Some(previous_platform) = previous_platform.clone() {
            platform = previous_platform;
        }
    }
    let hostname = hostname_hint
        .or(reverse_dns_hostname)
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
        && cached_platform.is_none()
        && previous_platform.is_some();
    let used_previous_scan_hint =
        used_previous_scan_hostname || used_previous_scan_label || used_previous_scan_platform;
    let label = hostname
        .clone()
        .or(trusted_label)
        .or(previous_label)
        .unwrap_or_else(|| network_neighbor_label(ip_address));

    ResolvedLinuxNeighborIdentity {
        platform,
        hostname,
        label,
        used_previous_scan_hint,
        name_scan_sources: linux_neighbor_name_scan_sources(reverse_dns_hostname_present),
    }
}

pub fn linux_neighbor_name_scan_sources(has_reverse_dns_name: bool) -> Vec<String> {
    let mut sources = Vec::new();
    if has_reverse_dns_name {
        push_unique_scan_source(
            &mut sources,
            constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE,
        );
    }
    sources
}

pub fn reverse_dns_hostname(ip_address: &str) -> Option<String> {
    command_stdout("getent", &["hosts", ip_address]).and_then(|output| {
        output
            .lines()
            .find_map(reverse_dns_hostname_from_getent_line)
    })
}

pub fn reverse_dns_hostname_from_getent_line(line: &str) -> Option<String> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.len() < 2 {
        return None;
    }
    reverse_dns_name_evidence(columns[1], &Utc::now().to_rfc3339(), None)
        .map(|evidence| evidence.value)
}

pub fn push_unique_scan_source(scan_sources: &mut Vec<String>, value: &str) {
    if scan_sources.iter().any(|existing| existing == value) {
        return;
    }
    scan_sources.push(value.to_string());
}

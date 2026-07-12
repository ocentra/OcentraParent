pub mod cache;
mod hostname;
mod source_mapping;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Mutex, OnceLock};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;

use super::{LanNeighborObservation, LanNetworkInventoryDevice};

pub static NEIGHBOR_HOSTNAME_CACHE: OnceLock<
    Mutex<HashMap<String, LanNeighborIdentityCacheEntry>>,
> = OnceLock::new();

pub const MAX_NEIGHBOR_HOSTNAME_BYTES: usize = 255;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanNeighborIdentityCacheEntry {
    pub hostname: String,
    pub platform: Option<String>,
}

pub fn previous_inventory_label(
    previous_device: Option<&LanNetworkInventoryDevice>,
) -> Option<String> {
    previous_device
        .map(|device| device.label.clone())
        .filter(|label| {
            !label.is_empty()
                && !label.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
        })
}

pub fn trusted_device_hostname(trusted_device: Option<&LanPairingDeviceRef>) -> Option<String> {
    trusted_device
        .and_then(|device| device.hostname.clone())
        .and_then(|hostname| normalize_neighbor_hostname(&hostname))
}

pub fn trusted_device_label(trusted_device: Option<&LanPairingDeviceRef>) -> Option<String> {
    trusted_device
        .map(|device| device.label.trim().to_string())
        .filter(|label| !label.is_empty())
}

pub fn trusted_device_platform(trusted_device: Option<&LanPairingDeviceRef>) -> Option<String> {
    trusted_device
        .map(|device| device.platform.trim().to_string())
        .filter(|platform| !platform.is_empty())
}

pub fn effective_scan_sources(device: &LanNetworkInventoryDevice) -> Vec<String> {
    if device.scan_sources.is_empty() {
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()]
    } else {
        device.scan_sources.clone()
    }
}

pub fn discovery_evidence_source_from_scan_source(
    scan_source: &str,
) -> Option<LanDiscoveryEvidenceSource> {
    source_mapping::discovery_evidence_source_from_scan_source(scan_source)
}

pub fn is_supported_neighbor_ip(ip_address: &str) -> bool {
    match ip_address.parse::<IpAddr>() {
        Ok(IpAddr::V4(ip)) => is_household_unicast(ip),
        Ok(IpAddr::V6(ip)) => !ip.is_loopback() && !ip.is_multicast() && !ip.is_unspecified(),
        Err(_) => false,
    }
}

pub fn likely_router_address_text(ip_address: &str) -> bool {
    ip_address
        .parse::<Ipv4Addr>()
        .ok()
        .map(likely_router_address)
        .unwrap_or(false)
}

pub fn network_neighbor_label(ip_address: &str) -> String {
    let mut label = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX);
    label.push_str(ip_address);
    label
}

pub fn normalized_interface_name(value: &str) -> Option<String> {
    let candidate = value.trim();
    (!candidate.is_empty()).then(|| candidate.to_string())
}

pub fn normalized_optional_interface_name(value: Option<String>) -> Option<String> {
    value.and_then(|interface| normalized_interface_name(&interface))
}

pub fn interface_matches_selected_scope(
    network_interface: Option<&str>,
    selected_interface: Option<&str>,
) -> bool {
    let Some(selected_interface) = selected_interface.and_then(normalized_interface_name) else {
        return true;
    };
    let Some(network_interface) = network_interface.and_then(normalized_interface_name) else {
        return false;
    };
    network_interface.eq_ignore_ascii_case(&selected_interface)
}

pub fn filter_neighbor_observations_for_selected_interface(
    observations: Vec<LanNeighborObservation>,
    selected_interface: Option<&str>,
) -> Vec<LanNeighborObservation> {
    observations
        .into_iter()
        .filter(|observation| {
            interface_matches_selected_scope(
                observation.network_interface.as_deref(),
                selected_interface,
            )
        })
        .collect()
}

pub fn is_household_unicast(ip: Ipv4Addr) -> bool {
    ip.is_private()
        && !ip.is_broadcast()
        && !ip.is_link_local()
        && !ip.is_loopback()
        && !ip.is_multicast()
        && !ip.is_unspecified()
}

pub fn likely_router_address(ip: Ipv4Addr) -> bool {
    matches!(ip.octets()[3], 1 | 254)
}

pub fn remember_neighbor_identity(mac_address: &str, hostname: &str, platform: &str) {
    let Some(hostname) = normalize_neighbor_hostname(hostname) else {
        return;
    };
    let platform = if platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        None
    } else {
        Some(platform.to_string())
    };
    let cache = NEIGHBOR_HOSTNAME_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let _ = cache.lock().map(|mut entries| {
        entries.insert(
            mac_address.to_ascii_lowercase(),
            LanNeighborIdentityCacheEntry { hostname, platform },
        )
    });
}

pub fn cached_neighbor_identity(mac_address: &str) -> Option<LanNeighborIdentityCacheEntry> {
    NEIGHBOR_HOSTNAME_CACHE
        .get()
        .and_then(|cache| cache.lock().ok())
        .and_then(|entries| entries.get(&mac_address.to_ascii_lowercase()).cloned())
}

pub fn normalize_neighbor_hostname(value: &str) -> Option<String> {
    hostname::normalize_neighbor_hostname(value)
}

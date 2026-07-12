use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use super::super::neighbor_support::{
    cached_neighbor_identity, network_neighbor_label, normalize_neighbor_hostname,
    previous_inventory_label, remember_neighbor_identity, trusted_device_hostname,
    trusted_device_label, trusted_device_platform, LanNeighborIdentityCacheEntry,
};
use super::super::LanNetworkInventoryDevice;
use super::netbios::{direct_netbios_hostname, warm_netbios_cache};

pub struct ResolvedWindowsNeighborIdentity {
    pub platform: String,
    pub hostname: Option<String>,
    pub label: String,
    pub used_previous_scan_hint: bool,
    pub name_scan_sources: Vec<String>,
}

pub struct WindowsNeighborIdentityInput<'a> {
    pub ip_address: &'a str,
    pub mac_address: &'a str,
    pub platform: String,
    pub supports_netbios: bool,
    pub reachability: &'a LanPairingDeviceReachability,
    pub dns_hostname: Option<String>,
    pub netbios_cache_hostname: Option<String>,
    pub trusted_device: Option<&'a LanPairingDeviceRef>,
    pub previous_device: Option<&'a LanNetworkInventoryDevice>,
}

pub struct WindowsNeighborReuseState {
    trusted_hostname: Option<String>,
    trusted_label: Option<String>,
    trusted_platform: Option<String>,
    cached_identity: Option<LanNeighborIdentityCacheEntry>,
    previous_hostname: Option<String>,
    previous_label: Option<String>,
    previous_platform: Option<String>,
    cached_platform: Option<String>,
    has_reusable_identity: bool,
}

pub fn resolved_windows_neighbor_identity(
    input: WindowsNeighborIdentityInput<'_>,
) -> ResolvedWindowsNeighborIdentity {
    let WindowsNeighborIdentityInput {
        ip_address,
        mac_address,
        mut platform,
        supports_netbios,
        reachability,
        dns_hostname,
        netbios_cache_hostname,
        trusted_device,
        previous_device,
    } = input;
    let dns_hostname_present = dns_hostname.is_some();
    let netbios_cache_hostname_present = netbios_cache_hostname.is_some();
    let netbios_cache_hostname_missing = netbios_cache_hostname.is_none();
    let reuse_state = windows_neighbor_reuse_state(mac_address, trusted_device, previous_device);
    let direct_hostname = windows_neighbor_direct_hostname(
        ip_address,
        reachability,
        &platform,
        dns_hostname.as_ref(),
        netbios_cache_hostname.as_ref(),
        &reuse_state,
        supports_netbios,
    );
    let direct_hostname_presence = (direct_hostname.is_some(), direct_hostname.is_none());
    let used_previous_scan_hostname = windows_neighbor_used_previous_scan_hostname_hint(
        &dns_hostname,
        &netbios_cache_hostname,
        &reuse_state,
    );
    platform = resolve_windows_neighbor_platform(
        platform,
        &reuse_state,
        netbios_cache_hostname.as_ref(),
        direct_hostname.as_ref(),
    );
    let hostname = resolve_windows_neighbor_hostname(
        dns_hostname,
        netbios_cache_hostname,
        &reuse_state,
        direct_hostname,
    );
    let used_previous_scan_label = hostname.is_none()
        && reuse_state.trusted_label.is_none()
        && reuse_state.previous_label.is_some();
    let used_previous_scan_platform = windows_neighbor_used_previous_scan_platform_hint(
        &platform,
        &reuse_state,
        netbios_cache_hostname_missing,
        direct_hostname_presence.1,
    );
    persist_windows_neighbor_identity_side_effects(
        ip_address,
        reachability,
        mac_address,
        &platform,
        hostname.as_deref(),
        reuse_state.has_reusable_identity,
        supports_netbios,
    );
    let used_previous_scan_hint =
        used_previous_scan_hostname || used_previous_scan_label || used_previous_scan_platform;
    let name_scan_sources = windows_neighbor_name_scan_sources(
        dns_hostname_present,
        netbios_cache_hostname_present || direct_hostname_presence.0,
    );
    let label = resolved_windows_neighbor_label(ip_address, &hostname, reuse_state);

    ResolvedWindowsNeighborIdentity {
        platform,
        hostname,
        label,
        used_previous_scan_hint,
        name_scan_sources,
    }
}

pub fn resolved_windows_neighbor_label(
    ip_address: &str,
    hostname: &Option<String>,
    reuse_state: WindowsNeighborReuseState,
) -> String {
    hostname
        .clone()
        .or(reuse_state.trusted_label)
        .or(reuse_state.previous_label)
        .unwrap_or_else(|| network_neighbor_label(ip_address))
}

pub fn windows_neighbor_name_scan_sources(
    has_dns_cache_name: bool,
    has_netbios_name: bool,
) -> Vec<String> {
    let mut sources = Vec::new();
    if has_dns_cache_name {
        push_unique_scan_source(
            &mut sources,
            constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE,
        );
    }
    if has_netbios_name {
        push_unique_scan_source(
            &mut sources,
            constants::lan_pairing::LAN_SCAN_SOURCE_NETBIOS,
        );
    }
    sources
}

pub fn windows_neighbor_direct_hostname(
    ip_address: &str,
    reachability: &LanPairingDeviceReachability,
    platform: &str,
    dns_hostname: Option<&String>,
    netbios_cache_hostname: Option<&String>,
    reuse_state: &WindowsNeighborReuseState,
    supports_netbios: bool,
) -> Option<String> {
    if dns_hostname.is_none()
        && netbios_cache_hostname.is_none()
        && reuse_state.cached_identity.is_none()
        && !reuse_state.has_reusable_identity
        && supports_netbios
    {
        direct_netbios_hostname(ip_address, reachability, platform)
    } else {
        None
    }
}

pub fn windows_neighbor_used_previous_scan_hostname_hint(
    dns_hostname: &Option<String>,
    netbios_cache_hostname: &Option<String>,
    reuse_state: &WindowsNeighborReuseState,
) -> bool {
    dns_hostname.is_none()
        && netbios_cache_hostname.is_none()
        && reuse_state.trusted_hostname.is_none()
        && reuse_state.cached_identity.is_none()
        && reuse_state.previous_hostname.is_some()
}

pub fn resolve_windows_neighbor_platform(
    platform: String,
    reuse_state: &WindowsNeighborReuseState,
    netbios_cache_hostname: Option<&String>,
    direct_hostname: Option<&String>,
) -> String {
    if platform != constants::lan_pairing::PLATFORM_UNKNOWN {
        return platform;
    }
    if let Some(trusted_platform) = reuse_state.trusted_platform.clone() {
        return trusted_platform;
    }
    if netbios_cache_hostname.is_some() || direct_hostname.is_some() {
        return constants::lan_pairing::PLATFORM_WINDOWS.to_string();
    }
    reuse_state
        .cached_platform
        .clone()
        .or_else(|| reuse_state.previous_platform.clone())
        .unwrap_or(platform)
}

pub fn resolve_windows_neighbor_hostname(
    dns_hostname: Option<String>,
    netbios_cache_hostname: Option<String>,
    reuse_state: &WindowsNeighborReuseState,
    direct_hostname: Option<String>,
) -> Option<String> {
    dns_hostname
        .or(netbios_cache_hostname)
        .or(reuse_state.trusted_hostname.clone())
        .or_else(|| {
            reuse_state
                .cached_identity
                .as_ref()
                .map(|identity| identity.hostname.clone())
        })
        .or(reuse_state.previous_hostname.clone())
        .or(direct_hostname)
        .filter(|value| !value.is_empty())
}

pub fn windows_neighbor_used_previous_scan_platform_hint(
    platform: &str,
    reuse_state: &WindowsNeighborReuseState,
    netbios_cache_hostname_missing: bool,
    direct_hostname_missing: bool,
) -> bool {
    platform != constants::lan_pairing::PLATFORM_ROUTER
        && reuse_state.trusted_platform.is_none()
        && netbios_cache_hostname_missing
        && direct_hostname_missing
        && reuse_state.cached_platform.is_none()
        && reuse_state.previous_platform.is_some()
}

pub fn persist_windows_neighbor_identity_side_effects(
    ip_address: &str,
    reachability: &LanPairingDeviceReachability,
    mac_address: &str,
    platform: &str,
    hostname: Option<&str>,
    has_reusable_identity: bool,
    supports_netbios: bool,
) {
    if let Some(hostname) = hostname {
        remember_neighbor_identity(mac_address, hostname, platform);
        return;
    }
    if !has_reusable_identity && supports_netbios {
        warm_netbios_cache(ip_address, reachability, platform);
    }
}

pub fn windows_neighbor_reuse_state(
    mac_address: &str,
    trusted_device: Option<&LanPairingDeviceRef>,
    previous_device: Option<&LanNetworkInventoryDevice>,
) -> WindowsNeighborReuseState {
    let trusted_hostname = trusted_device_hostname(trusted_device);
    let trusted_label = trusted_device_label(trusted_device);
    let trusted_platform = trusted_device_platform(trusted_device);
    let cached_identity = cached_neighbor_identity(mac_address);
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
    let has_reusable_identity = trusted_hostname.is_some()
        || trusted_label.is_some()
        || trusted_platform.is_some()
        || previous_hostname.is_some()
        || previous_label.is_some()
        || previous_platform.is_some();

    WindowsNeighborReuseState {
        trusted_hostname,
        trusted_label,
        trusted_platform,
        cached_identity,
        previous_hostname,
        previous_label,
        previous_platform,
        cached_platform,
        has_reusable_identity,
    }
}

pub fn push_unique_scan_source(scan_sources: &mut Vec<String>, value: &str) {
    if scan_sources.iter().any(|existing| existing == value) {
        return;
    }
    scan_sources.push(value.to_string());
}

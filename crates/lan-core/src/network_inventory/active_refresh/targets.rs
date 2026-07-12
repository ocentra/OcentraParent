use std::collections::HashSet;
use std::net::Ipv4Addr;

use ocentra_parent_agent_protocol::constants;

use super::super::neighbor_support::is_household_unicast;
use super::super::{LanDiscoveryRefreshMode, LanNetworkInventoryDevice};
use super::suppression::{
    scan_plan_suppressed_active_ipv4_targets, suppressed_active_ipv4_targets,
};
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

pub fn refresh_metrics(
    refresh_mode: LanDiscoveryRefreshMode,
    local_ip_address: Option<&str>,
    ipv4_cidr: Option<&str>,
    default_gateway: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> (u32, u32, u32, Option<u64>, Vec<String>) {
    if refresh_mode != LanDiscoveryRefreshMode::ActiveSubnetRefresh {
        return (0, 0, 0, None, Vec::new());
    }
    let candidate_targets = bounded_active_ipv4_candidate_target_ips(local_ip_address, ipv4_cidr);
    let suppressed_targets = scan_plan_suppressed_active_ipv4_targets(
        default_gateway,
        active_refresh_suppression_devices,
        previous_devices,
    );
    let prioritized_targets =
        prioritized_active_ipv4_targets(&candidate_targets, &suppressed_targets, previous_devices);
    let suppressed_active_ipv4_targets =
        sorted_ipv4_targets_to_strings(suppressed_targets.iter().copied());
    (
        saturating_u32(candidate_targets.len()),
        saturating_u32(active_target_count(&candidate_targets, &suppressed_targets)),
        saturating_u32(prioritized_targets.len()),
        active_ipv4_target_timeout_ms(),
        suppressed_active_ipv4_targets,
    )
}

pub fn active_target_count(
    candidate_targets: &[Ipv4Addr],
    suppressed_targets: &HashSet<Ipv4Addr>,
) -> usize {
    candidate_targets
        .iter()
        .filter(|target| !suppressed_targets.contains(target))
        .count()
}

pub fn active_ipv4_target_timeout_ms() -> Option<u64> {
    active_ipv4_target_timeout_for_platform()
}

#[cfg(target_os = "windows")]
fn active_ipv4_target_timeout_for_platform() -> Option<u64> {
    Some(200)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn active_ipv4_target_timeout_for_platform() -> Option<u64> {
    Some(1000)
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "android")))]
fn active_ipv4_target_timeout_for_platform() -> Option<u64> {
    None
}

pub fn saturating_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

pub fn bounded_active_ipv4_targets(
    ip_address: Option<&str>,
    ipv4_cidr: Option<&str>,
    default_gateway: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<String> {
    ipv4_targets_to_strings(bounded_active_ipv4_target_ips(
        ip_address,
        ipv4_cidr,
        default_gateway,
        active_refresh_suppression_devices,
        previous_devices,
    ))
}

pub fn bounded_active_ipv4_target_ips(
    ip_address: Option<&str>,
    ipv4_cidr: Option<&str>,
    default_gateway: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<Ipv4Addr> {
    let candidate_targets = bounded_active_ipv4_candidate_target_ips(ip_address, ipv4_cidr);
    if candidate_targets.is_empty() {
        return candidate_targets;
    }
    let suppressed_targets = suppressed_active_ipv4_targets(
        default_gateway,
        active_refresh_suppression_devices,
        previous_devices,
    );
    let mut prioritized_targets =
        prioritized_active_ipv4_targets(&candidate_targets, &suppressed_targets, previous_devices);
    let prioritized_index = prioritized_targets.iter().copied().collect::<HashSet<_>>();
    for target in candidate_targets {
        if suppressed_targets.contains(&target) || prioritized_index.contains(&target) {
            continue;
        }
        prioritized_targets.push(target);
    }
    prioritized_targets
}

pub fn bounded_active_ipv4_candidate_target_ips(
    ip_address: Option<&str>,
    ipv4_cidr: Option<&str>,
) -> Vec<Ipv4Addr> {
    let Some((host_ip, prefix_length)) = parse_ipv4_cidr(ip_address, ipv4_cidr) else {
        return Vec::new();
    };
    let effective_prefix_length = prefix_length.max(24);
    if effective_prefix_length >= 31 {
        return Vec::new();
    }
    let host_bits = 32_u32.saturating_sub(u32::from(effective_prefix_length));
    let mask = u32::MAX << host_bits;
    let network = u32::from(host_ip) & mask;
    let broadcast = network | !mask;
    let mut targets = Vec::new();
    for raw_ip in
        ((network.saturating_add(1))..broadcast).filter(|raw_ip| *raw_ip != u32::from(host_ip))
    {
        targets.push(Ipv4Addr::from(raw_ip));
        if targets.len() >= constants::lan_pairing::LAN_ACTIVE_IPV4_SWEEP_MAX_HOSTS as usize {
            break;
        }
    }
    targets
}

pub fn ipv4_targets_to_strings<T>(targets: T) -> Vec<String>
where
    T: IntoIterator<Item = Ipv4Addr>,
{
    targets
        .into_iter()
        .map(|target| target.to_string())
        .collect()
}

pub fn sorted_ipv4_targets_to_strings<T>(targets: T) -> Vec<String>
where
    T: IntoIterator<Item = Ipv4Addr>,
{
    let mut targets = ipv4_targets_to_strings(targets);
    targets.sort();
    targets
}

pub fn prioritized_active_ipv4_targets(
    candidate_targets: &[Ipv4Addr],
    suppressed_targets: &HashSet<Ipv4Addr>,
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<Ipv4Addr> {
    let candidate_index = candidate_targets.iter().copied().collect::<HashSet<_>>();
    let mut prioritized = Vec::new();
    let mut prioritized_index = HashSet::new();
    for prefer_unresolved in [true, false] {
        for previous_device in previous_devices {
            if prefer_unresolved != previous_device_needs_active_refresh_priority(previous_device) {
                continue;
            }
            let Some(ip_address) =
                normalized_household_ipv4_ip(Some(previous_device.ip_address.as_str()))
            else {
                continue;
            };
            if !candidate_index.contains(&ip_address)
                || suppressed_targets.contains(&ip_address)
                || !prioritized_index.insert(ip_address)
            {
                continue;
            }
            prioritized.push(ip_address);
        }
    }
    prioritized
}

pub fn previous_device_needs_active_refresh_priority(
    previous_device: &LanNetworkInventoryDevice,
) -> bool {
    previous_device.used_previous_scan_hint
        || previous_device.platform == constants::lan_pairing::PLATFORM_UNKNOWN
        || previous_device.hostname.is_none()
        || previous_device
            .label
            .starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
}

pub fn parse_ipv4_cidr(
    ip_address: Option<&str>,
    ipv4_cidr: Option<&str>,
) -> Option<(Ipv4Addr, u8)> {
    let host_ip = ip_address?.parse::<Ipv4Addr>().ok()?;
    let (cidr_ip, prefix_length) = ipv4_cidr?.split_once('/')?;
    let cidr_ip = cidr_ip.parse::<Ipv4Addr>().ok()?;
    let prefix_length = prefix_length.parse::<u8>().ok()?;
    (prefix_length <= 32 && same_ipv4_subnet(host_ip, cidr_ip, prefix_length))
        .then_some((host_ip, prefix_length))
}

pub fn normalized_household_ipv4_ip(ip_address: Option<&str>) -> Option<Ipv4Addr> {
    let ip = ip_address?.trim().parse::<Ipv4Addr>().ok()?;
    is_household_unicast(ip).then_some(ip)
}

pub fn same_ipv4_subnet(
    target_ip_address: Ipv4Addr,
    local_ip_address: Ipv4Addr,
    prefix_length: u8,
) -> bool {
    if prefix_length == 0 {
        return true;
    }
    let host_bits = 32_u32.saturating_sub(u32::from(prefix_length));
    let mask = u32::MAX << host_bits;
    (u32::from(target_ip_address) & mask) == (u32::from(local_ip_address) & mask)
}

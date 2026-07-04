use std::collections::{HashMap, HashSet};
use std::net::Ipv4Addr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use super::super::LanNetworkInventoryDevice;
use super::observations::current_active_refresh_ipv4_observations_by_ip;
use super::targets::normalized_household_ipv4_ip;

pub fn suppressed_active_ipv4_targets(
    default_gateway: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> HashSet<Ipv4Addr> {
    let current_observations = current_active_refresh_ipv4_observations_by_ip();
    suppressed_active_ipv4_targets_for_current_observations(
        default_gateway,
        active_refresh_suppression_devices,
        previous_devices,
        &current_observations,
    )
}

pub fn scan_plan_suppressed_active_ipv4_targets(
    default_gateway: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> HashSet<Ipv4Addr> {
    suppressed_active_ipv4_targets_for_current_observations(
        default_gateway,
        active_refresh_suppression_devices,
        previous_devices,
        &HashMap::new(),
    )
}

pub fn suppressed_active_ipv4_targets_for_current_observations(
    default_gateway: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    current_observations: &HashMap<Ipv4Addr, String>,
) -> HashSet<Ipv4Addr> {
    let mut targets = HashSet::new();
    for truth_device in active_refresh_suppression_devices {
        if let Some(ip_address) = normalized_household_ipv4_ip(truth_device.ip_address.as_deref()) {
            if !active_refresh_target_is_safely_confirmed(
                default_gateway,
                ip_address,
                truth_device.mac_address.as_deref(),
                current_observations,
            ) {
                continue;
            }
            targets.insert(ip_address);
        }
    }
    for previous_device in previous_devices {
        if !previous_device_should_skip_active_refresh(previous_device, default_gateway) {
            continue;
        }
        if let Some(ip_address) =
            normalized_household_ipv4_ip(Some(previous_device.ip_address.as_str()))
        {
            if !active_refresh_target_is_safely_confirmed(
                default_gateway,
                ip_address,
                Some(previous_device.mac_address.as_str()),
                current_observations,
            ) {
                continue;
            }
            targets.insert(ip_address);
        }
    }
    targets
}

pub fn active_refresh_target_is_safely_confirmed(
    default_gateway: Option<&str>,
    ip_address: Ipv4Addr,
    expected_mac_address: Option<&str>,
    current_observations: &HashMap<Ipv4Addr, String>,
) -> bool {
    if default_gateway
        .and_then(|gateway| normalized_household_ipv4_ip(Some(gateway)))
        .map(|gateway| gateway == ip_address)
        .unwrap_or(false)
    {
        return true;
    }

    current_observation_confirms_ip_and_mac(current_observations, ip_address, expected_mac_address)
}

pub fn current_observation_confirms_ip_and_mac(
    current_observations: &HashMap<Ipv4Addr, String>,
    ip_address: Ipv4Addr,
    expected_mac_address: Option<&str>,
) -> bool {
    let Some(expected_mac_address) = expected_mac_address
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return false;
    };

    current_observations
        .get(&ip_address)
        .map(|current_mac_address| current_mac_address.eq_ignore_ascii_case(expected_mac_address))
        .unwrap_or(false)
}

pub fn previous_device_should_skip_active_refresh(
    previous_device: &LanNetworkInventoryDevice,
    default_gateway: Option<&str>,
) -> bool {
    if previous_device.platform == constants::lan_pairing::PLATFORM_ROUTER {
        return true;
    }

    default_gateway
        .and_then(|gateway| normalized_household_ipv4_ip(Some(gateway)))
        .zip(normalized_household_ipv4_ip(Some(
            previous_device.ip_address.as_str(),
        )))
        .map(|(gateway, previous_device_ip)| gateway == previous_device_ip)
        .unwrap_or(false)
}

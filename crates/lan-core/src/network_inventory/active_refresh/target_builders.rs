use std::collections::HashSet;
use std::net::Ipv4Addr;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use crate::mac_identity::normalize_scan_mac_address;

use super::super::LanNetworkInventoryDevice;
use super::targets::{normalized_household_ipv4_ip, parse_ipv4_cidr, same_ipv4_subnet};
use super::TargetedArpRefreshTarget;

pub fn targeted_arp_refresh_targets(
    local_ip_address: Option<&str>,
    ipv4_cidr: Option<&str>,
    selected_interface: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<TargetedArpRefreshTarget> {
    let Some((local_ip_address, prefix_length)) = parse_ipv4_cidr(local_ip_address, ipv4_cidr)
    else {
        return Vec::new();
    };
    let Some(selected_interface) = selected_interface
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Vec::new();
    };

    let mut targets = Vec::new();
    let mut seen_targets = HashSet::new();

    for trusted_device in active_refresh_suppression_devices {
        let target = target_from_trusted_device(
            trusted_device,
            local_ip_address,
            prefix_length,
            selected_interface,
        );
        push_targeted_arp_target(&mut targets, &mut seen_targets, target);
    }

    for previous_device in previous_devices {
        let target = target_from_previous_device(
            previous_device,
            local_ip_address,
            prefix_length,
            selected_interface,
        );
        push_targeted_arp_target(&mut targets, &mut seen_targets, target);
    }

    targets
}

pub fn bounded_active_ipv4_refresh_targets(
    local_ip_address: Option<&str>,
    ipv4_cidr: Option<&str>,
    default_gateway: Option<&str>,
    selected_interface: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<TargetedArpRefreshTarget> {
    let Some((local_ip_address, prefix_length)) = parse_ipv4_cidr(local_ip_address, ipv4_cidr)
    else {
        return Vec::new();
    };
    let Some(selected_interface) = selected_interface
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Vec::new();
    };

    let mut targets = Vec::new();
    let mut seen_targets = HashSet::new();
    let local_ip_address_text = local_ip_address.to_string();
    for ip_address in super::targets::bounded_active_ipv4_target_ips(
        Some(local_ip_address_text.as_str()),
        ipv4_cidr,
        default_gateway,
        active_refresh_suppression_devices,
        previous_devices,
    ) {
        let expected_mac_address = expected_mac_address_for_ip(
            ip_address,
            active_refresh_suppression_devices,
            previous_devices,
        );
        let target = target_from_ip_and_interface(
            ip_address,
            expected_mac_address.as_deref(),
            None,
            local_ip_address,
            prefix_length,
            selected_interface,
        );
        push_targeted_arp_target(&mut targets, &mut seen_targets, target);
    }

    targets
}

fn expected_mac_address_for_ip(
    ip_address: Ipv4Addr,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> Option<String> {
    active_refresh_suppression_devices
        .iter()
        .find_map(|device| {
            (normalized_household_ipv4_ip(device.ip_address.as_deref()) == Some(ip_address))
                .then(|| {
                    device
                        .mac_address
                        .as_deref()
                        .and_then(normalize_scan_mac_address)
                })
                .flatten()
        })
        .or_else(|| {
            previous_devices.iter().find_map(|device| {
                (normalized_household_ipv4_ip(Some(device.ip_address.as_str())) == Some(ip_address))
                    .then(|| normalize_scan_mac_address(&device.mac_address))
                    .flatten()
            })
        })
}

pub fn target_from_trusted_device(
    trusted_device: &LanPairingDeviceRef,
    local_ip_address: Ipv4Addr,
    prefix_length: u8,
    selected_interface: &str,
) -> Option<TargetedArpRefreshTarget> {
    let ip_address = normalized_household_ipv4_ip(trusted_device.ip_address.as_deref())?;
    target_from_ip_and_interface(
        ip_address,
        trusted_device.mac_address.as_deref(),
        trusted_device.network_interface.as_deref(),
        local_ip_address,
        prefix_length,
        selected_interface,
    )
}

pub fn target_from_previous_device(
    previous_device: &LanNetworkInventoryDevice,
    local_ip_address: Ipv4Addr,
    prefix_length: u8,
    selected_interface: &str,
) -> Option<TargetedArpRefreshTarget> {
    let ip_address = normalized_household_ipv4_ip(Some(previous_device.ip_address.as_str()))?;
    target_from_ip_and_interface(
        ip_address,
        Some(previous_device.mac_address.as_str()),
        previous_device.network_interface.as_deref(),
        local_ip_address,
        prefix_length,
        selected_interface,
    )
}

pub fn target_from_ip_and_interface(
    ip_address: Ipv4Addr,
    expected_mac_address: Option<&str>,
    network_interface: Option<&str>,
    local_ip_address: Ipv4Addr,
    prefix_length: u8,
    selected_interface: &str,
) -> Option<TargetedArpRefreshTarget> {
    let target_ip_address = ip_address;
    if target_ip_address == local_ip_address {
        return None;
    }
    if !same_ipv4_subnet(target_ip_address, local_ip_address, prefix_length) {
        return None;
    }
    if let Some(network_interface) = network_interface
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        if !network_interface.eq_ignore_ascii_case(selected_interface) {
            return None;
        }
    }

    Some(TargetedArpRefreshTarget {
        ip_address: target_ip_address,
        expected_mac_address: expected_mac_address.and_then(normalize_scan_mac_address),
        network_interface: Some(selected_interface.to_string()),
    })
}

pub fn push_targeted_arp_target(
    targets: &mut Vec<TargetedArpRefreshTarget>,
    seen_targets: &mut HashSet<Ipv4Addr>,
    target: Option<TargetedArpRefreshTarget>,
) {
    let Some(mut target) = target else {
        return;
    };
    target.expected_mac_address = target
        .expected_mac_address
        .take()
        .and_then(|value| normalize_scan_mac_address(&value));
    if !seen_targets.insert(target.ip_address) {
        if let Some(existing) = targets
            .iter_mut()
            .find(|existing| existing.ip_address == target.ip_address)
        {
            if existing.expected_mac_address.is_none() {
                existing.expected_mac_address = target.expected_mac_address;
            }
        }
        return;
    }
    targets.push(target);
}

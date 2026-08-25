use std::collections::HashSet;
use std::net::Ipv4Addr;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

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
    for ip_address in super::targets::bounded_active_ipv4_target_ips(
        Some(&local_ip_address.to_string()),
        Some(ipv4_cidr.unwrap_or_default()),
        default_gateway,
        active_refresh_suppression_devices,
        previous_devices,
    ) {
        let expected_mac_address = previous_devices.iter().find_map(|previous_device| {
            (normalized_household_ipv4_ip(Some(previous_device.ip_address.as_str()))
                == Some(ip_address))
            .then_some(previous_device.mac_address.as_str())
        });
        let target = target_from_ip_and_interface(
            ip_address,
            expected_mac_address,
            None,
            local_ip_address,
            prefix_length,
            selected_interface,
        );
        push_targeted_arp_target(&mut targets, &mut seen_targets, target);
    }

    for target in targeted_arp_refresh_targets(
        Some(&local_ip_address.to_string()),
        ipv4_cidr,
        Some(selected_interface),
        active_refresh_suppression_devices,
        previous_devices,
    ) {
        push_targeted_arp_target(&mut targets, &mut seen_targets, Some(target));
    }

    targets
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
        expected_mac_address: expected_mac_address
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string()),
        network_interface: Some(selected_interface.to_string()),
    })
}

pub fn push_targeted_arp_target(
    targets: &mut Vec<TargetedArpRefreshTarget>,
    seen_targets: &mut HashSet<Ipv4Addr>,
    target: Option<TargetedArpRefreshTarget>,
) {
    let Some(target) = target else {
        return;
    };
    if !seen_targets.insert(target.ip_address) {
        return;
    }
    targets.push(target);
}

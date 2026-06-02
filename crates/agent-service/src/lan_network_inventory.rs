use std::net::Ipv4Addr;

use ocentra_parent_agent_protocol::{constants, LanPairingDeviceReachability, LanPairingDeviceRef};

use crate::lan_network_inventory_command::{
    command_json_records, command_json_single_owned, normalize_mac_address, record_text, value_text,
};
use crate::lan_network_inventory_hardware::{local_hardware_profile, local_network_identity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanNetworkInventoryDevice {
    pub(crate) device_id: String,
    pub(crate) label: String,
    pub(crate) platform: String,
    pub(crate) ip_address: String,
    pub(crate) mac_address: String,
    pub(crate) hostname: Option<String>,
    pub(crate) network_interface: Option<String>,
    pub(crate) reachability: LanPairingDeviceReachability,
}

pub(crate) fn discover_lan_network_devices() -> Vec<LanNetworkInventoryDevice> {
    windows_lan_neighbors()
}

pub(crate) fn local_agent_device_ref(device_id: String, platform: String) -> LanPairingDeviceRef {
    let hardware_profile = local_hardware_profile();
    let network_identity = local_network_identity();
    let hostname = hardware_profile.hostname.clone();
    let mut device = LanPairingDeviceRef::new(
        device_id,
        None,
        constants::lan_pairing::LOCAL_AGENT_LABEL.to_string(),
        platform,
    );
    device.hostname = hostname;
    if let Some(identity) = network_identity {
        device.ip_address = identity.ip_address;
        device.mac_address = identity.mac_address;
        device.network_interface = identity.network_interface;
    }
    device.agent_status = Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    device.hardware_profile = Some(hardware_profile.into_protocol_profile());
    device
}

fn windows_lan_neighbors() -> Vec<LanNetworkInventoryDevice> {
    command_json_records(
        constants::lan_pairing::POWERSHELL_EXE,
        &[
            constants::lan_pairing::POWERSHELL_NO_PROFILE_ARG,
            constants::lan_pairing::POWERSHELL_EXECUTION_POLICY_ARG,
            constants::lan_pairing::POWERSHELL_BYPASS_ARG,
            constants::lan_pairing::POWERSHELL_COMMAND_ARG,
            constants::lan_pairing::POWERSHELL_LAN_NEIGHBOR_COMMAND,
        ],
    )
    .into_iter()
    .filter_map(network_device_from_windows_neighbor)
    .collect()
}

fn network_device_from_windows_neighbor(
    record: serde_json::Value,
) -> Option<LanNetworkInventoryDevice> {
    let ip_address = record_text(&record, constants::lan_pairing::JSON_KEY_IP_ADDRESS)?;
    let mac_address = normalize_mac_address(record_text(
        &record,
        constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS,
    )?)?;
    let ip = ip_address.parse::<Ipv4Addr>().ok()?;
    if !is_household_unicast(ip) {
        return None;
    }

    let platform = if likely_router_address(ip) {
        constants::lan_pairing::PLATFORM_ROUTER
    } else {
        constants::lan_pairing::PLATFORM_UNKNOWN
    }
    .to_string();
    let mut device_id = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_DEVICE_PREFIX);
    device_id.push_str(
        &mac_address
            .chars()
            .filter(|character| *character != '-')
            .collect::<String>(),
    );
    let hostname = record_text(&record, constants::lan_pairing::JSON_KEY_HOSTNAME)
        .or_else(|| reverse_dns_hostname(&ip_address));
    let label = hostname.clone().unwrap_or_else(|| {
        let mut fallback = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX);
        fallback.push_str(&ip_address);
        fallback
    });

    Some(LanNetworkInventoryDevice {
        device_id,
        label,
        platform,
        ip_address,
        mac_address,
        hostname,
        network_interface: record_text(&record, constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS),
        reachability: reachability_from_windows_state(
            record.get(constants::lan_pairing::JSON_KEY_STATE),
        ),
    })
}

fn is_household_unicast(ip: Ipv4Addr) -> bool {
    ip.is_private()
        && !ip.is_broadcast()
        && !ip.is_link_local()
        && !ip.is_loopback()
        && !ip.is_multicast()
        && !ip.is_unspecified()
}

fn likely_router_address(ip: Ipv4Addr) -> bool {
    matches!(ip.octets()[3], 1 | 254)
}

fn reverse_dns_hostname(ip_address: &str) -> Option<String> {
    let command = format!(
        "{}{}{}",
        constants::lan_pairing::POWERSHELL_REVERSE_DNS_COMMAND_PREFIX,
        ip_address,
        constants::lan_pairing::POWERSHELL_REVERSE_DNS_COMMAND_SUFFIX
    );
    command_json_single_owned(
        constants::lan_pairing::POWERSHELL_EXE,
        vec![
            constants::lan_pairing::POWERSHELL_NO_PROFILE_ARG.to_string(),
            constants::lan_pairing::POWERSHELL_EXECUTION_POLICY_ARG.to_string(),
            constants::lan_pairing::POWERSHELL_BYPASS_ARG.to_string(),
            constants::lan_pairing::POWERSHELL_COMMAND_ARG.to_string(),
            command,
        ],
    )
    .and_then(|value| value_text(&value))
}

fn reachability_from_windows_state(
    state: Option<&serde_json::Value>,
) -> LanPairingDeviceReachability {
    match state
        .and_then(value_text)
        .map(|value| value.to_ascii_lowercase())
    {
        Some(value)
            if value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE_NUMBER
                || value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_PERMANENT_NUMBER
                || value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE
                || value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_PERMANENT =>
        {
            LanPairingDeviceReachability::Online
        }
        Some(value)
            if value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_STALE_NUMBER
                || value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_STALE =>
        {
            LanPairingDeviceReachability::Stale
        }
        _ => LanPairingDeviceReachability::Offline,
    }
}

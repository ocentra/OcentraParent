use std::collections::{HashMap, HashSet};
use std::net::Ipv4Addr;
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};

use ocentra_parent_agent_protocol::{constants, LanPairingDeviceReachability, LanPairingDeviceRef};

use crate::lan_network_inventory_command::{
    command_json_records, command_stdout, normalize_mac_address, record_text, value_text,
};
use crate::lan_network_inventory_hardware::{local_hardware_profile, local_network_identity};

static NETBIOS_CACHE_WARMED_IPS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

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
    let label = hostname
        .clone()
        .unwrap_or_else(|| constants::lan_pairing::LOCAL_AGENT_LABEL.to_string());
    let mut device = LanPairingDeviceRef::new(device_id, None, label, platform);
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
    let netbios_names = windows_netbios_cache_names();
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
    .filter_map(|record| network_device_from_windows_neighbor(record, &netbios_names))
    .collect()
}

fn network_device_from_windows_neighbor(
    record: serde_json::Value,
    netbios_names: &HashMap<String, String>,
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
    let reachability =
        reachability_from_windows_state(record.get(constants::lan_pairing::JSON_KEY_STATE));
    let hostname = record_text(&record, constants::lan_pairing::JSON_KEY_HOSTNAME)
        .map(|value| value.trim_end_matches('.').to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| netbios_names.get(&ip_address).cloned());
    if hostname.is_none() {
        warm_netbios_cache(&ip_address, &reachability, &platform);
    }
    let label = hostname
        .clone()
        .unwrap_or_else(|| network_neighbor_label(&ip_address));

    Some(LanNetworkInventoryDevice {
        device_id,
        label,
        platform,
        ip_address,
        mac_address,
        hostname,
        network_interface: record_text(&record, constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS),
        reachability,
    })
}

fn windows_netbios_cache_names() -> HashMap<String, String> {
    command_stdout(
        constants::lan_pairing::NBTSTAT_EXE,
        &[constants::lan_pairing::NBTSTAT_CACHE_ARG],
    )
    .map(|output| {
        output
            .lines()
            .filter_map(netbios_cache_entry)
            .collect::<HashMap<_, _>>()
    })
    .unwrap_or_default()
}

fn netbios_cache_entry(line: &str) -> Option<(String, String)> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.len() < 4 || columns[2] != constants::lan_pairing::NBTSTAT_UNIQUE_MARKER {
        return None;
    }
    let ip = columns[3].parse::<Ipv4Addr>().ok()?;
    if !is_household_unicast(ip) {
        return None;
    }
    Some((columns[3].to_string(), columns[0].to_string()))
}

fn warm_netbios_cache(
    ip_address: &str,
    reachability: &LanPairingDeviceReachability,
    platform: &str,
) {
    if reachability != &LanPairingDeviceReachability::Online
        || platform == constants::lan_pairing::PLATFORM_ROUTER
    {
        return;
    }
    let cache = NETBIOS_CACHE_WARMED_IPS.get_or_init(|| Mutex::new(HashSet::new()));
    if !cache
        .lock()
        .map(|mut warmed| warmed.insert(ip_address.to_string()))
        .unwrap_or(false)
    {
        return;
    }
    let _ = Command::new(constants::lan_pairing::NBTSTAT_EXE)
        .arg(constants::lan_pairing::NBTSTAT_ADAPTER_STATUS_ARG)
        .arg(ip_address)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

fn network_neighbor_label(ip_address: &str) -> String {
    let mut label = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX);
    label.push_str(ip_address);
    label
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

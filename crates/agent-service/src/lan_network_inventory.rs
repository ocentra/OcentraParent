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
static NEIGHBOR_HOSTNAME_CACHE: OnceLock<Mutex<HashMap<String, LanNeighborIdentityCacheEntry>>> =
    OnceLock::new();

#[derive(Clone, Debug, PartialEq, Eq)]
struct LanNeighborIdentityCacheEntry {
    hostname: String,
    platform: Option<String>,
}

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

    let mut platform = if likely_router_address(ip) {
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
    let dns_hostname = record_text(&record, constants::lan_pairing::JSON_KEY_HOSTNAME)
        .map(|value| value.trim_end_matches('.').to_string())
        .filter(|value| !value.is_empty());
    let netbios_cache_hostname = netbios_names.get(&ip_address).cloned();
    let cached_identity = cached_neighbor_identity(&mac_address);
    let direct_hostname = if dns_hostname.is_none()
        && netbios_cache_hostname.is_none()
        && cached_identity.is_none()
    {
        direct_netbios_hostname(&ip_address, &reachability, &platform)
    } else {
        None
    };
    if platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        if netbios_cache_hostname.is_some() || direct_hostname.is_some() {
            platform = constants::lan_pairing::PLATFORM_WINDOWS.to_string();
        } else if let Some(cached_platform) = cached_identity
            .as_ref()
            .and_then(|identity| identity.platform.clone())
        {
            platform = cached_platform;
        }
    }
    let hostname = dns_hostname
        .or(netbios_cache_hostname)
        .or_else(|| cached_identity.map(|identity| identity.hostname))
        .or(direct_hostname)
        .filter(|value| !value.is_empty());
    if hostname.is_none() {
        warm_netbios_cache(&ip_address, &reachability, &platform);
    } else if let Some(hostname) = hostname.as_deref() {
        remember_neighbor_identity(&mac_address, hostname, &platform);
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

fn remember_neighbor_identity(mac_address: &str, hostname: &str, platform: &str) {
    if hostname == constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME {
        return;
    }
    let platform = if platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        None
    } else {
        Some(platform.to_string())
    };
    let cache = NEIGHBOR_HOSTNAME_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let _ = cache.lock().map(|mut entries| {
        entries.insert(
            mac_address.to_ascii_lowercase(),
            LanNeighborIdentityCacheEntry {
                hostname: hostname.to_string(),
                platform,
            },
        )
    });
}

fn cached_neighbor_identity(mac_address: &str) -> Option<LanNeighborIdentityCacheEntry> {
    NEIGHBOR_HOSTNAME_CACHE
        .get()
        .and_then(|cache| cache.lock().ok())
        .and_then(|entries| entries.get(&mac_address.to_ascii_lowercase()).cloned())
}

fn direct_netbios_hostname(
    ip_address: &str,
    reachability: &LanPairingDeviceReachability,
    platform: &str,
) -> Option<String> {
    if reachability != &LanPairingDeviceReachability::Online
        || platform == constants::lan_pairing::PLATFORM_ROUTER
    {
        return None;
    }
    command_stdout(
        constants::lan_pairing::NBTSTAT_EXE,
        &[
            constants::lan_pairing::NBTSTAT_ADAPTER_STATUS_ARG,
            ip_address,
        ],
    )
    .and_then(|output| output.lines().find_map(netbios_adapter_status_name))
}

fn netbios_adapter_status_name(line: &str) -> Option<String> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.len() < 3 || columns[2] != constants::lan_pairing::NBTSTAT_UNIQUE_MARKER {
        return None;
    }
    if columns[1] != constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER
        && columns[1] != constants::lan_pairing::NBTSTAT_SERVER_SERVICE_MARKER
    {
        return None;
    }
    Some(columns[0].to_string()).filter(|value| !value.is_empty())
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value};

    #[test]
    fn windows_neighbor_parser_keeps_cached_hostname_when_later_rows_are_ip_only() {
        if let Some(cache) = NEIGHBOR_HOSTNAME_CACHE.get() {
            cache
                .lock()
                .expect(constants::lan_pairing::TEST_NEIGHBOR_CACHE_LOCK_EXPECT)
                .clear();
        }
        let named = network_device_from_windows_neighbor(
            neighbor_record(Some(constants::lan_pairing::TEST_HOSTNAME)),
            &HashMap::new(),
        )
        .expect(constants::lan_pairing::TEST_NAMED_NEIGHBOR_ROW_PARSE_EXPECT);

        assert_eq!(
            named.hostname,
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
        );
        assert_eq!(named.label, constants::lan_pairing::TEST_HOSTNAME);

        let unnamed = network_device_from_windows_neighbor(neighbor_record(None), &HashMap::new())
            .expect(constants::lan_pairing::TEST_UNNAMED_NEIGHBOR_ROW_PARSE_EXPECT);

        assert_eq!(
            unnamed.hostname,
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
        );
        assert_eq!(unnamed.label, constants::lan_pairing::TEST_HOSTNAME);
    }

    #[test]
    fn netbios_adapter_status_name_prefers_unique_device_service_rows() {
        assert_eq!(
            netbios_adapter_status_name(
                &[
                    constants::lan_pairing::TEST_HOSTNAME,
                    constants::lan_pairing::NBTSTAT_SERVER_SERVICE_MARKER,
                    constants::lan_pairing::NBTSTAT_UNIQUE_MARKER,
                ]
                .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
            ),
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
        );
        assert_eq!(
            netbios_adapter_status_name(
                &[
                    constants::lan_pairing::TEST_HOSTNAME,
                    constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
                    constants::lan_pairing::NBTSTAT_UNIQUE_MARKER,
                ]
                .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
            ),
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
        );
        assert!(netbios_adapter_status_name(
            &[
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
                constants::lan_pairing::NBTSTAT_GROUP_MARKER,
            ]
            .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
        )
        .is_none());
    }

    fn neighbor_record(hostname: Option<&str>) -> Value {
        let mut record = Map::new();
        record.insert(
            constants::lan_pairing::JSON_KEY_IP_ADDRESS.to_string(),
            Value::String(constants::lan_pairing::TEST_LAN_IP.to_string()),
        );
        record.insert(
            constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS.to_string(),
            Value::String(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        );
        record.insert(
            constants::lan_pairing::JSON_KEY_STATE.to_string(),
            Value::String(constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE.to_string()),
        );
        if let Some(hostname) = hostname {
            record.insert(
                constants::lan_pairing::JSON_KEY_HOSTNAME.to_string(),
                Value::String(hostname.to_string()),
            );
        }
        Value::Object(record)
    }
}

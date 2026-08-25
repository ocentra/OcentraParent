use std::collections::{HashMap, HashSet};
use std::net::Ipv4Addr;
use std::sync::{atomic::AtomicBool, Mutex, OnceLock};
use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::network_inventory_command::{
    command_stdout, command_stdout_with_timeout_and_cancellation, command_succeeded_with_timeout,
};

use super::super::name_evidence::normalize_name_evidence_value;
use super::super::neighbor_support::is_household_unicast;

pub static NETBIOS_CACHE_WARMED_IPS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
pub const WINDOWS_FOREGROUND_NETBIOS_LOOKUP_ENABLED: bool = false;

pub fn windows_neighbor_supports_netbios(ip_address: &str) -> bool {
    ip_address
        .parse::<Ipv4Addr>()
        .map(is_household_unicast)
        .unwrap_or(false)
}

pub fn direct_netbios_hostname(
    ip_address: &str,
    reachability: &LanPairingDeviceReachability,
    platform: &str,
) -> Option<String> {
    if !WINDOWS_FOREGROUND_NETBIOS_LOOKUP_ENABLED {
        return None;
    }
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

pub fn netbios_adapter_status_name(line: &str) -> Option<String> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.len() < 3 || columns[2] != constants::lan_pairing::NBTSTAT_UNIQUE_MARKER {
        return None;
    }
    if columns[1] != constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER
        && columns[1] != constants::lan_pairing::NBTSTAT_SERVER_SERVICE_MARKER
    {
        return None;
    }
    normalize_name_evidence_value(columns[0])
}

pub fn windows_netbios_cache_names() -> HashMap<String, String> {
    windows_netbios_cache_names_with_cancellation(None)
}

pub fn windows_netbios_cache_names_with_cancellation(
    cancellation: Option<&AtomicBool>,
) -> HashMap<String, String> {
    let args = &[constants::lan_pairing::NBTSTAT_CACHE_ARG];
    let output = match cancellation {
        Some(cancellation) => command_stdout_with_timeout_and_cancellation(
            constants::lan_pairing::NBTSTAT_EXE,
            args,
            Duration::from_millis(constants::lan_pairing::LAN_NETWORK_INVENTORY_COMMAND_TIMEOUT_MS),
            cancellation,
        ),
        None => command_stdout(constants::lan_pairing::NBTSTAT_EXE, args),
    };
    output
        .map(|output| {
            output
                .lines()
                .filter_map(netbios_cache_entry)
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default()
}

pub fn netbios_cache_entry(line: &str) -> Option<(String, String)> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.len() < 4 || columns[2] != constants::lan_pairing::NBTSTAT_UNIQUE_MARKER {
        return None;
    }
    let ip = columns[3].parse::<Ipv4Addr>().ok()?;
    if !is_household_unicast(ip) {
        return None;
    }
    normalize_name_evidence_value(columns[0]).map(|hostname| (columns[3].to_string(), hostname))
}

pub fn warm_netbios_cache(
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
    let timeout =
        Duration::from_millis(constants::lan_pairing::LAN_NETWORK_INVENTORY_COMMAND_TIMEOUT_MS);
    let _ = command_succeeded_with_timeout(
        constants::lan_pairing::NBTSTAT_EXE,
        &[
            constants::lan_pairing::NBTSTAT_ADAPTER_STATUS_ARG,
            ip_address,
        ],
        timeout,
    );
}

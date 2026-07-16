use std::collections::HashMap;
use std::fs;
use std::net::Ipv4Addr;
use std::time::Duration;

use chrono::Utc;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::network_inventory_command::{
    command_json_records, command_json_records_with_timeout, normalize_mac_address, record_text,
};

use super::super::neighbor_support::{
    is_household_unicast, is_supported_neighbor_ip, normalized_optional_interface_name,
};
use super::super::LanNeighborObservation;
use super::merge::merge_neighbor_observations;

mod state;

pub fn current_linux_neighbor_ipv4_observations_with_timeout(
    timeout: Duration,
) -> HashMap<String, String> {
    let observed_at = Utc::now().to_rfc3339();
    current_linux_neighbor_ipv4_observations_from_observations(
        linux_ip_neigh_observations_with_timeout_and_observed_at(timeout, &observed_at)
            .into_iter()
            .chain(linux_proc_net_arp_observations_with_observed_at(
                &observed_at,
            ))
            .collect(),
    )
}

pub fn current_linux_proc_net_arp_ipv4_observations_with_timeout(
    _timeout: Duration,
) -> HashMap<String, String> {
    current_linux_neighbor_ipv4_observations_from_observations(
        linux_proc_net_arp_observations_with_observed_at(&Utc::now().to_rfc3339()),
    )
}

pub fn current_linux_ip_neigh_ipv4_observations_with_timeout(
    timeout: Duration,
) -> HashMap<String, String> {
    current_linux_neighbor_ipv4_observations_from_observations(
        linux_ip_neigh_observations_with_timeout_and_observed_at(timeout, &Utc::now().to_rfc3339()),
    )
}

pub fn current_linux_neighbor_ipv4_observations_from_observations(
    observations: Vec<LanNeighborObservation>,
) -> HashMap<String, String> {
    let mut by_ip = HashMap::new();
    for observation in merge_neighbor_observations(observations) {
        let Some(ip_address) = observation.ip_address.parse::<Ipv4Addr>().ok() else {
            continue;
        };
        if !is_household_unicast(ip_address) {
            continue;
        }
        by_ip
            .entry(ip_address.to_string().to_ascii_lowercase())
            .or_insert(observation.mac_address.to_ascii_lowercase());
    }
    by_ip
}

pub fn linux_ip_neigh_observations() -> Vec<LanNeighborObservation> {
    linux_ip_neigh_observations_with_observed_at(&Utc::now().to_rfc3339())
}

pub fn linux_ip_neigh_observations_with_observed_at(
    observed_at: &str,
) -> Vec<LanNeighborObservation> {
    command_json_records(
        constants::lan_pairing::IP_EXE,
        &[
            constants::lan_pairing::IP_JSON_ARG,
            constants::lan_pairing::IP_NEIGH_ARG,
        ],
    )
    .into_iter()
    .filter_map(|record| linux_ip_neigh_observation_with_observed_at(&record, observed_at))
    .collect()
}

pub fn linux_ip_neigh_observations_with_timeout(timeout: Duration) -> Vec<LanNeighborObservation> {
    linux_ip_neigh_observations_with_timeout_and_observed_at(timeout, &Utc::now().to_rfc3339())
}

pub fn linux_ip_neigh_observations_with_timeout_and_observed_at(
    timeout: Duration,
    observed_at: &str,
) -> Vec<LanNeighborObservation> {
    command_json_records_with_timeout(
        constants::lan_pairing::IP_EXE,
        &[
            constants::lan_pairing::IP_JSON_ARG,
            constants::lan_pairing::IP_NEIGH_ARG,
        ],
        timeout,
    )
    .into_iter()
    .filter_map(|record| linux_ip_neigh_observation_with_observed_at(&record, observed_at))
    .collect()
}

pub fn linux_ip_neigh_observation(record: &serde_json::Value) -> Option<LanNeighborObservation> {
    linux_ip_neigh_observation_with_observed_at(record, &Utc::now().to_rfc3339())
}

pub fn linux_ip_neigh_observation_with_observed_at(
    record: &serde_json::Value,
    observed_at: &str,
) -> Option<LanNeighborObservation> {
    let ip_address = record_text(record, constants::lan_pairing::JSON_KEY_DST)?;
    if !is_supported_neighbor_ip(&ip_address) {
        return None;
    }
    let mac_address = normalize_mac_address(&record_text(
        record,
        constants::lan_pairing::JSON_KEY_LLADDR,
    )?)?;
    let reachability =
        reachability_from_linux_state(record.get(constants::lan_pairing::JSON_KEY_LOWER_STATE))?;

    Some(LanNeighborObservation {
        ip_address,
        mac_address,
        network_interface: normalized_optional_interface_name(record_text(
            record,
            constants::lan_pairing::JSON_KEY_DEV,
        )),
        hostname: None,
        observed_at: observed_at.to_string(),
        reachability,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()],
    })
}

pub fn linux_proc_net_arp_observations() -> Vec<LanNeighborObservation> {
    linux_proc_net_arp_observations_with_observed_at(&Utc::now().to_rfc3339())
}

pub fn linux_proc_net_arp_observations_with_observed_at(
    observed_at: &str,
) -> Vec<LanNeighborObservation> {
    fs::read_to_string(constants::lan_pairing::LINUX_PROC_NET_ARP_PATH)
        .ok()
        .map(|output| {
            output
                .lines()
                .skip(1)
                .filter_map(|line| {
                    linux_proc_net_arp_observation_with_observed_at(line, observed_at)
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn linux_proc_net_arp_observation(line: &str) -> Option<LanNeighborObservation> {
    linux_proc_net_arp_observation_with_observed_at(line, &Utc::now().to_rfc3339())
}

pub fn linux_proc_net_arp_observation_with_observed_at(
    line: &str,
    observed_at: &str,
) -> Option<LanNeighborObservation> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.len() < 6 {
        return None;
    }
    let ip_address = columns[0].trim().to_string();
    if !is_supported_neighbor_ip(&ip_address) {
        return None;
    }
    let flags = parse_proc_net_arp_flags(columns[2])?;
    if flags & 0x2 == 0 {
        return None;
    }
    let mac_address = normalize_mac_address(columns[3])?;

    Some(LanNeighborObservation {
        ip_address,
        mac_address,
        network_interface: normalized_optional_interface_name(Some(columns[5].to_string())),
        hostname: None,
        observed_at: observed_at.to_string(),
        reachability: LanPairingDeviceReachability::Stale,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string()],
    })
}

pub fn parse_proc_net_arp_flags(value: &str) -> Option<u32> {
    let normalized = value
        .trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X");
    u32::from_str_radix(normalized, 16).ok()
}

pub fn reachability_from_linux_state(
    state: Option<&serde_json::Value>,
) -> Option<LanPairingDeviceReachability> {
    state::reachability_from_linux_state(state)
}

pub fn linux_state_labels(state: Option<&serde_json::Value>) -> Vec<String> {
    state::linux_state_labels(state)
}

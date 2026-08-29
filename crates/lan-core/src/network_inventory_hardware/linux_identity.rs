use std::{fs::read_to_string, net::Ipv4Addr};

use ocentra_parent_agent_protocol::constants;

use super::interface_map::{
    linux_local_network_interface_candidate, merge_interface_candidate,
    windows_local_network_interface_candidate,
};
use super::network_identity_support::{
    push_unique_string_if, supported_dns_server_text, supported_local_ipv4_text,
};
use super::{LocalNetworkInterfaceMap, LocalNetworkIdentity};
use crate::network_inventory_command::record_text;

mod address;

pub fn preferred_windows_local_network_identity(
    records: &[serde_json::Value],
) -> Option<LocalNetworkIdentity> {
    windows_local_network_interface_map(records).selected_identity(None)
}

pub fn windows_local_network_interface_map(
    records: &[serde_json::Value],
) -> LocalNetworkInterfaceMap {
    let mut interfaces = Vec::new();
    for record in records {
        let Some(candidate) = windows_local_network_interface_candidate(record) else {
            continue;
        };
        merge_interface_candidate(&mut interfaces, candidate);
    }
    let eligible = interfaces
        .iter()
        .filter(|interface| interface.is_eligible_by_default())
        .collect::<Vec<_>>();
    let route_candidates = eligible
        .iter()
        .filter(|interface| interface.has_default_route)
        .collect::<Vec<_>>();
    let recommended_interface_id = if route_candidates.len() == 1 {
        Some(route_candidates[0].id.clone())
    } else if route_candidates.is_empty() && eligible.len() == 1 {
        Some(eligible[0].id.clone())
    } else {
        None
    };
    LocalNetworkInterfaceMap::new(interfaces, recommended_interface_id)
}

pub fn preferred_linux_local_network_identity(
    route_records: &[serde_json::Value],
    address_records: &[serde_json::Value],
    dns_servers: &[String],
) -> Option<LocalNetworkIdentity> {
    linux_local_network_interface_map(route_records, address_records, dns_servers)
        .selected_identity(None)
}

pub fn linux_local_network_interface_map(
    route_records: &[serde_json::Value],
    address_records: &[serde_json::Value],
    dns_servers: &[String],
) -> LocalNetworkInterfaceMap {
    let default_route = linux_default_route(route_records);
    let route = default_route.as_ref().ok().and_then(Option::as_ref);
    let mut interfaces = Vec::new();
    for record in address_records {
        let Some(candidate) = linux_local_network_interface_candidate(record, route, dns_servers)
        else {
            continue;
        };
        merge_interface_candidate(&mut interfaces, candidate);
    }
    let recommended_interface_id = match default_route {
        Err(()) => None,
        Ok(Some(default_route)) => interfaces
            .iter()
            .find(|interface| {
                interface.has_default_route
                    && interface.is_eligible_by_default()
                    && interface.name.eq_ignore_ascii_case(&default_route.device)
            })
            .map(|interface| interface.id.clone()),
        Ok(None) => interfaces
            .iter()
            .find(|interface| interface.is_eligible_by_default())
            .map(|interface| interface.id.clone()),
    };
    LocalNetworkInterfaceMap::new(interfaces, recommended_interface_id)
}

pub(super) fn linux_ipv4_address(record: &serde_json::Value) -> Option<(String, u8)> {
    address::linux_ipv4_address(record)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct LinuxDefaultRoute {
    pub(super) device: String,
    pub(super) gateway: Option<String>,
}

fn linux_default_route(
    route_records: &[serde_json::Value],
) -> Result<Option<LinuxDefaultRoute>, ()> {
    let mut routes = route_records.iter().filter_map(|record| {
        if record_text(record, constants::lan_pairing::JSON_KEY_DST).as_deref() != Some("default") {
            return None;
        }
        let device = record_text(record, constants::lan_pairing::JSON_KEY_DEV)?;
        let gateway = record_text(record, constants::lan_pairing::JSON_KEY_GATEWAY)
            .filter(|value| supported_local_ipv4_text(value));
        Some(LinuxDefaultRoute { device, gateway })
    });
    let route = routes.next();
    if route.is_some() && routes.next().is_some() {
        return Err(());
    }
    Ok(route)
}

pub(super) fn linux_dns_servers_from_resolv_conf() -> Vec<String> {
    read_to_string(constants::lan_pairing::LINUX_RESOLV_CONF_PATH)
        .map(|text| linux_dns_servers_from_resolv_conf_text(&text))
        .unwrap_or_default()
}

pub fn linux_dns_servers_from_resolv_conf_text(text: &str) -> Vec<String> {
    let mut dns_servers = Vec::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or_default().trim();
        let mut parts = line.split_whitespace();
        if parts.next() != Some("nameserver") {
            continue;
        }
        if let Some(server) = parts.next() {
            push_unique_string_if(&mut dns_servers, server, supported_dns_server_text(server));
        }
    }
    dns_servers
}

pub(super) fn linux_ipv6_prefixes(record: &serde_json::Value) -> Vec<String> {
    address::linux_ipv6_prefixes(record)
}

pub(super) fn cidr_summary(ip_address: &str, prefix_length: Option<u8>) -> Option<String> {
    prefix_length
        .filter(|prefix_length| *prefix_length <= 32)
        .map(|prefix_length| format!("{ip_address}/{prefix_length}"))
}

pub(super) fn broadcast_address_for(ip_address: &str, prefix_length: Option<u8>) -> Option<String> {
    let prefix_length = prefix_length?;
    if prefix_length > 32 {
        return None;
    }
    let ip_address = ip_address.parse::<Ipv4Addr>().ok()?;
    let mask = if prefix_length == 0 {
        0
    } else {
        u32::MAX << (32_u32.saturating_sub(u32::from(prefix_length)))
    };
    let broadcast = u32::from(ip_address) | !mask;
    Some(Ipv4Addr::from(broadcast).to_string())
}

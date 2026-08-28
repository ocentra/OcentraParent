use std::{fs::read_to_string, net::Ipv4Addr};

use ocentra_parent_agent_protocol::constants;

use super::network_identity_support::{
    default_gateway_preference, ignored_interface_name, normalized_ipv6_prefixes,
    push_unique_string_if, record_text_values, sanitized_dns_servers, supported_dns_server_text,
    supported_local_ipv4_text,
};
use super::LocalNetworkIdentity;
use crate::network_inventory_command::{normalize_mac_address, record_text, record_u64};

mod address;

pub fn preferred_windows_local_network_identity(
    records: &[serde_json::Value],
) -> Option<LocalNetworkIdentity> {
    let candidates = records
        .iter()
        .filter_map(windows_local_network_identity_candidate)
        .collect::<Vec<_>>();
    let best_preference = candidates
        .iter()
        .map(|identity| default_gateway_preference(identity.default_gateway.as_deref()))
        .min()?;
    let mut best_candidates = candidates.into_iter().filter(|identity| {
        default_gateway_preference(identity.default_gateway.as_deref()) == best_preference
    });
    let identity = best_candidates.next()?;
    best_candidates.next().is_none().then_some(identity)
}

fn windows_local_network_identity_candidate(
    record: &serde_json::Value,
) -> Option<LocalNetworkIdentity> {
    let interface_name = record_text(record, constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS)?;
    if ignored_interface_name(&interface_name) {
        return None;
    }
    let is_wireless_interface = is_wireless_interface_alias(&interface_name);
    let ip_address = record_text(record, constants::lan_pairing::JSON_KEY_IP_ADDRESS)?;
    if !supported_local_ipv4_text(&ip_address) {
        return None;
    }
    let prefix_length = match record_u64(record, constants::lan_pairing::JSON_KEY_PREFIX_LENGTH) {
        Some(value) if value <= 32 => Some(value as u8),
        Some(_) => return None,
        None => None,
    };
    let dns_servers = sanitized_dns_servers(record_text_values(
        record,
        constants::lan_pairing::JSON_KEY_DNS_SERVERS,
    ));
    Some(LocalNetworkIdentity {
        ip_address: Some(ip_address.clone()),
        mac_address: record_text(record, constants::lan_pairing::JSON_KEY_MAC_ADDRESS)
            .and_then(|value| normalize_mac_address(&value)),
        network_interface: Some(interface_name),
        wifi_ssid: record_text(record, constants::lan_pairing::JSON_KEY_WIFI_SSID)
            .filter(|_| is_wireless_interface)
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        default_gateway: record_text(record, constants::lan_pairing::JSON_KEY_DEFAULT_GATEWAY)
            .filter(|value| supported_local_ipv4_text(value)),
        ipv4_cidr: cidr_summary(&ip_address, prefix_length),
        dns_servers,
        dhcp_server: record_text(record, constants::lan_pairing::JSON_KEY_DHCP_SERVER)
            .filter(|value| supported_dns_server_text(value)),
        broadcast_address: broadcast_address_for(&ip_address, prefix_length),
        ipv6_prefixes: normalized_ipv6_prefixes(record_text_values(
            record,
            constants::lan_pairing::JSON_KEY_IPV6_PREFIXES,
        )),
    })
}

fn is_wireless_interface_alias(interface_name: &str) -> bool {
    let normalized = interface_name.to_ascii_lowercase();
    normalized.contains("wi-fi")
        || normalized.contains("wifi")
        || normalized.contains("wlan")
        || normalized.contains("wireless")
}

pub fn preferred_linux_local_network_identity(
    route_records: &[serde_json::Value],
    address_records: &[serde_json::Value],
    dns_servers: &[String],
) -> Option<LocalNetworkIdentity> {
    let default_route = linux_default_route(route_records).ok()?;
    if let Some(default_route) = default_route.as_ref() {
        return address_records.iter().find_map(|record| {
            linux_local_network_identity_candidate(record, Some(default_route), dns_servers)
        });
    }
    address_records
        .iter()
        .find_map(|record| linux_local_network_identity_candidate(record, None, dns_servers))
}

fn linux_local_network_identity_candidate(
    record: &serde_json::Value,
    default_route: Option<&LinuxDefaultRoute>,
    dns_servers: &[String],
) -> Option<LocalNetworkIdentity> {
    let interface_name = record_text(record, constants::lan_pairing::JSON_KEY_IFNAME)?;
    if ignored_interface_name(&interface_name) {
        return None;
    }
    let (ip_address, prefix_length) = linux_ipv4_address(record)?;
    let route_matches_interface = default_route
        .map(|route| route.device == interface_name)
        .unwrap_or(true);
    if !route_matches_interface {
        return None;
    }
    Some(LocalNetworkIdentity {
        ip_address: Some(ip_address.clone()),
        mac_address: record_text(record, constants::lan_pairing::JSON_KEY_ADDRESS)
            .and_then(|value| normalize_mac_address(&value)),
        network_interface: Some(interface_name),
        wifi_ssid: None,
        default_gateway: default_route.and_then(|route| route.gateway.clone()),
        ipv4_cidr: cidr_summary(&ip_address, Some(prefix_length)),
        dns_servers: dns_servers.to_vec(),
        dhcp_server: None,
        broadcast_address: broadcast_address_for(&ip_address, Some(prefix_length)),
        ipv6_prefixes: linux_ipv6_prefixes(record),
    })
}

pub(super) fn linux_ipv4_address(record: &serde_json::Value) -> Option<(String, u8)> {
    address::linux_ipv4_address(record)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LinuxDefaultRoute {
    device: String,
    gateway: Option<String>,
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

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::network_inventory_command::value_text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalNetworkInterfaceIgnoreReason {
    EmptyName,
    Loopback,
    VirtualEthernet,
    ContainerBridge,
    VirtualMachineBridge,
    VpnOrTunnel,
    ZeroTier,
    Wsl,
}

pub(super) fn supported_local_ipv4_text(value: &str) -> bool {
    value
        .parse::<Ipv4Addr>()
        .map(supported_local_ipv4)
        .unwrap_or(false)
}

fn supported_local_ipv6_text(value: &str) -> bool {
    value
        .parse::<Ipv6Addr>()
        .map(supported_local_ipv6)
        .unwrap_or(false)
}

fn supported_local_ipv4(ip_address: Ipv4Addr) -> bool {
    !ip_address.is_loopback()
        && !ip_address.is_multicast()
        && !ip_address.is_unspecified()
        && !ip_address.is_link_local()
        && ip_address != Ipv4Addr::BROADCAST
}

fn supported_local_ipv6(ip_address: Ipv6Addr) -> bool {
    !ip_address.is_loopback()
        && !ip_address.is_multicast()
        && !ip_address.is_unspecified()
        && !ip_address.is_unicast_link_local()
}

pub(super) fn supported_dns_server_text(value: &str) -> bool {
    value
        .parse::<IpAddr>()
        .map(|ip_address| !ip_address.is_loopback() && !ip_address.is_unspecified())
        .unwrap_or(false)
}

pub(super) fn sanitized_dns_servers(values: Vec<String>) -> Vec<String> {
    let mut dns_servers = Vec::new();
    for value in values {
        push_unique_string_if(&mut dns_servers, &value, supported_dns_server_text(&value));
    }
    dns_servers
}

pub(super) fn record_text_values(record: &serde_json::Value, field_name: &str) -> Vec<String> {
    record
        .get(field_name)
        .map(value_text_values)
        .unwrap_or_default()
}

fn value_text_values(value: &serde_json::Value) -> Vec<String> {
    match value {
        serde_json::Value::Array(values) => {
            let mut texts = Vec::new();
            for value in values {
                if let Some(text) = value_text(value) {
                    push_unique_string(&mut texts, text);
                }
            }
            texts
        }
        _ => value_text(value)
            .map(|value| vec![value])
            .unwrap_or_default(),
    }
}

pub(super) fn normalized_ipv6_prefixes(values: Vec<String>) -> Vec<String> {
    let mut prefixes = Vec::new();
    for value in values {
        if let Some(prefix) = normalized_ipv6_prefix(&value) {
            push_unique_string(&mut prefixes, prefix);
        }
    }
    prefixes
}

pub(super) fn normalized_ipv6_prefix(value: &str) -> Option<String> {
    let (address, prefix_length) = value.trim().split_once('/')?;
    let prefix_length = prefix_length.parse::<u8>().ok()?;
    if prefix_length > 128 || !supported_local_ipv6_text(address) {
        return None;
    }
    Some(format!("{address}/{prefix_length}"))
}

pub(super) fn push_unique_string(values: &mut Vec<String>, value: String) {
    if !values
        .iter()
        .any(|existing| existing.eq_ignore_ascii_case(&value))
    {
        values.push(value);
    }
}

pub(super) fn push_unique_string_if(values: &mut Vec<String>, value: &str, include: bool) {
    if include {
        push_unique_string(values, value.to_string());
    }
}

pub(super) fn ignored_interface_name(interface_name: &str) -> bool {
    ignored_interface_reason(interface_name).is_some()
}

pub fn ignored_interface_reason(interface_name: &str) -> Option<LocalNetworkInterfaceIgnoreReason> {
    let normalized = interface_name.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Some(LocalNetworkInterfaceIgnoreReason::EmptyName);
    }
    if normalized == "lo" || normalized.contains("loopback") {
        return Some(LocalNetworkInterfaceIgnoreReason::Loopback);
    }
    if normalized.starts_with("vethernet") {
        return Some(LocalNetworkInterfaceIgnoreReason::VirtualEthernet);
    }
    if normalized.starts_with("docker")
        || normalized.starts_with("veth")
        || normalized.starts_with("br-")
    {
        return Some(LocalNetworkInterfaceIgnoreReason::ContainerBridge);
    }
    if normalized.starts_with("virbr") || normalized.starts_with("vboxnet") {
        return Some(LocalNetworkInterfaceIgnoreReason::VirtualMachineBridge);
    }
    if normalized.starts_with("tailscale")
        || normalized.starts_with("wg")
        || normalized.starts_with("tun")
        || normalized.starts_with("tap")
    {
        return Some(LocalNetworkInterfaceIgnoreReason::VpnOrTunnel);
    }
    if normalized.starts_with("zt") {
        return Some(LocalNetworkInterfaceIgnoreReason::ZeroTier);
    }
    if normalized.contains("wsl") {
        return Some(LocalNetworkInterfaceIgnoreReason::Wsl);
    }
    None
}

pub(super) fn default_gateway_preference(default_gateway: Option<&str>) -> u8 {
    if default_gateway.is_some() {
        0
    } else {
        1
    }
}

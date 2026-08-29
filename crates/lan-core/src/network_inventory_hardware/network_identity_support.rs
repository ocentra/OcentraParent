mod address;
mod prefix;
mod record_values;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalNetworkInterfaceIgnoreReason {
    EmptyName,
    Loopback,
    VirtualEthernet,
    ContainerBridge,
    VirtualMachineBridge,
    VpnOrTunnel,
    ZeroTier,
    Wsl,
    Down,
    Disconnected,
    LinkLocalOnly,
}

pub(super) fn supported_local_ipv4_text(value: &str) -> bool {
    address::supported_local_ipv4_text(value)
}

pub(super) fn supported_dns_server_text(value: &str) -> bool {
    address::supported_dns_server_text(value)
}

pub(super) fn supported_local_ipv6_text(value: &str) -> bool {
    address::supported_local_ipv6_text(value)
}

pub(super) fn sanitized_dns_servers(values: Vec<String>) -> Vec<String> {
    let mut dns_servers = Vec::new();
    for value in values {
        push_unique_string_if(&mut dns_servers, &value, supported_dns_server_text(&value));
    }
    dns_servers
}

pub(super) fn record_text_values(record: &serde_json::Value, field_name: &str) -> Vec<String> {
    record_values::record_text_values(record, field_name)
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
    prefix::normalized_ipv6_prefix(value)
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
    interface_ignore_reason(&normalized)
}

pub fn stable_interface_id(
    interface_name: &str,
    interface_index: Option<u32>,
    mac_address: Option<&str>,
) -> Option<String> {
    mac_address
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("mac:{}", value.to_ascii_lowercase()))
        .or_else(|| {
            interface_index
                .filter(|value| *value > 0)
                .map(|value| format!("index:{value}"))
        })
        .or_else(|| {
            let normalized = interface_name.trim().to_ascii_lowercase();
            (!normalized.is_empty()).then(|| format!("name:{normalized}"))
        })
}

pub(super) fn default_gateway_preference(default_gateway: Option<&str>) -> u8 {
    u8::from(default_gateway.is_none())
}

fn interface_ignore_reason(normalized: &str) -> Option<LocalNetworkInterfaceIgnoreReason> {
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
    normalized
        .contains("wsl")
        .then_some(LocalNetworkInterfaceIgnoreReason::Wsl)
}

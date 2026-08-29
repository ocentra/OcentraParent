mod address;
mod classification;
mod record;
mod state;

use std::net::{IpAddr, Ipv4Addr};

use ocentra_parent_agent_protocol::constants;
use serde_json::Value;

use super::super::network_identity_support::{normalized_ipv6_prefix, push_unique_string};
use super::super::{LocalNetworkInterface, LocalNetworkInterfaceClassification};

pub(super) type ParsedInterfaceAddress = (String, IpAddr, Option<u8>);

pub(super) const INTERFACE_NAME_KEYS: &[&str] = &[
    constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS,
    constants::lan_pairing::JSON_KEY_IFNAME,
    "Name",
    "name",
];
pub(super) const INTERFACE_INDEX_KEYS: &[&str] = &["InterfaceIndex", "ifindex", "Index", "index"];
pub(super) const MAC_ADDRESS_KEYS: &[&str] = &[
    constants::lan_pairing::JSON_KEY_MAC_ADDRESS,
    constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS,
    constants::lan_pairing::JSON_KEY_ADDRESS,
    "mac",
    "Mac",
];
pub(super) const DESCRIPTION_KEYS: &[&str] = &[
    "InterfaceDescription",
    "Description",
    "description",
    "ifalias",
    "Alias",
    "alias",
];
pub(super) const DEFAULT_GATEWAY_KEYS: &[&str] = &[
    constants::lan_pairing::JSON_KEY_DEFAULT_GATEWAY,
    "IPv4DefaultGateway",
    constants::lan_pairing::JSON_KEY_GATEWAY,
    "Gateway",
    "gateway",
];
pub(super) const DNS_SERVER_KEYS: &[&str] = &[
    constants::lan_pairing::JSON_KEY_DNS_SERVERS,
    "DNSServerSearchOrder",
    "DnsServer",
    "dnsServers",
    "dns_servers",
    "dns",
];
pub(super) const DHCP_SERVER_KEYS: &[&str] = &[
    constants::lan_pairing::JSON_KEY_DHCP_SERVER,
    "DHCPServer",
    "dhcpServer",
    "dhcp_server",
    "dhcp",
];
pub(super) const BROADCAST_ADDRESS_KEYS: &[&str] = &[
    "BroadcastAddress",
    "broadcastAddress",
    "broadcast_address",
    "broadcast",
];
pub(super) const WIFI_SSID_KEYS: &[&str] = &[
    constants::lan_pairing::JSON_KEY_WIFI_SSID,
    "WifiSSID",
    "SSID",
    "Ssid",
    "ssid",
];
pub(super) const WIFI_SIGNAL_KEYS: &[&str] = &[
    "WifiSignalPercent",
    "wifiSignalPercent",
    "SignalPercent",
    "signalPercent",
    "SignalStrength",
    "signalStrength",
    "LinkQuality",
    "linkQuality",
];
const STATE_KEYS: &[&str] = &[
    "Status",
    "status",
    "OperStatus",
    "operstate",
    "OperState",
    "ConnectionState",
    "connectionState",
    "ConnectionStatus",
    "LinkState",
];
const BOOLEAN_UP_KEYS: &[&str] = &["IsUp", "isUp", "is_up", "Up", "up"];
const BOOLEAN_CONNECTED_KEYS: &[&str] = &[
    "IsConnected",
    "isConnected",
    "is_connected",
    "Connected",
    "connected",
];
const LOOPBACK_KEYS: &[&str] = &[
    "IsLoopback",
    "isLoopback",
    "is_loopback",
    "Loopback",
    "loopback",
];
const MEDIA_CONNECT_STATE_KEYS: &[&str] = &["MediaConnectState", "mediaConnectState"];
const NET_CONNECTION_STATUS_KEYS: &[&str] = &["NetConnectionStatus", "netConnectionStatus"];
const ADDRESS_KEYS: &[&str] = &[
    "IPAddresses",
    "IpAddresses",
    constants::lan_pairing::JSON_KEY_IP_ADDRESS,
    "IPv4Addresses",
    "IPv6Addresses",
    "IPv4Address",
    "IPv6Address",
    "Addresses",
    "LocalAddresses",
    "Ipv6Prefixes",
    "IPv6Prefixes",
    "local",
];


pub(super) fn record_value<'a>(record: &'a Value, keys: &[&str]) -> Option<&'a Value> {
    record::record_value(record, keys)
}

pub(super) fn record_text_values_any(record: &Value, keys: &[&str]) -> Vec<String> {
    record::record_text_values_any(record, keys)
}

pub(super) fn record_text_any(record: &Value, keys: &[&str]) -> Option<String> {
    record::record_text_any(record, keys)
}

pub(super) fn record_u64_any(record: &Value, keys: &[&str]) -> Option<u64> {
    record::record_u64_any(record, keys)
}

pub(super) fn record_bool_any(record: &Value, keys: &[&str]) -> Option<bool> {
    record::record_bool_any(record, keys)
}

pub(super) fn record_percent(record: &Value, keys: &[&str]) -> Option<u8> {
    record::record_percent(record, keys)
}

pub(super) fn interface_state(record: &Value) -> (bool, bool, bool) {
    state::interface_state(record)
}

pub(super) fn interface_is_loopback(record: &Value, interface_name: &str) -> bool {
    state::interface_is_loopback(record, interface_name)
}

pub(super) fn interface_addresses(
    record: &Value,
    default_prefix_length: Option<u8>,
) -> Vec<ParsedInterfaceAddress> {
    address::interface_addresses(record, default_prefix_length)
}

pub(super) fn interface_classification_hint(
    record: &Value,
    interface_name: &str,
) -> LocalNetworkInterfaceClassification {
    classification::interface_classification_hint(record, interface_name)
}

pub(super) fn is_wireless_interface_name(record: &Value) -> bool {
    classification::is_wireless_interface_name(record)
}

pub(super) fn build_interface(
    id: String,
    name: String,
    description: Option<String>,
    index: Option<u32>,
    mac_address: Option<String>,
    addresses: Vec<ParsedInterfaceAddress>,
    default_gateway: Option<String>,
    dns_servers: Vec<String>,
    dhcp_server: Option<String>,
    broadcast_address: Option<String>,
    wifi_ssid: Option<String>,
    wifi_signal_percent: Option<u8>,
    is_up: bool,
    is_connected: bool,
    state_observed: bool,
    is_loopback: bool,
    has_default_route: bool,
    classification: LocalNetworkInterfaceClassification,
    fallback_prefix_length: Option<u8>,
) -> LocalNetworkInterface {
    let mut ip_addresses = Vec::new();
    let mut ipv6_prefixes = Vec::new();
    let mut ipv4_cidr = None;
    for (text, parsed, prefix_length) in addresses {
        push_unique_string(&mut ip_addresses, text.clone());
        match parsed {
            IpAddr::V4(_) => {
                if ipv4_cidr.is_none() {
                    ipv4_cidr = super::super::linux_identity::cidr_summary(&text, prefix_length);
                }
            }
            IpAddr::V6(_) => {
                if let Some(prefix_length) = prefix_length {
                    if let Some(prefix) = normalized_ipv6_prefix(&format!("{text}/{prefix_length}"))
                    {
                        push_unique_string(&mut ipv6_prefixes, prefix);
                    }
                }
            }
        }
    }
    if ipv4_cidr.is_none() {
        if let Some(prefix_length) = fallback_prefix_length {
            ipv4_cidr = ip_addresses
                .iter()
                .find(|value| value.parse::<Ipv4Addr>().is_ok())
                .and_then(|value| {
                    super::super::linux_identity::cidr_summary(value, Some(prefix_length))
                });
        }
    }
    let mut interface = LocalNetworkInterface {
        id,
        name,
        description,
        index,
        mac_address,
        ip_addresses,
        default_gateway,
        dns_servers,
        dhcp_server,
        broadcast_address,
        ipv4_cidr,
        ipv6_prefixes,
        is_up,
        is_connected,
        state_observed,
        is_loopback,
        classification,
        ignored_reason: None,
        is_link_local_only: false,
        wifi_ssid: wifi_ssid
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        wifi_signal_percent,
        has_default_route,
    };
    if interface.broadcast_address.is_none() {
        let prefix_length = interface
            .ipv4_cidr
            .as_deref()
            .and_then(|cidr| cidr.rsplit_once('/'))
            .and_then(|(_, prefix)| prefix.parse::<u8>().ok());
        interface.broadcast_address = interface
            .ip_addresses
            .iter()
            .find(|value| value.parse::<Ipv4Addr>().is_ok())
            .and_then(|value| {
                super::super::linux_identity::broadcast_address_for(value, prefix_length)
            });
    }
    interface.refresh_derived_state();
    interface
}

pub(super) fn has_invalid_ipv4_prefix(record: &Value) -> bool {
    address::has_invalid_ipv4_prefix(record)
}

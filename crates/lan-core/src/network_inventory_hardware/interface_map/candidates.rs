use ocentra_parent_agent_protocol::constants;
use serde_json::Value;

use super::super::linux_identity::LinuxDefaultRoute;
use super::super::network_identity_support::{
    sanitized_dns_servers, stable_interface_id, supported_dns_server_text,
    supported_local_ipv4_text,
};
use super::super::LocalNetworkInterface;
use super::parsing::{
    build_interface, has_invalid_ipv4_prefix, interface_addresses, interface_classification_hint,
    interface_is_loopback, interface_state, is_wireless_interface_name, record_percent,
    record_text_any, record_text_values_any, record_u64_any, record_value, InterfaceBuildInput,
    BROADCAST_ADDRESS_KEYS, DEFAULT_GATEWAY_KEYS, DESCRIPTION_KEYS, DHCP_SERVER_KEYS,
    DNS_SERVER_KEYS, INTERFACE_INDEX_KEYS, INTERFACE_NAME_KEYS, MAC_ADDRESS_KEYS, WIFI_SIGNAL_KEYS,
    WIFI_SSID_KEYS,
};
use crate::network_inventory_command::normalize_mac_address;

pub(super) fn windows_local_network_interface_candidate(
    record: &Value,
) -> Option<LocalNetworkInterface> {
    let interface_name = record_text_any(record, INTERFACE_NAME_KEYS).unwrap_or_default();
    let interface_index =
        record_u64_any(record, INTERFACE_INDEX_KEYS).and_then(|value| u32::try_from(value).ok());
    let mac_address =
        record_text_any(record, MAC_ADDRESS_KEYS).and_then(|value| normalize_mac_address(&value));
    let id = stable_interface_id(&interface_name, interface_index, mac_address.as_deref())?;
    let prefix_length = match record_value(
        record,
        &[
            constants::lan_pairing::JSON_KEY_PREFIX_LENGTH,
            "prefixLength",
        ],
    ) {
        None => None,
        Some(value) => {
            let value = crate::network_inventory_command::value_text(value)
                .and_then(|value| value.trim().parse::<u64>().ok())?;
            if value > 32 {
                return None;
            }
            Some(value as u8)
        }
    };
    let addresses = interface_addresses(record, prefix_length);
    let default_gateway = record_text_any(record, DEFAULT_GATEWAY_KEYS)
        .filter(|value| supported_local_ipv4_text(value));
    let (is_up, is_connected, state_observed) = interface_state(record);
    Some(build_interface(InterfaceBuildInput {
        id,
        name: interface_name.clone(),
        description: record_text_any(record, DESCRIPTION_KEYS),
        index: interface_index,
        mac_address,
        addresses,
        default_gateway: default_gateway.clone(),
        dns_servers: sanitized_dns_servers(record_text_values_any(record, DNS_SERVER_KEYS)),
        dhcp_server: record_text_any(record, DHCP_SERVER_KEYS)
            .filter(|value| supported_dns_server_text(value)),
        broadcast_address: record_text_any(record, BROADCAST_ADDRESS_KEYS)
            .filter(|value| supported_local_ipv4_text(value)),
        wifi_ssid: record_text_any(record, WIFI_SSID_KEYS)
            .filter(|_| is_wireless_interface_name(record)),
        wifi_signal_percent: record_percent(record, WIFI_SIGNAL_KEYS),
        is_up,
        is_connected,
        state_observed,
        is_loopback: interface_is_loopback(record, &interface_name),
        has_default_route: default_gateway.is_some(),
        classification: interface_classification_hint(record, &interface_name),
        fallback_prefix_length: prefix_length,
    }))
}

pub(super) fn linux_local_network_interface_candidate(
    record: &Value,
    default_route: Option<&LinuxDefaultRoute>,
    dns_servers: &[String],
) -> Option<LocalNetworkInterface> {
    let interface_name = record_text_any(record, INTERFACE_NAME_KEYS).unwrap_or_default();
    let interface_index =
        record_u64_any(record, INTERFACE_INDEX_KEYS).and_then(|value| u32::try_from(value).ok());
    let mac_address =
        record_text_any(record, MAC_ADDRESS_KEYS).and_then(|value| normalize_mac_address(&value));
    let id = stable_interface_id(&interface_name, interface_index, mac_address.as_deref())?;
    if has_invalid_ipv4_prefix(record) {
        return None;
    }
    let addresses = interface_addresses(record, None);
    let (is_up, is_connected, state_observed) = interface_state(record);
    let has_default_route = default_route
        .map(|route| route.device.eq_ignore_ascii_case(&interface_name))
        .unwrap_or(false);
    Some(build_interface(InterfaceBuildInput {
        id,
        name: interface_name.clone(),
        description: record_text_any(record, DESCRIPTION_KEYS),
        index: interface_index,
        mac_address,
        addresses,
        default_gateway: default_route
            .filter(|route| route.device.eq_ignore_ascii_case(&interface_name))
            .and_then(|route| route.gateway.clone()),
        dns_servers: sanitized_dns_servers(dns_servers.to_vec()),
        dhcp_server: record_text_any(record, DHCP_SERVER_KEYS)
            .filter(|value| supported_dns_server_text(value)),
        broadcast_address: record_text_any(record, BROADCAST_ADDRESS_KEYS)
            .filter(|value| supported_local_ipv4_text(value)),
        wifi_ssid: record_text_any(record, WIFI_SSID_KEYS),
        wifi_signal_percent: record_percent(record, WIFI_SIGNAL_KEYS),
        is_up,
        is_connected,
        state_observed,
        is_loopback: interface_is_loopback(record, &interface_name),
        has_default_route,
        classification: interface_classification_hint(record, &interface_name),
        fallback_prefix_length: None,
    }))
}

pub(super) fn merge_interface_candidate(
    interfaces: &mut Vec<LocalNetworkInterface>,
    candidate: LocalNetworkInterface,
) {
    if let Some(existing) = interfaces.iter_mut().find(|interface| {
        interface.id.eq_ignore_ascii_case(&candidate.id)
            || (!interface.name.trim().is_empty()
                && !candidate.name.trim().is_empty()
                && interface.name.eq_ignore_ascii_case(&candidate.name))
    }) {
        existing.merge_from(candidate);
    } else {
        interfaces.push(candidate);
    }
}

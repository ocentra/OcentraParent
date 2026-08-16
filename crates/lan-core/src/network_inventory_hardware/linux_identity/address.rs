use ocentra_parent_agent_protocol::constants;

use super::super::network_identity_support::{
    normalized_ipv6_prefix, push_unique_string, supported_local_ipv4_text,
};
use crate::network_inventory_command::{record_text, record_u64};

pub(super) fn linux_ipv4_address(record: &serde_json::Value) -> Option<(String, u8)> {
    let addr_info = record
        .get(constants::lan_pairing::JSON_KEY_ADDR_INFO)?
        .as_array()?;
    addr_info.iter().find_map(|addr| {
        let family = record_text(addr, constants::lan_pairing::JSON_KEY_FAMILY)?;
        if family != "inet" {
            return None;
        }
        let scope = record_text(addr, constants::lan_pairing::JSON_KEY_SCOPE);
        let local = record_text(addr, constants::lan_pairing::JSON_KEY_LOCAL)?;
        let prefix_length = record_u64(addr, constants::lan_pairing::JSON_KEY_PREFIXLEN)
            .map(|value| value as u8)?;
        if !supported_local_ipv4_text(&local) || scope.as_deref() == Some("host") {
            return None;
        }
        Some((local, prefix_length))
    })
}

pub(super) fn linux_ipv6_prefixes(record: &serde_json::Value) -> Vec<String> {
    let mut ipv6_prefixes = Vec::new();
    let Some(addr_info) = record
        .get(constants::lan_pairing::JSON_KEY_ADDR_INFO)
        .and_then(serde_json::Value::as_array)
    else {
        return ipv6_prefixes;
    };
    for addr in addr_info {
        let family = record_text(addr, constants::lan_pairing::JSON_KEY_FAMILY);
        if family.as_deref() != Some("inet6") {
            continue;
        }
        let scope = record_text(addr, constants::lan_pairing::JSON_KEY_SCOPE);
        let local = record_text(addr, constants::lan_pairing::JSON_KEY_LOCAL);
        let prefix_length = record_u64(addr, constants::lan_pairing::JSON_KEY_PREFIXLEN);
        if scope.as_deref() == Some("host") {
            continue;
        }
        if let (Some(local), Some(prefix_length)) = (local, prefix_length) {
            let prefix = format!("{local}/{prefix_length}");
            if let Some(prefix) = normalized_ipv6_prefix(&prefix) {
                push_unique_string(&mut ipv6_prefixes, prefix);
            }
        }
    }
    ipv6_prefixes
}

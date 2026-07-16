use crate::network_inventory_command::normalize_mac_address;

use super::super::text::compact_summary;

pub(super) fn passive_dhcp_client_mac(
    payload: &[u8],
    hardware_type: u8,
    hardware_address_len: usize,
) -> Option<String> {
    if hardware_type != 1 || hardware_address_len == 0 || hardware_address_len > 16 {
        return None;
    }
    passive_dhcp_mac_bytes(payload.get(28..28 + hardware_address_len)?)
}

pub(super) fn passive_dhcp_client_id(payload: &[u8]) -> Option<String> {
    if payload.is_empty() {
        return None;
    }
    if payload.first().copied() == Some(1) {
        return passive_dhcp_mac_bytes(payload.get(1..)?).map(|mac| format!("ethernet:{mac}"));
    }
    passive_dhcp_ascii_option(payload).or_else(|| Some(passive_dhcp_hex_bytes(payload)))
}

pub(super) fn passive_dhcp_parameter_request_fingerprint(payload: &[u8]) -> Option<String> {
    (!payload.is_empty()).then(|| {
        payload
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    })
}

pub(super) fn passive_dhcp_ascii_option(payload: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(payload).ok()?.trim();
    (!text.is_empty())
        .then(|| compact_summary(text))
        .filter(|compact| !compact.is_empty())
}

pub(super) fn passive_dhcp_mac_bytes(payload: &[u8]) -> Option<String> {
    if payload.len() < 6 {
        return None;
    }
    let mac = payload
        .iter()
        .take(6)
        .map(|value| format!("{value:02x}"))
        .collect::<Vec<_>>()
        .join(":");
    normalize_mac_address(&mac)
}

pub(super) fn passive_dhcp_hex_bytes(payload: &[u8]) -> String {
    payload
        .iter()
        .map(|value| format!("{value:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

use crate::network_inventory_command::normalize_mac_address;

use super::labels::compact_identifier;
use super::text::compact_summary;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PassiveDhcpObservation {
    message_type: Option<String>,
    hostname: Option<String>,
    vendor_class: Option<String>,
    client_id: Option<String>,
    parameter_request_fingerprint: Option<String>,
    client_mac: Option<String>,
}

pub fn passive_dhcp_summary(payload: &[u8]) -> Option<String> {
    let observation = parse_passive_dhcp_observation(payload)?;
    let mut parts = Vec::new();
    if let Some(message_type) = observation.message_type {
        parts.push(format!("type={message_type}"));
    }
    if let Some(client_mac) = observation.client_mac {
        parts.push(format!("client-mac={client_mac}"));
    }
    if let Some(client_id) = observation.client_id {
        parts.push(format!("client-id={client_id}"));
    }
    if let Some(hostname) = observation.hostname {
        parts.push(format!("hostname={hostname}"));
    }
    if let Some(vendor_class) = observation.vendor_class {
        parts.push(format!("vendor-class={vendor_class}"));
    }
    if let Some(parameter_request_fingerprint) = observation.parameter_request_fingerprint {
        parts.push(format!("params={parameter_request_fingerprint}"));
    }
    if parts.is_empty() {
        return None;
    }
    Some(compact_summary(format!(
        "DHCP packet: {}",
        parts.join("; ")
    )))
}

pub fn passive_dhcp_device_id(payload: &[u8]) -> Option<String> {
    let observation = parse_passive_dhcp_observation(payload)?;
    observation.client_mac.or_else(|| {
        observation
            .client_id
            .map(|value| compact_identifier(&value))
            .filter(|value| !value.is_empty())
    })
}

pub fn parse_passive_dhcp_observation(payload: &[u8]) -> Option<PassiveDhcpObservation> {
    if payload.len() < 240 || payload.get(236..240)? != [99, 130, 83, 99] {
        return None;
    }

    let hardware_type = *payload.get(1)?;
    let hardware_address_len = usize::from(*payload.get(2)?);
    let client_mac = passive_dhcp_client_mac(payload, hardware_type, hardware_address_len);
    let mut observation = PassiveDhcpObservation {
        client_mac,
        ..PassiveDhcpObservation::default()
    };
    parse_passive_dhcp_options(payload, &mut observation)?;
    Some(observation)
}

pub fn parse_passive_dhcp_options(
    payload: &[u8],
    observation: &mut PassiveDhcpObservation,
) -> Option<()> {
    let mut cursor = 240_usize;
    while cursor < payload.len() {
        let option_code = *payload.get(cursor)?;
        cursor += 1;
        match option_code {
            0 => continue,
            255 => break,
            _ => {}
        }
        let option_len = usize::from(*payload.get(cursor)?);
        cursor += 1;
        let option_end = cursor.checked_add(option_len)?;
        let option_value = payload.get(cursor..option_end)?;
        cursor = option_end;
        apply_passive_dhcp_option(observation, option_code, option_value);
    }
    Some(())
}

pub fn apply_passive_dhcp_option(
    observation: &mut PassiveDhcpObservation,
    option_code: u8,
    option_value: &[u8],
) {
    match option_code {
        12 => observation.hostname = passive_dhcp_ascii_option(option_value),
        53 => {
            observation.message_type = option_value
                .first()
                .map(|value| dhcp_message_type_label(*value));
        }
        55 => {
            observation.parameter_request_fingerprint =
                passive_dhcp_parameter_request_fingerprint(option_value);
        }
        60 => observation.vendor_class = passive_dhcp_ascii_option(option_value),
        61 => observation.client_id = passive_dhcp_client_id(option_value),
        _ => {}
    }
}

pub fn passive_dhcp_client_mac(
    payload: &[u8],
    hardware_type: u8,
    hardware_address_len: usize,
) -> Option<String> {
    if hardware_type != 1 || hardware_address_len == 0 || hardware_address_len > 16 {
        return None;
    }
    let client_hardware_address = payload.get(28..28 + hardware_address_len)?;
    passive_dhcp_mac_bytes(client_hardware_address)
}

pub fn passive_dhcp_client_id(payload: &[u8]) -> Option<String> {
    if payload.is_empty() {
        return None;
    }
    if payload.first().copied() == Some(1) {
        let mac = passive_dhcp_mac_bytes(payload.get(1..)?)?;
        return Some(format!("ethernet:{mac}"));
    }
    passive_dhcp_ascii_option(payload).or_else(|| Some(passive_dhcp_hex_bytes(payload)))
}

pub fn passive_dhcp_parameter_request_fingerprint(payload: &[u8]) -> Option<String> {
    if payload.is_empty() {
        return None;
    }
    Some(
        payload
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}

pub fn passive_dhcp_ascii_option(payload: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(payload).ok()?.trim();
    if text.is_empty() {
        return None;
    }
    let compact = compact_summary(text);
    (!compact.is_empty()).then_some(compact)
}

pub fn passive_dhcp_mac_bytes(payload: &[u8]) -> Option<String> {
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

pub fn passive_dhcp_hex_bytes(payload: &[u8]) -> String {
    payload
        .iter()
        .map(|value| format!("{value:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

pub fn dhcp_message_type_label(value: u8) -> String {
    let label = match value {
        1 => "discover",
        2 => "offer",
        3 => "request",
        4 => "decline",
        5 => "ack",
        6 => "nak",
        7 => "release",
        8 => "inform",
        _ => return value.to_string(),
    };
    label.to_string()
}

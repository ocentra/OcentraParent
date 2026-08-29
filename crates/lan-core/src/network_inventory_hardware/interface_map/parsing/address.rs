use ocentra_parent_agent_protocol::constants;
use serde_json::Value;
use std::net::IpAddr;

use super::{
    record_text_any, record_u64_any, record_value, ParsedInterfaceAddress, ADDRESS_KEYS,
};

pub(super) fn interface_addresses(
    record: &Value,
    default_prefix_length: Option<u8>,
) -> Vec<ParsedInterfaceAddress> {
    let mut addresses = Vec::new();
    if let Some(addr_info) = record_value(record, &[constants::lan_pairing::JSON_KEY_ADDR_INFO])
        .and_then(Value::as_array)
    {
        for address in addr_info {
            let Some(local) = record_text_any(address, &[constants::lan_pairing::JSON_KEY_LOCAL])
            else {
                continue;
            };
            let prefix_text = record_text_any(address, &[constants::lan_pairing::JSON_KEY_PREFIXLEN]);
            let prefix_length = match prefix_text {
                Some(prefix) => match prefix.trim().parse::<u8>() {
                    Ok(prefix) => Some(prefix),
                    Err(_) => continue,
                },
                None => None,
            };
            let Some(parsed) = parse_interface_address(&local, prefix_length, true) else {
                continue;
            };
            let family = record_text_any(address, &[constants::lan_pairing::JSON_KEY_FAMILY]);
            if address_family_matches(family.as_deref(), parsed.1) {
                push_interface_address(&mut addresses, parsed);
            }
        }
    }
    for key in ADDRESS_KEYS {
        let Some(value) = record_value(record, &[*key]) else {
            continue;
        };
        append_interface_addresses(&mut addresses, value, default_prefix_length);
    }
    addresses
}

fn append_interface_addresses(
    addresses: &mut Vec<ParsedInterfaceAddress>,
    value: &Value,
    default_prefix_length: Option<u8>,
) {
    match value {
        Value::Array(values) => {
            for value in values {
                append_interface_addresses(addresses, value, default_prefix_length);
            }
        }
        Value::Object(values) => {
            let prefix_value = values
                .iter()
                .find(|(key, _)| {
                    [
                        constants::lan_pairing::JSON_KEY_PREFIX_LENGTH,
                        constants::lan_pairing::JSON_KEY_PREFIXLEN,
                        "prefixLength",
                    ]
                    .iter()
                    .any(|candidate| key.eq_ignore_ascii_case(candidate))
                })
                .map(|(_, value)| value);
            let object_prefix_length = prefix_value
                .and_then(crate::network_inventory_command::value_text)
                .and_then(|value| value.trim().parse::<u8>().ok());
            if prefix_value.is_some() && object_prefix_length.is_none() {
                return;
            }
            let mut found_address = false;
            for key in [
                constants::lan_pairing::JSON_KEY_IP_ADDRESS,
                constants::lan_pairing::JSON_KEY_LOCAL,
                "Address",
                "address",
                "IpAddress",
                "IPAddress",
                "local",
            ] {
                if let Some((_, value)) = values
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(key))
                {
                    found_address = true;
                    append_interface_addresses(
                        addresses,
                        value,
                        object_prefix_length.or(default_prefix_length),
                    );
                }
            }
            if !found_address {
                for value in values.values() {
                    append_interface_addresses(addresses, value, default_prefix_length);
                }
            }
        }
        _ => {
            let Some(text) = crate::network_inventory_command::value_text(value) else {
                return;
            };
            let Some(parsed) = parse_interface_address(&text, default_prefix_length, false) else {
                return;
            };
            push_interface_address(addresses, parsed);
        }
    }
}

fn parse_interface_address(
    value: &str,
    default_prefix_length: Option<u8>,
    apply_default_to_ipv6: bool,
) -> Option<ParsedInterfaceAddress> {
    let value = value.trim();
    let (address, explicit_prefix) = match value.split_once('/') {
        Some((address, prefix)) => {
            let prefix = prefix.trim().parse::<u8>().ok()?;
            (address, Some(prefix))
        }
        None => (value, None),
    };
    let address = address.split('%').next().unwrap_or(address);
    let parsed = address.parse::<IpAddr>().ok()?;
    if apply_default_to_ipv6 {
        if let Some(default_prefix_length) = default_prefix_length {
            let maximum = if parsed.is_ipv4() { 32 } else { 128 };
            if default_prefix_length > maximum {
                return None;
            }
        }
    }
    let prefix_length = explicit_prefix
        .or_else(|| {
            default_prefix_length.filter(|_| parsed.is_ipv4() || apply_default_to_ipv6)
        });
    if let Some(prefix_length) = prefix_length {
        let maximum = if parsed.is_ipv4() { 32 } else { 128 };
        if prefix_length > maximum {
            return None;
        }
    }
    Some((
        parsed.to_string(),
        parsed,
        prefix_length,
    ))
}

fn address_family_matches(family: Option<&str>, address: IpAddr) -> bool {
    match family.map(str::trim) {
        None => true,
        Some(family) if family.eq_ignore_ascii_case("inet") => address.is_ipv4(),
        Some(family) if family.eq_ignore_ascii_case("inet6") => address.is_ipv6(),
        Some(_) => false,
    }
}

fn push_interface_address(
    addresses: &mut Vec<ParsedInterfaceAddress>,
    candidate: ParsedInterfaceAddress,
) {
    if let Some(existing) = addresses.iter_mut().find(|existing| existing.0 == candidate.0) {
        if existing.2.is_none() {
            existing.2 = candidate.2;
        }
        return;
    }
    addresses.push(candidate);
}


pub(super) fn has_invalid_ipv4_prefix(record: &Value) -> bool {
    let Some(addr_info) = record_value(record, &[constants::lan_pairing::JSON_KEY_ADDR_INFO])
        .and_then(Value::as_array)
    else {
        return false;
    };
    addr_info.iter().any(|address| {
        record_text_any(address, &[constants::lan_pairing::JSON_KEY_FAMILY])
            .is_some_and(|family| family.eq_ignore_ascii_case("inet"))
            && record_u64_any(address, &[constants::lan_pairing::JSON_KEY_PREFIXLEN])
                .is_some_and(|prefix_length| prefix_length > 32)
    })
}

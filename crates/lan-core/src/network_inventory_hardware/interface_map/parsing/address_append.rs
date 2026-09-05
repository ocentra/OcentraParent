use ocentra_parent_agent_protocol::constants;
use serde_json::Value;

use super::{address_parse, ParsedInterfaceAddress};

pub(super) fn append_interface_addresses(
    addresses: &mut Vec<ParsedInterfaceAddress>,
    value: &Value,
    default_prefix_length: Option<u8>,
) {
    match value {
        Value::Array(values) => append_array_addresses(addresses, values, default_prefix_length),
        Value::Object(values) => append_object_addresses(addresses, values, default_prefix_length),
        _ => append_scalar_address(addresses, value, default_prefix_length),
    }
}

fn append_array_addresses(
    addresses: &mut Vec<ParsedInterfaceAddress>,
    values: &[Value],
    default_prefix_length: Option<u8>,
) {
    for value in values {
        append_interface_addresses(addresses, value, default_prefix_length);
    }
}

fn append_object_addresses(
    addresses: &mut Vec<ParsedInterfaceAddress>,
    values: &serde_json::Map<String, Value>,
    default_prefix_length: Option<u8>,
) {
    let prefix_value = values
        .iter()
        .find(|(key, _)| is_prefix_key(key))
        .map(|(_, value)| value);
    let object_prefix_length = prefix_value
        .and_then(crate::network_inventory_command::value_text)
        .and_then(|value| value.trim().parse::<u8>().ok());
    if prefix_value.is_some() && object_prefix_length.is_none() {
        return;
    }

    let prefix_length = object_prefix_length.or(default_prefix_length);
    let mut found_address = false;
    for key in ADDRESS_VALUE_KEYS {
        if let Some((_, value)) = values
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key))
        {
            found_address = true;
            append_interface_addresses(addresses, value, prefix_length);
        }
    }
    if !found_address {
        append_nested_object_values(addresses, values, default_prefix_length);
    }
}

fn append_nested_object_values(
    addresses: &mut Vec<ParsedInterfaceAddress>,
    values: &serde_json::Map<String, Value>,
    default_prefix_length: Option<u8>,
) {
    for value in values.values() {
        append_interface_addresses(addresses, value, default_prefix_length);
    }
}

fn append_scalar_address(
    addresses: &mut Vec<ParsedInterfaceAddress>,
    value: &Value,
    default_prefix_length: Option<u8>,
) {
    let Some(text) = crate::network_inventory_command::value_text(value) else {
        return;
    };
    let Some(parsed) = address_parse::parse_interface_address(&text, default_prefix_length, false)
    else {
        return;
    };
    super::address_collection::push_interface_address(addresses, parsed);
}

fn is_prefix_key(key: &str) -> bool {
    [
        constants::lan_pairing::JSON_KEY_PREFIX_LENGTH,
        constants::lan_pairing::JSON_KEY_PREFIXLEN,
        "prefixLength",
    ]
    .iter()
    .any(|candidate| key.eq_ignore_ascii_case(candidate))
}

const ADDRESS_VALUE_KEYS: &[&str] = &[
    constants::lan_pairing::JSON_KEY_IP_ADDRESS,
    constants::lan_pairing::JSON_KEY_LOCAL,
    "Address",
    "address",
    "IpAddress",
    "IPAddress",
    "local",
];

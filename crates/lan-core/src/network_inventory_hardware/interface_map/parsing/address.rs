use ocentra_parent_agent_protocol::constants;
use serde_json::Value;

use super::{record_text_any, record_u64_any, record_value, ParsedInterfaceAddress, ADDRESS_KEYS};

#[path = "address_append.rs"]
mod address_append;
#[path = "address_collection.rs"]
mod address_collection;
#[path = "address_parse.rs"]
mod address_parse;

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
            let prefix_text =
                record_text_any(address, &[constants::lan_pairing::JSON_KEY_PREFIXLEN]);
            let Some(prefix_length) = address_parse::parse_optional_prefix(prefix_text) else {
                continue;
            };
            let Some(parsed) = address_parse::parse_interface_address(&local, prefix_length, true)
            else {
                continue;
            };
            let family = record_text_any(address, &[constants::lan_pairing::JSON_KEY_FAMILY]);
            if address_collection::address_family_matches(family.as_deref(), parsed.address) {
                address_collection::push_interface_address(&mut addresses, parsed);
            }
        }
    }
    for key in ADDRESS_KEYS {
        let Some(value) = record_value(record, &[*key]) else {
            continue;
        };
        address_append::append_interface_addresses(&mut addresses, value, default_prefix_length);
    }
    addresses
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

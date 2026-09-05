use std::net::IpAddr;

use super::ParsedInterfaceAddress;

pub(super) fn parse_optional_prefix(value: Option<String>) -> Option<Option<u8>> {
    match value {
        None => Some(None),
        Some(prefix) => prefix.trim().parse::<u8>().ok().map(Some),
    }
}

pub(super) fn parse_interface_address(
    value: &str,
    default_prefix_length: Option<u8>,
    apply_default_to_ipv6: bool,
) -> Option<ParsedInterfaceAddress> {
    let value = value.trim();
    let (address, explicit_prefix) = split_address_prefix(value)?;
    let address = address.split('%').next().unwrap_or(address);
    let parsed = address.parse::<IpAddr>().ok()?;
    if apply_default_to_ipv6 && !prefix_is_valid(parsed, default_prefix_length) {
        return None;
    }
    let prefix_length = explicit_prefix
        .or_else(|| default_prefix_length.filter(|_| parsed.is_ipv4() || apply_default_to_ipv6));
    prefix_is_valid(parsed, prefix_length).then(|| ParsedInterfaceAddress {
        text: parsed.to_string(),
        address: parsed,
        prefix_length,
    })
}

fn split_address_prefix(value: &str) -> Option<(&str, Option<u8>)> {
    match value.split_once('/') {
        Some((address, prefix)) => Some((address, Some(prefix.trim().parse::<u8>().ok()?))),
        None => Some((value, None)),
    }
}

fn prefix_is_valid(address: IpAddr, prefix_length: Option<u8>) -> bool {
    prefix_length
        .map(|prefix| prefix <= if address.is_ipv4() { 32 } else { 128 })
        .unwrap_or(true)
}

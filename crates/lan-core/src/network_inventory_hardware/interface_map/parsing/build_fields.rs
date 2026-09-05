use std::net::Ipv4Addr;

use super::super::super::network_identity_support::{normalized_ipv6_prefix, push_unique_string};
use super::ParsedInterfaceAddress;

pub(super) fn address_fields(
    addresses: Vec<ParsedInterfaceAddress>,
    fallback_prefix_length: Option<u8>,
) -> (Vec<String>, Vec<String>, Option<String>) {
    let mut ip_addresses = Vec::new();
    let mut ipv6_prefixes = Vec::new();
    let mut ipv4_cidr = None;
    for address in addresses {
        push_unique_string(&mut ip_addresses, address.text.clone());
        if address.address.is_ipv4() {
            if ipv4_cidr.is_none() {
                ipv4_cidr = super::super::super::linux_identity::cidr_summary(
                    &address.text,
                    address.prefix_length,
                );
            }
        } else if let Some(prefix_length) = address.prefix_length {
            if let Some(prefix) =
                normalized_ipv6_prefix(&format!("{}/{}", address.text, prefix_length))
            {
                push_unique_string(&mut ipv6_prefixes, prefix);
            }
        }
    }
    if ipv4_cidr.is_none() {
        ipv4_cidr = fallback_ipv4_cidr(&ip_addresses, fallback_prefix_length);
    }
    (ip_addresses, ipv6_prefixes, ipv4_cidr)
}

fn fallback_ipv4_cidr(
    ip_addresses: &[String],
    fallback_prefix_length: Option<u8>,
) -> Option<String> {
    let prefix_length = fallback_prefix_length?;
    ip_addresses
        .iter()
        .find(|value| value.parse::<Ipv4Addr>().is_ok())
        .and_then(|value| {
            super::super::super::linux_identity::cidr_summary(value, Some(prefix_length))
        })
}

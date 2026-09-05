use std::net::IpAddr;

use super::ParsedInterfaceAddress;

pub(super) fn address_family_matches(family: Option<&str>, address: IpAddr) -> bool {
    match family.map(str::trim) {
        None => true,
        Some(family) if family.eq_ignore_ascii_case("inet") => address.is_ipv4(),
        Some(family) if family.eq_ignore_ascii_case("inet6") => address.is_ipv6(),
        Some(_) => false,
    }
}

pub(super) fn push_interface_address(
    addresses: &mut Vec<ParsedInterfaceAddress>,
    candidate: ParsedInterfaceAddress,
) {
    let Some(existing) = addresses
        .iter_mut()
        .find(|existing| existing.text == candidate.text)
    else {
        addresses.push(candidate);
        return;
    };
    if existing.prefix_length.is_none() {
        existing.prefix_length = candidate.prefix_length;
    }
}

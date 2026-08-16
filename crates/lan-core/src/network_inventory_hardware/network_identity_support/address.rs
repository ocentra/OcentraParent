use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub(super) fn supported_local_ipv4_text(value: &str) -> bool {
    value.parse::<Ipv4Addr>().is_ok_and(supported_local_ipv4)
}

pub(super) fn supported_local_ipv6_text(value: &str) -> bool {
    value.parse::<Ipv6Addr>().is_ok_and(supported_local_ipv6)
}

pub(super) fn supported_dns_server_text(value: &str) -> bool {
    value
        .parse::<IpAddr>()
        .is_ok_and(|ip_address| !ip_address.is_loopback() && !ip_address.is_unspecified())
}

fn supported_local_ipv4(ip_address: Ipv4Addr) -> bool {
    !ip_address.is_loopback()
        && !ip_address.is_multicast()
        && !ip_address.is_unspecified()
        && !ip_address.is_link_local()
        && ip_address != Ipv4Addr::BROADCAST
}

fn supported_local_ipv6(ip_address: Ipv6Addr) -> bool {
    !ip_address.is_loopback()
        && !ip_address.is_multicast()
        && !ip_address.is_unspecified()
        && !ip_address.is_unicast_link_local()
}

use std::net::IpAddr;

use super::super::super::LocalNetworkInterface;

pub(super) fn refresh(interface: &mut LocalNetworkInterface) {
    interface.is_link_local_only = all_link_local(interface);
    interface.is_loopback |= all_loopback(interface);
    interface.classification = super::model_classification::classify_interface(
        &interface.name,
        interface.is_loopback,
        interface.is_link_local_only,
        interface.classification,
    );
    interface.ignored_reason = super::model_ignored_reason::interface_ignored_reason(
        &interface.name,
        interface.is_up,
        interface.is_connected,
        interface.is_loopback,
        interface.is_link_local_only,
    );
}

fn all_link_local(interface: &LocalNetworkInterface) -> bool {
    interface
        .ip_addresses
        .iter()
        .any(|address| is_link_local(address))
        && interface
            .ip_addresses
            .iter()
            .all(|address| is_link_local(address))
}

fn all_loopback(interface: &LocalNetworkInterface) -> bool {
    interface
        .ip_addresses
        .iter()
        .any(|address| is_loopback(address))
        && interface
            .ip_addresses
            .iter()
            .all(|address| is_loopback(address))
}

fn is_link_local(address: &str) -> bool {
    address.parse::<IpAddr>().is_ok_and(|parsed| match parsed {
        IpAddr::V4(value) => value.is_link_local(),
        IpAddr::V6(value) => value.is_unicast_link_local(),
    })
}

fn is_loopback(address: &str) -> bool {
    address
        .parse::<IpAddr>()
        .is_ok_and(|parsed| parsed.is_loopback())
}

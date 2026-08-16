use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use super::super::LanNeighborObservation;

pub(super) fn should_replace_primary_observation(
    existing: &LanNeighborObservation,
    incoming: &LanNeighborObservation,
) -> bool {
    let existing_is_private_ipv4 = parse_private_ipv4(&existing.ip_address).is_some();
    let incoming_is_private_ipv4 = parse_private_ipv4(&incoming.ip_address).is_some();
    if !existing_is_private_ipv4 && incoming_is_private_ipv4 {
        return true;
    }
    if existing.ip_address.is_empty() {
        return true;
    }
    if existing
        .ip_address
        .eq_ignore_ascii_case(&incoming.ip_address)
    {
        return false;
    }
    existing_is_private_ipv4
        && incoming_is_private_ipv4
        && reachability_rank(&incoming.reachability) > reachability_rank(&existing.reachability)
}

fn parse_private_ipv4(value: &str) -> Option<std::net::Ipv4Addr> {
    let ip = value.parse::<std::net::Ipv4Addr>().ok()?;
    ip.is_private().then_some(ip)
}

pub(super) fn reachability_rank(reachability: &LanPairingDeviceReachability) -> u8 {
    match reachability {
        LanPairingDeviceReachability::Online => 3,
        LanPairingDeviceReachability::Stale => 2,
        LanPairingDeviceReachability::Offline => 1,
    }
}

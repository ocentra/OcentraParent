use std::net::Ipv4Addr;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use super::super::LanNeighborObservation;
use super::identity::push_unique_scan_source;

pub fn merge_neighbor_observations(
    observations: Vec<LanNeighborObservation>,
) -> Vec<LanNeighborObservation> {
    let mut merged: Vec<LanNeighborObservation> = Vec::new();
    for observation in observations {
        if let Some(existing) = merged.iter_mut().find(|candidate| {
            candidate
                .mac_address
                .eq_ignore_ascii_case(&observation.mac_address)
        }) {
            merge_linux_neighbor_observation(existing, observation);
        } else {
            merged.push(observation);
        }
    }
    merged
}

pub fn merge_linux_neighbor_observation(
    existing: &mut LanNeighborObservation,
    incoming: LanNeighborObservation,
) {
    let replace_primary_identity = should_replace_linux_primary_observation(existing, &incoming);
    if replace_primary_identity {
        existing.ip_address = incoming.ip_address.clone();
        if incoming.network_interface.is_some() {
            existing.network_interface = incoming.network_interface.clone();
        }
        if incoming.hostname.is_some() {
            existing.hostname = incoming.hostname.clone();
        }
    } else {
        if existing.network_interface.is_none() {
            existing.network_interface = incoming.network_interface.clone();
        }
        if existing.hostname.is_none() {
            existing.hostname = incoming.hostname.clone();
        }
    }
    if linux_neighbor_reachability_rank(&incoming.reachability)
        > linux_neighbor_reachability_rank(&existing.reachability)
    {
        existing.reachability = incoming.reachability.clone();
    }
    super::super::neighbor_merge::merge_observed_at(
        &mut existing.observed_at,
        &incoming.observed_at,
    );
    for scan_source in incoming.scan_sources {
        push_unique_scan_source(&mut existing.scan_sources, &scan_source);
    }
}

pub fn should_replace_linux_primary_observation(
    existing: &LanNeighborObservation,
    incoming: &LanNeighborObservation,
) -> bool {
    let existing_is_private_ipv4 = parse_linux_private_ipv4(&existing.ip_address).is_some();
    let incoming_is_private_ipv4 = parse_linux_private_ipv4(&incoming.ip_address).is_some();
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
    if existing_is_private_ipv4 && incoming_is_private_ipv4 {
        let incoming_rank = linux_neighbor_reachability_rank(&incoming.reachability);
        let existing_rank = linux_neighbor_reachability_rank(&existing.reachability);
        return incoming_rank >= existing_rank;
    }
    false
}

pub fn parse_linux_private_ipv4(value: &str) -> Option<Ipv4Addr> {
    let ip = value.parse::<Ipv4Addr>().ok()?;
    ip.is_private().then_some(ip)
}

pub fn linux_neighbor_reachability_rank(reachability: &LanPairingDeviceReachability) -> u8 {
    match reachability {
        LanPairingDeviceReachability::Online => 3,
        LanPairingDeviceReachability::Stale => 2,
        LanPairingDeviceReachability::Offline => 1,
    }
}

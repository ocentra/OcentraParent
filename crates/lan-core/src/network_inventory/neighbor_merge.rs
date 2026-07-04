use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use super::LanNeighborObservation;

pub(super) fn merge_neighbor_observations_by_mac(
    observations: Vec<LanNeighborObservation>,
) -> Vec<LanNeighborObservation> {
    let mut merged: Vec<LanNeighborObservation> = Vec::new();
    for observation in observations {
        if let Some(existing) = merged.iter_mut().find(|candidate| {
            candidate
                .mac_address
                .eq_ignore_ascii_case(&observation.mac_address)
        }) {
            merge_neighbor_observation(existing, observation);
        } else {
            merged.push(observation);
        }
    }
    merged
}

fn merge_neighbor_observation(
    existing: &mut LanNeighborObservation,
    incoming: LanNeighborObservation,
) {
    let replace_primary_identity = should_replace_primary_observation(existing, &incoming);
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
    if reachability_rank(&incoming.reachability) > reachability_rank(&existing.reachability) {
        existing.reachability = incoming.reachability.clone();
    }
    merge_observed_at(&mut existing.observed_at, &incoming.observed_at);
    for scan_source in incoming.scan_sources {
        push_unique_scan_source(&mut existing.scan_sources, &scan_source);
    }
}

fn should_replace_primary_observation(
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

fn reachability_rank(reachability: &LanPairingDeviceReachability) -> u8 {
    match reachability {
        LanPairingDeviceReachability::Online => 3,
        LanPairingDeviceReachability::Stale => 2,
        LanPairingDeviceReachability::Offline => 1,
    }
}

fn push_unique_scan_source(scan_sources: &mut Vec<String>, value: &str) {
    if scan_sources.iter().any(|existing| existing == value) {
        return;
    }
    scan_sources.push(value.to_string());
}

pub(super) fn merge_observed_at(existing: &mut String, incoming: &str) {
    if incoming.is_empty() {
        return;
    }
    if existing.is_empty() || incoming < existing.as_str() {
        *existing = incoming.to_string();
    }
}

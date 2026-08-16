mod decision;
mod details;

use super::super::LanNeighborObservation;

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
    details::merge_linux_neighbor_observation(existing, incoming)
}

pub fn should_replace_linux_primary_observation(
    existing: &LanNeighborObservation,
    incoming: &LanNeighborObservation,
) -> bool {
    decision::should_replace_linux_primary_observation(existing, incoming)
}

pub fn parse_linux_private_ipv4(value: &str) -> Option<std::net::Ipv4Addr> {
    decision::parse_linux_private_ipv4(value)
}

pub fn linux_neighbor_reachability_rank(
    reachability: &ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability,
) -> u8 {
    decision::linux_neighbor_reachability_rank(reachability)
}

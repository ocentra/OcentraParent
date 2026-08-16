use super::super::super::LanNeighborObservation;
use super::super::identity::push_unique_scan_source;
use super::decision::{linux_neighbor_reachability_rank, should_replace_linux_primary_observation};

pub(super) fn merge_linux_neighbor_observation(
    existing: &mut LanNeighborObservation,
    incoming: LanNeighborObservation,
) {
    if should_replace_linux_primary_observation(existing, &incoming) {
        replace_primary_identity(existing, &incoming);
    } else {
        fill_missing_identity(existing, &incoming);
    }
    if linux_neighbor_reachability_rank(&incoming.reachability)
        > linux_neighbor_reachability_rank(&existing.reachability)
    {
        existing.reachability = incoming.reachability.clone();
    }
    super::super::super::neighbor_merge::merge_observed_at(
        &mut existing.observed_at,
        &incoming.observed_at,
    );
    for scan_source in incoming.scan_sources {
        push_unique_scan_source(&mut existing.scan_sources, &scan_source);
    }
}

fn replace_primary_identity(
    existing: &mut LanNeighborObservation,
    incoming: &LanNeighborObservation,
) {
    existing.ip_address = incoming.ip_address.clone();
    if incoming.network_interface.is_some() {
        existing.network_interface = incoming.network_interface.clone();
    }
    if incoming.hostname.is_some() {
        existing.hostname = incoming.hostname.clone();
    }
}

fn fill_missing_identity(existing: &mut LanNeighborObservation, incoming: &LanNeighborObservation) {
    if existing.network_interface.is_none() {
        existing.network_interface = incoming.network_interface.clone();
    }
    if existing.hostname.is_none() {
        existing.hostname = incoming.hostname.clone();
    }
}

use super::super::LanNeighborObservation;

pub(super) fn by_mac(observations: Vec<LanNeighborObservation>) -> Vec<LanNeighborObservation> {
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
    if super::priority::should_replace_primary_observation(existing, &incoming) {
        existing.ip_address = incoming.ip_address.clone();
        replace_missing_identity(existing, &incoming);
    } else {
        fill_missing_identity(existing, &incoming);
    }
    if super::priority::reachability_rank(&incoming.reachability)
        > super::priority::reachability_rank(&existing.reachability)
    {
        existing.reachability = incoming.reachability.clone();
    }
    super::observed_at::merge(&mut existing.observed_at, &incoming.observed_at);
    for scan_source in incoming.scan_sources {
        push_unique_scan_source(&mut existing.scan_sources, &scan_source);
    }
}

fn replace_missing_identity(
    existing: &mut LanNeighborObservation,
    incoming: &LanNeighborObservation,
) {
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

fn push_unique_scan_source(scan_sources: &mut Vec<String>, value: &str) {
    if !scan_sources.iter().any(|existing| existing == value) {
        scan_sources.push(value.to_string());
    }
}

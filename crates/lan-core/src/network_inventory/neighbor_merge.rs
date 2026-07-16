mod merge;
mod observed_at;
mod priority;

use super::LanNeighborObservation;

pub(super) fn merge_neighbor_observations_by_mac(
    observations: Vec<LanNeighborObservation>,
) -> Vec<LanNeighborObservation> {
    merge::by_mac(observations)
}

pub(super) fn merge_observed_at(existing: &mut String, incoming: &str) {
    observed_at::merge(existing, incoming)
}

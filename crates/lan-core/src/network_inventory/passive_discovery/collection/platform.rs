use std::collections::HashMap;
use std::time::Duration;

use super::super::super::linux_neighbors::{
    current_linux_ip_neigh_ipv4_observations_with_timeout,
    current_linux_proc_net_arp_ipv4_observations_with_timeout,
};
use super::super::super::macos_neighbors::current_macos_neighbor_ipv4_observations_with_timeout;
use super::super::super::windows_neighbors::current_windows_neighbor_ipv4_observations_with_timeout;
use super::super::LanPassiveDiscoveryLocalNeighborSource;
use super::local_neighbor_collection_support_for_platform;

mod catalog;
mod rules;

pub(super) fn collect_observations(
    source: LanPassiveDiscoveryLocalNeighborSource,
    read_timeout: Duration,
) -> HashMap<String, String> {
    match source {
        LanPassiveDiscoveryLocalNeighborSource::WindowsNeighborTable => {
            current_windows_neighbor_ipv4_observations_with_timeout(read_timeout)
        }
        LanPassiveDiscoveryLocalNeighborSource::LinuxProcNetArp => {
            current_linux_proc_net_arp_ipv4_observations_with_timeout(read_timeout)
        }
        LanPassiveDiscoveryLocalNeighborSource::LinuxIpNeigh => {
            current_linux_ip_neigh_ipv4_observations_with_timeout(read_timeout)
        }
        LanPassiveDiscoveryLocalNeighborSource::MacosArp => {
            current_macos_neighbor_ipv4_observations_with_timeout(read_timeout)
        }
    }
}

pub(super) fn local_neighbor_sources_for_platform(
    platform: &str,
) -> Vec<LanPassiveDiscoveryLocalNeighborSource> {
    all_local_neighbor_sources()
        .into_iter()
        .filter(|source| local_neighbor_collection_support_for_platform(*source, platform).is_ok())
        .collect()
}

pub(super) fn local_neighbor_source_labels_for_platform(platform: &str) -> Vec<String> {
    local_neighbor_source_labels(&local_neighbor_sources_for_platform(platform))
}

pub(super) fn local_neighbor_source_labels(
    sources: &[LanPassiveDiscoveryLocalNeighborSource],
) -> Vec<String> {
    sources
        .iter()
        .map(local_neighbor_source_label)
        .map(str::to_string)
        .collect()
}

pub(super) fn all_local_neighbor_sources() -> Vec<LanPassiveDiscoveryLocalNeighborSource> {
    catalog::all_local_neighbor_sources()
}

pub(super) fn local_neighbor_source_label(
    source: &LanPassiveDiscoveryLocalNeighborSource,
) -> &'static str {
    catalog::local_neighbor_source_label(source)
}

pub(super) fn local_neighbor_source_supported(
    source: LanPassiveDiscoveryLocalNeighborSource,
    platform: &str,
) -> bool {
    rules::local_neighbor_source_supported(source, platform)
}

pub(super) fn unsupported_local_neighbor_source_reason(
    source: LanPassiveDiscoveryLocalNeighborSource,
    source_label: &str,
    platform: &str,
) -> String {
    rules::unsupported_local_neighbor_source_reason(source, source_label, platform)
}

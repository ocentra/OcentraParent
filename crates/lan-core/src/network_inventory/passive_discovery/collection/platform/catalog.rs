use super::super::super::LanPassiveDiscoveryLocalNeighborSource;

pub(super) fn all_local_neighbor_sources() -> Vec<LanPassiveDiscoveryLocalNeighborSource> {
    vec![
        LanPassiveDiscoveryLocalNeighborSource::WindowsNeighborTable,
        LanPassiveDiscoveryLocalNeighborSource::LinuxProcNetArp,
        LanPassiveDiscoveryLocalNeighborSource::LinuxIpNeigh,
        LanPassiveDiscoveryLocalNeighborSource::MacosArp,
    ]
}

pub(super) fn local_neighbor_source_label(
    source: &LanPassiveDiscoveryLocalNeighborSource,
) -> &'static str {
    match source {
        LanPassiveDiscoveryLocalNeighborSource::WindowsNeighborTable => "windows-neighbor-table",
        LanPassiveDiscoveryLocalNeighborSource::LinuxProcNetArp => "linux-proc-net-arp",
        LanPassiveDiscoveryLocalNeighborSource::LinuxIpNeigh => "linux-ip-neigh",
        LanPassiveDiscoveryLocalNeighborSource::MacosArp => "macos-arp",
    }
}

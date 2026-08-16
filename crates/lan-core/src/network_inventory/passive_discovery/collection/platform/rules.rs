use super::super::super::LanPassiveDiscoveryLocalNeighborSource;

pub(super) fn local_neighbor_source_supported(
    source: LanPassiveDiscoveryLocalNeighborSource,
    platform: &str,
) -> bool {
    match source {
        LanPassiveDiscoveryLocalNeighborSource::WindowsNeighborTable => {
            platform.eq_ignore_ascii_case("windows")
        }
        LanPassiveDiscoveryLocalNeighborSource::LinuxProcNetArp
        | LanPassiveDiscoveryLocalNeighborSource::LinuxIpNeigh => {
            platform.eq_ignore_ascii_case("linux") || platform.eq_ignore_ascii_case("android")
        }
        LanPassiveDiscoveryLocalNeighborSource::MacosArp => platform.eq_ignore_ascii_case("macos"),
    }
}

pub(super) fn unsupported_local_neighbor_source_reason(
    source: LanPassiveDiscoveryLocalNeighborSource,
    source_label: &str,
    platform: &str,
) -> String {
    match source {
        LanPassiveDiscoveryLocalNeighborSource::WindowsNeighborTable => format!(
            "{source_label} passive collection is only available on windows; current platform is {platform}"
        ),
        LanPassiveDiscoveryLocalNeighborSource::LinuxProcNetArp
        | LanPassiveDiscoveryLocalNeighborSource::LinuxIpNeigh => format!(
            "{source_label} passive collection is only available on linux or android; current platform is {platform}"
        ),
        LanPassiveDiscoveryLocalNeighborSource::MacosArp => format!(
            "{source_label} passive collection is only available on macos; current platform is {platform}"
        ),
    }
}

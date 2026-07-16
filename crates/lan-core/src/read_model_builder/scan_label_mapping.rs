use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;

#[path = "scan_label_mapping_extra.rs"]
mod extra;

pub(super) fn scan_source_label(source: &LanDiscoveryEvidenceSource) -> Option<&'static str> {
    match source {
        LanDiscoveryEvidenceSource::LocalService => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE)
        }
        LanDiscoveryEvidenceSource::WindowsNeighborTable => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR)
        }
        LanDiscoveryEvidenceSource::LinuxProcNetArp => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP)
        }
        LanDiscoveryEvidenceSource::LinuxIpNeigh => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH)
        }
        LanDiscoveryEvidenceSource::MacosArp => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP)
        }
        LanDiscoveryEvidenceSource::ServiceIdentityProbe => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE)
        }
        _ => extra::scan_source_label(source),
    }
}

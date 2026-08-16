use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;

pub(super) fn discovery_evidence_source_from_scan_source(
    scan_source: &str,
) -> Option<LanDiscoveryEvidenceSource> {
    match scan_source {
        constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR => {
            Some(LanDiscoveryEvidenceSource::WindowsNeighborTable)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP => {
            Some(LanDiscoveryEvidenceSource::LinuxProcNetArp)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH => {
            Some(LanDiscoveryEvidenceSource::LinuxIpNeigh)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP => {
            Some(LanDiscoveryEvidenceSource::MacosArp)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE => {
            Some(LanDiscoveryEvidenceSource::ServiceIdentityProbe)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD => {
            Some(LanDiscoveryEvidenceSource::MdnsDnsSdQuery)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP => {
            Some(LanDiscoveryEvidenceSource::SsdpUpnpQuery)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE => {
            Some(LanDiscoveryEvidenceSource::DnsCache)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_NETBIOS => {
            Some(LanDiscoveryEvidenceSource::Netbios)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_LLMNR => Some(LanDiscoveryEvidenceSource::Llmnr),
        _ => None,
    }
}

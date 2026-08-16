use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;

pub(super) fn scan_source_label(source: &LanDiscoveryEvidenceSource) -> Option<&'static str> {
    match source {
        LanDiscoveryEvidenceSource::MdnsDnsSdQuery => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD)
        }
        LanDiscoveryEvidenceSource::SsdpUpnpQuery => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP)
        }
        LanDiscoveryEvidenceSource::DnsCache => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE)
        }
        LanDiscoveryEvidenceSource::Netbios => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_NETBIOS)
        }
        LanDiscoveryEvidenceSource::Llmnr => Some(constants::lan_pairing::LAN_SCAN_SOURCE_LLMNR),
        LanDiscoveryEvidenceSource::PreviousScanSnapshot
        | LanDiscoveryEvidenceSource::TrustedRegistry
        | LanDiscoveryEvidenceSource::ParentAssignment
        | LanDiscoveryEvidenceSource::ChildAgentHello
        | LanDiscoveryEvidenceSource::ChildAgentHeartbeat => None,
        _ => None,
    }
}

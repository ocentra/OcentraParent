use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;

pub(super) fn is_network_neighbor_evidence_source(source: &LanDiscoveryEvidenceSource) -> bool {
    matches!(
        source,
        LanDiscoveryEvidenceSource::WindowsNeighborTable
            | LanDiscoveryEvidenceSource::LinuxProcNetArp
            | LanDiscoveryEvidenceSource::LinuxIpNeigh
            | LanDiscoveryEvidenceSource::MacosArp
            | LanDiscoveryEvidenceSource::MdnsDnsSdQuery
            | LanDiscoveryEvidenceSource::SsdpUpnpQuery
            | LanDiscoveryEvidenceSource::DnsCache
            | LanDiscoveryEvidenceSource::Netbios
            | LanDiscoveryEvidenceSource::Llmnr
            | LanDiscoveryEvidenceSource::ServiceIdentityProbe
    )
}

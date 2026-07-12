use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceSource,
};

pub(super) fn source_label(source: &LanDiscoveryEvidenceSource) -> &'static str {
    match source {
        LanDiscoveryEvidenceSource::DnsCache => constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE,
        LanDiscoveryEvidenceSource::Netbios => constants::lan_pairing::LAN_SCAN_SOURCE_NETBIOS,
        LanDiscoveryEvidenceSource::Llmnr => constants::lan_pairing::LAN_SCAN_SOURCE_LLMNR,
        _ => "unsupported-name-source",
    }
}

pub(super) fn confidence_label(confidence: &LanDiscoveryEvidenceConfidence) -> &'static str {
    match confidence {
        LanDiscoveryEvidenceConfidence::Weak => "weak",
        LanDiscoveryEvidenceConfidence::Confirmed => "confirmed",
        LanDiscoveryEvidenceConfidence::Strong => "strong",
        LanDiscoveryEvidenceConfidence::ManualRequired => "manual-required",
        LanDiscoveryEvidenceConfidence::Rejected => "rejected",
    }
}

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence;

pub(super) fn evidence_confidence_rank(confidence: &LanDiscoveryEvidenceConfidence) -> u8 {
    match confidence {
        LanDiscoveryEvidenceConfidence::Confirmed => 5,
        LanDiscoveryEvidenceConfidence::Strong => 4,
        LanDiscoveryEvidenceConfidence::Weak => 3,
        LanDiscoveryEvidenceConfidence::ManualRequired => 2,
        LanDiscoveryEvidenceConfidence::Rejected => 1,
    }
}

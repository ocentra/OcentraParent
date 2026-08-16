use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord;

pub(super) fn preferred_display_name(
    existing: &str,
    incoming: &str,
    incoming_evidence: &[LanDiscoveryEvidenceRecord],
) -> String {
    if incoming_has_parent_rename_evidence(incoming_evidence) && !incoming.trim().is_empty() {
        return incoming.to_string();
    }
    if existing.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
        && !incoming.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
    {
        incoming.to_string()
    } else {
        existing.to_string()
    }
}

fn incoming_has_parent_rename_evidence(incoming_evidence: &[LanDiscoveryEvidenceRecord]) -> bool {
    incoming_evidence.iter().any(|record| {
        record.evidence_kind == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::ParentDecision
            && record.value == constants::lan_pairing::HOUSEHOLD_ACTION_RENAME
    })
}

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord,
};

use crate::mac_identity::assess_mac_address;

pub(super) fn evidence_kind_overlaps(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    kinds: &[LanDiscoveryEvidenceKind],
) -> bool {
    existing
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| trusted_merge_evidence(record, kinds))
        .any(|existing_record| {
            incoming
                .network_identity
                .evidence_records
                .iter()
                .filter(|record| trusted_merge_evidence(record, kinds))
                .any(|incoming_record| {
                    existing_record.evidence_kind == incoming_record.evidence_kind
                        && existing_record
                            .normalized_value
                            .eq_ignore_ascii_case(&incoming_record.normalized_value)
                })
        })
}

fn trusted_merge_evidence(
    record: &LanDiscoveryEvidenceRecord,
    kinds: &[LanDiscoveryEvidenceKind],
) -> bool {
    if !kinds.contains(&record.evidence_kind) {
        return false;
    }
    if record.evidence_kind == LanDiscoveryEvidenceKind::MacAddress
        && !stable_mac_evidence_value(&record.normalized_value)
    {
        return false;
    }
    matches!(
        record.confidence,
        LanDiscoveryEvidenceConfidence::Confirmed | LanDiscoveryEvidenceConfidence::Strong
    )
}

fn stable_mac_evidence_value(value: &str) -> bool {
    assess_mac_address(Some(value))
        .map(|assessment| assessment.stable_identity_key_allowed())
        .unwrap_or(false)
}

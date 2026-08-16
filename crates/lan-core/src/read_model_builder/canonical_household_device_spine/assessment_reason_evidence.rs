use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
};

pub(super) fn evidence_kind_overlaps(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    kinds: &[LanDiscoveryEvidenceKind],
) -> bool {
    super::super::values::evidence_kind_overlaps(existing, incoming, kinds)
}

pub(super) fn strong_service_hint_overlap(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    prefixes: &[&str],
) -> bool {
    existing
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| strong_service_hint_record(record, prefixes))
        .any(|existing_record| {
            incoming
                .network_identity
                .evidence_records
                .iter()
                .filter(|record| strong_service_hint_record(record, prefixes))
                .any(|incoming_record| {
                    existing_record
                        .normalized_value
                        .eq_ignore_ascii_case(&incoming_record.normalized_value)
                })
        })
}

fn strong_service_hint_record(record: &LanDiscoveryEvidenceRecord, prefixes: &[&str]) -> bool {
    record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
        && prefixes.iter().any(|prefix| {
            record
                .value
                .get(..prefix.len())
                .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
        })
}

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceRecord,
};

pub(super) fn merge_evidence_records(
    existing: &mut Vec<LanDiscoveryEvidenceRecord>,
    incoming: Vec<LanDiscoveryEvidenceRecord>,
) {
    for record in incoming {
        if let Some(existing_record) = existing
            .iter_mut()
            .find(|entry| same_evidence_record_identity(entry, &record))
        {
            merge_evidence_record_fields(existing_record, &record);
            continue;
        }
        existing.push(record);
    }
}

fn merge_evidence_record_fields(
    existing_record: &mut LanDiscoveryEvidenceRecord,
    record: &LanDiscoveryEvidenceRecord,
) {
    if record.first_seen_at < existing_record.first_seen_at {
        existing_record.first_seen_at = record.first_seen_at.clone();
    }
    if record.last_seen_at > existing_record.last_seen_at {
        existing_record.last_seen_at = record.last_seen_at.clone();
    }
    if evidence_confidence_rank(&record.confidence)
        > evidence_confidence_rank(&existing_record.confidence)
    {
        existing_record.confidence = record.confidence.clone();
    }
    if existing_record.note.is_none() {
        existing_record.note = record.note.clone();
    }
}

fn same_evidence_record_identity(
    existing: &LanDiscoveryEvidenceRecord,
    incoming: &LanDiscoveryEvidenceRecord,
) -> bool {
    existing.source == incoming.source
        && existing.evidence_kind == incoming.evidence_kind
        && existing.merge_key.eq_ignore_ascii_case(&incoming.merge_key)
        && existing.device_id.eq_ignore_ascii_case(&incoming.device_id)
}

fn evidence_confidence_rank(confidence: &LanDiscoveryEvidenceConfidence) -> u8 {
    match confidence {
        LanDiscoveryEvidenceConfidence::Confirmed => 5,
        LanDiscoveryEvidenceConfidence::Strong => 4,
        LanDiscoveryEvidenceConfidence::Weak => 3,
        LanDiscoveryEvidenceConfidence::ManualRequired => 2,
        LanDiscoveryEvidenceConfidence::Rejected => 1,
    }
}

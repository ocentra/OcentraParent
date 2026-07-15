use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord;

use super::evidence_rank::evidence_confidence_rank;

pub(super) fn merge_evidence_records(
    existing: &mut Vec<LanDiscoveryEvidenceRecord>,
    incoming: Vec<LanDiscoveryEvidenceRecord>,
) {
    for record in incoming {
        if let Some(existing_record) = existing
            .iter_mut()
            .find(|entry| same_evidence_record_identity(entry, &record))
        {
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
            if record.expires_at.is_some() {
                existing_record.expires_at = record.expires_at.clone();
            }
            continue;
        }
        existing.push(record);
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

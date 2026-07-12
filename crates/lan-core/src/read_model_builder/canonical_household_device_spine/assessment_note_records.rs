#[path = "assessment_note_record_attribute_matches.rs"]
mod attribute_matches;
#[path = "assessment_note_record_identity_matches.rs"]
mod identity_matches;
#[path = "assessment_note_record_service_matches.rs"]
mod service_matches;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord;

use super::super::assessment_reasons::MergeDecisionReason;

pub(super) fn merge_reason_matches_record(
    reason: MergeDecisionReason,
    record: &LanDiscoveryEvidenceRecord,
) -> bool {
    identity_matches::matches(reason, record)
        || service_matches::matches(reason, record)
        || attribute_matches::matches(reason, record)
}

pub(super) fn append_merge_note(note: &mut Option<String>, merge_note: &str) {
    match note {
        Some(existing) if !existing.contains(merge_note) => {
            existing.push_str(" | ");
            existing.push_str(merge_note);
        }
        Some(_) => {}
        None => *note = Some(merge_note.to_string()),
    }
}

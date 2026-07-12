use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceConfidence,
};

use super::assessment::{MergeAssessment, MergeDecisionState};
use note_labels::{merge_reason_label, merge_state_label};
use note_records::{append_merge_note, merge_reason_matches_record};

#[path = "assessment_note_labels.rs"]
mod note_labels;
#[path = "assessment_note_records.rs"]
mod note_records;

const DEDUPE_DECISION_NOTE_PREFIX: &str = "dedupe-decision=";

pub(super) fn annotate_merge_assessment(
    device: &mut LanCanonicalHouseholdDevice,
    assessment: &MergeAssessment,
) {
    if assessment.state == MergeDecisionState::NoMatch || assessment.reasons.is_empty() {
        return;
    }
    if assessment.state != MergeDecisionState::Automatic {
        device.network_identity.confidence = LanCanonicalHouseholdDeviceConfidence::ManualRequired;
    }
    let note = merge_assessment_note(assessment);
    if !annotate_matching_records(device, assessment, &note) {
        if let Some(record) = device.network_identity.evidence_records.first_mut() {
            append_merge_note(&mut record.note, &note);
        }
    }
}

fn annotate_matching_records(
    device: &mut LanCanonicalHouseholdDevice,
    assessment: &MergeAssessment,
    note: &str,
) -> bool {
    let mut annotated = false;
    for reason in &assessment.reasons {
        for record in device
            .network_identity
            .evidence_records
            .iter_mut()
            .filter(|record| merge_reason_matches_record(*reason, record))
        {
            append_merge_note(&mut record.note, note);
            annotated = true;
        }
    }
    annotated
}

fn merge_assessment_note(assessment: &MergeAssessment) -> String {
    let reasons = assessment
        .reasons
        .iter()
        .map(|reason| merge_reason_label(*reason))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{DEDUPE_DECISION_NOTE_PREFIX}{} score={} reasons={reasons}",
        merge_state_label(assessment.state),
        assessment.score
    )
}

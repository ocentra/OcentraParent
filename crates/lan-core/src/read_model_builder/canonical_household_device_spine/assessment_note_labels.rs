#[path = "assessment_note_attribute_reason_labels.rs"]
mod attribute_reason_labels;
#[path = "assessment_note_identity_reason_labels.rs"]
mod identity_reason_labels;

use super::super::assessment::MergeDecisionState;
use super::super::assessment_reasons::MergeDecisionReason;

pub(super) fn merge_state_label(state: MergeDecisionState) -> &'static str {
    match state {
        MergeDecisionState::Automatic => "automatic",
        MergeDecisionState::ManualRequired => "manual-required",
        MergeDecisionState::Forbidden => "forbidden",
        MergeDecisionState::NoMatch => "no-match",
    }
}

pub(super) fn merge_reason_label(reason: MergeDecisionReason) -> &'static str {
    identity_reason_labels::label(reason).unwrap_or_else(|| attribute_reason_labels::label(reason))
}

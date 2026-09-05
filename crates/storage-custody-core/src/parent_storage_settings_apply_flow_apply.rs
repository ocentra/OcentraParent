use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::{
    parent_storage_settings_apply_flow_intent_digest::derive_parent_storage_apply_intent_digest,
    ParentStorageApplyDecisionInput, ParentStorageSettingsApplyFlowError,
};

const MANUAL_REQUIRED_NOTE_SEPARATOR: &str = "\n";

pub(super) fn derive_parent_storage_apply_decision(
    preview: &contracts::ParentStorageRestorePreview,
    input: ParentStorageApplyDecisionInput,
) -> Result<contracts::ParentStorageApplyDecision, ParentStorageSettingsApplyFlowError> {
    if !preview.confirmation_required {
        return Err(ParentStorageSettingsApplyFlowError::ApplyCannotProceedWithoutPreview);
    }

    if manual_required_note_is_empty(preview.manual_required_note.as_deref())
        || manual_required_note_is_empty(input.manual_required_note.as_deref())
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }

    let apply_intent_digest = derive_parent_storage_apply_intent_digest(preview, &input)?;
    let manual_required_note = combine_manual_required_notes(
        preview.manual_required_note.as_deref(),
        input.manual_required_note.as_deref(),
    );
    if !input.manual_review_required.is_empty()
        && manual_required_note_is_empty(manual_required_note.as_deref())
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }
    let ready_for_confirmation = matches!(
        preview.preview_state,
        contracts::ParentStoragePreviewState::ImportPreviewPassed
            | contracts::ParentStoragePreviewState::PartialRestore
    ) && preview.household_match
        && preview.device_match
        && manual_required_note.is_none();
    if !ready_for_confirmation && manual_required_note.is_none() {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }
    let apply_state = if ready_for_confirmation {
        contracts::ParentStorageApplyState::ApplyRequiresConfirmation
    } else {
        contracts::ParentStorageApplyState::BlockedManualRequired
    };

    Ok(contracts::ParentStorageApplyDecision {
        apply_id: input.apply_id,
        apply_intent_digest,
        apply_state,
        confirmation_required: true,
        will_change: input.will_change,
        will_not_change: input.will_not_change,
        preserved_tombstones: input.preserved_tombstones,
        manual_review_required: input.manual_review_required,
        rollback_available: false,
        manual_required_note,
    })
}

fn manual_required_note_is_empty(note: Option<&str>) -> bool {
    matches!(note, Some(value) if value.trim().is_empty())
}

fn combine_manual_required_notes(
    preview_note: Option<&str>,
    apply_note: Option<&str>,
) -> Option<String> {
    match (preview_note, apply_note) {
        (None, None) => None,
        // ALLOC-JUSTIFICATION: the public proof contract owns its note text.
        (Some(note), None) | (None, Some(note)) => Some(note.to_owned()),
        // Preserve both independently supplied blockers in stable preview/apply order.
        (Some(preview_note), Some(apply_note)) if preview_note == apply_note => {
            // ALLOC-JUSTIFICATION: the public proof contract owns its note text.
            Some(preview_note.to_owned())
        }
        (Some(preview_note), Some(apply_note)) => {
            // ALLOC-JUSTIFICATION: one contract field must retain both material blockers.
            Some(format!(
                "{preview_note}{MANUAL_REQUIRED_NOTE_SEPARATOR}{apply_note}"
            ))
        }
    }
}

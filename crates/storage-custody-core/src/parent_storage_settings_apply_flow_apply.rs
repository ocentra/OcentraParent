use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::{
    parent_storage_settings_apply_flow_intent_digest::derive_parent_storage_apply_intent_digest,
    ParentStorageApplyDecisionInput, ParentStorageSettingsApplyFlowError,
};

pub(super) fn derive_parent_storage_apply_decision(
    preview: &contracts::ParentStorageRestorePreview,
    input: ParentStorageApplyDecisionInput,
) -> Result<contracts::ParentStorageApplyDecision, ParentStorageSettingsApplyFlowError> {
    if !preview.confirmation_required {
        return Err(ParentStorageSettingsApplyFlowError::ApplyCannotProceedWithoutPreview);
    }

    let apply_intent_digest = derive_parent_storage_apply_intent_digest(preview, &input)?;
    let ready_for_confirmation = matches!(
        preview.preview_state,
        contracts::ParentStoragePreviewState::ImportPreviewPassed
            | contracts::ParentStoragePreviewState::PartialRestore
    ) && preview.household_match
        && preview.device_match
        && input.manual_required_note.is_none();
    let manual_required_note = if ready_for_confirmation {
        None
    } else {
        input
            .manual_required_note
            .or_else(|| preview.manual_required_note.clone())
    };
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
        rollback_available: input.rollback_available,
        manual_required_note,
    })
}

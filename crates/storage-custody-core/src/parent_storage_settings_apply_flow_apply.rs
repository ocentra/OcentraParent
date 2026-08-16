use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::{ParentStorageApplyDecisionInput, ParentStorageSettingsApplyFlowError};

pub(super) fn derive_parent_storage_apply_decision(
    preview: &contracts::ParentStorageRestorePreview,
    input: ParentStorageApplyDecisionInput,
) -> Result<contracts::ParentStorageApplyDecision, ParentStorageSettingsApplyFlowError> {
    match input.apply_state {
        contracts::ParentStorageApplyState::ApplyRequiresConfirmation
        | contracts::ParentStorageApplyState::ApplyPending
        | contracts::ParentStorageApplyState::Applied
        | contracts::ParentStorageApplyState::Partial
        | contracts::ParentStorageApplyState::RollbackManualRequired
        | contracts::ParentStorageApplyState::BlockedManualRequired
        | contracts::ParentStorageApplyState::NotStarted => {}
    }

    if preview.confirmation_required
        && matches!(
            input.apply_state,
            contracts::ParentStorageApplyState::Applied
                | contracts::ParentStorageApplyState::Partial
        )
    {
        return Err(ParentStorageSettingsApplyFlowError::ApplyMustStayConfirmationGated);
    }
    if input.apply_state == contracts::ParentStorageApplyState::ApplyRequiresConfirmation
        && !preview.confirmation_required
    {
        return Err(ParentStorageSettingsApplyFlowError::ApplyCannotProceedWithoutPreview);
    }
    if matches!(
        input.apply_state,
        contracts::ParentStorageApplyState::BlockedManualRequired
            | contracts::ParentStorageApplyState::RollbackManualRequired
    ) && input.manual_required_note.is_none()
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }

    Ok(contracts::ParentStorageApplyDecision {
        apply_id: input.apply_id,
        apply_state: input.apply_state,
        confirmation_required: true,
        will_change: input.will_change,
        will_not_change: input.will_not_change,
        preserved_tombstones: input.preserved_tombstones,
        manual_review_required: input.manual_review_required,
        rollback_available: input.rollback_available,
        manual_required_note: input.manual_required_note,
    })
}

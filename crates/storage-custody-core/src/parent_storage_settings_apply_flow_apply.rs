use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::{
    parent_storage_settings_apply_flow_confirmation::validate_confirmation_receipt,
    ParentStorageApplyDecisionInput, ParentStorageSettingsApplyFlowError,
};

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

    let confirmed_apply = matches!(
        input.apply_state,
        contracts::ParentStorageApplyState::Applied | contracts::ParentStorageApplyState::Partial
    );
    if confirmed_apply {
        let receipt = input
            .confirmation_receipt
            .as_ref()
            .ok_or(ParentStorageSettingsApplyFlowError::ConfirmationRequired)?;
        validate_confirmation_receipt(preview, receipt)?;
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
        confirmation_required: !confirmed_apply,
        will_change: input.will_change,
        will_not_change: input.will_not_change,
        preserved_tombstones: input.preserved_tombstones,
        manual_review_required: input.manual_review_required,
        rollback_available: input.rollback_available,
        manual_required_note: input.manual_required_note,
    })
}

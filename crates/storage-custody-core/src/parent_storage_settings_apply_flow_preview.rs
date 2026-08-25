use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::{ParentStorageRestorePreviewInput, ParentStorageSettingsApplyFlowError};

pub(super) fn derive_parent_storage_restore_preview(
    input: ParentStorageRestorePreviewInput,
) -> Result<contracts::ParentStorageRestorePreview, ParentStorageSettingsApplyFlowError> {
    if !matches!(
        input.preview_state,
        contracts::ParentStoragePreviewState::WrongHousehold
            | contracts::ParentStoragePreviewState::WrongKey
            | contracts::ParentStoragePreviewState::SchemaUnsupported
            | contracts::ParentStoragePreviewState::BundleCorrupt
            | contracts::ParentStoragePreviewState::TombstoneConflict
            | contracts::ParentStoragePreviewState::PartialRestore
            | contracts::ParentStoragePreviewState::ImportPreviewPassed
            | contracts::ParentStoragePreviewState::ManualRequired
    ) {
        return Err(ParentStorageSettingsApplyFlowError::RestorePreviewMustRequireConfirmation);
    }

    validate_restore_preview_state(&input)?;
    if input.preview_state == contracts::ParentStoragePreviewState::WrongHousehold
        && input.household_match
    {
        return Err(ParentStorageSettingsApplyFlowError::WrongHouseholdPreviewMustNotMatch);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::ManualRequired
        && input
            .manual_required_note
            .as_deref()
            .is_none_or(|note| note.trim().is_empty())
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::TombstoneConflict
        && input.rejected_sections.is_empty()
    {
        return Err(ParentStorageSettingsApplyFlowError::PartialRestoreMustNameRejectedSections);
    }

    Ok(contracts::ParentStorageRestorePreview {
        preview_id: input.preview_id,
        household_ref: input.household_ref,
        preview_state: input.preview_state,
        created_at: input.created_at,
        product_version: input.product_version,
        schema_version: input.schema_version,
        household_match: input.household_match,
        device_match: input.device_match,
        data_classes: input.data_classes,
        conflicts: input.conflicts,
        rejected_sections: input.rejected_sections,
        partial_restore: input.partial_restore,
        confirmation_required: true,
        local_truth_authoritative: true,
        tombstones_preserved: true,
        manual_required_note: input.manual_required_note,
    })
}

fn validate_restore_preview_state(
    input: &ParentStorageRestorePreviewInput,
) -> Result<(), ParentStorageSettingsApplyFlowError> {
    let is_partial_restore =
        input.preview_state == contracts::ParentStoragePreviewState::PartialRestore;
    if is_partial_restore != input.partial_restore {
        return Err(ParentStorageSettingsApplyFlowError::PartialRestoreStateMustMatchFlag);
    }
    if is_partial_restore && input.rejected_sections.is_empty() {
        return Err(ParentStorageSettingsApplyFlowError::PartialRestoreMustNameRejectedSections);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::ImportPreviewPassed
        && !input.rejected_sections.is_empty()
    {
        return Err(ParentStorageSettingsApplyFlowError::ImportPreviewPassedMustBeComplete);
    }
    Ok(())
}

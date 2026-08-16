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

    if input.preview_state == contracts::ParentStoragePreviewState::PartialRestore
        && input.rejected_sections.is_empty()
    {
        return Err(ParentStorageSettingsApplyFlowError::PartialRestoreMustNameRejectedSections);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::WrongHousehold
        && input.household_match
    {
        return Err(ParentStorageSettingsApplyFlowError::WrongHouseholdPreviewMustNotMatch);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::WrongKey && input.device_match {
        // wrong key can still be device-matched; don't reject
    }
    if input.preview_state == contracts::ParentStoragePreviewState::ManualRequired
        && input.manual_required_note.is_none()
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::WrongHousehold
        && input.device_match
        && input.household_match
    {
        return Err(ParentStorageSettingsApplyFlowError::WrongHouseholdPreviewMustNotMatch);
    }
    if input.preview_state == contracts::ParentStoragePreviewState::TombstoneConflict
        && input.rejected_sections.is_empty()
    {
        return Err(ParentStorageSettingsApplyFlowError::PartialRestoreMustNameRejectedSections);
    }

    Ok(contracts::ParentStorageRestorePreview {
        preview_id: input.preview_id,
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

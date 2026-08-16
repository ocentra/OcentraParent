use ocentra_schema::parent_owned_sync_export as sync_contracts;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::ParentStorageSettingsApplyFlowError;

pub(super) fn validate_parent_storage_mode_card(
    current_mode_label: contracts::ParentStorageModeLabel,
    manual_required_visible: bool,
    provider_status: sync_contracts::ParentOwnedSyncProviderStatus,
) -> Result<(), ParentStorageSettingsApplyFlowError> {
    if current_mode_label == contracts::ParentStorageModeLabel::ManualRequired
        && !manual_required_visible
    {
        return Err(ParentStorageSettingsApplyFlowError::ManualRequiredMustStayVisible);
    }
    if current_mode_label == contracts::ParentStorageModeLabel::Disabled
        && provider_status != sync_contracts::ParentOwnedSyncProviderStatus::Disabled
        && provider_status != sync_contracts::ParentOwnedSyncProviderStatus::NotConfigured
    {
        return Err(ParentStorageSettingsApplyFlowError::DisabledModeMustStayDisabled);
    }

    Ok(())
}

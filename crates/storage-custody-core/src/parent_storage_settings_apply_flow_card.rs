use ocentra_schema::parent_owned_sync_export as sync_contracts;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;

use super::{ParentStorageModeCardInput, ParentStorageSettingsApplyFlowError};

#[path = "parent_storage_settings_apply_flow_card_summary.rs"]
mod parent_storage_settings_apply_flow_card_summary;
#[path = "parent_storage_settings_apply_flow_card_ui.rs"]
mod parent_storage_settings_apply_flow_card_ui;
#[path = "parent_storage_settings_apply_flow_card_validate.rs"]
mod parent_storage_settings_apply_flow_card_validate;

pub(super) fn derive_parent_storage_mode_card(
    input: ParentStorageModeCardInput,
) -> Result<contracts::ParentStorageModeCard, ParentStorageSettingsApplyFlowError> {
    let current_mode_label = match input.provider_status {
        sync_contracts::ParentOwnedSyncProviderStatus::Disabled
        | sync_contracts::ParentOwnedSyncProviderStatus::NotConfigured => {
            contracts::ParentStorageModeLabel::Disabled
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Disconnected => {
            contracts::ParentStorageModeLabel::ProviderDisconnected
        }
        sync_contracts::ParentOwnedSyncProviderStatus::WrongAccount
        | sync_contracts::ParentOwnedSyncProviderStatus::FolderUnavailable
        | sync_contracts::ParentOwnedSyncProviderStatus::PartialUpload
        | sync_contracts::ParentOwnedSyncProviderStatus::Revoked => {
            contracts::ParentStorageModeLabel::ProviderError
        }
        sync_contracts::ParentOwnedSyncProviderStatus::ManualRequired => {
            contracts::ParentStorageModeLabel::ManualRequired
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Ready => {
            if input.provider_mode == sync_contracts::ParentOwnedSyncProviderMode::LocalFolder {
                contracts::ParentStorageModeLabel::LocalPlusEncryptedBackup
            } else {
                contracts::ParentStorageModeLabel::LocalPlusEncryptedProviderSync
            }
        }
    };

    let ui_state = parent_storage_settings_apply_flow_card_ui::parent_storage_ui_state(
        input.provider_status,
        input.sync_state,
    );

    let manual_required_visible = current_mode_label
        == contracts::ParentStorageModeLabel::ManualRequired
        || ui_state == contracts::ParentStorageUiState::ManualRequired
        || input.key_status == contracts::ParentStorageKeyStatus::ManualRequired;
    parent_storage_settings_apply_flow_card_validate::validate_parent_storage_mode_card(
        current_mode_label,
        manual_required_visible,
        input.provider_status,
    )?;

    Ok(contracts::ParentStorageModeCard {
        row_id: input.row_id,
        current_mode_label,
        ui_state,
        provider_mode: input.provider_mode,
        provider_status: input.provider_status,
        sync_state: input.sync_state,
        encryption_status: input.encryption_status,
        key_status: input.key_status,
        manual_required_visible,
        disconnect_visible: input.provider_status
            == sync_contracts::ParentOwnedSyncProviderStatus::Disconnected,
        delete_visible: input.provider_status
            != sync_contracts::ParentOwnedSyncProviderStatus::Disabled
            && input.provider_status
                != sync_contracts::ParentOwnedSyncProviderStatus::NotConfigured,
        restore_preview_available: true,
        apply_back_available: input.provider_status
            == sync_contracts::ParentOwnedSyncProviderStatus::Ready
            && input.sync_state != sync_contracts::ParentOwnedSyncState::ManualRequired,
        last_success_at: input.last_success_at,
        last_failure_at: input.last_failure_at,
        summary: parent_storage_settings_apply_flow_card_summary::summary_for_mode(
            current_mode_label,
            ui_state,
        )
        .to_string(),
    })
}

use ocentra_schema::parent_owned_sync_export as sync_contracts;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;

#[path = "parent_storage_settings_apply_flow_card_ui_ready.rs"]
mod parent_storage_settings_apply_flow_card_ui_ready;

pub(super) fn parent_storage_ui_state(
    provider_status: sync_contracts::ParentOwnedSyncProviderStatus,
    sync_state: sync_contracts::ParentOwnedSyncState,
) -> contracts::ParentStorageUiState {
    match provider_status {
        sync_contracts::ParentOwnedSyncProviderStatus::NotConfigured => {
            contracts::ParentStorageUiState::ProviderNotConfigured
        }
        sync_contracts::ParentOwnedSyncProviderStatus::ManualRequired => {
            contracts::ParentStorageUiState::ManualRequired
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Revoked => {
            contracts::ParentStorageUiState::ProviderRevoked
        }
        sync_contracts::ParentOwnedSyncProviderStatus::WrongAccount => {
            contracts::ParentStorageUiState::ProviderAuthExpired
        }
        sync_contracts::ParentOwnedSyncProviderStatus::FolderUnavailable => {
            contracts::ParentStorageUiState::ProviderPermissionMissing
        }
        sync_contracts::ParentOwnedSyncProviderStatus::PartialUpload => {
            contracts::ParentStorageUiState::ProviderQuotaExceeded
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Disconnected => {
            contracts::ParentStorageUiState::RemoteDisabled
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Disabled => {
            contracts::ParentStorageUiState::SyncDisabled
        }
        sync_contracts::ParentOwnedSyncProviderStatus::Ready => {
            parent_storage_settings_apply_flow_card_ui_ready::ready_ui_state(sync_state)
        }
    }
}

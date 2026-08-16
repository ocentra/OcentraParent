use ocentra_schema::parent_owned_sync_export as sync_contracts;
use ocentra_schema::parent_storage_settings_apply_flow as contracts;

pub(super) fn ready_ui_state(
    sync_state: sync_contracts::ParentOwnedSyncState,
) -> contracts::ParentStorageUiState {
    match sync_state {
        sync_contracts::ParentOwnedSyncState::OfflineRetryPending => {
            contracts::ParentStorageUiState::OfflineQueued
        }
        sync_contracts::ParentOwnedSyncState::ManualRequired => {
            contracts::ParentStorageUiState::ManualRequired
        }
        _ => contracts::ParentStorageUiState::Ready,
    }
}

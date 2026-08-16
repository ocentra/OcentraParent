use ocentra_schema::parent_owned_sync_export as contracts;

use super::{ParentOwnedSyncExportDerivationError, ParentOwnedSyncProviderStatusInput};

pub(super) fn validate_provider_visibility(
    input: &ParentOwnedSyncProviderStatusInput,
) -> Result<(), ParentOwnedSyncExportDerivationError> {
    match input.provider_status {
        contracts::ParentOwnedSyncProviderStatus::Disconnected => {
            if input.disconnect_visibility_state
                != contracts::ParentOwnedSyncDisconnectVisibilityState::DisconnectVisible
            {
                return Err(
                    ParentOwnedSyncExportDerivationError::DisconnectedProviderMustBeVisible,
                );
            }
        }
        contracts::ParentOwnedSyncProviderStatus::ManualRequired => {
            if input.disconnect_visibility_state
                != contracts::ParentOwnedSyncDisconnectVisibilityState::ManualRequired
                && input.delete_visibility_state
                    != contracts::ParentOwnedSyncDeleteVisibilityState::ManualRequired
            {
                return Err(
                    ParentOwnedSyncExportDerivationError::ManualProviderStateMustStayVisible,
                );
            }
        }
        contracts::ParentOwnedSyncProviderStatus::Ready
        | contracts::ParentOwnedSyncProviderStatus::Disabled
        | contracts::ParentOwnedSyncProviderStatus::NotConfigured
        | contracts::ParentOwnedSyncProviderStatus::Revoked
        | contracts::ParentOwnedSyncProviderStatus::WrongAccount
        | contracts::ParentOwnedSyncProviderStatus::FolderUnavailable
        | contracts::ParentOwnedSyncProviderStatus::PartialUpload => {}
    }
    Ok(())
}

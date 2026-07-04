use ocentra_schema::parent_owned_sync_export as contracts;

use super::{ParentOwnedSyncExportDerivationError, ParentOwnedSyncProviderStatusInput};

pub(super) fn validate_provider_status_refs(
    input: &ParentOwnedSyncProviderStatusInput,
) -> Result<(), ParentOwnedSyncExportDerivationError> {
    match input.provider_status {
        contracts::ParentOwnedSyncProviderStatus::Ready => {
            if input.account_ref.is_none() || input.folder_ref.is_none() {
                return Err(ParentOwnedSyncExportDerivationError::ReadyProviderMissingLocationRefs);
            }
        }
        contracts::ParentOwnedSyncProviderStatus::Revoked => {
            if input.revocation_ref.is_none() {
                return Err(
                    ParentOwnedSyncExportDerivationError::RevokedProviderMissingRevocationRef,
                );
            }
        }
        contracts::ParentOwnedSyncProviderStatus::Disabled => {
            if input.account_ref.is_some()
                || input.folder_ref.is_some()
                || input.revocation_ref.is_some()
            {
                return Err(ParentOwnedSyncExportDerivationError::DisabledProviderMustNotKeepRefs);
            }
        }
        contracts::ParentOwnedSyncProviderStatus::NotConfigured => {
            if input.account_ref.is_some()
                || input.folder_ref.is_some()
                || input.revocation_ref.is_some()
            {
                return Err(
                    ParentOwnedSyncExportDerivationError::NotConfiguredProviderMustNotKeepRefs,
                );
            }
        }
        contracts::ParentOwnedSyncProviderStatus::Disconnected
        | contracts::ParentOwnedSyncProviderStatus::ManualRequired
        | contracts::ParentOwnedSyncProviderStatus::WrongAccount
        | contracts::ParentOwnedSyncProviderStatus::FolderUnavailable
        | contracts::ParentOwnedSyncProviderStatus::PartialUpload => {}
    }
    Ok(())
}

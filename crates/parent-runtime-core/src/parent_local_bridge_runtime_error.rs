use ocentra_family_identity_core::account_identity_authority_repository::AccountIdentityAuthorityRepositoryError;

use super::ParentLocalBridgeRuntimeError;

pub(super) fn from_account_repository(
    error: &AccountIdentityAuthorityRepositoryError,
) -> ParentLocalBridgeRuntimeError {
    match error {
        AccountIdentityAuthorityRepositoryError::Unavailable => {
            ParentLocalBridgeRuntimeError::AccountOwnerUnavailable
        }
        AccountIdentityAuthorityRepositoryError::InvalidGeneration => {
            ParentLocalBridgeRuntimeError::AccountOwnerInvalidGeneration
        }
        AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority => {
            ParentLocalBridgeRuntimeError::AccountOwnerInvalidStoredAuthority
        }
        AccountIdentityAuthorityRepositoryError::CurrentnessConflict => {
            ParentLocalBridgeRuntimeError::AccountOwnerCurrentnessConflict
        }
    }
}

use super::super::{AccountIdentityAuthorityIssuerClientError, AccountIssuerRepositoryError};

pub(super) fn map(
    error: &AccountIdentityAuthorityIssuerClientError,
) -> Option<AccountIssuerRepositoryError> {
    match error {
        AccountIdentityAuthorityIssuerClientError::InvalidPath => {
            Some(AccountIssuerRepositoryError::InvalidPath)
        }
        AccountIdentityAuthorityIssuerClientError::InvalidSchema => {
            Some(AccountIssuerRepositoryError::InvalidSchema)
        }
        AccountIdentityAuthorityIssuerClientError::CurrentnessUnavailable => {
            Some(AccountIssuerRepositoryError::CurrentnessUnavailable)
        }
        AccountIdentityAuthorityIssuerClientError::CurrentnessRejected => {
            Some(AccountIssuerRepositoryError::CurrentnessRejected)
        }
        AccountIdentityAuthorityIssuerClientError::KeyUnavailable => {
            Some(AccountIssuerRepositoryError::KeyUnavailable)
        }
        AccountIdentityAuthorityIssuerClientError::InvalidKey => {
            Some(AccountIssuerRepositoryError::InvalidKey)
        }
        AccountIdentityAuthorityIssuerClientError::InvalidReceipt
        | AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable
        | AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable => {
            Some(AccountIssuerRepositoryError::ReceiptUnavailable)
        }
        AccountIdentityAuthorityIssuerClientError::ReplayDetected => {
            Some(AccountIssuerRepositoryError::ReplayDetected)
        }
        _ => None,
    }
}

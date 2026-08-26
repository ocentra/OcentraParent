use super::{AccountIdentityAuthorityIssuerClientError, AccountIssuerRepositoryError};

impl From<AccountIdentityAuthorityIssuerClientError> for AccountIssuerRepositoryError {
    fn from(error: AccountIdentityAuthorityIssuerClientError) -> Self {
        match error {
            AccountIdentityAuthorityIssuerClientError::InvalidPath => Self::InvalidPath,
            AccountIdentityAuthorityIssuerClientError::InvalidSchema => Self::InvalidSchema,
            AccountIdentityAuthorityIssuerClientError::CurrentnessUnavailable => {
                Self::CurrentnessUnavailable
            }
            AccountIdentityAuthorityIssuerClientError::CurrentnessRejected => {
                Self::CurrentnessRejected
            }
            AccountIdentityAuthorityIssuerClientError::KeyUnavailable => Self::KeyUnavailable,
            AccountIdentityAuthorityIssuerClientError::InvalidKey => Self::InvalidKey,
            AccountIdentityAuthorityIssuerClientError::InvalidReceipt
            | AccountIdentityAuthorityIssuerClientError::ReceiptUnavailable
            | AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable => {
                Self::ReceiptUnavailable
            }
            AccountIdentityAuthorityIssuerClientError::ReplayDetected => Self::ReplayDetected,
            AccountIdentityAuthorityIssuerClientError::Producer(_) => Self::Producer,
            _ => Self::Unavailable,
        }
    }
}

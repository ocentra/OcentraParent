use super::super::{AccountIdentityAuthorityIssuerClientError, AccountIssuerRepositoryError};

pub(super) fn map(
    error: &AccountIdentityAuthorityIssuerClientError,
) -> Option<AccountIssuerRepositoryError> {
    match error {
        AccountIdentityAuthorityIssuerClientError::ReservationUnavailable => {
            Some(AccountIssuerRepositoryError::ReservationUnavailable)
        }
        AccountIdentityAuthorityIssuerClientError::ReservationExpired => {
            Some(AccountIssuerRepositoryError::ReservationExpired)
        }
        AccountIdentityAuthorityIssuerClientError::ManualRequired => {
            Some(AccountIssuerRepositoryError::ManualRequired)
        }
        AccountIdentityAuthorityIssuerClientError::Producer(_) => {
            Some(AccountIssuerRepositoryError::Producer)
        }
        AccountIdentityAuthorityIssuerClientError::Unavailable
        | AccountIdentityAuthorityIssuerClientError::ClockUnavailable => {
            Some(AccountIssuerRepositoryError::Unavailable)
        }
        _ => None,
    }
}

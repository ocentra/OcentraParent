#[path = "repository_error_common.rs"]
mod common;
#[path = "repository_error_reservation.rs"]
mod reservation;

use super::{AccountIdentityAuthorityIssuerClientError, AccountIssuerRepositoryError};

impl From<AccountIdentityAuthorityIssuerClientError> for AccountIssuerRepositoryError {
    fn from(error: AccountIdentityAuthorityIssuerClientError) -> Self {
        common::map(&error)
            .or_else(|| reservation::map(&error))
            .unwrap_or(Self::Unavailable)
    }
}

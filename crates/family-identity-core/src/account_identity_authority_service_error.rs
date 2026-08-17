use super::{AccountIdentityAuthorityRepositoryError, AccountIdentityAuthorityServiceError};
use crate::account_identity_authority::AccountIdentityCurrentBindingReadError;

impl From<AccountIdentityCurrentBindingReadError<AccountIdentityAuthorityRepositoryError>>
    for AccountIdentityAuthorityServiceError
{
    fn from(
        error: AccountIdentityCurrentBindingReadError<AccountIdentityAuthorityRepositoryError>,
    ) -> Self {
        match error {
            AccountIdentityCurrentBindingReadError::Repository(error) => Self::Repository(error),
            AccountIdentityCurrentBindingReadError::Missing => Self::Missing,
            AccountIdentityCurrentBindingReadError::ProviderMismatch
            | AccountIdentityCurrentBindingReadError::ProviderSubjectMismatch
            | AccountIdentityCurrentBindingReadError::InactiveProviderMapping
            | AccountIdentityCurrentBindingReadError::MappingAccountMismatch
            | AccountIdentityCurrentBindingReadError::MemberAuthorityInvalid
            | AccountIdentityCurrentBindingReadError::SessionInvalid
            | AccountIdentityCurrentBindingReadError::SupportReceiptInvalid => {
                Self::InvalidAuthority
            }
        }
    }
}

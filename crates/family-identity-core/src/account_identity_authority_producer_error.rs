use std::fmt;

use crate::account_identity_authority_repository::AccountIdentityAuthorityServiceError;

#[derive(Debug)]
pub enum AccountIdentityAuthorityProducerError {
    Authority(AccountIdentityAuthorityServiceError),
    SignerCustodyUnavailable,
    VerificationKeyUnavailable,
    SignatureInvalid,
    InvalidWire,
    AuthorityExpired,
}

impl fmt::Display for AccountIdentityAuthorityProducerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for AccountIdentityAuthorityProducerError {}

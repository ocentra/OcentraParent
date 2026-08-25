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
        let label = match self {
            Self::Authority(_) => "authority_unavailable",
            Self::SignerCustodyUnavailable => "signer_custody_unavailable",
            Self::VerificationKeyUnavailable => "verification_key_unavailable",
            Self::SignatureInvalid => "signature_invalid",
            Self::InvalidWire => "invalid_wire",
            Self::AuthorityExpired => "authority_expired",
        };
        formatter.write_str(label)
    }
}

impl std::error::Error for AccountIdentityAuthorityProducerError {}

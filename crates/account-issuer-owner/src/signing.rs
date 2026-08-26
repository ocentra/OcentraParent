//! Protected signing-capability boundary.
//!
//! The owner never accepts a caller-implemented signer. A later broker or
//! Windows custody adapter supplies an opaque capability carrying a signed
//! request proof; this module binds that proof to the exact request bytes and
//! lets the family producer perform the fixed P-256 verification.

use ocentra_family_identity_core::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Transport,
};
use ocentra_protected_capability_custody_protocol::account_issuer_contract::ProtectedAccountIssuerSignerCapability;
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_ISSUER_SIGNING_ERROR;
use ring::digest::{digest, SHA256};

#[derive(Debug)]
pub enum AccountIssuerSigningError {
    OwnerUnavailable,
    Rejected,
}

impl std::fmt::Display for AccountIssuerSigningError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(ACCOUNT_ISSUER_SIGNING_ERROR)
    }
}

impl std::error::Error for AccountIssuerSigningError {}

pub(crate) fn finalize_with_protected_capability(
    request: AccountIdentityAuthorityProducerV2Request,
    capability: &ProtectedAccountIssuerSignerCapability,
) -> Result<AccountIdentityAuthorityProducerV2Transport, AccountIssuerSigningError> {
    let request_digest = digest(&SHA256, request.signing_bytes());
    if request_digest.as_ref() != capability.request_digest() {
        return Err(AccountIssuerSigningError::Rejected);
    }
    let signature: [u8; 64] = (*capability.signature())
        .try_into()
        .map_err(|_| AccountIssuerSigningError::Rejected)?;
    request
        .finalize(signature)
        .map_err(|_| AccountIssuerSigningError::Rejected)
}

pub(crate) fn fail_closed() -> AccountIssuerSigningError {
    AccountIssuerSigningError::OwnerUnavailable
}

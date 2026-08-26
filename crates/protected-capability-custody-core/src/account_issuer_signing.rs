//! Private AccountIssuer-to-CNG adapter.
//!
//! The FFI dependency is intentionally confined here. Its unsafe mechanics
//! receive only the protocol-owned prepared request built from the family
//! request; no FFI handle, key, digest, or generic byte-signer crosses this
//! module's crate boundary.

#[cfg(windows)]
use ocentra_family_identity_core::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;
#[cfg(windows)]
use ocentra_protected_capability_custody_protocol::account_issuer_contract::PreparedAccountIssuerV2Request;

use crate::account_issuer::{AccountIssuerP256SignerError, AccountIssuerSignerCapability};

#[cfg(windows)]
pub(crate) struct BoundAccountIssuerP256Key {
    key: ocentra_protected_capability_custody_windows_ffi::BoundAccountIssuerP256Key,
}

#[cfg(windows)]
impl BoundAccountIssuerP256Key {
    pub(crate) fn from_ffi(
        key: ocentra_protected_capability_custody_windows_ffi::BoundAccountIssuerP256Key,
    ) -> Self {
        Self { key }
    }

    pub(crate) fn sign_request(
        &self,
        request: &AccountIdentityAuthorityProducerV2Request,
    ) -> Result<AccountIssuerSignerCapability, AccountIssuerP256SignerError> {
        let prepared = PreparedAccountIssuerV2Request::from_owner_request(
            request.signing_bytes(),
            request.binding().clone(),
        )
        .map_err(|_| AccountIssuerP256SignerError::Rejected)?;
        let capability = self
            .key
            .sign_prepared_account_issuer_v2(prepared)
            .map_err(|_| AccountIssuerP256SignerError::Rejected)?;
        Ok(AccountIssuerSignerCapability::from_protocol(&capability))
    }
}

//! Private AccountIssuer-to-CNG adapter.
//!
//! The FFI dependency is intentionally confined here. Its unsafe mechanics
//! receive only the family-owned non-mintable request; no FFI handle, key,
//! digest, or generic byte-signer crosses this module's crate boundary.

use crate::account_issuer::{AccountIssuerP256SignerError, AccountIssuerSignerCapability};
#[cfg(windows)]
use ocentra_family_identity_core::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;

#[cfg(windows)]
pub(crate) struct BoundAccountIssuerP256Key {
    key: ocentra_protected_capability_custody_windows_ffi::BoundAccountIssuerP256Key,
}

#[cfg(windows)]
impl BoundAccountIssuerP256Key {
    pub(crate) fn sign_request(
        &self,
        request: &AccountIdentityAuthorityProducerV2Request,
    ) -> Result<AccountIssuerSignerCapability, AccountIssuerP256SignerError> {
        let signature = self
            .key
            .sign_account_issuer_v2_request(request)
            .map_err(|_signing_error| AccountIssuerP256SignerError::Rejected)?;
        Ok(AccountIssuerSignerCapability::from_signed_request(
            request,
            *signature.as_bytes(),
        ))
    }
}

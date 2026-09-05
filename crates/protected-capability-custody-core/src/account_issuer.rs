//! Account-owned, domain-bound P-256 signing seam.
//!
//! This module is the only safe-crate adapter allowed to consume the Windows
//! CNG boundary. It accepts the family-owned canonical request directly and
//! returns a core-owned capability. There is no generic signer trait, raw
//! `sign(bytes)` operation, caller-selected key, or exported platform handle.

use ocentra_family_identity_core::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::account_issuer_signing;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum AccountIssuerP256SignerError {
    #[error("account issuer protected signer deployment is required")]
    DeploymentRequired,
    #[error("account issuer protected signer rejected the request")]
    Rejected,
}

/// A core-owned proof that the protected AccountIssuer signer produced a
/// signature for the exact family request digest. The capability is not
/// cloneable and exposes only fixed read-only fields needed by the Account
/// owner to verify and durably record the transport.
pub struct AccountIssuerSignerCapability {
    request_digest: [u8; 32],
    signature: [u8; 64],
}

impl AccountIssuerSignerCapability {
    pub fn into_signature_for(
        self,
        request: &AccountIdentityAuthorityProducerV2Request,
    ) -> Result<[u8; 64], AccountIssuerP256SignerError> {
        let request_digest: [u8; 32] = Sha256::digest(request.signing_bytes()).into();
        if request_digest != self.request_digest {
            return Err(AccountIssuerP256SignerError::Rejected);
        }
        Ok(self.signature)
    }

    pub(crate) fn from_signed_request(
        request: &AccountIdentityAuthorityProducerV2Request,
        signature: [u8; 64],
    ) -> Self {
        Self {
            request_digest: Sha256::digest(request.signing_bytes()).into(),
            signature,
        }
    }
}

/// Account-specific protected signer facade.
pub struct AccountIssuerP256Signer {
    #[cfg(windows)]
    bound: account_issuer_signing::BoundAccountIssuerP256Key,
}

impl AccountIssuerP256Signer {
    /// Protected enrollment and service-bound key mounting are installer-owned
    /// prerequisites. Until that composition exists, startup fails closed.
    pub fn mount_account_owned() -> Result<Self, AccountIssuerP256SignerError> {
        Err(AccountIssuerP256SignerError::DeploymentRequired)
    }

    /// Sign only the exact family-owned AccountIssuer v2 request. The request
    /// is borrowed so its owner can retain it for final signature validation
    /// and atomic durable recording.
    pub fn sign_request(
        &self,
        request: &AccountIdentityAuthorityProducerV2Request,
    ) -> Result<AccountIssuerSignerCapability, AccountIssuerP256SignerError> {
        #[cfg(windows)]
        {
            self.bound.sign_request(request)
        }
        #[cfg(not(windows))]
        {
            let _ = request;
            Err(AccountIssuerP256SignerError::DeploymentRequired)
        }
    }
}

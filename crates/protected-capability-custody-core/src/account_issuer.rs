//! Account-owned, domain-bound P-256 signing seam.
//!
//! This module is the only safe-crate adapter allowed to consume the Windows
//! CNG boundary. It accepts the family-owned canonical request directly and
//! returns a core-owned capability. There is no generic signer trait, raw
//! `sign(bytes)` operation, caller-selected key, or exported platform handle.

use ocentra_family_identity_core::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;
use sha2::{Digest, Sha256};
use thiserror::Error;

use ocentra_protected_capability_custody_protocol::account_issuer_contract::ProtectedAccountIssuerSignerCapability;

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
    pub fn matches_request(&self, request: &AccountIdentityAuthorityProducerV2Request) -> bool {
        let request_digest: [u8; 32] = Sha256::digest(request.signing_bytes()).into();
        request_digest == self.request_digest
    }

    pub fn signature(&self) -> &[u8; 64] {
        &self.signature
    }

    pub(crate) fn from_protocol(capability: &ProtectedAccountIssuerSignerCapability) -> Self {
        Self {
            request_digest: *capability.request_digest(),
            signature: *capability.signature(),
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
            return self.bound.sign_request(request);
        }
        #[cfg(not(windows))]
        {
            let _ = request;
            Err(AccountIssuerP256SignerError::DeploymentRequired)
        }
    }

    #[cfg(windows)]
    pub(crate) fn from_bound_key(bound: account_issuer_signing::BoundAccountIssuerP256Key) -> Self {
        Self { bound }
    }
}

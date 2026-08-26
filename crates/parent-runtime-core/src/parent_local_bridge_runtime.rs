#![forbid(unsafe_code)]

//! Parent desktop composition gate for the Account-owned local bridge.
//!
//! The protected fixed store and a provider-verified identity adapter are both
//! required before the parent runtime may obtain a bridge capability. Neither
//! may be replaced by request fields, Origin, loopback provenance, or an
//! environment-selected database path.

use ocentra_family_identity_core::account_identity_authority_repository::AccountIdentityAuthorityService;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParentLocalBridgeRuntimeError {
    AccountOwnerUnavailable,
    VerifiedProviderAdapterUnavailable,
}

impl std::fmt::Display for ParentLocalBridgeRuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("parent-local bridge Account authorization unavailable")
    }
}

impl std::error::Error for ParentLocalBridgeRuntimeError {}

pub(crate) fn require_authenticated_transport_owner() -> Result<(), ParentLocalBridgeRuntimeError> {
    let _account_owner = AccountIdentityAuthorityService::mount_account_owned()
        .map_err(|_| ParentLocalBridgeRuntimeError::AccountOwnerUnavailable)?;

    // Account currentness accepts only a non-mintable
    // VerifiedAccountIdentityAuthority. The shipped Rust parent runtime has no
    // provider verifier that can produce it, so opening a socket here would
    // bypass the owner-issued capability and must remain fail-closed.
    Err(ParentLocalBridgeRuntimeError::VerifiedProviderAdapterUnavailable)
}

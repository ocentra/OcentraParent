#![forbid(unsafe_code)]

//! Parent desktop composition gate for the Account-owned local bridge.
//!
//! The protected fixed store and a provider-verified identity adapter are both
//! required before the parent runtime may obtain a bridge capability. Neither
//! may be replaced by request fields, Origin, loopback provenance, or an
//! environment-selected database path.

use ocentra_family_identity_core::account_identity_authority_repository::AccountIdentityAuthorityService;
use ocentra_family_identity_core::session_lifecycle_custody::parent_local_bridge::IssuedParentLocalBridgeSession;

#[path = "parent_local_bridge_runtime_error.rs"]
mod runtime_error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParentLocalBridgeRuntimeError {
    AccountOwnerUnavailable,
    AccountOwnerInvalidGeneration,
    AccountOwnerInvalidStoredAuthority,
    AccountOwnerCurrentnessConflict,
    VerifiedProviderAdapterUnavailable,
}

impl std::fmt::Display for ParentLocalBridgeRuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::AccountOwnerUnavailable => {
                "parent-local bridge Account owner repository is unavailable"
            }
            Self::AccountOwnerInvalidGeneration => {
                "parent-local bridge Account owner generation is invalid"
            }
            Self::AccountOwnerInvalidStoredAuthority => {
                "parent-local bridge Account owner authority is invalid"
            }
            Self::AccountOwnerCurrentnessConflict => {
                "parent-local bridge Account owner currentness conflicts"
            }
            Self::VerifiedProviderAdapterUnavailable => {
                "parent-local bridge verified provider adapter is unavailable"
            }
        })
    }
}

impl std::error::Error for ParentLocalBridgeRuntimeError {}

pub(crate) fn require_authenticated_transport_owner(
) -> Result<IssuedParentLocalBridgeSession, ParentLocalBridgeRuntimeError> {
    let _account_owner = AccountIdentityAuthorityService::mount_account_owned()
        .map_err(|source| runtime_error::from_account_repository(&source))?;

    // Account currentness accepts only a non-mintable
    // VerifiedAccountIdentityAuthority. The shipped Rust parent runtime has no
    // provider verifier that can produce it, so opening a socket here would
    // bypass the owner-issued capability and must remain fail-closed.
    Err(ParentLocalBridgeRuntimeError::VerifiedProviderAdapterUnavailable)
}

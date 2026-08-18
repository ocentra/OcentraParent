#![forbid(unsafe_code)]

//! Device-bound entitlement authority.
//!
//! This module provides the crate-private verifier composition for a future
//! entitlement owner. A signed wire value is never authority until a concrete
//! owner composition and local checks succeed. No public unlock, capability
//! selector, or final action-consumption route is exposed in this packet.

use std::sync::Arc;

use ocentra_family_identity_core::{
    account_identity_authority::VerifiedAccountIdentityAuthority,
    device_trust_current_binding::CurrentChildDeviceTrustBinding,
    device_trust_lifecycle::DeviceTrustLifecycleState,
};

use crate::{
    entitlement_snapshot::SignedEntitlementSnapshot,
    entitlement_snapshot_cache::revocation_state::EntitlementRevocationStateStore,
};

#[path = "entitlement_snapshot_authority_currentness.rs"]
mod currentness;
#[path = "entitlement_snapshot_authority_ports.rs"]
pub(crate) mod ports;
#[path = "entitlement_snapshot_authority_revocation.rs"]
mod revocation;
#[path = "entitlement_snapshot_authority_verifier.rs"]
pub(crate) mod verifier;
#[path = "entitlement_snapshot_authority_verifier_binding.rs"]
mod verifier_binding;
#[path = "entitlement_snapshot_authority_verifier_currentness.rs"]
mod verifier_currentness;
#[path = "entitlement_snapshot_authority_verifier_request.rs"]
mod verifier_request;
#[path = "entitlement_snapshot_authority_verifier_signature.rs"]
mod verifier_signature;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementSnapshotVerificationFailure {
    InvalidSnapshotShape,
    MissingSignature,
    InvalidSignature,
    WeakVerificationKey,
    SignatureKeyMismatch,
    WrongAccount,
    WrongHousehold,
    WrongDevice,
    WrongPackageBuild,
    NotYetValid,
    Expired,
    Stale,
    Revoked,
    CurrentAuthorityUnavailable,
    AuthorityUnavailable,
    RevocationStateCorrupt,
    StaleAuthorityState,
}

pub struct EntitlementSnapshotAuthority {
    pub(crate) key_provider: Arc<dyn ports::EntitlementSnapshotVerificationKeyProvider>,
    pub(crate) installed_package: Arc<dyn ports::EntitlementInstalledPackageAuthority>,
    pub(crate) currentness: Arc<dyn ports::EntitlementCurrentnessAuthority>,
    pub(crate) revocation_state: EntitlementRevocationStateStore,
}

impl EntitlementSnapshotAuthority {
    /// Internal owner-composition constructor. It is deliberately not public:
    /// accepting arbitrary caller-provided ports would let a command mount a
    /// self-authored key/package/currentness authority. A future concrete
    /// owner repository composer in this crate may call this after it has
    /// obtained the private port implementations from their owners.
    pub(crate) fn open(
        revocation_state_path: impl Into<std::path::PathBuf>,
        key_provider: Arc<dyn ports::EntitlementSnapshotVerificationKeyProvider>,
        installed_package: Arc<dyn ports::EntitlementInstalledPackageAuthority>,
        currentness: Arc<dyn ports::EntitlementCurrentnessAuthority>,
    ) -> Result<Self, EntitlementSnapshotVerificationFailure> {
        let revocation_state = EntitlementRevocationStateStore::open(revocation_state_path)
            .map_err(revocation::map_cache_error)?;
        Ok(Self {
            key_provider,
            installed_package,
            currentness,
            revocation_state,
        })
    }

    /// Future owner-composed verifier entry point. It accepts only a signed
    /// transport value already obtained by the owner and returns an opaque
    /// verification result; no public unlock or capability handoff is exposed.
    pub(crate) fn verify_current_account_and_device(
        &self,
        snapshot: &SignedEntitlementSnapshot,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<verifier::VerifiedEntitlementSnapshot, EntitlementSnapshotVerificationFailure> {
        self.currentness
            .validate_current_identity(account_authority, device_binding)?;
        if device_binding.state() != DeviceTrustLifecycleState::Trusted {
            return Err(EntitlementSnapshotVerificationFailure::CurrentAuthorityUnavailable);
        }
        let package_identity = self
            .installed_package
            .current_package_identity(account_authority, device_binding)?;
        let request =
            verifier_request::EntitlementSnapshotVerificationRequest::from_current_account_authority(
                account_authority,
                device_binding,
                package_identity.package_build_ref().clone(),
                package_identity.release_channel(),
            )?;
        if request.household_ref.as_str() != device_binding.family_id() {
            return Err(EntitlementSnapshotVerificationFailure::WrongHousehold);
        }
        if request.trusted_device_ref.as_str() != device_binding.child_device_id() {
            return Err(EntitlementSnapshotVerificationFailure::WrongDevice);
        }
        verifier::verify(self, snapshot, &request)
    }
}

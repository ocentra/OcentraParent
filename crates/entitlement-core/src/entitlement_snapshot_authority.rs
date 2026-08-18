#![forbid(unsafe_code)]

//! Device-bound entitlement authority.
//!
//! This module owns the durable snapshot handoff and composes focused ports,
//! verifier, currentness, and revocation modules. A signed wire value is never
//! authority until a concrete owner composition and local checks succeed.

use std::sync::Arc;

use ocentra_family_identity_core::{
    account_identity_authority::VerifiedAccountIdentityAuthority,
    device_trust_current_binding::CurrentChildDeviceTrustBinding,
    device_trust_lifecycle::DeviceTrustLifecycleState,
};

use crate::{
    entitlement_access::{
        evaluate_entitlement_capability, EntitlementCapability, EntitlementCapabilityAccessState,
        EntitlementCapabilityGrant, EntitlementCapabilityInput, EntitlementCapabilityScope,
        OfflineGraceState,
    },
    entitlement_snapshot::SignedEntitlementSnapshot,
    entitlement_snapshot_cache::revocation_state::EntitlementRevocationStateStore,
    entitlement_snapshot_cache::{EntitlementSnapshotCache, SignedEntitlementRevocationUpdate},
};

#[path = "entitlement_snapshot_authority_currentness.rs"]
mod currentness;
#[path = "entitlement_snapshot_authority_ports.rs"]
pub(crate) mod ports;
#[path = "entitlement_snapshot_authority_revocation.rs"]
mod revocation;
#[path = "entitlement_snapshot_authority_verifier.rs"]
mod verifier;
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
    CapabilityNotEntitled,
    CapabilityGateRejected,
    CapabilityGrantStale,
    GraceRestricted,
}

pub struct EntitlementSnapshotAuthority {
    pub(crate) key_provider: Arc<dyn ports::EntitlementSnapshotVerificationKeyProvider>,
    pub(crate) installed_package: Arc<dyn ports::EntitlementInstalledPackageAuthority>,
    pub(crate) currentness: Arc<dyn ports::EntitlementCurrentnessAuthority>,
    pub(crate) snapshot_cache: EntitlementSnapshotCache,
    pub(crate) revocation_state: EntitlementRevocationStateStore,
}

impl EntitlementSnapshotAuthority {
    /// Internal owner-composition constructor. It is deliberately not public:
    /// accepting arbitrary caller-provided ports would let a command mount a
    /// self-authored key/package/currentness authority. A future concrete
    /// owner repository composer in this crate may call this after it has
    /// obtained the private port implementations from their owners.
    pub(crate) fn open(
        snapshot_path: impl Into<std::path::PathBuf>,
        revocation_state_path: impl Into<std::path::PathBuf>,
        key_provider: Arc<dyn ports::EntitlementSnapshotVerificationKeyProvider>,
        installed_package: Arc<dyn ports::EntitlementInstalledPackageAuthority>,
        currentness: Arc<dyn ports::EntitlementCurrentnessAuthority>,
    ) -> Result<Self, EntitlementSnapshotVerificationFailure> {
        let snapshot_cache =
            EntitlementSnapshotCache::open(snapshot_path).map_err(revocation::map_cache_error)?;
        let revocation_state = EntitlementRevocationStateStore::open(revocation_state_path)
            .map_err(revocation::map_cache_error)?;
        Ok(Self {
            key_provider,
            installed_package,
            currentness,
            snapshot_cache,
            revocation_state,
        })
    }

    pub fn open_manual_required(
        snapshot_path: impl Into<std::path::PathBuf>,
        revocation_state_path: impl Into<std::path::PathBuf>,
    ) -> Result<Self, EntitlementSnapshotVerificationFailure> {
        Self::open(
            snapshot_path,
            revocation_state_path,
            Arc::new(ports::ManualRequiredEntitlementSnapshotVerificationKeyProvider),
            Arc::new(ports::ManualRequiredEntitlementInstalledPackageAuthority),
            Arc::new(ports::ManualRequiredEntitlementCurrentnessAuthority),
        )
    }

    fn verify_current_account_and_device(
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

    pub fn unlock_capability(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        capability: EntitlementCapability,
    ) -> Result<EntitlementCapabilityGrant, EntitlementSnapshotVerificationFailure> {
        let snapshot = self
            .snapshot_cache
            .read()
            .map_err(revocation::map_cache_error)?
            .ok_or(EntitlementSnapshotVerificationFailure::AuthorityUnavailable)?;
        let verified =
            self.verify_current_account_and_device(&snapshot, account_authority, device_binding)?;
        if !verified.enables(capability) {
            return Err(EntitlementSnapshotVerificationFailure::CapabilityNotEntitled);
        }
        self.evaluate_current_gate(
            capability,
            account_authority,
            device_binding,
            verified.context(),
        )?;
        Ok(EntitlementCapabilityGrant::from_verified(
            capability,
            verified.snapshot_id().clone(),
            verified.authority_generation(),
        ))
    }

    /// Consume a non-cloneable grant only after re-reading and re-verifying the
    /// durable snapshot, current revocation generation, package identity,
    /// account/device binding, and owner-provided billing/policy states.
    pub fn consume_capability_grant(
        &self,
        grant: EntitlementCapabilityGrant,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<EntitlementCapability, EntitlementSnapshotVerificationFailure> {
        let capability = grant.capability();
        let snapshot = self
            .snapshot_cache
            .read()
            .map_err(revocation::map_cache_error)?
            .ok_or(EntitlementSnapshotVerificationFailure::AuthorityUnavailable)?;
        let verified =
            self.verify_current_account_and_device(&snapshot, account_authority, device_binding)?;
        if verified.snapshot_id() != grant.snapshot_id()
            || verified.authority_generation() != grant.authority_generation()
        {
            return Err(EntitlementSnapshotVerificationFailure::CapabilityGrantStale);
        }
        if !verified.enables(capability) {
            return Err(EntitlementSnapshotVerificationFailure::CapabilityNotEntitled);
        }
        self.evaluate_current_gate(
            capability,
            account_authority,
            device_binding,
            verified.context(),
        )?;
        Ok(capability)
    }

    pub fn apply_revocation_update(
        &self,
        update: &SignedEntitlementRevocationUpdate,
    ) -> Result<(), EntitlementSnapshotVerificationFailure> {
        revocation::verify_revocation_update(update, self.key_provider.as_ref())?;
        self.currentness
            .validate_revocation_generation(update.authority_generation)?;
        self.revocation_state
            .replace_signed(update)
            .map_err(revocation::map_cache_error)
    }

    fn evaluate_current_gate(
        &self,
        capability: EntitlementCapability,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        snapshot_context: crate::entitlement_snapshot::EntitlementSnapshotContext,
    ) -> Result<(), EntitlementSnapshotVerificationFailure> {
        let subscription_state = self
            .currentness
            .subscription_state(account_authority, device_binding)?;
        let offline_grace_state = self
            .currentness
            .offline_grace_state(account_authority, device_binding)?;
        let family_setup_state = self
            .currentness
            .family_setup_state(account_authority, device_binding)?;
        let policy_state = self
            .currentness
            .policy_state(account_authority, device_binding)?;
        if snapshot_context.freshness_state
            == crate::entitlement_snapshot_values::EntitlementSnapshotFreshnessState::Grace
            && (capability != EntitlementCapability::Tracking
                || offline_grace_state != OfflineGraceState::Active)
        {
            return Err(EntitlementSnapshotVerificationFailure::GraceRestricted);
        }
        let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
            capability,
            subscription_state,
            offline_grace_state,
            family_setup_state,
            policy_state,
            capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
            snapshot_context,
        });
        if decision.access_state != EntitlementCapabilityAccessState::Allowed {
            return Err(EntitlementSnapshotVerificationFailure::CapabilityGateRejected);
        }
        Ok(())
    }
}

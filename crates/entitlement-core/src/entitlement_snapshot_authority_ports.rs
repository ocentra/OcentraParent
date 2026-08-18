#![forbid(unsafe_code)]

//! Crate-private authority ports used by the entitlement verifier.
//!
//! These are intentionally not exported as a public dependency-injection
//! surface. A future owner repository composer must construct the opaque
//! values inside this crate after authenticating its durable owners; until
//! then the only public authority startup path is manual-required.

use ed25519_dalek::VerifyingKey;
use ocentra_family_identity_core::{
    account_identity_authority::VerifiedAccountIdentityAuthority,
    device_trust_current_binding::CurrentChildDeviceTrustBinding,
};

use crate::{
    entitlement_access::{
        EntitlementPolicyState, FamilySetupState, OfflineGraceState, SubscriptionState,
    },
    entitlement_snapshot::SignedEntitlementSnapshot,
    entitlement_snapshot_cache::SignedEntitlementRevocationUpdate,
    entitlement_snapshot_values::{
        EntitlementPackageBuildRef, EntitlementSignatureKeyId, EntitlementSnapshotFreshnessState,
        EntitlementSnapshotReleaseChannel,
    },
};

use super::EntitlementSnapshotVerificationFailure;

#[path = "entitlement_snapshot_authority_currentness_ports.rs"]
mod currentness;

/// Pinned public-key custody for issuer signature verification.
///
/// Implementations must resolve only an operator-pinned key set. The owner
/// still binds the returned key to the signed key identifier and rejects weak
/// keys before verification.
pub trait EntitlementSnapshotVerificationKeyProvider: Send + Sync {
    fn verifying_key(
        &self,
        issuer_key_id: &EntitlementSignatureKeyId,
    ) -> Result<VerifyingKey, EntitlementSnapshotVerificationFailure>;
}

#[derive(Debug, Default)]
pub struct ManualRequiredEntitlementSnapshotVerificationKeyProvider;

impl EntitlementSnapshotVerificationKeyProvider
    for ManualRequiredEntitlementSnapshotVerificationKeyProvider
{
    fn verifying_key(
        &self,
        _issuer_key_id: &EntitlementSignatureKeyId,
    ) -> Result<VerifyingKey, EntitlementSnapshotVerificationFailure> {
        Err(EntitlementSnapshotVerificationFailure::AuthorityUnavailable)
    }
}

/// One package identity from the installed-package owner. The private fields
/// prevent a command caller from pairing an arbitrary build with a release
/// channel; only the owner-side constructor can produce the value.
pub struct EntitlementInstalledPackageIdentity {
    package_build_ref: EntitlementPackageBuildRef,
    release_channel: EntitlementSnapshotReleaseChannel,
}

impl std::fmt::Debug for EntitlementInstalledPackageIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EntitlementInstalledPackageIdentity")
            .field("authority", &"opaque")
            .finish()
    }
}

impl EntitlementInstalledPackageIdentity {
    pub(crate) fn from_owner_current_install(
        package_build_ref: EntitlementPackageBuildRef,
        release_channel: EntitlementSnapshotReleaseChannel,
    ) -> Self {
        Self {
            package_build_ref,
            release_channel,
        }
    }

    pub(crate) fn package_build_ref(&self) -> &EntitlementPackageBuildRef {
        &self.package_build_ref
    }

    pub(crate) fn release_channel(&self) -> EntitlementSnapshotReleaseChannel {
        self.release_channel
    }
}

/// Supplies the currently installed package identity from the package/install
/// owner. A command caller cannot supply a package-build string.
pub trait EntitlementInstalledPackageAuthority: Send + Sync {
    fn current_package_identity(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<EntitlementInstalledPackageIdentity, EntitlementSnapshotVerificationFailure>;
}

#[derive(Debug, Default)]
pub struct ManualRequiredEntitlementInstalledPackageAuthority;

impl EntitlementInstalledPackageAuthority for ManualRequiredEntitlementInstalledPackageAuthority {
    fn current_package_identity(
        &self,
        _account_authority: &VerifiedAccountIdentityAuthority,
        _device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<EntitlementInstalledPackageIdentity, EntitlementSnapshotVerificationFailure> {
        Err(EntitlementSnapshotVerificationFailure::AuthorityUnavailable)
    }
}

/// Reads current billing/account/policy state from the owning authorities.
/// Every method is queried for both grant preparation and final consumption;
/// missing ownership fails closed instead of manufacturing a positive state.
pub trait EntitlementCurrentnessAuthority: Send + Sync {
    /// Check the signed generation against a monotonic owner-held fence that
    /// survives rollback of both local snapshot and revocation files.
    fn validate_revocation_generation(
        &self,
        authority_generation: u64,
    ) -> Result<(), EntitlementSnapshotVerificationFailure>;

    /// Re-resolve the live account session/support receipt and Device Trust
    /// generations. Borrowing an opaque authority is not proof that it is
    /// still current; this method must consult the durable owners again and
    /// evaluate session expiry through the same owner-controlled trusted-time
    /// boundary used by `evaluate_snapshot_freshness`.
    fn validate_current_identity(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<(), EntitlementSnapshotVerificationFailure>;

    /// Evaluate the signed active window through the owner-controlled trusted
    /// time/currentness boundary. The implementation must re-read its
    /// restart-safe monotonic generation fence on every call, reject a
    /// snapshot/update generation or cursor rollback, and use the owner's
    /// configured maximum grace interval rather than a crate-wide constant.
    ///
    /// The boundary owns the clock decision: callers cannot inject a clock or
    /// turn a signed `grace_until` value into entitlement by themselves. If
    /// the owner has no trusted time, configured grace policy, or rollback
    /// fence, it must return `AuthorityUnavailable`.
    fn evaluate_snapshot_freshness(
        &self,
        snapshot: &SignedEntitlementSnapshot,
        revocation_update: &SignedEntitlementRevocationUpdate,
    ) -> Result<EntitlementSnapshotFreshnessState, EntitlementSnapshotVerificationFailure>;

    fn subscription_state(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<SubscriptionState, EntitlementSnapshotVerificationFailure>;

    fn offline_grace_state(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<OfflineGraceState, EntitlementSnapshotVerificationFailure>;

    fn family_setup_state(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<FamilySetupState, EntitlementSnapshotVerificationFailure>;

    fn policy_state(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<EntitlementPolicyState, EntitlementSnapshotVerificationFailure>;
}

#[derive(Debug, Default)]
pub struct ManualRequiredEntitlementCurrentnessAuthority;

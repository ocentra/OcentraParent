#![forbid(unsafe_code)]

//! Child-runtime consumer for the entitlement license authority.
//!
//! The public startup route is manual-required until a concrete owner
//! repository composition mounts key, package, billing, and currentness
//! custody. The consumer never accepts caller snapshots, package strings,
//! positive gate facts, or serialized grants.

use std::path::PathBuf;

use ocentra_entitlement_core::entitlement_access::{
    EntitlementCapability, EntitlementCapabilityGrant,
};
use ocentra_entitlement_core::entitlement_snapshot_authority::{
    EntitlementSnapshotAuthority, EntitlementSnapshotVerificationFailure,
};
use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use ocentra_family_identity_core::device_trust_current_binding::CurrentChildDeviceTrustBinding;

pub struct ChildRuntimeEntitlementLicenseStore {
    authority: EntitlementSnapshotAuthority,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildRuntimeEntitlementLicenseError {
    Authority(EntitlementSnapshotVerificationFailure),
}

impl ChildRuntimeEntitlementLicenseStore {
    pub fn open_manual_required(
        snapshot_path: impl Into<PathBuf>,
        revocation_state_path: impl Into<PathBuf>,
    ) -> Result<Self, ChildRuntimeEntitlementLicenseError> {
        let authority = EntitlementSnapshotAuthority::open_manual_required(
            snapshot_path,
            revocation_state_path,
        )
        .map_err(ChildRuntimeEntitlementLicenseError::Authority)?;
        Ok(Self { authority })
    }

    /// Prepare a short-lived, non-cloneable handoff after all currentness and
    /// entitlement gates pass. The handoff is not itself durable authority;
    /// callers must consume it by value through `consume_capability_grant`.
    pub fn authorize_capability(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        capability: EntitlementCapability,
    ) -> Result<EntitlementCapabilityGrant, ChildRuntimeEntitlementLicenseError> {
        self.authority
            .unlock_capability(account_authority, device_binding, capability)
            .map_err(ChildRuntimeEntitlementLicenseError::Authority)
    }

    /// Final gated-action handoff. The authority revalidates expiry,
    /// revocation generation, installed build, account/device binding, and
    /// current billing/policy state before returning the capability by value.
    pub fn consume_capability_grant(
        &self,
        grant: EntitlementCapabilityGrant,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<EntitlementCapability, ChildRuntimeEntitlementLicenseError> {
        self.authority
            .consume_capability_grant(grant, account_authority, device_binding)
            .map_err(ChildRuntimeEntitlementLicenseError::Authority)
    }
}

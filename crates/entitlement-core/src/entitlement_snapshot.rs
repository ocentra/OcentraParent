#![forbid(unsafe_code)]

//! Signed entitlement snapshot derivation and verification context.

use crate::entitlement_access::EntitlementCapability;
use crate::entitlement_snapshot_values::{
    EntitlementAccountRef, EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState,
    EntitlementHouseholdRef, EntitlementPackageBuildRef, EntitlementPackageBuildState,
    EntitlementRevocationCursor, EntitlementSignatureKeyId, EntitlementSnapshotBindingState,
    EntitlementSnapshotFreshnessState, EntitlementSnapshotId, EntitlementSnapshotPlanTier,
    EntitlementSnapshotSignatureState, EntitlementTrustedDeviceRef,
};
use serde::{Deserialize, Serialize};

mod entitlement_snapshot_authority;
mod entitlement_snapshot_context;
mod entitlement_snapshot_validation;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementSnapshotFeatureFlag {
    pub capability: EntitlementCapability,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementSnapshotLimitBundle {
    pub child_device_limit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedEntitlementSnapshot {
    schema_version: u16,
    snapshot_id: EntitlementSnapshotId,
    account_ref: EntitlementAccountRef,
    household_ref: EntitlementHouseholdRef,
    trusted_device_ref: EntitlementTrustedDeviceRef,
    plan_tier: EntitlementSnapshotPlanTier,
    feature_flags: Vec<EntitlementSnapshotFeatureFlag>,
    limits: EntitlementSnapshotLimitBundle,
    base_child_device_limit: u32,
    active_referral_credits: u32,
    paid_extra_child_device_seats: u32,
    effective_child_device_limit: u32,
    issued_at: String,
    expires_at: String,
    grace_until: Option<String>,
    livemode: bool,
    revocation_cursor: EntitlementRevocationCursor,
    device_trust_required: bool,
    package_build_ref: EntitlementPackageBuildRef,
    signature_key_id: EntitlementSignatureKeyId,
    signature: String,
}

/// A verified, device-bound snapshot context.
///
/// The state is crate-owned: callers may carry a context returned by the
/// verifier, but cannot deserialize or construct one with trusted states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct EntitlementSnapshotContext {
    pub(crate) signature_state: EntitlementSnapshotSignatureState,
    pub(crate) freshness_state: EntitlementSnapshotFreshnessState,
    pub(crate) household_binding_state: EntitlementSnapshotBindingState,
    pub(crate) device_binding_state: EntitlementSnapshotBindingState,
    pub(crate) device_trust_requirement_state: EntitlementDeviceTrustRequirementState,
    pub(crate) device_trust_state: EntitlementDeviceTrustState,
    pub(crate) package_build_state: EntitlementPackageBuildState,
}

/// Verification output remains opaque until the entitlement owner projects it
/// into a capability context.  In particular, a downstream verifier cannot
/// manufacture a trusted result by deserializing this DTO.
#[derive(Debug, PartialEq, Eq)]
pub struct EntitlementSnapshotVerificationContext {
    pub(crate) signature_state: EntitlementSnapshotSignatureState,
    pub(crate) freshness_state: EntitlementSnapshotFreshnessState,
    pub(crate) household_binding_state: EntitlementSnapshotBindingState,
    pub(crate) device_binding_state: EntitlementSnapshotBindingState,
    pub(crate) device_trust_state: EntitlementDeviceTrustState,
    pub(crate) package_build_state: EntitlementPackageBuildState,
    pub(crate) authority_binding:
        entitlement_snapshot_authority::EntitlementSnapshotAuthorityBinding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntitlementSnapshotVerificationRequest {
    pub account_ref: EntitlementAccountRef,
    pub household_ref: EntitlementHouseholdRef,
    pub trusted_device_ref: EntitlementTrustedDeviceRef,
    pub package_build_ref: EntitlementPackageBuildRef,
    pub observed_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementSnapshotVerificationFailure {
    InvalidSnapshotShape,
    MissingSignature,
    InvalidSignature,
    WrongAccount,
    WrongHousehold,
    WrongDevice,
    WrongPackageBuild,
    Expired,
    TimestampInvalid,
    AuthorityUnavailable,
}

pub trait EntitlementSnapshotAuthorityVerifier {
    fn verify_signature_and_revocation(
        &mut self,
        snapshot: &SignedEntitlementSnapshot,
        request: &EntitlementSnapshotVerificationRequest,
    ) -> Result<EntitlementSnapshotVerificationContext, EntitlementSnapshotVerificationFailure>;
}

#[derive(Debug, Default)]
pub struct UnavailableEntitlementSnapshotAuthorityVerifier;

impl EntitlementSnapshotAuthorityVerifier for UnavailableEntitlementSnapshotAuthorityVerifier {
    fn verify_signature_and_revocation(
        &mut self,
        _snapshot: &SignedEntitlementSnapshot,
        _request: &EntitlementSnapshotVerificationRequest,
    ) -> Result<EntitlementSnapshotVerificationContext, EntitlementSnapshotVerificationFailure>
    {
        Err(EntitlementSnapshotVerificationFailure::AuthorityUnavailable)
    }
}

impl EntitlementSnapshotContext {
    pub(crate) fn unavailable() -> Self {
        Self {
            signature_state: EntitlementSnapshotSignatureState::Missing,
            freshness_state: EntitlementSnapshotFreshnessState::Revoked,
            household_binding_state: EntitlementSnapshotBindingState::Mismatched,
            device_binding_state: EntitlementSnapshotBindingState::Mismatched,
            device_trust_requirement_state: EntitlementDeviceTrustRequirementState::Required,
            device_trust_state: EntitlementDeviceTrustState::Missing,
            package_build_state: EntitlementPackageBuildState::Invalid,
        }
    }
}

pub fn verify_device_bound_entitlement_snapshot(
    verifier: &mut impl EntitlementSnapshotAuthorityVerifier,
    snapshot: &SignedEntitlementSnapshot,
    request: &EntitlementSnapshotVerificationRequest,
) -> Result<EntitlementSnapshotContext, EntitlementSnapshotVerificationFailure> {
    entitlement_snapshot_validation::validate_snapshot_shape(snapshot, request)?;
    let verification = verifier.verify_signature_and_revocation(snapshot, request)?;
    entitlement_snapshot_authority::validate_verifier_owned_binding(
        snapshot,
        request,
        &verification,
    )?;
    match &verification.signature_state {
        EntitlementSnapshotSignatureState::Missing => {
            return Err(EntitlementSnapshotVerificationFailure::MissingSignature);
        }
        EntitlementSnapshotSignatureState::Invalid => {
            return Err(EntitlementSnapshotVerificationFailure::InvalidSignature);
        }
        EntitlementSnapshotSignatureState::Trusted => {}
    }

    Ok(entitlement_snapshot_context::snapshot_context_from_signed_snapshot(snapshot, verification))
}

#![forbid(unsafe_code)]

//! Entitlement snapshot projection and verifier-owned context.

use crate::entitlement_access::{EntitlementCapability, SubscriptionState};
use crate::entitlement_snapshot_values::{
    EntitlementAccountAuthorityState, EntitlementAccountRef,
    EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState, EntitlementHouseholdRef,
    EntitlementPackageBuildRef, EntitlementPackageBuildState, EntitlementProviderStateBoundary,
    EntitlementRevocationCursor, EntitlementSafetyFeatureState, EntitlementSignatureKeyId,
    EntitlementSnapshotBindingState, EntitlementSnapshotFreshnessState, EntitlementSnapshotId,
    EntitlementSnapshotPlanTier, EntitlementSnapshotReleaseChannel,
    EntitlementSnapshotSignatureState, EntitlementTrustedDeviceRef,
};
use serde::{Deserialize, Serialize};

#[path = "entitlement_snapshot_derivation.rs"]
mod derivation;
#[path = "entitlement_snapshot_shape.rs"]
mod shape;

pub(crate) const ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION: u16 = 1;
pub(crate) const ENTITLEMENT_SNAPSHOT_SIGNATURE_BYTES: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntitlementSnapshotFeatureFlag {
    pub capability: EntitlementCapability,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntitlementSnapshotLimitBundle {
    pub child_device_limit: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntitlementBillingLedgerState {
    pub subscription_state: SubscriptionState,
    pub plan_tier: EntitlementSnapshotPlanTier,
    pub base_child_device_limit: u32,
    pub paid_extra_child_device_seats: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntitlementReferralLedgerState {
    pub active_referral_credits: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntitlementLedgerProjectionState {
    pub account_ref: EntitlementAccountRef,
    pub household_ref: EntitlementHouseholdRef,
    pub trusted_device_ref: EntitlementTrustedDeviceRef,
    pub feature_flags: Vec<EntitlementSnapshotFeatureFlag>,
    pub revocation_cursor: EntitlementRevocationCursor,
    pub device_trust_required: bool,
    pub package_build_ref: EntitlementPackageBuildRef,
    pub release_channel: EntitlementSnapshotReleaseChannel,
    pub account_authority_state: EntitlementAccountAuthorityState,
    pub safety_feature_state: EntitlementSafetyFeatureState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntitlementProviderStateInput {
    pub authority_boundary: EntitlementProviderStateBoundary,
    pub livemode: bool,
    pub provider_plan_tier_echo: Option<EntitlementSnapshotPlanTier>,
    pub provider_child_device_limit_hint: Option<u32>,
}

/// A data-only projection used before the entitlement owner issues authority.
///
/// This type deliberately has no signature, key identifier, or trust state. It
/// is not an entitlement authority and must never be used to authorize a
/// capability. The issuer boundary consumes it only through its opaque,
/// owner-produced `TrustedEntitlementIssuanceProjection`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnsignedEntitlementSnapshotProjection {
    pub schema_version: u16,
    pub snapshot_id: EntitlementSnapshotId,
    pub account_ref: EntitlementAccountRef,
    pub household_ref: EntitlementHouseholdRef,
    pub trusted_device_ref: EntitlementTrustedDeviceRef,
    pub plan_tier: EntitlementSnapshotPlanTier,
    pub feature_flags: Vec<EntitlementSnapshotFeatureFlag>,
    pub limits: EntitlementSnapshotLimitBundle,
    pub base_child_device_limit: u32,
    pub active_referral_credits: u32,
    pub paid_extra_child_device_seats: u32,
    pub effective_child_device_limit: u32,
    pub issued_at: String,
    pub expires_at: String,
    pub grace_until: Option<String>,
    pub livemode: bool,
    pub revocation_cursor: EntitlementRevocationCursor,
    pub authority_generation: u64,
    pub device_trust_required: bool,
    pub package_build_ref: EntitlementPackageBuildRef,
    pub release_channel: EntitlementSnapshotReleaseChannel,
}

/// Signed wire material received from the entitlement issuer.
///
/// This is transport data only. A decoded value is never capability authority.
/// The crate deliberately exports no verifier, issuer, revocation store, or
/// unlock route until those owners are composed by a shipped runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignedEntitlementSnapshot {
    pub schema_version: u16,
    pub snapshot_id: EntitlementSnapshotId,
    pub account_ref: EntitlementAccountRef,
    pub household_ref: EntitlementHouseholdRef,
    pub trusted_device_ref: EntitlementTrustedDeviceRef,
    pub plan_tier: EntitlementSnapshotPlanTier,
    pub feature_flags: Vec<EntitlementSnapshotFeatureFlag>,
    pub limits: EntitlementSnapshotLimitBundle,
    pub base_child_device_limit: u32,
    pub active_referral_credits: u32,
    pub paid_extra_child_device_seats: u32,
    pub effective_child_device_limit: u32,
    pub issued_at: String,
    pub expires_at: String,
    pub grace_until: Option<String>,
    pub livemode: bool,
    pub revocation_cursor: EntitlementRevocationCursor,
    pub authority_generation: u64,
    pub device_trust_required: bool,
    pub package_build_ref: EntitlementPackageBuildRef,
    pub release_channel: EntitlementSnapshotReleaseChannel,
    pub signature_key_id: EntitlementSignatureKeyId,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementSnapshotShapeError {
    UnsupportedSchemaVersion,
    InvalidSignatureLength,
    InvalidTimestamp,
    InvalidTimeWindow,
    InvalidGraceWindow,
    DuplicateCapability,
    InvalidAuthorityGeneration,
    InvalidEffectiveChildDeviceLimit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntitlementSnapshotDerivationInput {
    pub snapshot_id: EntitlementSnapshotId,
    pub billing_ledger_state: EntitlementBillingLedgerState,
    pub referral_ledger_state: EntitlementReferralLedgerState,
    pub entitlement_ledger_state: EntitlementLedgerProjectionState,
    pub provider_state: EntitlementProviderStateInput,
    pub authority_generation: u64,
    pub issued_at: String,
    pub expires_at: String,
    pub grace_until: Option<String>,
}

/// Capability context held by the entitlement owner.
///
/// Its state is crate-private, deserialization always fails, and serialization
/// is deliberately unavailable. Only the fail-closed unavailable context is
/// currently constructible; no signed wire value can manufacture authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntitlementSnapshotContext {
    pub(crate) signature_state: EntitlementSnapshotSignatureState,
    pub(crate) freshness_state: EntitlementSnapshotFreshnessState,
    pub(crate) household_binding_state: EntitlementSnapshotBindingState,
    pub(crate) device_binding_state: EntitlementSnapshotBindingState,
    pub(crate) device_trust_requirement_state: EntitlementDeviceTrustRequirementState,
    pub(crate) device_trust_state: EntitlementDeviceTrustState,
    pub(crate) package_build_state: EntitlementPackageBuildState,
}

impl Serialize for EntitlementSnapshotContext {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(serde::ser::Error::custom(
            "entitlement snapshot context is verifier-owned and cannot be serialized",
        ))
    }
}

impl<'de> Deserialize<'de> for EntitlementSnapshotContext {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Err(serde::de::Error::custom(
            "entitlement snapshot context must come from verifier authority",
        ))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementSnapshotDerivationError {
    ZeroBaseChildDeviceLimit,
    InvalidStarterBaseChildDeviceLimit,
    ZeroProviderChildDeviceLimitHint,
    SeatLimitOverflow,
}

pub fn checked_effective_child_device_limit(
    base_child_device_limit: u32,
    active_referral_credits: u32,
    paid_extra_child_device_seats: u32,
) -> Result<u32, EntitlementSnapshotDerivationError> {
    derivation::checked_effective_child_device_limit(
        base_child_device_limit,
        active_referral_credits,
        paid_extra_child_device_seats,
    )
}

pub fn derive_unsigned_entitlement_snapshot(
    input: EntitlementSnapshotDerivationInput,
) -> Result<UnsignedEntitlementSnapshotProjection, EntitlementSnapshotDerivationError> {
    derivation::derive_unsigned_entitlement_snapshot(input)
}

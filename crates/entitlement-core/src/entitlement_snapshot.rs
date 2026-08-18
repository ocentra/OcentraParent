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

#[path = "entitlement_snapshot_capability_wire_names.rs"]
mod capability_wire_names;
#[path = "entitlement_snapshot_derivation.rs"]
mod derivation;
#[path = "entitlement_snapshot_shape.rs"]
mod shape;
#[path = "entitlement_snapshot_signing.rs"]
mod signing;
#[path = "entitlement_snapshot_wire_names.rs"]
mod wire_names;

pub(crate) const ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION: u16 = 1;
pub(crate) const ENTITLEMENT_SNAPSHOT_SIGNATURE_BYTES: usize = 64;

const ENTITLEMENT_SNAPSHOT_SIGNING_DOMAIN: &[u8] = b"ocentra.entitlement.snapshot.signing.v1\0";
const CAPABILITY_TRACKING: &str = "tracking";
const CAPABILITY_SCREEN_EVIDENCE: &str = "screen-evidence";
const CAPABILITY_REMOTE_ACCESS: &str = "remote-access";
const CAPABILITY_ENFORCEMENT: &str = "enforcement";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementBillingLedgerState {
    pub subscription_state: SubscriptionState,
    pub plan_tier: EntitlementSnapshotPlanTier,
    pub base_child_device_limit: u32,
    pub paid_extra_child_device_seats: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementReferralLedgerState {
    pub active_referral_credits: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
/// This is transport data only.  A decoded value is never capability
/// authority until `entitlement_snapshot_authority` verifies its signature,
/// exact revocation generation/cursor, account handoff, and device binding.
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

impl SignedEntitlementSnapshot {
    pub(crate) fn from_projection(
        projection: UnsignedEntitlementSnapshotProjection,
        signature_key_id: EntitlementSignatureKeyId,
    ) -> Self {
        Self {
            schema_version: projection.schema_version,
            snapshot_id: projection.snapshot_id,
            account_ref: projection.account_ref,
            household_ref: projection.household_ref,
            trusted_device_ref: projection.trusted_device_ref,
            plan_tier: projection.plan_tier,
            feature_flags: projection.feature_flags,
            limits: projection.limits,
            base_child_device_limit: projection.base_child_device_limit,
            active_referral_credits: projection.active_referral_credits,
            paid_extra_child_device_seats: projection.paid_extra_child_device_seats,
            effective_child_device_limit: projection.effective_child_device_limit,
            issued_at: projection.issued_at,
            expires_at: projection.expires_at,
            grace_until: projection.grace_until,
            livemode: projection.livemode,
            revocation_cursor: projection.revocation_cursor,
            authority_generation: projection.authority_generation,
            device_trust_required: projection.device_trust_required,
            package_build_ref: projection.package_build_ref,
            release_channel: projection.release_channel,
            signature_key_id,
            signature: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
/// is deliberately unavailable. The authority module is the only shipped
/// producer of trusted context, and it requires external key, package, and
/// currentness authorities before returning a capability handoff.
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

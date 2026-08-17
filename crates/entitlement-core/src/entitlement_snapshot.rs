#![forbid(unsafe_code)]

//! Unsigned entitlement snapshot projection and verifier-owned context.

use crate::entitlement_access::{EntitlementCapability, SubscriptionState};
use crate::entitlement_snapshot_values::{
    EntitlementAccountAuthorityState, EntitlementAccountRef,
    EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState, EntitlementHouseholdRef,
    EntitlementPackageBuildRef, EntitlementPackageBuildState, EntitlementProviderStateBoundary,
    EntitlementRevocationCursor, EntitlementSafetyFeatureState, EntitlementSnapshotBindingState,
    EntitlementSnapshotFreshnessState, EntitlementSnapshotId, EntitlementSnapshotPlanTier,
    EntitlementSnapshotSignatureState, EntitlementTrustedDeviceRef,
};
use serde::{Deserialize, Serialize};

const ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION: u16 = 1;

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

/// A data-only projection used before an entitlement verifier issues authority.
///
/// This type deliberately has no signature, key identifier, or trust state. It
/// is not an entitlement authority and must never be used to authorize a
/// capability. A verifier-owned issuer may later consume this projection and
/// return an opaque context through an owner boundary.
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
    pub device_trust_required: bool,
    pub package_build_ref: EntitlementPackageBuildRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementSnapshotDerivationInput {
    pub snapshot_id: EntitlementSnapshotId,
    pub billing_ledger_state: EntitlementBillingLedgerState,
    pub referral_ledger_state: EntitlementReferralLedgerState,
    pub entitlement_ledger_state: EntitlementLedgerProjectionState,
    pub provider_state: EntitlementProviderStateInput,
    pub issued_at: String,
    pub expires_at: String,
    pub grace_until: Option<String>,
}

/// Capability context held by the entitlement owner.
///
/// Its state is crate-private, deserialization always fails, and serialization
/// is deliberately unavailable. No verifier issuer is shipped in this crate,
/// so no public constructor can mint trusted context.
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
    let base_child_device_limit = std::num::NonZeroU32::new(base_child_device_limit)
        .ok_or(EntitlementSnapshotDerivationError::ZeroBaseChildDeviceLimit)?
        .get();
    base_child_device_limit
        .checked_add(active_referral_credits)
        .and_then(|subtotal| subtotal.checked_add(paid_extra_child_device_seats))
        .ok_or(EntitlementSnapshotDerivationError::SeatLimitOverflow)
}

pub fn derive_unsigned_entitlement_snapshot(
    input: EntitlementSnapshotDerivationInput,
) -> Result<UnsignedEntitlementSnapshotProjection, EntitlementSnapshotDerivationError> {
    input
        .provider_state
        .provider_child_device_limit_hint
        .map(|hint| {
            std::num::NonZeroU32::new(hint)
                .ok_or(EntitlementSnapshotDerivationError::ZeroProviderChildDeviceLimitHint)
        })
        .transpose()?;
    let effective_child_device_limit = checked_effective_child_device_limit(
        input.billing_ledger_state.base_child_device_limit,
        input.referral_ledger_state.active_referral_credits,
        input.billing_ledger_state.paid_extra_child_device_seats,
    )?;

    Ok(UnsignedEntitlementSnapshotProjection {
        schema_version: ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION,
        snapshot_id: input.snapshot_id,
        account_ref: input.entitlement_ledger_state.account_ref,
        household_ref: input.entitlement_ledger_state.household_ref,
        trusted_device_ref: input.entitlement_ledger_state.trusted_device_ref,
        plan_tier: input.billing_ledger_state.plan_tier,
        feature_flags: input.entitlement_ledger_state.feature_flags,
        limits: EntitlementSnapshotLimitBundle {
            child_device_limit: effective_child_device_limit,
        },
        base_child_device_limit: input.billing_ledger_state.base_child_device_limit,
        active_referral_credits: input.referral_ledger_state.active_referral_credits,
        paid_extra_child_device_seats: input.billing_ledger_state.paid_extra_child_device_seats,
        effective_child_device_limit,
        issued_at: input.issued_at,
        expires_at: input.expires_at,
        grace_until: input.grace_until,
        livemode: input.provider_state.livemode,
        revocation_cursor: input.entitlement_ledger_state.revocation_cursor,
        device_trust_required: input.entitlement_ledger_state.device_trust_required,
        package_build_ref: input.entitlement_ledger_state.package_build_ref,
    })
}

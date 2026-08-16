#![forbid(unsafe_code)]

//! Signed entitlement snapshot derivation and verification context.

use chrono::{DateTime, FixedOffset};

use crate::entitlement_access::{EntitlementCapability, SubscriptionState};
use crate::entitlement_snapshot_values::{
    EntitlementAccountAuthorityState, EntitlementAccountRef,
    EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState, EntitlementHouseholdRef,
    EntitlementPackageBuildRef, EntitlementPackageBuildState, EntitlementProviderStateBoundary,
    EntitlementRevocationCursor, EntitlementSafetyFeatureState, EntitlementSignatureKeyId,
    EntitlementSnapshotBindingState, EntitlementSnapshotFreshnessState, EntitlementSnapshotId,
    EntitlementSnapshotPlanTier, EntitlementSnapshotSignatureState, EntitlementTrustedDeviceRef,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub device_trust_required: bool,
    pub package_build_ref: EntitlementPackageBuildRef,
    pub signature_key_id: EntitlementSignatureKeyId,
    pub signature: String,
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
    pub signature_key_id: EntitlementSignatureKeyId,
    pub signature: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementSnapshotContext {
    pub signature_state: EntitlementSnapshotSignatureState,
    pub freshness_state: EntitlementSnapshotFreshnessState,
    pub household_binding_state: EntitlementSnapshotBindingState,
    pub device_binding_state: EntitlementSnapshotBindingState,
    pub device_trust_requirement_state: EntitlementDeviceTrustRequirementState,
    pub device_trust_state: EntitlementDeviceTrustState,
    pub package_build_state: EntitlementPackageBuildState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementSnapshotVerificationContext {
    pub signature_state: EntitlementSnapshotSignatureState,
    pub freshness_state: EntitlementSnapshotFreshnessState,
    pub household_binding_state: EntitlementSnapshotBindingState,
    pub device_binding_state: EntitlementSnapshotBindingState,
    pub device_trust_state: EntitlementDeviceTrustState,
    pub package_build_state: EntitlementPackageBuildState,
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

pub fn verify_device_bound_entitlement_snapshot(
    verifier: &mut impl EntitlementSnapshotAuthorityVerifier,
    snapshot: &SignedEntitlementSnapshot,
    request: &EntitlementSnapshotVerificationRequest,
) -> Result<EntitlementSnapshotContext, EntitlementSnapshotVerificationFailure> {
    validate_snapshot_shape(snapshot, request)?;
    let verification = verifier.verify_signature_and_revocation(snapshot, request)?;
    match verification.signature_state {
        EntitlementSnapshotSignatureState::Missing => {
            return Err(EntitlementSnapshotVerificationFailure::MissingSignature);
        }
        EntitlementSnapshotSignatureState::Invalid => {
            return Err(EntitlementSnapshotVerificationFailure::InvalidSignature);
        }
        EntitlementSnapshotSignatureState::Trusted => {}
    }

    Ok(snapshot_context_from_signed_snapshot(
        snapshot,
        EntitlementSnapshotVerificationContext {
            signature_state: verification.signature_state,
            freshness_state: verification.freshness_state,
            household_binding_state: EntitlementSnapshotBindingState::Matched,
            device_binding_state: EntitlementSnapshotBindingState::Matched,
            device_trust_state: verification.device_trust_state,
            package_build_state: verification.package_build_state,
        },
    ))
}

fn validate_snapshot_shape(
    snapshot: &SignedEntitlementSnapshot,
    request: &EntitlementSnapshotVerificationRequest,
) -> Result<(), EntitlementSnapshotVerificationFailure> {
    if snapshot.schema_version != ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION {
        return Err(EntitlementSnapshotVerificationFailure::InvalidSnapshotShape);
    }
    if snapshot.signature.is_empty() {
        return Err(EntitlementSnapshotVerificationFailure::MissingSignature);
    }
    if snapshot.account_ref != request.account_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongAccount);
    }
    if snapshot.household_ref != request.household_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongHousehold);
    }
    if snapshot.trusted_device_ref != request.trusted_device_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongDevice);
    }
    if snapshot.package_build_ref != request.package_build_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongPackageBuild);
    }

    let issued_at = parse_snapshot_timestamp(&snapshot.issued_at)?;
    let expires_at = parse_snapshot_timestamp(&snapshot.expires_at)?;
    let observed_at = parse_snapshot_timestamp(&request.observed_at)?;
    if issued_at > observed_at || observed_at >= expires_at {
        return Err(EntitlementSnapshotVerificationFailure::Expired);
    }
    if let Some(grace_until) = snapshot.grace_until.as_deref() {
        if parse_snapshot_timestamp(grace_until)? < expires_at {
            return Err(EntitlementSnapshotVerificationFailure::InvalidSnapshotShape);
        }
    }
    Ok(())
}

fn parse_snapshot_timestamp(
    value: &str,
) -> Result<DateTime<FixedOffset>, EntitlementSnapshotVerificationFailure> {
    DateTime::parse_from_rfc3339(value)
        .map_err(|_error| EntitlementSnapshotVerificationFailure::TimestampInvalid)
}

pub fn derive_signed_entitlement_snapshot(
    input: EntitlementSnapshotDerivationInput,
) -> SignedEntitlementSnapshot {
    let effective_child_device_limit = input.billing_ledger_state.base_child_device_limit
        + input.referral_ledger_state.active_referral_credits
        + input.billing_ledger_state.paid_extra_child_device_seats;

    SignedEntitlementSnapshot {
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
        signature_key_id: input.signature_key_id,
        signature: input.signature,
    }
}

pub fn snapshot_context_from_signed_snapshot(
    snapshot: &SignedEntitlementSnapshot,
    verification: EntitlementSnapshotVerificationContext,
) -> EntitlementSnapshotContext {
    EntitlementSnapshotContext {
        signature_state: verification.signature_state,
        freshness_state: verification.freshness_state,
        household_binding_state: verification.household_binding_state,
        device_binding_state: verification.device_binding_state,
        device_trust_requirement_state: if snapshot.device_trust_required {
            EntitlementDeviceTrustRequirementState::Required
        } else {
            EntitlementDeviceTrustRequirementState::NotRequired
        },
        device_trust_state: verification.device_trust_state,
        package_build_state: verification.package_build_state,
    }
}

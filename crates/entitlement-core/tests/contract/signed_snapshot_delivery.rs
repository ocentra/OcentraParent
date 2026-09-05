use ocentra_entitlement_core::entitlement_access::{EntitlementCapability, SubscriptionState};
use ocentra_entitlement_core::entitlement_snapshot::{
    checked_effective_child_device_limit, derive_unsigned_entitlement_snapshot,
    EntitlementBillingLedgerState, EntitlementLedgerProjectionState, EntitlementProviderStateInput,
    EntitlementReferralLedgerState, EntitlementSnapshotDerivationError,
    EntitlementSnapshotDerivationInput, EntitlementSnapshotFeatureFlag,
    EntitlementSnapshotLimitBundle, EntitlementSnapshotShapeError, SignedEntitlementSnapshot,
};
use ocentra_entitlement_core::entitlement_snapshot_values::{
    EntitlementAccountAuthorityState, EntitlementAccountRef, EntitlementHouseholdRef,
    EntitlementPackageBuildRef, EntitlementProviderStateBoundary, EntitlementRevocationCursor,
    EntitlementSafetyFeatureState, EntitlementSignatureKeyId, EntitlementSnapshotId,
    EntitlementSnapshotPlanTier, EntitlementSnapshotReleaseChannel, EntitlementTrustedDeviceRef,
};
use serde_json::json;
use std::error::Error;

const ISSUED_AT: &str = "2026-06-28T20:00:00.000Z";
const EXPIRES_AT: &str = "2026-07-05T20:00:00.000Z";
const GRACE_UNTIL: &str = "2026-07-07T20:00:00.000Z";
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[test]
fn signed_wire_round_trips_and_validates_public_shape() -> TestResult {
    let snapshot = signed_snapshot()?;
    assert_eq!(snapshot.validate_shape(), Ok(()));

    let wire = serde_json::to_value(&snapshot)?;
    assert_eq!(wire["schemaVersion"], json!(1));
    assert_eq!(wire["releaseChannel"], json!("stable"));
    assert_eq!(wire["signatureKeyId"], json!("entitlement-key-2026-06"));
    assert_eq!(wire["signature"].as_array().map(Vec::len), Some(64));
    let decoded: SignedEntitlementSnapshot = serde_json::from_value(wire)?;
    assert_eq!(decoded, snapshot);
    Ok(())
}

#[test]
fn signed_shape_rejects_malformed_signature_length() -> TestResult {
    let mut snapshot = signed_snapshot()?;
    snapshot.signature = vec![0; 63];
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidSignatureLength)
    );
    Ok(())
}

#[test]
fn signed_shape_rejects_unsupported_schema_and_zero_generation() -> TestResult {
    let mut snapshot = signed_snapshot()?;
    snapshot.schema_version = 2;
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::UnsupportedSchemaVersion)
    );

    let mut snapshot = signed_snapshot()?;
    snapshot.authority_generation = 0;
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidAuthorityGeneration)
    );
    Ok(())
}

#[test]
fn signed_shape_rejects_malformed_timestamps_and_windows() -> TestResult {
    let mut snapshot = signed_snapshot()?;
    snapshot.issued_at = String::from("not-an-rfc3339-timestamp");
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidTimestamp)
    );

    let mut snapshot = signed_snapshot()?;
    snapshot.expires_at = String::from(ISSUED_AT);
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidTimeWindow)
    );

    let mut snapshot = signed_snapshot()?;
    snapshot.grace_until = Some(String::from("2026-07-01T20:00:00.000Z"));
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidGraceWindow)
    );
    Ok(())
}

#[test]
fn signed_shape_rejects_inconsistent_and_overflowed_limits() -> TestResult {
    let mut snapshot = signed_snapshot()?;
    snapshot.limits = EntitlementSnapshotLimitBundle {
        child_device_limit: 3,
    };
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidEffectiveChildDeviceLimit)
    );

    let mut snapshot = signed_snapshot()?;
    snapshot.base_child_device_limit = u32::MAX;
    snapshot.paid_extra_child_device_seats = 1;
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidEffectiveChildDeviceLimit)
    );
    Ok(())
}

#[test]
fn signed_shape_rejects_duplicate_capabilities() -> TestResult {
    let mut snapshot = signed_snapshot()?;
    snapshot.feature_flags.push(EntitlementSnapshotFeatureFlag {
        capability: EntitlementCapability::Tracking,
        enabled: false,
    });
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::DuplicateCapability)
    );
    Ok(())
}

#[test]
fn signed_wire_rejects_unknown_fields() -> TestResult {
    let mut wire = serde_json::to_value(signed_snapshot()?)?;
    wire.as_object_mut()
        .ok_or("signed snapshot wire must be an object")?
        .insert(String::from("unexpectedField"), json!(true));
    let error = require_json_error(
        serde_json::from_value::<SignedEntitlementSnapshot>(wire),
        "unknown signed wire fields must be rejected",
    )?;
    assert!(error.to_string().contains("unknown field"));
    assert!(error.to_string().contains("unexpectedField"));
    Ok(())
}

#[test]
fn unsigned_derivation_uses_ledger_truth_and_stays_unsigned() -> TestResult {
    let mut input = derivation_input()?;
    input.provider_state.provider_plan_tier_echo = Some(EntitlementSnapshotPlanTier::Paid);
    input.provider_state.provider_child_device_limit_hint = Some(99);

    let projection = derive_unsigned_entitlement_snapshot(input)
        .map_err(|error| std::io::Error::other(format!("unsigned projection failed: {error:?}")))?;
    assert_eq!(projection.plan_tier, EntitlementSnapshotPlanTier::Starter);
    assert_eq!(projection.effective_child_device_limit, 4);
    assert_eq!(projection.limits.child_device_limit, 4);
    assert_eq!(
        projection.release_channel,
        EntitlementSnapshotReleaseChannel::Stable
    );
    assert_eq!(projection.authority_generation, 7);
    let wire = serde_json::to_value(projection)?;
    assert_eq!(wire.get("signature"), None);
    assert_eq!(wire.get("signatureKeyId"), None);
    Ok(())
}

#[test]
fn unsigned_derivation_rejects_zero_provider_hint() -> TestResult {
    let mut input = derivation_input()?;
    input.provider_state.provider_child_device_limit_hint = Some(0);
    assert_eq!(
        derive_unsigned_entitlement_snapshot(input),
        Err(EntitlementSnapshotDerivationError::ZeroProviderChildDeviceLimitHint)
    );
    Ok(())
}

#[test]
fn checked_limit_rejects_zero_base_and_overflow() -> TestResult {
    assert_eq!(
        checked_effective_child_device_limit(0, 1, 1),
        Err(EntitlementSnapshotDerivationError::ZeroBaseChildDeviceLimit)
    );
    assert_eq!(
        checked_effective_child_device_limit(u32::MAX, 1, 0),
        Err(EntitlementSnapshotDerivationError::SeatLimitOverflow)
    );
    Ok(())
}

fn signed_snapshot() -> TestResult<SignedEntitlementSnapshot> {
    Ok(SignedEntitlementSnapshot {
        schema_version: 1,
        snapshot_id: EntitlementSnapshotId::parse("snapshot-household-default")?,
        account_ref: EntitlementAccountRef::parse("acct-default")?,
        household_ref: EntitlementHouseholdRef::parse("household-default")?,
        trusted_device_ref: EntitlementTrustedDeviceRef::parse("device-default")?,
        plan_tier: EntitlementSnapshotPlanTier::Starter,
        feature_flags: vec![
            EntitlementSnapshotFeatureFlag {
                capability: EntitlementCapability::Tracking,
                enabled: true,
            },
            EntitlementSnapshotFeatureFlag {
                capability: EntitlementCapability::RemoteAccess,
                enabled: false,
            },
        ],
        limits: EntitlementSnapshotLimitBundle {
            child_device_limit: 4,
        },
        base_child_device_limit: 1,
        active_referral_credits: 2,
        paid_extra_child_device_seats: 1,
        effective_child_device_limit: 4,
        issued_at: String::from(ISSUED_AT),
        expires_at: String::from(EXPIRES_AT),
        grace_until: Some(String::from(GRACE_UNTIL)),
        livemode: true,
        revocation_cursor: EntitlementRevocationCursor::parse("revoke-cursor-2026-06-28")?,
        authority_generation: 7,
        device_trust_required: true,
        package_build_ref: EntitlementPackageBuildRef::parse("windows-msi-stable")?,
        release_channel: EntitlementSnapshotReleaseChannel::Stable,
        signature_key_id: EntitlementSignatureKeyId::parse("entitlement-key-2026-06")?,
        signature: vec![0; 64],
    })
}

fn derivation_input() -> TestResult<EntitlementSnapshotDerivationInput> {
    Ok(EntitlementSnapshotDerivationInput {
        snapshot_id: EntitlementSnapshotId::parse("snapshot-household-default")?,
        billing_ledger_state: EntitlementBillingLedgerState {
            subscription_state: SubscriptionState::Active,
            plan_tier: EntitlementSnapshotPlanTier::Starter,
            base_child_device_limit: 1,
            paid_extra_child_device_seats: 1,
        },
        referral_ledger_state: EntitlementReferralLedgerState {
            active_referral_credits: 2,
        },
        entitlement_ledger_state: EntitlementLedgerProjectionState {
            account_ref: EntitlementAccountRef::parse("acct-default")?,
            household_ref: EntitlementHouseholdRef::parse("household-default")?,
            trusted_device_ref: EntitlementTrustedDeviceRef::parse("device-default")?,
            feature_flags: vec![EntitlementSnapshotFeatureFlag {
                capability: EntitlementCapability::Tracking,
                enabled: true,
            }],
            revocation_cursor: EntitlementRevocationCursor::parse("revoke-cursor-2026-06-28")?,
            device_trust_required: true,
            package_build_ref: EntitlementPackageBuildRef::parse("windows-msi-stable")?,
            release_channel: EntitlementSnapshotReleaseChannel::Stable,
            account_authority_state: EntitlementAccountAuthorityState::VerifiedAccountHandoff,
            safety_feature_state: EntitlementSafetyFeatureState::PreservedOutsidePaidGates,
        },
        provider_state: EntitlementProviderStateInput {
            authority_boundary: EntitlementProviderStateBoundary::InputOnly,
            livemode: true,
            provider_plan_tier_echo: Some(EntitlementSnapshotPlanTier::Paid),
            provider_child_device_limit_hint: Some(99),
        },
        authority_generation: 7,
        issued_at: String::from(ISSUED_AT),
        expires_at: String::from(EXPIRES_AT),
        grace_until: Some(String::from(GRACE_UNTIL)),
    })
}

fn require_json_error<T>(
    result: Result<T, serde_json::Error>,
    message: &'static str,
) -> TestResult<serde_json::Error> {
    match result {
        Ok(_) => Err(message.into()),
        Err(error) => Ok(error),
    }
}

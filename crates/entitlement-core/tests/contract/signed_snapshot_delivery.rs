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

const ISSUED_AT: &str = "2026-06-28T20:00:00.000Z";
const EXPIRES_AT: &str = "2026-07-05T20:00:00.000Z";
const GRACE_UNTIL: &str = "2026-07-07T20:00:00.000Z";

#[test]
fn signed_wire_round_trips_and_validates_public_shape() {
    let snapshot = signed_snapshot();
    assert_eq!(snapshot.validate_shape(), Ok(()));

    let wire = serde_json::to_value(&snapshot).expect("signed snapshot serializes");
    assert_eq!(wire["schemaVersion"], json!(1));
    assert_eq!(wire["releaseChannel"], json!("stable"));
    assert_eq!(wire["signatureKeyId"], json!("entitlement-key-2026-06"));
    assert_eq!(wire["signature"].as_array().map(Vec::len), Some(64));
    let decoded: SignedEntitlementSnapshot =
        serde_json::from_value(wire).expect("signed snapshot wire decodes");
    assert_eq!(decoded, snapshot);
}

#[test]
fn signed_shape_rejects_malformed_signature_length() {
    let mut snapshot = signed_snapshot();
    snapshot.signature = vec![0; 63];
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidSignatureLength)
    );
}

#[test]
fn signed_shape_rejects_unsupported_schema_and_zero_generation() {
    let mut snapshot = signed_snapshot();
    snapshot.schema_version = 2;
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::UnsupportedSchemaVersion)
    );

    let mut snapshot = signed_snapshot();
    snapshot.authority_generation = 0;
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidAuthorityGeneration)
    );
}

#[test]
fn signed_shape_rejects_malformed_timestamps_and_windows() {
    let mut snapshot = signed_snapshot();
    snapshot.issued_at = String::from("not-an-rfc3339-timestamp");
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidTimestamp)
    );

    let mut snapshot = signed_snapshot();
    snapshot.expires_at = String::from(ISSUED_AT);
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidTimeWindow)
    );

    let mut snapshot = signed_snapshot();
    snapshot.grace_until = Some(String::from("2026-07-01T20:00:00.000Z"));
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidGraceWindow)
    );
}

#[test]
fn signed_shape_rejects_inconsistent_and_overflowed_limits() {
    let mut snapshot = signed_snapshot();
    snapshot.limits = EntitlementSnapshotLimitBundle {
        child_device_limit: 4,
    };
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidEffectiveChildDeviceLimit)
    );

    let mut snapshot = signed_snapshot();
    snapshot.base_child_device_limit = u32::MAX;
    snapshot.paid_extra_child_device_seats = 1;
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::InvalidEffectiveChildDeviceLimit)
    );
}

#[test]
fn signed_shape_rejects_duplicate_capabilities() {
    let mut snapshot = signed_snapshot();
    snapshot.feature_flags.push(EntitlementSnapshotFeatureFlag {
        capability: EntitlementCapability::Tracking,
        enabled: false,
    });
    assert_eq!(
        snapshot.validate_shape(),
        Err(EntitlementSnapshotShapeError::DuplicateCapability)
    );
}

#[test]
fn signed_wire_rejects_unknown_fields() {
    let mut wire = serde_json::to_value(signed_snapshot()).expect("signed snapshot serializes");
    wire.as_object_mut()
        .expect("signed snapshot is an object")
        .insert(String::from("unexpectedField"), json!(true));
    let error = serde_json::from_value::<SignedEntitlementSnapshot>(wire)
        .expect_err("unknown signed wire fields are rejected");
    assert!(error.to_string().contains("unknown field"));
    assert!(error.to_string().contains("unexpectedField"));
}

#[test]
fn unsigned_derivation_uses_ledger_truth_and_stays_unsigned() {
    let mut input = derivation_input();
    input.provider_state.provider_plan_tier_echo = Some(EntitlementSnapshotPlanTier::Paid);
    input.provider_state.provider_child_device_limit_hint = Some(99);

    let projection = derive_unsigned_entitlement_snapshot(input).expect("unsigned projection");
    assert_eq!(projection.plan_tier, EntitlementSnapshotPlanTier::Starter);
    assert_eq!(projection.effective_child_device_limit, 5);
    assert_eq!(projection.limits.child_device_limit, 5);
    assert_eq!(
        projection.release_channel,
        EntitlementSnapshotReleaseChannel::Stable
    );
    assert_eq!(projection.authority_generation, 7);
    let wire = serde_json::to_value(projection).expect("unsigned projection serializes");
    assert_eq!(wire.get("signature"), None);
    assert_eq!(wire.get("signatureKeyId"), None);
}

#[test]
fn unsigned_derivation_rejects_zero_provider_hint() {
    let mut input = derivation_input();
    input.provider_state.provider_child_device_limit_hint = Some(0);
    assert_eq!(
        derive_unsigned_entitlement_snapshot(input),
        Err(EntitlementSnapshotDerivationError::ZeroProviderChildDeviceLimitHint)
    );
}

#[test]
fn checked_limit_rejects_zero_base_and_overflow() {
    assert_eq!(
        checked_effective_child_device_limit(0, 1, 1),
        Err(EntitlementSnapshotDerivationError::ZeroBaseChildDeviceLimit)
    );
    assert_eq!(
        checked_effective_child_device_limit(u32::MAX, 1, 0),
        Err(EntitlementSnapshotDerivationError::SeatLimitOverflow)
    );
}

fn signed_snapshot() -> SignedEntitlementSnapshot {
    SignedEntitlementSnapshot {
        schema_version: 1,
        snapshot_id: EntitlementSnapshotId::parse("snapshot-household-default").unwrap(),
        account_ref: EntitlementAccountRef::parse("acct-default").unwrap(),
        household_ref: EntitlementHouseholdRef::parse("household-default").unwrap(),
        trusted_device_ref: EntitlementTrustedDeviceRef::parse("device-default").unwrap(),
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
            child_device_limit: 5,
        },
        base_child_device_limit: 2,
        active_referral_credits: 2,
        paid_extra_child_device_seats: 1,
        effective_child_device_limit: 5,
        issued_at: String::from(ISSUED_AT),
        expires_at: String::from(EXPIRES_AT),
        grace_until: Some(String::from(GRACE_UNTIL)),
        livemode: true,
        revocation_cursor: EntitlementRevocationCursor::parse("revoke-cursor-2026-06-28").unwrap(),
        authority_generation: 7,
        device_trust_required: true,
        package_build_ref: EntitlementPackageBuildRef::parse("windows-msi-stable").unwrap(),
        release_channel: EntitlementSnapshotReleaseChannel::Stable,
        signature_key_id: EntitlementSignatureKeyId::parse("entitlement-key-2026-06").unwrap(),
        signature: vec![0; 64],
    }
}

fn derivation_input() -> EntitlementSnapshotDerivationInput {
    EntitlementSnapshotDerivationInput {
        snapshot_id: EntitlementSnapshotId::parse("snapshot-household-default").unwrap(),
        billing_ledger_state: EntitlementBillingLedgerState {
            subscription_state: SubscriptionState::Active,
            plan_tier: EntitlementSnapshotPlanTier::Starter,
            base_child_device_limit: 2,
            paid_extra_child_device_seats: 1,
        },
        referral_ledger_state: EntitlementReferralLedgerState {
            active_referral_credits: 2,
        },
        entitlement_ledger_state: EntitlementLedgerProjectionState {
            account_ref: EntitlementAccountRef::parse("acct-default").unwrap(),
            household_ref: EntitlementHouseholdRef::parse("household-default").unwrap(),
            trusted_device_ref: EntitlementTrustedDeviceRef::parse("device-default").unwrap(),
            feature_flags: vec![EntitlementSnapshotFeatureFlag {
                capability: EntitlementCapability::Tracking,
                enabled: true,
            }],
            revocation_cursor: EntitlementRevocationCursor::parse("revoke-cursor-2026-06-28")
                .unwrap(),
            device_trust_required: true,
            package_build_ref: EntitlementPackageBuildRef::parse("windows-msi-stable").unwrap(),
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
    }
}

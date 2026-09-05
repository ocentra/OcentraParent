use ocentra_entitlement_core::entitlement_access::{EntitlementCapability, SubscriptionState};
use ocentra_entitlement_core::entitlement_snapshot::{
    checked_effective_child_device_limit, derive_unsigned_entitlement_snapshot,
    EntitlementBillingLedgerState, EntitlementLedgerProjectionState, EntitlementProviderStateInput,
    EntitlementReferralLedgerState, EntitlementSnapshotDerivationError,
    EntitlementSnapshotDerivationInput, EntitlementSnapshotFeatureFlag,
    UnsignedEntitlementSnapshotProjection,
};
use ocentra_entitlement_core::entitlement_snapshot_values::{
    EntitlementAccountAuthorityState, EntitlementAccountRef, EntitlementHouseholdRef,
    EntitlementPackageBuildRef, EntitlementProviderStateBoundary, EntitlementRevocationCursor,
    EntitlementSafetyFeatureState, EntitlementSnapshotId, EntitlementSnapshotPlanTier,
    EntitlementSnapshotReleaseChannel, EntitlementTrustedDeviceRef,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

const ISSUED_AT: &str = "2026-08-29T20:00:00Z";
const EXPIRES_AT: &str = "2026-09-29T20:00:00Z";

#[test]
fn derives_starter_plus_paid_and_referral_seats() {
    assert_eq!(checked_effective_child_device_limit(1, 2, 3), Ok(6));
}

#[test]
fn derives_unsigned_projection_only_from_app_owned_seat_ledgers() {
    let projection = result_value(derive_unsigned_entitlement_snapshot(derivation_input()));

    assert_eq!(
        (
            projection.plan_tier,
            projection.base_child_device_limit,
            projection.active_referral_credits,
            projection.paid_extra_child_device_seats,
            projection.effective_child_device_limit,
            projection.limits.child_device_limit,
            projection.authority_generation,
            projection.livemode,
        ),
        (EntitlementSnapshotPlanTier::Starter, 1, 2, 3, 6, 6, 7, true,)
    );

    let wire = result_value(serde_json::to_value(projection));
    let fields = object_fields(&wire);
    assert_eq!(fields.get("signature"), None);
    assert_eq!(fields.get("signatureKeyId"), None);
}

#[test]
fn losing_a_referral_reduces_future_limit_without_rewriting_other_sources() {
    let previous = result_value(derive_unsigned_entitlement_snapshot(derivation_input()));
    let mut current_input = derivation_input();
    current_input.referral_ledger_state.active_referral_credits = 1;
    let current = result_value(derive_unsigned_entitlement_snapshot(current_input));

    assert_eq!(
        (
            previous.base_child_device_limit,
            previous.active_referral_credits,
            previous.paid_extra_child_device_seats,
            previous.effective_child_device_limit,
        ),
        (1, 2, 3, 6)
    );
    assert_eq!(
        (
            current.base_child_device_limit,
            current.active_referral_credits,
            current.paid_extra_child_device_seats,
            current.effective_child_device_limit,
        ),
        (1, 1, 3, 5)
    );
}

#[test]
fn rejects_zero_base_and_both_overflow_orders() {
    assert_eq!(
        checked_effective_child_device_limit(0, 1, 1),
        Err(EntitlementSnapshotDerivationError::ZeroBaseChildDeviceLimit)
    );
    assert_eq!(
        checked_effective_child_device_limit(u32::MAX, 1, 0),
        Err(EntitlementSnapshotDerivationError::SeatLimitOverflow)
    );
    assert_eq!(
        checked_effective_child_device_limit(1, u32::MAX - 1, 1),
        Err(EntitlementSnapshotDerivationError::SeatLimitOverflow)
    );
}

#[test]
fn rejects_zero_provider_hint_without_treating_provider_echo_as_ledger_truth() {
    let mut input = derivation_input();
    input.provider_state.provider_child_device_limit_hint = Some(0);

    assert_eq!(
        derive_unsigned_entitlement_snapshot(input),
        Err(EntitlementSnapshotDerivationError::ZeroProviderChildDeviceLimitHint)
    );
}

#[test]
fn starter_bundle_requires_one_base_child_seat_without_restricting_paid_base_seats() {
    let mut starter = derivation_input();
    starter.billing_ledger_state.base_child_device_limit = 2;
    assert_eq!(
        derive_unsigned_entitlement_snapshot(starter),
        Err(EntitlementSnapshotDerivationError::InvalidStarterBaseChildDeviceLimit)
    );

    let mut paid = derivation_input();
    paid.billing_ledger_state.plan_tier = EntitlementSnapshotPlanTier::Paid;
    paid.billing_ledger_state.base_child_device_limit = 2;
    let projection = result_value(derive_unsigned_entitlement_snapshot(paid));
    assert_eq!(
        (
            projection.plan_tier,
            projection.base_child_device_limit,
            projection.effective_child_device_limit,
        ),
        (EntitlementSnapshotPlanTier::Paid, 2, 7)
    );
}

#[test]
fn rejects_hidden_provider_secret_child_metadata_and_referral_cash_fields() {
    let input = derivation_input();

    assert_unknown_field_rejected::<EntitlementSnapshotDerivationInput>(
        result_value(serde_json::to_value(&input)),
        "providerSecret",
    );
    assert_unknown_field_rejected::<EntitlementBillingLedgerState>(
        result_value(serde_json::to_value(input.billing_ledger_state)),
        "providerSecret",
    );
    assert_unknown_field_rejected::<EntitlementReferralLedgerState>(
        result_value(serde_json::to_value(input.referral_ledger_state)),
        "childActivity",
    );
    assert_unknown_field_rejected::<EntitlementReferralLedgerState>(
        result_value(serde_json::to_value(input.referral_ledger_state)),
        "cashValueCents",
    );
    assert_unknown_field_rejected::<EntitlementLedgerProjectionState>(
        result_value(serde_json::to_value(&input.entitlement_ledger_state)),
        "childTelemetry",
    );
    assert_unknown_field_rejected::<EntitlementProviderStateInput>(
        result_value(serde_json::to_value(&input.provider_state)),
        "providerToken",
    );
}

#[test]
fn unsigned_projection_rejects_signature_shaped_unknown_fields() {
    let projection = result_value(derive_unsigned_entitlement_snapshot(derivation_input()));
    let mut wire = result_value(serde_json::to_value(projection));
    object_fields_mut(&mut wire).insert(String::from("signature"), json!([0, 1, 2]));

    let error = result_error(serde_json::from_value::<
        UnsignedEntitlementSnapshotProjection,
    >(wire));
    assert_eq!(
        unknown_field_prefix(&error.to_string()),
        "unknown field `signature`"
    );
}

fn derivation_input() -> EntitlementSnapshotDerivationInput {
    EntitlementSnapshotDerivationInput {
        snapshot_id: result_value(EntitlementSnapshotId::parse("snapshot-pricing-wp01")),
        billing_ledger_state: EntitlementBillingLedgerState {
            subscription_state: SubscriptionState::Active,
            plan_tier: EntitlementSnapshotPlanTier::Starter,
            base_child_device_limit: 1,
            paid_extra_child_device_seats: 3,
        },
        referral_ledger_state: EntitlementReferralLedgerState {
            active_referral_credits: 2,
        },
        entitlement_ledger_state: EntitlementLedgerProjectionState {
            account_ref: result_value(EntitlementAccountRef::parse("account-pricing-wp01")),
            household_ref: result_value(EntitlementHouseholdRef::parse("household-pricing-wp01")),
            trusted_device_ref: result_value(EntitlementTrustedDeviceRef::parse(
                "device-pricing-wp01",
            )),
            feature_flags: vec![EntitlementSnapshotFeatureFlag {
                capability: EntitlementCapability::Tracking,
                enabled: true,
            }],
            revocation_cursor: result_value(EntitlementRevocationCursor::parse(
                "revocation-pricing-wp01",
            )),
            device_trust_required: true,
            package_build_ref: result_value(EntitlementPackageBuildRef::parse(
                "package-pricing-wp01",
            )),
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
        grace_until: None,
    }
}

fn assert_unknown_field_rejected<T>(mut wire: Value, field: &str)
where
    T: DeserializeOwned,
{
    object_fields_mut(&mut wire).insert(String::from(field), json!("private-value"));
    let error = result_error(serde_json::from_value::<T>(wire));
    assert_eq!(
        unknown_field_prefix(&error.to_string()),
        format!("unknown field `{field}`")
    );
}

fn unknown_field_prefix(message: &str) -> &str {
    message.split(',').next().unwrap_or(message)
}

fn object_fields(value: &Value) -> &serde_json::Map<String, Value> {
    match value.as_object() {
        Some(fields) => fields,
        None => std::process::abort(),
    }
}

fn object_fields_mut(value: &mut Value) -> &mut serde_json::Map<String, Value> {
    match value.as_object_mut() {
        Some(fields) => fields,
        None => std::process::abort(),
    }
}

fn result_value<T, E>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    }
}

fn result_error<T, E>(result: Result<T, E>) -> E {
    match result {
        Ok(_) => std::process::abort(),
        Err(error) => error,
    }
}

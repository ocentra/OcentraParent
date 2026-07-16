use std::fmt::Debug;

use ocentra_entitlement_core::entitlement_access::{
    evaluate_entitlement_capability, EntitlementCapability, EntitlementCapabilityAccessState,
    EntitlementCapabilityInput, EntitlementCapabilityRejectionReason, EntitlementCapabilityScope,
    EntitlementPolicyState, FamilySetupState, OfflineGraceState, SubscriptionState,
};
use ocentra_entitlement_core::entitlement_snapshot::{
    derive_signed_entitlement_snapshot, snapshot_context_from_signed_snapshot,
    EntitlementBillingLedgerState, EntitlementLedgerProjectionState, EntitlementProviderStateInput,
    EntitlementReferralLedgerState, EntitlementSnapshotDerivationInput,
    EntitlementSnapshotFeatureFlag, EntitlementSnapshotVerificationContext,
};
use ocentra_entitlement_core::entitlement_snapshot_values::{
    EntitlementAccountAuthorityState, EntitlementAccountRef, EntitlementDeviceTrustState,
    EntitlementHouseholdRef, EntitlementPackageBuildRef, EntitlementPackageBuildState,
    EntitlementProviderStateBoundary, EntitlementRevocationCursor, EntitlementSafetyFeatureState,
    EntitlementSignatureKeyId, EntitlementSnapshotBindingState, EntitlementSnapshotFreshnessState,
    EntitlementSnapshotId, EntitlementSnapshotPlanTier, EntitlementSnapshotSignatureState,
    EntitlementTrustedDeviceRef,
};

const ISSUED_AT: &str = "2026-06-28T20:00:00.000Z";
const EXPIRES_AT: &str = "2026-07-05T20:00:00.000Z";
const GRACE_UNTIL: &str = "2026-07-07T20:00:00.000Z";

fn verification_context(
    device_trust_state: EntitlementDeviceTrustState,
) -> EntitlementSnapshotVerificationContext {
    EntitlementSnapshotVerificationContext {
        signature_state: EntitlementSnapshotSignatureState::Trusted,
        freshness_state: EntitlementSnapshotFreshnessState::Fresh,
        household_binding_state: EntitlementSnapshotBindingState::Matched,
        device_binding_state: EntitlementSnapshotBindingState::Matched,
        device_trust_state,
        package_build_state: EntitlementPackageBuildState::Valid,
    }
}

#[test]
fn signed_snapshot_derives_effective_entitlement_from_billing_referral_and_entitlement_ledgers(
) -> Result<(), TestError> {
    let snapshot = derive_signed_entitlement_snapshot(derivation_input()?);

    assert_eq!(snapshot.schema_version, 1);
    assert_eq!(snapshot.snapshot_id.as_str(), "snapshot-household-default");
    assert_eq!(snapshot.account_ref.as_str(), "acct-default");
    assert_eq!(snapshot.household_ref.as_str(), "household-default");
    assert_eq!(snapshot.trusted_device_ref.as_str(), "device-default");
    assert_eq!(snapshot.plan_tier, EntitlementSnapshotPlanTier::Starter);
    assert_eq!(snapshot.base_child_device_limit, 2);
    assert_eq!(snapshot.active_referral_credits, 2);
    assert_eq!(snapshot.paid_extra_child_device_seats, 1);
    assert_eq!(snapshot.effective_child_device_limit, 5);
    assert_eq!(snapshot.limits.child_device_limit, 5);
    assert_eq!(snapshot.issued_at, ISSUED_AT);
    assert_eq!(snapshot.expires_at, EXPIRES_AT);
    assert_eq!(snapshot.grace_until.as_deref(), Some(GRACE_UNTIL));
    assert!(snapshot.livemode);
    assert_eq!(
        snapshot.revocation_cursor.as_str(),
        "revoke-cursor-2026-06-28"
    );
    assert!(snapshot.device_trust_required);
    assert_eq!(snapshot.package_build_ref.as_str(), "windows-msi-stable");
    assert_eq!(
        snapshot.signature_key_id.as_str(),
        "entitlement-key-2026-06"
    );
    assert_eq!(snapshot.signature, "signed-payload");
    assert_eq!(
        snapshot.feature_flags,
        vec![
            EntitlementSnapshotFeatureFlag {
                capability: EntitlementCapability::Tracking,
                enabled: true,
            },
            EntitlementSnapshotFeatureFlag {
                capability: EntitlementCapability::RemoteAccess,
                enabled: false,
            },
        ]
    );

    Ok(())
}

#[test]
fn provider_state_is_input_only_and_never_replaces_ledger_owned_plan_or_limit_truth(
) -> Result<(), TestError> {
    let mut input = derivation_input()?;
    input.provider_state.provider_plan_tier_echo = Some(EntitlementSnapshotPlanTier::Paid);
    input.provider_state.provider_child_device_limit_hint = Some(42);

    let snapshot = derive_signed_entitlement_snapshot(input);

    assert_eq!(snapshot.plan_tier, EntitlementSnapshotPlanTier::Starter);
    assert_eq!(snapshot.effective_child_device_limit, 5);
    assert_eq!(snapshot.limits.child_device_limit, 5);

    Ok(())
}

#[test]
fn snapshot_context_requires_local_device_trust_when_snapshot_model_marks_it_required(
) -> Result<(), TestError> {
    let snapshot = derive_signed_entitlement_snapshot(derivation_input()?);
    let snapshot_context = snapshot_context_from_signed_snapshot(
        &snapshot,
        verification_context(EntitlementDeviceTrustState::Missing),
    );

    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::Tracking,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
        snapshot_context,
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingDeviceTrust)
    );

    Ok(())
}

#[test]
fn snapshot_context_allows_missing_device_trust_when_snapshot_model_does_not_require_it(
) -> Result<(), TestError> {
    let mut input = derivation_input()?;
    input.entitlement_ledger_state.device_trust_required = false;

    let snapshot = derive_signed_entitlement_snapshot(input);
    let snapshot_context = snapshot_context_from_signed_snapshot(
        &snapshot,
        verification_context(EntitlementDeviceTrustState::Missing),
    );

    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::Tracking,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
        snapshot_context,
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Allowed
    );
    assert_eq!(decision.rejection_reason, None);

    Ok(())
}

fn value_or_test_error<T, E>(result: Result<T, E>) -> Result<T, TestError>
where
    E: Debug,
{
    result.map_err(|error| TestError(format!("{error:?}")))
}

fn derivation_input() -> Result<EntitlementSnapshotDerivationInput, TestError> {
    Ok(EntitlementSnapshotDerivationInput {
        snapshot_id: value_or_test_error(EntitlementSnapshotId::parse(
            "snapshot-household-default",
        ))?,
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
            account_ref: value_or_test_error(EntitlementAccountRef::parse("acct-default"))?,
            household_ref: value_or_test_error(EntitlementHouseholdRef::parse(
                "household-default",
            ))?,
            trusted_device_ref: value_or_test_error(EntitlementTrustedDeviceRef::parse(
                "device-default",
            ))?,
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
            revocation_cursor: value_or_test_error(EntitlementRevocationCursor::parse(
                "revoke-cursor-2026-06-28",
            ))?,
            device_trust_required: true,
            package_build_ref: value_or_test_error(EntitlementPackageBuildRef::parse(
                "windows-msi-stable",
            ))?,
            account_authority_state: EntitlementAccountAuthorityState::VerifiedAccountHandoff,
            safety_feature_state: EntitlementSafetyFeatureState::PreservedOutsidePaidGates,
        },
        provider_state: EntitlementProviderStateInput {
            authority_boundary: EntitlementProviderStateBoundary::InputOnly,
            livemode: true,
            provider_plan_tier_echo: Some(EntitlementSnapshotPlanTier::Paid),
            provider_child_device_limit_hint: Some(99),
        },
        issued_at: String::from(ISSUED_AT),
        expires_at: String::from(EXPIRES_AT),
        grace_until: Some(String::from(GRACE_UNTIL)),
        signature_key_id: value_or_test_error(EntitlementSignatureKeyId::parse(
            "entitlement-key-2026-06",
        ))?,
        signature: String::from("signed-payload"),
    })
}

#[derive(Debug)]
struct TestError(String);

impl std::fmt::Display for TestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for TestError {}

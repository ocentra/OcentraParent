use ocentra_schema::billing_entitlement_proof::{
    billing_entitlement_proof_is_honest, BillingDeviceLimitDecisionWitness,
    BillingEntitlementProofWitness, BillingFailureStateWitness,
    BillingSubscriptionStatusProofRowWitness, BillingSyncEventWitness,
};
use ocentra_schema::billing_entitlement_proof_ts::billing_entitlement_proof_typescript;

#[test]
fn billing_entitlement_proof_typescript_artifact_stays_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-billing-entitlement-proof.ts"
    );
    let generated = billing_entitlement_proof_typescript();

    assert_eq!(checked_in, generated);
    assert_generated_line_eq(
        &generated,
        "export const billingEntitlementRequiredNonClaims = [",
        "export const billingEntitlementRequiredNonClaims = [",
    );
    assert_generated_line_eq(
        &generated,
        "export function billingEntitlementProofIsHonest(",
        "export function billingEntitlementProofIsHonest(proof: {",
    );
}

fn assert_generated_line_eq(source: &str, line_start: &str, expected: &str) {
    let line = match source
        .lines()
        .find(|line| line.trim_start().starts_with(line_start))
    {
        Some(line) => line,
        None => std::process::abort(),
    };

    assert_eq!(line, expected);
}

#[test]
fn billing_entitlement_proof_is_honest_requires_all_claims_and_thresholds() {
    let honest = sample_witness();
    assert!(billing_entitlement_proof_is_honest(&honest));

    let missing_claim = BillingEntitlementProofWitness {
        non_claims: &[
            "no-stripe-sdk",
            "no-billing-provider-backend",
            "no-provider-token-custody",
            "no-child-activity-custody",
            "no-safety-shutdown",
        ],
        ..honest
    };
    assert!(!billing_entitlement_proof_is_honest(&missing_claim));

    let insufficient_threshold = BillingEntitlementProofWitness {
        failure_states: &[BillingFailureStateWitness {
            retain_evidence_export_access: true,
        }],
        ..honest
    };
    assert!(!billing_entitlement_proof_is_honest(
        &insufficient_threshold
    ));
}

#[test]
fn billing_entitlement_proof_rejects_mismatched_referral_and_missing_status_rows() {
    let mismatched_referral = BillingEntitlementProofWitness {
        referral_credit_summary_active_referral_credits: 1,
        ..sample_witness()
    };
    assert!(!billing_entitlement_proof_is_honest(&mismatched_referral));

    static MISSING_UNAVAILABLE: [BillingSubscriptionStatusProofRowWitness<'static>; 5] = [
        subscription_status_row("trialing", "available", "unchanged", "allow-new-device"),
        subscription_status_row("active", "available", "unchanged", "allow-new-device"),
        subscription_status_row("past-due", "past-due", "local-only", "deny-new-device"),
        subscription_status_row("expired", "locked", "local-only", "deny-new-device"),
        subscription_status_row(
            "grace",
            "grace",
            "grace-with-local-safety",
            "grace-existing-devices",
        ),
    ];
    let missing_status = BillingEntitlementProofWitness {
        subscription_status_proof_rows: &MISSING_UNAVAILABLE,
        ..sample_witness()
    };
    assert!(!billing_entitlement_proof_is_honest(&missing_status));
}

#[test]
fn billing_entitlement_proof_rejects_failure_states_that_remove_safety_evidence() {
    static UNSAFE_FAILURE_STATES: [BillingFailureStateWitness; 3] = [
        BillingFailureStateWitness {
            retain_evidence_export_access: true,
        },
        BillingFailureStateWitness {
            retain_evidence_export_access: false,
        },
        BillingFailureStateWitness {
            retain_evidence_export_access: true,
        },
    ];
    let proof = BillingEntitlementProofWitness {
        failure_states: &UNSAFE_FAILURE_STATES,
        ..sample_witness()
    };

    assert!(!billing_entitlement_proof_is_honest(&proof));
}

#[test]
fn billing_entitlement_proof_rejects_unsafe_sync_events_and_impossible_plan_limits() {
    static UNSAFE_SYNC_EVENTS: [BillingSyncEventWitness; 1] = [BillingSyncEventWitness {
        failure_state_retain_evidence_export_access: Some(false),
    }];
    let unsafe_sync = BillingEntitlementProofWitness {
        billing_sync_events: &UNSAFE_SYNC_EVENTS,
        ..sample_witness()
    };
    assert!(!billing_entitlement_proof_is_honest(&unsafe_sync));

    static ZERO_PLAN_LIMIT: [BillingDeviceLimitDecisionWitness<'static>; 1] =
        [device_limit_decision(
            "denied",
            0,
            0,
            false,
            "deny-new-device",
            "local-only",
        )];
    let impossible_limit = BillingEntitlementProofWitness {
        device_limit_decisions: &ZERO_PLAN_LIMIT,
        ..sample_witness()
    };
    assert!(!billing_entitlement_proof_is_honest(&impossible_limit));
}

#[test]
fn billing_entitlement_proof_rejects_label_only_grace_and_unsafe_limit_behavior() {
    static LABEL_ONLY_GRACE: [BillingSubscriptionStatusProofRowWitness<'static>; 6] = [
        subscription_status_row("trialing", "available", "unchanged", "allow-new-device"),
        subscription_status_row("active", "available", "unchanged", "allow-new-device"),
        subscription_status_row("past-due", "past-due", "local-only", "deny-new-device"),
        subscription_status_row("expired", "locked", "local-only", "deny-new-device"),
        subscription_status_row("grace", "available", "unchanged", "allow-new-device"),
        subscription_status_row(
            "unavailable",
            "unavailable",
            "manual-review-with-local-safety",
            "manual-review-required",
        ),
    ];
    let label_only_grace = BillingEntitlementProofWitness {
        subscription_status_proof_rows: &LABEL_ONLY_GRACE,
        ..sample_witness()
    };
    assert!(!billing_entitlement_proof_is_honest(&label_only_grace));

    static ALLOW_NEW_OVER_LIMIT: [BillingDeviceLimitDecisionWitness<'static>; 2] = [
        device_limit_decision("denied", 5, 5, false, "allow-new-device", "local-only"),
        device_limit_decision(
            "grace",
            5,
            5,
            true,
            "grace-existing-devices",
            "grace-with-local-safety",
        ),
    ];
    let allow_new_over_limit = BillingEntitlementProofWitness {
        device_limit_decisions: &ALLOW_NEW_OVER_LIMIT,
        ..sample_witness()
    };
    assert!(!billing_entitlement_proof_is_honest(&allow_new_over_limit));

    static UNSAFE_GRACE: [BillingDeviceLimitDecisionWitness<'static>; 2] = [
        device_limit_decision("denied", 5, 5, false, "deny-new-device", "local-only"),
        device_limit_decision("grace", 5, 5, true, "grace-existing-devices", "local-only"),
    ];
    let unsafe_grace = BillingEntitlementProofWitness {
        device_limit_decisions: &UNSAFE_GRACE,
        ..sample_witness()
    };
    assert!(!billing_entitlement_proof_is_honest(&unsafe_grace));
}

fn sample_witness() -> BillingEntitlementProofWitness<'static> {
    static BILLING_SYNC_EVENTS: [BillingSyncEventWitness; 2] = [
        BillingSyncEventWitness {
            failure_state_retain_evidence_export_access: None,
        },
        BillingSyncEventWitness {
            failure_state_retain_evidence_export_access: Some(true),
        },
    ];
    static FAILURE_STATES: [BillingFailureStateWitness; 3] = [
        BillingFailureStateWitness {
            retain_evidence_export_access: true,
        },
        BillingFailureStateWitness {
            retain_evidence_export_access: true,
        },
        BillingFailureStateWitness {
            retain_evidence_export_access: true,
        },
    ];
    static NON_CLAIMS: [&str; 6] = [
        "no-stripe-sdk",
        "no-billing-provider-backend",
        "no-provider-token-custody",
        "no-child-activity-custody",
        "no-safety-shutdown",
        "no-portal-ui",
    ];
    static SUBSCRIPTION_STATUS_PROOF_ROWS: [BillingSubscriptionStatusProofRowWitness<'static>; 6] = [
        subscription_status_row("trialing", "available", "unchanged", "allow-new-device"),
        subscription_status_row("active", "available", "unchanged", "allow-new-device"),
        subscription_status_row("past-due", "past-due", "local-only", "deny-new-device"),
        subscription_status_row("expired", "locked", "local-only", "deny-new-device"),
        subscription_status_row(
            "grace",
            "grace",
            "grace-with-local-safety",
            "grace-existing-devices",
        ),
        subscription_status_row(
            "unavailable",
            "unavailable",
            "manual-review-with-local-safety",
            "manual-review-required",
        ),
    ];
    static DEVICE_LIMIT_DECISIONS: [BillingDeviceLimitDecisionWitness<'static>; 2] = [
        device_limit_decision("denied", 5, 5, false, "deny-new-device", "local-only"),
        device_limit_decision(
            "grace",
            5,
            5,
            true,
            "grace-existing-devices",
            "grace-with-local-safety",
        ),
    ];

    BillingEntitlementProofWitness {
        entitlement_snapshot_active_referral_credits: 2,
        referral_credit_summary_active_referral_credits: 2,
        billing_sync_events: &BILLING_SYNC_EVENTS,
        failure_states: &FAILURE_STATES,
        non_claims: &NON_CLAIMS,
        subscription_status_proof_rows: &SUBSCRIPTION_STATUS_PROOF_ROWS,
        device_limit_decisions: &DEVICE_LIMIT_DECISIONS,
    }
}

const fn subscription_status_row(
    subscription_status: &'static str,
    parent_visible_state: &'static str,
    local_safety_behavior: &'static str,
    device_activation_behavior: &'static str,
) -> BillingSubscriptionStatusProofRowWitness<'static> {
    BillingSubscriptionStatusProofRowWitness {
        subscription_status,
        parent_visible_state,
        local_safety_behavior,
        device_activation_behavior,
    }
}

const fn device_limit_decision(
    decision: &'static str,
    active_device_count: u32,
    plan_device_limit: u32,
    requested_device_already_trusted: bool,
    device_activation_behavior: &'static str,
    existing_local_safety_behavior: &'static str,
) -> BillingDeviceLimitDecisionWitness<'static> {
    BillingDeviceLimitDecisionWitness {
        decision,
        reason_code: "limit-exceeded",
        active_device_count,
        plan_device_limit,
        requested_device_already_trusted,
        device_activation_behavior,
        existing_local_safety_behavior,
    }
}

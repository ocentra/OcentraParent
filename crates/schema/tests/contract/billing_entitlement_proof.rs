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
            retain_evidence_export_access: false,
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
        BillingSubscriptionStatusProofRowWitness {
            subscription_status: "trialing",
        },
        BillingSubscriptionStatusProofRowWitness {
            subscription_status: "active",
        },
        BillingSubscriptionStatusProofRowWitness {
            subscription_status: "past-due",
        },
        BillingSubscriptionStatusProofRowWitness {
            subscription_status: "expired",
        },
        BillingSubscriptionStatusProofRowWitness {
            subscription_status: "grace",
        },
        BillingSubscriptionStatusProofRowWitness {
            subscription_status: "unavailable",
        },
    ];
    static DEVICE_LIMIT_DECISIONS: [BillingDeviceLimitDecisionWitness<'static>; 1] =
        [BillingDeviceLimitDecisionWitness {
            decision: "denied",
            reason_code: "limit-exceeded",
            active_device_count: 5,
            plan_device_limit: 5,
        }];

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

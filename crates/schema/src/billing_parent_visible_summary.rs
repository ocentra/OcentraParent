use serde_json::{json, Value};

pub fn billing_parent_visible_summary_read_model() -> Value {
    json!({
        "parentAccountId": "parent-account-billing-entitlement-proof-1",
        "familyId": "family-billing-entitlement-proof-1",
        "currentPlanId": "family-plus-monthly",
        "currentSubscriptionStatus": "active",
        "childDeviceUsage": child_device_usage_summary(),
        "visibleFailureCounts": visible_failure_counts_summary(),
        "snapshotStates": snapshot_states_summary(),
        "deviceConsumptionStates": device_consumption_states_summary(),
        "seatComposition": seat_composition_summary(),
        "referralCreditSummary": referral_credit_summary(),
        "licenseSnapshot": license_snapshot_summary(),
        "invoiceSummary": invoice_summary(),
        "portalHandoff": portal_handoff_summary(),
        "changePlanAction": change_plan_action_summary(),
        "cancellationAction": cancellation_action_summary(),
        "safetyNonClaims": safety_non_claims_summary(),
    })
}

fn child_device_usage_summary() -> Value {
    json!({
        "limit": 5,
        "activeCount": 2,
    })
}

fn visible_failure_counts_summary() -> Value {
    json!({
        "provider-unavailable": 1,
        "network-unavailable": 1,
        "stale-snapshot": 1,
        "payment-required": 1,
        "account-mismatch": 1,
        "validation-failed": 1,
    })
}

fn snapshot_states_summary() -> Value {
    json!({
        "snapshot-active": 1,
        "snapshot-stale": 1,
        "payment-required": 1,
        "provider-unavailable": 1,
        "manual-review": 1,
    })
}

fn device_consumption_states_summary() -> Value {
    json!({
        "accepted-local": 1,
        "accepted-grace": 1,
        "blocked-new-device": 1,
        "manual-required": 1,
        "unavailable-local-safety": 0,
    })
}

fn seat_composition_summary() -> Value {
    json!({
        "baseChildDeviceLimit": 1,
        "activeReferralCredits": 2,
        "paidExtraChildDeviceSeats": 2,
        "effectiveChildDeviceLimit": 5,
    })
}

fn referral_credit_summary() -> Value {
    json!({
        "activeQualifiedReferralParents": 2,
        "activeReferralCredits": 2,
        "pendingReferralInvites": 1,
        "revokedReferralCredits": 1,
    })
}

fn license_snapshot_summary() -> Value {
    json!({
        "source": "signed-local-snapshot",
        "signatureState": "schema-valid-local",
        "subscriptionStatus": "active",
        "parentVisibleState": "available",
        "localSafetyBehavior": "unchanged",
        "generatedAt": "2026-06-03T09:57:32.000Z",
        "expiresAt": "2026-06-10T09:57:32.000Z",
        "failureKind": null,
    })
}

fn invoice_summary() -> Value {
    json!({
        "visibilityStates": {
            "customer-portal-hosted": 11,
            "download-link-issued": 1,
            "manual-support-required": 3,
        },
        "recoveryStates": {
            "active": 6,
            "trialing": 0,
            "past-due": 0,
            "grace": 2,
            "cancelled": 3,
            "unpaid": 1,
            "support-required": 3,
        },
        "hostedInvoiceSurface": "customer-portal-hosted-only",
        "providerMode": "stripe-hosted",
        "nextRenewalAt": "2026-07-14T00:00:00.000Z",
        "manualInvoiceState": {
            "visible": true,
            "manualSupportRequiredCount": 3,
            "manualReviewStateCount": 3,
        },
    })
}

fn portal_handoff_summary() -> Value {
    json!({
        "sessionKind": "billing-portal-session-create",
        "returnPath": "/family/billing/manage",
        "hostedUrlVisible": true,
    })
}

fn change_plan_action_summary() -> Value {
    json!({
        "selfServiceVisible": true,
        "managedBy": "billing-portal-session-create",
        "currentPlanId": "family-plus-monthly",
        "returnPath": "/family/billing/manage",
    })
}

fn cancellation_action_summary() -> Value {
    json!({
        "selfServiceVisible": true,
        "currentSubscriptionStatus": "active",
        "immediate": {
            "recoveryState": "cancelled",
            "parentVisibleState": "locked",
        },
        "periodEnd": {
            "recoveryState": "grace",
            "parentVisibleState": "grace",
        },
    })
}

fn safety_non_claims_summary() -> Value {
    json!({
        "noChildActivityCustody": true,
        "noPortalUi": true,
        "noProductionBillingClaim": true,
    })
}

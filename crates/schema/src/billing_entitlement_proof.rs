pub const BILLING_ENTITLEMENT_REQUIRED_NON_CLAIMS: [&str; 6] = [
    "no-stripe-sdk",
    "no-billing-provider-backend",
    "no-provider-token-custody",
    "no-child-activity-custody",
    "no-safety-shutdown",
    "no-portal-ui",
];

pub const BILLING_ENTITLEMENT_REQUIRED_SUBSCRIPTION_STATUSES: [&str; 6] = [
    "trialing",
    "active",
    "past-due",
    "expired",
    "grace",
    "unavailable",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BillingEntitlementProofWitness<'a> {
    pub entitlement_snapshot_active_referral_credits: u32,
    pub referral_credit_summary_active_referral_credits: u32,
    pub billing_sync_events: &'a [BillingSyncEventWitness],
    pub failure_states: &'a [BillingFailureStateWitness],
    pub non_claims: &'a [&'a str],
    pub subscription_status_proof_rows: &'a [BillingSubscriptionStatusProofRowWitness<'a>],
    pub device_limit_decisions: &'a [BillingDeviceLimitDecisionWitness<'a>],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BillingSyncEventWitness {
    pub failure_state_retain_evidence_export_access: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BillingFailureStateWitness {
    pub retain_evidence_export_access: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BillingSubscriptionStatusProofRowWitness<'a> {
    pub subscription_status: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BillingDeviceLimitDecisionWitness<'a> {
    pub decision: &'a str,
    pub reason_code: &'a str,
    pub active_device_count: u32,
    pub plan_device_limit: u32,
}

pub fn billing_entitlement_proof_is_honest(proof: &BillingEntitlementProofWitness<'_>) -> bool {
    proof.referral_credit_summary_active_referral_credits
        == proof.entitlement_snapshot_active_referral_credits
        && BILLING_ENTITLEMENT_REQUIRED_NON_CLAIMS
            .iter()
            .all(|claim| proof.non_claims.contains(claim))
        && BILLING_ENTITLEMENT_REQUIRED_SUBSCRIPTION_STATUSES
            .iter()
            .all(|status| {
                proof
                    .subscription_status_proof_rows
                    .iter()
                    .any(|row| row.subscription_status == *status)
            })
        && proof.failure_states.len() >= 3
        && proof.device_limit_decisions.iter().any(|decision| {
            decision.decision == "denied"
                && decision.reason_code == "limit-exceeded"
                && decision.active_device_count >= decision.plan_device_limit
        })
        && proof.billing_sync_events.iter().all(|event| {
            event
                .failure_state_retain_evidence_export_access
                .unwrap_or(true)
        })
}

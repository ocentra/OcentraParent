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
    pub parent_visible_state: &'a str,
    pub local_safety_behavior: &'a str,
    pub device_activation_behavior: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BillingDeviceLimitDecisionWitness<'a> {
    pub decision: &'a str,
    pub reason_code: &'a str,
    pub active_device_count: u32,
    pub plan_device_limit: u32,
    pub requested_device_already_trusted: bool,
    pub device_activation_behavior: &'a str,
    pub existing_local_safety_behavior: &'a str,
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
        && proof
            .failure_states
            .iter()
            .all(|failure| failure.retain_evidence_export_access)
        && proof
            .subscription_status_proof_rows
            .iter()
            .any(subscription_status_row_proves_grace)
        && proof
            .device_limit_decisions
            .iter()
            .any(device_limit_decision_proves_denial)
        && proof
            .device_limit_decisions
            .iter()
            .any(device_limit_decision_proves_grace)
        && proof.billing_sync_events.iter().all(|event| {
            event
                .failure_state_retain_evidence_export_access
                .unwrap_or(true)
        })
}

fn subscription_status_row_proves_grace(
    row: &BillingSubscriptionStatusProofRowWitness<'_>,
) -> bool {
    row.subscription_status == "grace"
        && row.parent_visible_state == "grace"
        && row.local_safety_behavior == "grace-with-local-safety"
        && row.device_activation_behavior == "grace-existing-devices"
}

fn device_limit_decision_proves_denial(decision: &BillingDeviceLimitDecisionWitness<'_>) -> bool {
    device_limit_decision_is_over_limit(decision)
        && !decision.requested_device_already_trusted
        && decision.decision == "denied"
        && decision.device_activation_behavior == "deny-new-device"
        && local_safety_behavior_is_preserved(decision.existing_local_safety_behavior)
}

fn device_limit_decision_proves_grace(decision: &BillingDeviceLimitDecisionWitness<'_>) -> bool {
    device_limit_decision_is_over_limit(decision)
        && decision.requested_device_already_trusted
        && decision.decision == "grace"
        && decision.device_activation_behavior == "grace-existing-devices"
        && decision.existing_local_safety_behavior == "grace-with-local-safety"
}

fn device_limit_decision_is_over_limit(decision: &BillingDeviceLimitDecisionWitness<'_>) -> bool {
    decision.plan_device_limit > 0
        && decision.reason_code == "limit-exceeded"
        && decision.active_device_count >= decision.plan_device_limit
}

fn local_safety_behavior_is_preserved(behavior: &str) -> bool {
    matches!(
        behavior,
        "local-only" | "grace-with-local-safety" | "manual-review-with-local-safety"
    )
}

use super::billing_entitlement_proof::{
    BILLING_ENTITLEMENT_REQUIRED_NON_CLAIMS, BILLING_ENTITLEMENT_REQUIRED_SUBSCRIPTION_STATUSES,
};

const BILLING_ENTITLEMENT_PROOF_TEMPLATE: &str = r#"/* generated from crates/schema/src/billing_entitlement_proof_ts.rs */

import type { BillingFailureState } from './billing-entitlement';

export const billingEntitlementRequiredNonClaims = [
__BILLING_ENTITLEMENT_REQUIRED_NON_CLAIMS__
] as const;

export const billingEntitlementRequiredSubscriptionStatuses = [
__BILLING_ENTITLEMENT_REQUIRED_SUBSCRIPTION_STATUSES__
] as const;

export function billingEntitlementProofIsHonest(proof: {
  readonly entitlementSnapshot: {
    readonly activeReferralCredits: number;
  };
  readonly referralCreditSummary: {
    readonly activeReferralCredits: number;
  };
  readonly billingSyncEvents: ReadonlyArray<{ readonly failureState: BillingFailureState | null }>;
  readonly failureStates: ReadonlyArray<BillingFailureState>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly subscriptionStatusProofRows: ReadonlyArray<{ readonly subscriptionStatus: string }>;
  readonly deviceLimitDecisions: ReadonlyArray<{
    readonly decision: string;
    readonly reasonCode: string;
    readonly activeDeviceCount: number;
    readonly planDeviceLimit: number;
  }>;
}): boolean {
  return (
    proof.referralCreditSummary.activeReferralCredits === proof.entitlementSnapshot.activeReferralCredits &&
    billingEntitlementRequiredNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    billingEntitlementRequiredSubscriptionStatuses.every((status) =>
      proof.subscriptionStatusProofRows.some((row) => row.subscriptionStatus === status)
    ) &&
    proof.failureStates.length >= 3 &&
    proof.deviceLimitDecisions.some(
      (decision) =>
        decision.decision === 'denied' &&
        decision.reasonCode === 'limit-exceeded' &&
        decision.activeDeviceCount >= decision.planDeviceLimit
    ) &&
    proof.billingSyncEvents.every(
      (event) => event.failureState === null || event.failureState.retainEvidenceExportAccess
    )
  );
}
"#;

pub fn billing_entitlement_proof_typescript() -> String {
    let required_non_claims = BILLING_ENTITLEMENT_REQUIRED_NON_CLAIMS
        .iter()
        .map(|claim| format!("  '{}',", claim))
        .collect::<Vec<_>>()
        .join("\n");
    let required_subscription_statuses = BILLING_ENTITLEMENT_REQUIRED_SUBSCRIPTION_STATUSES
        .iter()
        .map(|status| format!("  '{}',", status))
        .collect::<Vec<_>>()
        .join("\n");

    BILLING_ENTITLEMENT_PROOF_TEMPLATE
        .replace(
            "__BILLING_ENTITLEMENT_REQUIRED_NON_CLAIMS__",
            &required_non_claims,
        )
        .replace(
            "__BILLING_ENTITLEMENT_REQUIRED_SUBSCRIPTION_STATUSES__",
            &required_subscription_statuses,
        )
}

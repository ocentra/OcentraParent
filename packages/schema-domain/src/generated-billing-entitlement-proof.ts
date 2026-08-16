/* generated from crates/schema/src/billing_entitlement_proof_ts.rs */

import type { BillingFailureState } from './billing-entitlement';

export const billingEntitlementRequiredNonClaims = [
  'no-stripe-sdk',
  'no-billing-provider-backend',
  'no-provider-token-custody',
  'no-child-activity-custody',
  'no-safety-shutdown',
  'no-portal-ui',
] as const;

export const billingEntitlementRequiredSubscriptionStatuses = [
  'trialing',
  'active',
  'past-due',
  'expired',
  'grace',
  'unavailable',
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

/* generated from crates/schema/src/billing_entitlement_runtime_proof_values_ts.rs */

export const GeneratedBillingEntitlementRuntimeSchemaVersion = 'billing-entitlement-runtime-proof' as const;

export const GeneratedBillingEntitlementRuntimeSnapshotStates = [
  'snapshot-active',
  'snapshot-stale',
  'payment-required',
  'provider-unavailable',
  'manual-review',
] as const;

export const GeneratedBillingEntitlementRuntimeSources = [
  'signed-local-snapshot',
  'account-runtime-boundary',
  'manual-support-review',
  'unavailable',
] as const;

export const GeneratedBillingEntitlementRuntimeOperations = [
  'account-entitlement-snapshot-consumption',
  'device-limit-decision-consumption',
  'billing-failure-state-consumption',
] as const;

export const GeneratedBillingEntitlementRuntimeConsumptionStates = [
  'accepted-local',
  'accepted-grace',
  'blocked-new-device',
  'manual-required',
  'unavailable-local-safety',
] as const;

export const GeneratedBillingEntitlementRuntimeProviderExecutionClaims = ['not-implemented'] as const;

export const GeneratedBillingEntitlementRuntimeProviderContactClaims = ['manual-required'] as const;

export const GeneratedBillingEntitlementRuntimeRefundCreditClaims = ['manual-required'] as const;

export const GeneratedBillingEntitlementRuntimeProductionBillingClaims = ['not-claimed'] as const;

export const GeneratedBillingEntitlementRuntimePortalUiClaims = ['not-implemented'] as const;

export const GeneratedBillingEntitlementRuntimeChildCustodyClaims = [
  'signed-snapshot-consumption-contract',
  'not-supported',
] as const;

export const GeneratedBillingEntitlementRuntimeNonClaims = [
  'no-stripe-sdk',
  'no-live-provider-execution',
  'no-provider-contact',
  'no-refund-credit-runtime',
  'no-child-activity-custody',
  'no-production-billing-claim',
  'no-portal-ui',
] as const;

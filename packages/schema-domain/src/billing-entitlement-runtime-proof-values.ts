import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';

export const BillingEntitlementRuntimeSchemaVersionSchema = withParser(
  Schema.Literal('billing-entitlement-runtime-proof')
);

export const BillingEntitlementRuntimeSnapshotStateSchema = withParser(
  Schema.Literal('snapshot-active', 'snapshot-stale', 'payment-required', 'provider-unavailable', 'manual-review')
);

export const BillingEntitlementRuntimeSourceSchema = withParser(
  Schema.Literal('signed-local-snapshot', 'account-runtime-boundary', 'manual-support-review', 'unavailable')
);

export const BillingEntitlementRuntimeOperationSchema = withParser(
  Schema.Literal(
    'account-entitlement-snapshot-consumption',
    'device-limit-decision-consumption',
    'billing-failure-state-consumption'
  )
);

export const BillingEntitlementRuntimeConsumptionStateSchema = withParser(
  Schema.Literal(
    'accepted-local',
    'accepted-grace',
    'blocked-new-device',
    'manual-required',
    'unavailable-local-safety'
  )
);

export const BillingEntitlementRuntimeProviderExecutionClaimSchema = withParser(Schema.Literal('not-implemented'));
export const BillingEntitlementRuntimeProviderContactClaimSchema = withParser(Schema.Literal('manual-required'));
export const BillingEntitlementRuntimeRefundCreditClaimSchema = withParser(Schema.Literal('manual-required'));
export const BillingEntitlementRuntimeProductionBillingClaimSchema = withParser(Schema.Literal('not-claimed'));
export const BillingEntitlementRuntimePortalUiClaimSchema = withParser(Schema.Literal('not-implemented'));
export const BillingEntitlementRuntimeChildCustodyClaimSchema = withParser(
  Schema.Literal('signed-snapshot-consumption-contract', 'not-supported')
);

export const BillingEntitlementRuntimeNonClaimSchema = withParser(
  Schema.Literal(
    'no-stripe-sdk',
    'no-live-provider-execution',
    'no-provider-contact',
    'no-refund-credit-runtime',
    'no-child-activity-custody',
    'no-production-billing-claim',
    'no-portal-ui'
  )
);

export const BillingEntitlementRuntimeBoundaryIdSchema = brandedNonEmptyStringSchema(
  'BillingEntitlementRuntimeBoundaryId'
);

export const BillingEntitlementRuntimeAuditReferenceSchema = brandedNonEmptyStringSchema(
  'BillingEntitlementRuntimeAuditReference'
);

export type BillingEntitlementRuntimeSnapshotState = Infer<typeof BillingEntitlementRuntimeSnapshotStateSchema>;
export type BillingEntitlementRuntimeOperation = Infer<typeof BillingEntitlementRuntimeOperationSchema>;
export type BillingEntitlementRuntimeConsumptionState = Infer<typeof BillingEntitlementRuntimeConsumptionStateSchema>;
export type BillingEntitlementRuntimeNonClaim = Infer<typeof BillingEntitlementRuntimeNonClaimSchema>;

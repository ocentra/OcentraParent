/* generated from crates/schema/src/billing_entitlement_values_ts.rs */

import { Schema, withParser, brandedNonEmptyStringSchema } from './effect';

export const PositiveBillingLimitSchema = Schema.Number.pipe(
  Schema.filter((value) => (Number.isInteger(value) && value > 0) || 'Expected billing limits to be positive integers')
);
export const NonNegativeBillingCountSchema = Schema.Number.pipe(
  Schema.filter(
    (value) => (Number.isInteger(value) && value >= 0) || 'Expected billing counts to be non-negative integers'
  )
);

export const BillingEntitlementSchemaVersionSchema = withParser(Schema.Literal('billing-entitlement-contract-proof'));
export const BillingPlanActiveStateSchema = withParser(
  Schema.Literal('active', 'trial-only', 'retired', 'manual-required')
);
export const BillingIncludedParentPortalCountSchema = withParser(Schema.Literal(1));
export const BillingExtraParentSlotStateSchema = withParser(
  Schema.Literal('not-included', 'manual-required', 'separate-paid-addon')
);
export const BillingSubscriptionStatusSchema = withParser(
  Schema.Literal('trialing', 'active', 'past-due', 'cancelled', 'expired', 'grace', 'unknown', 'unavailable')
);
export const BillingEntitlementSourceSchema = withParser(
  Schema.Literal('billing-backend', 'signed-local-snapshot', 'manual-admin-review', 'unavailable')
);
export const BillingSignatureStateSchema = withParser(
  Schema.Literal('signed', 'schema-valid-local', 'manual-required', 'unavailable')
);
export const BillingEntitlementDecisionStateSchema = withParser(
  Schema.Literal('available', 'locked', 'grace', 'local-only', 'manual-required', 'unavailable')
);
export const BillingLocalSafetyBehaviorSchema = withParser(
  Schema.Literal('unchanged', 'local-only', 'grace-with-local-safety', 'manual-review-with-local-safety')
);
export const BillingChildActivityCustodySchema = withParser(Schema.Literal('not-included'));
export const BillingEvidenceExportAccessSchema = withParser(Schema.Literal('retained'));
export const BillingProviderBoundarySchema = withParser(Schema.Literal('backend-reference-only', 'none'));
export const BillingDeviceLimitDecisionStateSchema = withParser(
  Schema.Literal('allowed', 'denied', 'grace', 'manual-review')
);
export const BillingDeviceActivationBehaviorSchema = withParser(
  Schema.Literal('allow-new-device', 'deny-new-device', 'grace-existing-devices', 'manual-review-required')
);
export const BillingDeviceLimitReasonSchema = withParser(
  Schema.Literal(
    'within-plan',
    'limit-exceeded',
    'snapshot-stale',
    'billing-unavailable',
    'payment-required',
    'manual-review'
  )
);
export const BillingFailureKindSchema = withParser(
  Schema.Literal(
    'provider-unavailable',
    'network-unavailable',
    'stale-snapshot',
    'payment-required',
    'account-mismatch',
    'validation-failed'
  )
);
export const BillingParentVisibleStateSchema = withParser(
  Schema.Literal('available', 'locked', 'past-due', 'grace', 'stale', 'unavailable', 'manual-review')
);
export const BillingParentResolutionSchema = withParser(
  Schema.Literal('none', 'payment-update', 'manual-support-review', 'wait-for-provider')
);
export const BillingEntitlementNonClaimSchema = withParser(
  Schema.Literal(
    'no-stripe-sdk',
    'no-billing-provider-backend',
    'no-provider-token-custody',
    'no-child-activity-custody',
    'no-safety-shutdown',
    'no-portal-ui'
  )
);
export const BillingStripeSdkClaimSchema = withParser(Schema.Literal('not-included'));
export const BillingProviderBackendClaimSchema = withParser(Schema.Literal('not-implemented'));
export const BillingPortalUiClaimSchema = withParser(Schema.Literal('not-implemented'));
export const BillingChildActivityCustodyClaimSchema = withParser(Schema.Literal('not-included'));

export const BillingPlanIdSchema = brandedNonEmptyStringSchema('BillingPlanId');
export const BillingDisplayTextTokenSchema = brandedNonEmptyStringSchema('BillingDisplayTextToken');
export const BillingPriceReferenceSchema = brandedNonEmptyStringSchema('BillingPriceReference');
export const BillingFeatureCodeSchema = brandedNonEmptyStringSchema('BillingFeatureCode');
export const BillingEntitlementSnapshotIdSchema = brandedNonEmptyStringSchema('BillingEntitlementSnapshotId');
export const BillingReasonCodeSchema = brandedNonEmptyStringSchema('BillingReasonCode');
export const BillingSyncEventIdSchema = brandedNonEmptyStringSchema('BillingSyncEventId');
export const BillingProviderReferenceSchema = brandedNonEmptyStringSchema('BillingProviderReference');
export const BillingDeviceLimitDecisionIdSchema = brandedNonEmptyStringSchema('BillingDeviceLimitDecisionId');
export const BillingAuditReferenceSchema = brandedNonEmptyStringSchema('BillingAuditReference');

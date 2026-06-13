import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const BillingAccountRuntimeSchemaVersionSchema = withParser(
  Schema.Literal('billing-account-runtime-boundary-proof')
);
export const BillingAccountRuntimeStatusSchema = withParser(
  Schema.Literal('trialing', 'active', 'past-due', 'backend-unavailable', 'provider-unavailable', 'manual-review')
);
export const BillingAccountRuntimeSourceSchema = withParser(
  Schema.Literal('account-backend', 'signed-local-snapshot', 'manual-support-review', 'unavailable')
);
export const BillingAccountBackendRuntimeStateSchema = withParser(
  Schema.Literal('available', 'backend-unavailable', 'provider-unavailable', 'manual-required', 'not-implemented')
);
export const BillingAccountRuntimeOperationSchema = withParser(
  Schema.Literal(
    'account-status-read',
    'subscription-status-read',
    'entitlement-snapshot-read',
    'device-limit-decision-read',
    'download-status-read',
    'provider-webhook-sync'
  )
);
export const BillingAccountRuntimeParentVisibleStateSchema = withParser(
  Schema.Literal('available', 'past-due', 'stale', 'unavailable', 'manual-review')
);
export const BillingAccountRuntimeProviderBoundarySchema = withParser(Schema.Literal('backend-reference-only', 'none'));
export const BillingAccountRuntimeProviderSecretCustodySchema = withParser(Schema.Literal('not-present'));
export const BillingAccountRuntimeChildDeviceConsumptionSchema = withParser(
  Schema.Literal('signed-snapshot-consumed', 'manual-required', 'not-implemented')
);
export const BillingAccountRuntimeEntitlementSigningStateSchema = withParser(
  Schema.Literal('signed-snapshot-accepted', 'schema-valid-local', 'manual-required', 'unavailable')
);
export const BillingAccountRuntimeNonClaimSchema = withParser(
  Schema.Literal(
    'no-stripe-sdk',
    'no-provider-secrets',
    'no-billing-provider-runtime',
    'no-account-backend',
    'no-entitlement-signing-runtime',
    'no-portal-ui',
    'no-child-activity-custody'
  )
);

export const BillingAccountRuntimeStripeSdkClaimSchema = withParser(Schema.Literal('not-included'));
export const BillingAccountRuntimeProviderSecretClaimSchema = withParser(Schema.Literal('not-included'));
export const BillingAccountRuntimeBackendClaimSchema = withParser(Schema.Literal('not-implemented'));
export const BillingAccountRuntimePortalUiClaimSchema = withParser(Schema.Literal('not-implemented'));
export const BillingAccountRuntimeChildDeviceConsumptionClaimSchema = withParser(
  Schema.Literal('signed-snapshot-consumption-contract', 'not-supported')
);
export const BillingAccountRuntimeChildActivityCustodyClaimSchema = withParser(Schema.Literal('not-included'));

export const BillingAccountRuntimeBoundaryIdSchema = brandedNonEmptyStringSchema('BillingAccountRuntimeBoundaryId');
export const BillingAccountRuntimeAuditReferenceSchema = brandedNonEmptyStringSchema(
  'BillingAccountRuntimeAuditReference'
);

export type BillingAccountRuntimeStatus = Infer<typeof BillingAccountRuntimeStatusSchema>;
export type BillingAccountRuntimeOperation = Infer<typeof BillingAccountRuntimeOperationSchema>;
export type BillingAccountBackendRuntimeState = Infer<typeof BillingAccountBackendRuntimeStateSchema>;
export type BillingAccountRuntimeNonClaim = Infer<typeof BillingAccountRuntimeNonClaimSchema>;

export function summarizeBillingAccountRuntimeStatuses(
  rows: ReadonlyArray<{ readonly accountStatus: BillingAccountRuntimeStatus }>
): Record<BillingAccountRuntimeStatus, number> {
  const counts = {
    trialing: 0,
    active: 0,
    'past-due': 0,
    'backend-unavailable': 0,
    'provider-unavailable': 0,
    'manual-review': 0,
  };
  for (const row of rows) {
    counts[row.accountStatus] += 1;
  }
  return counts;
}

export function summarizeBillingAccountRuntimeOperations(
  rows: ReadonlyArray<{ readonly operation: BillingAccountRuntimeOperation }>
): Record<BillingAccountRuntimeOperation, number> {
  const counts = {
    'account-status-read': 0,
    'subscription-status-read': 0,
    'entitlement-snapshot-read': 0,
    'device-limit-decision-read': 0,
    'download-status-read': 0,
    'provider-webhook-sync': 0,
  };
  for (const row of rows) {
    counts[row.operation] += 1;
  }
  return counts;
}


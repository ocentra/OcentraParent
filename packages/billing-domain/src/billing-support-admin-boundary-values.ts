import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const BillingSupportAdminSchemaVersionSchema = withParser(
  Schema.Literal('billing-support-admin-boundary-proof')
);
export const BillingSupportAdminActionSchema = withParser(
  Schema.Literal(
    'support-case-triage',
    'account-status-review',
    'billing-escalation-request',
    'provider-contact-manual-required',
    'entitlement-admin-override-manual-required',
    'refund-credit-manual-required'
  )
);
export const BillingSupportAdminRuntimeStateSchema = withParser(
  Schema.Literal('read-only-local-proof', 'manual-required', 'not-implemented')
);
export const BillingSupportAdminDataClassSchema = withParser(
  Schema.Literal(
    'support-case-status-ref',
    'account-status-ref',
    'subscription-status-ref',
    'entitlement-snapshot-ref',
    'device-limit-decision-ref',
    'billing-failure-state-ref',
    'redaction-audit-ref'
  )
);
export const BillingSupportAdminNonClaimSchema = withParser(
  Schema.Literal(
    'no-stripe-sdk',
    'no-provider-secrets',
    'no-billing-provider-contact',
    'no-account-backend-admin-runtime',
    'no-entitlement-admin-override-runtime',
    'no-refund-credit-runtime',
    'no-portal-admin-ui',
    'no-support-backend-upload',
    'no-child-activity-custody'
  )
);
export const BillingSupportAdminProviderSecretCustodySchema = withParser(Schema.Literal('not-present'));
export const BillingSupportAdminPortalUiClaimSchema = withParser(Schema.Literal('not-implemented'));
export const BillingSupportAdminProviderContactClaimSchema = withParser(Schema.Literal('not-executed'));
export const BillingSupportAdminBackendUploadClaimSchema = withParser(Schema.Literal('not-executed'));
export const BillingSupportAdminChildActivityCustodyClaimSchema = withParser(Schema.Literal('not-supported'));

export const BillingSupportAdminBoundaryIdSchema = brandedNonEmptyStringSchema('BillingSupportAdminBoundaryId');
export const BillingSupportAdminCaseReferenceSchema = brandedNonEmptyStringSchema('BillingSupportAdminCaseReference');
export const BillingSupportAdminAuditReferenceSchema = brandedNonEmptyStringSchema('BillingSupportAdminAuditReference');

export type BillingSupportAdminAction = Infer<typeof BillingSupportAdminActionSchema>;
export type BillingSupportAdminRuntimeState = Infer<typeof BillingSupportAdminRuntimeStateSchema>;
export type BillingSupportAdminDataClass = Infer<typeof BillingSupportAdminDataClassSchema>;
export type BillingSupportAdminNonClaim = Infer<typeof BillingSupportAdminNonClaimSchema>;

export function summarizeBillingSupportAdminActions(
  rows: ReadonlyArray<{ readonly action: BillingSupportAdminAction }>
): Record<BillingSupportAdminAction, number> {
  const counts = {
    'support-case-triage': 0,
    'account-status-review': 0,
    'billing-escalation-request': 0,
    'provider-contact-manual-required': 0,
    'entitlement-admin-override-manual-required': 0,
    'refund-credit-manual-required': 0,
  };
  for (const row of rows) {
    counts[row.action] += 1;
  }
  return counts;
}

export function summarizeBillingSupportAdminRuntimeStates(
  rows: ReadonlyArray<{ readonly runtimeState: BillingSupportAdminRuntimeState }>
): Record<BillingSupportAdminRuntimeState, number> {
  const counts = {
    'read-only-local-proof': 0,
    'manual-required': 0,
    'not-implemented': 0,
  };
  for (const row of rows) {
    counts[row.runtimeState] += 1;
  }
  return counts;
}


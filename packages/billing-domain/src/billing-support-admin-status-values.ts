import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyBillingSupportAdminStatusText = Schema.String.pipe(Schema.minLength(1));

export const BillingSupportAdminStatusSchemaVersionSchema = withParser(
  Schema.Literal('billing-support-admin-status-proof')
);
export const BillingSupportAdminStatusRowSchema = withParser(
  Schema.Literal(
    'case-triage-visible',
    'account-review-visible',
    'billing-escalation-visible',
    'provider-contact-manual-required',
    'entitlement-override-manual-required',
    'refund-credit-manual-required',
    'resolution-update-ready'
  )
);
export const BillingSupportAdminStatusRuntimeStateSchema = withParser(
  Schema.Literal('source-contract-ready', 'manual-required', 'not-implemented')
);
export const BillingSupportAdminStatusDataClassSchema = withParser(
  Schema.Literal(
    'support-case-status-ref',
    'account-status-ref',
    'subscription-status-ref',
    'billing-failure-state-ref',
    'entitlement-snapshot-ref',
    'device-limit-decision-ref',
    'redaction-audit-ref',
    'manual-proof-ref'
  )
);
export const BillingSupportAdminStatusNonClaimSchema = withParser(
  Schema.Literal(
    'no-stripe-sdk',
    'no-provider-secrets',
    'no-billing-provider-contact-execution',
    'no-account-lookup-execution',
    'no-entitlement-admin-override-runtime',
    'no-refund-credit-runtime',
    'no-portal-admin-ui',
    'no-support-backend-upload',
    'no-child-activity-custody'
  )
);
export const BillingSupportAdminStatusProofRefSchema = withParser(
  Schema.Literal(
    'billing-support-admin-boundary-proof',
    'billing-entitlement-contract-proof',
    'billing-entitlement-runtime-proof',
    'billing-subscription-device-limit-proof',
    'billing-failure-state-proof'
  )
);
export const BillingSupportAdminStatusEvidenceExportAccessSchema = withParser(Schema.Literal('retained'));
export const BillingSupportAdminStatusLocalSafetyClaimSchema = withParser(Schema.Literal('continues'));
export const BillingSupportAdminStatusProviderClaimSchema = withParser(Schema.Literal('not-executed'));
export const BillingSupportAdminStatusPortalClaimSchema = withParser(Schema.Literal('not-implemented'));
export const BillingSupportAdminStatusChildActivityCustodyClaimSchema = withParser(Schema.Literal('not-supported'));

export const BillingSupportAdminStatusIdSchema = NonEmptyBillingSupportAdminStatusText.pipe(
  Schema.brand('BillingSupportAdminStatusId')
);
export const BillingSupportAdminStatusAuditReferenceSchema = NonEmptyBillingSupportAdminStatusText.pipe(
  Schema.brand('BillingSupportAdminStatusAuditReference')
);

export type BillingSupportAdminStatusRow = Infer<typeof BillingSupportAdminStatusRowSchema>;
export type BillingSupportAdminStatusRuntimeState = Infer<typeof BillingSupportAdminStatusRuntimeStateSchema>;
export type BillingSupportAdminStatusDataClass = Infer<typeof BillingSupportAdminStatusDataClassSchema>;
export type BillingSupportAdminStatusNonClaim = Infer<typeof BillingSupportAdminStatusNonClaimSchema>;
export type BillingSupportAdminStatusProofRef = Infer<typeof BillingSupportAdminStatusProofRefSchema>;

export function summarizeBillingSupportAdminStatusRows(
  rows: ReadonlyArray<{ readonly statusRow: BillingSupportAdminStatusRow }>
): Record<BillingSupportAdminStatusRow, number> {
  const counts = {
    'case-triage-visible': 0,
    'account-review-visible': 0,
    'billing-escalation-visible': 0,
    'provider-contact-manual-required': 0,
    'entitlement-override-manual-required': 0,
    'refund-credit-manual-required': 0,
    'resolution-update-ready': 0,
  };
  for (const row of rows) {
    counts[row.statusRow] += 1;
  }
  return counts;
}

export function summarizeBillingSupportAdminStatusRuntimeStates(
  rows: ReadonlyArray<{ readonly runtimeState: BillingSupportAdminStatusRuntimeState }>
): Record<BillingSupportAdminStatusRuntimeState, number> {
  const counts = {
    'source-contract-ready': 0,
    'manual-required': 0,
    'not-implemented': 0,
  };
  for (const row of rows) {
    counts[row.runtimeState] += 1;
  }
  return counts;
}

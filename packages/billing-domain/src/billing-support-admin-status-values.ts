import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  BillingSupportAdminChildActivityCustodyExcludedSchema,
  BillingSupportAdminEvidenceExportRetainedSchema,
  BillingSupportAdminLocalSafetyContinuesSchema,
  BillingSupportAdminPortalUiNotImplementedSchema,
  BillingSupportAdminProviderNotExecutedSchema,
  BillingSupportAdminSharedDataClassValues,
  BillingSupportAdminSharedNonClaimValues,
  summarizeBillingSupportAdminValues,
} from './billing-support-admin-common-values';

export const BillingSupportAdminStatusSchemaVersionSchema = withParser(
  Schema.Literal('billing-support-admin-status-proof')
);
const BillingSupportAdminStatusRowValues = [
  'case-triage-visible',
  'account-review-visible',
  'billing-escalation-visible',
  'provider-contact-manual-required',
  'entitlement-override-manual-required',
  'refund-credit-manual-required',
  'resolution-update-ready',
] as const;
const BillingSupportAdminStatusRuntimeStateValues = [
  'source-contract-ready',
  'manual-required',
  'not-implemented',
] as const;
const BillingSupportAdminStatusDataClassValues = [
  ...BillingSupportAdminSharedDataClassValues,
  'manual-proof-ref',
] as const;
const BillingSupportAdminStatusNonClaimValues = [
  ...BillingSupportAdminSharedNonClaimValues,
  'no-billing-provider-contact-execution',
  'no-account-lookup-execution',
] as const;
export const BillingSupportAdminStatusRowSchema = withParser(
  Schema.Literal(...BillingSupportAdminStatusRowValues)
);
export const BillingSupportAdminStatusRuntimeStateSchema = withParser(
  Schema.Literal(...BillingSupportAdminStatusRuntimeStateValues)
);
export const BillingSupportAdminStatusDataClassSchema = withParser(
  Schema.Literal(...BillingSupportAdminStatusDataClassValues)
);
export const BillingSupportAdminStatusNonClaimSchema = withParser(
  Schema.Literal(...BillingSupportAdminStatusNonClaimValues)
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
export const BillingSupportAdminStatusEvidenceExportAccessSchema = BillingSupportAdminEvidenceExportRetainedSchema;
export const BillingSupportAdminStatusLocalSafetyClaimSchema = BillingSupportAdminLocalSafetyContinuesSchema;
export const BillingSupportAdminStatusProviderClaimSchema = BillingSupportAdminProviderNotExecutedSchema;
export const BillingSupportAdminStatusPortalClaimSchema = BillingSupportAdminPortalUiNotImplementedSchema;
export const BillingSupportAdminStatusChildActivityCustodyClaimSchema =
  BillingSupportAdminChildActivityCustodyExcludedSchema;

export const BillingSupportAdminStatusIdSchema = brandedNonEmptyStringSchema('BillingSupportAdminStatusId');
export const BillingSupportAdminStatusAuditReferenceSchema = brandedNonEmptyStringSchema('BillingSupportAdminStatusAuditReference');

export type BillingSupportAdminStatusRow = Infer<typeof BillingSupportAdminStatusRowSchema>;
export type BillingSupportAdminStatusRuntimeState = Infer<typeof BillingSupportAdminStatusRuntimeStateSchema>;
export type BillingSupportAdminStatusDataClass = Infer<typeof BillingSupportAdminStatusDataClassSchema>;
export type BillingSupportAdminStatusNonClaim = Infer<typeof BillingSupportAdminStatusNonClaimSchema>;
export type BillingSupportAdminStatusProofRef = Infer<typeof BillingSupportAdminStatusProofRefSchema>;

export function summarizeBillingSupportAdminStatusRows(
  rows: ReadonlyArray<{ readonly statusRow: BillingSupportAdminStatusRow }>
): Record<BillingSupportAdminStatusRow, number> {
  return summarizeBillingSupportAdminValues(BillingSupportAdminStatusRowValues, rows, 'statusRow');
}

export function summarizeBillingSupportAdminStatusRuntimeStates(
  rows: ReadonlyArray<{ readonly runtimeState: BillingSupportAdminStatusRuntimeState }>
): Record<BillingSupportAdminStatusRuntimeState, number> {
  return summarizeBillingSupportAdminValues(BillingSupportAdminStatusRuntimeStateValues, rows, 'runtimeState');
}


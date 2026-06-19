import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  BillingSupportAdminAuditReferenceSchema as BillingSupportAdminSharedAuditReferenceSchema,
  BillingSupportAdminChildActivityCustodyExcludedSchema,
  BillingSupportAdminPortalUiNotImplementedSchema,
  BillingSupportAdminProviderNotExecutedSchema,
  BillingSupportAdminProviderSecretCustodyAbsentSchema,
  BillingSupportAdminSharedNonClaimValues,
  BillingSupportAdminSharedDataClassSchema,
  summarizeBillingSupportAdminValues,
} from './billing-support-admin-common-values';

export const BillingSupportAdminSchemaVersionSchema = withParser(
  Schema.Literal('billing-support-admin-boundary-proof')
);
const BillingSupportAdminActionValues = [
  'support-case-triage',
  'account-status-review',
  'billing-escalation-request',
  'provider-contact-manual-required',
  'entitlement-admin-override-manual-required',
  'refund-credit-manual-required',
] as const;
const BillingSupportAdminRuntimeStateValues = [
  'read-only-local-proof',
  'manual-required',
  'not-implemented',
] as const;
const BillingSupportAdminNonClaimValues = [
  ...BillingSupportAdminSharedNonClaimValues,
  'no-billing-provider-contact',
  'no-account-backend-admin-runtime',
] as const;
export const BillingSupportAdminActionSchema = withParser(
  Schema.Literal(...BillingSupportAdminActionValues)
);
export const BillingSupportAdminRuntimeStateSchema = withParser(
  Schema.Literal(...BillingSupportAdminRuntimeStateValues)
);
export const BillingSupportAdminDataClassSchema = BillingSupportAdminSharedDataClassSchema;
export const BillingSupportAdminNonClaimSchema = withParser(
  Schema.Literal(...BillingSupportAdminNonClaimValues)
);
export const BillingSupportAdminProviderSecretCustodySchema = BillingSupportAdminProviderSecretCustodyAbsentSchema;
export const BillingSupportAdminPortalUiClaimSchema = BillingSupportAdminPortalUiNotImplementedSchema;
export const BillingSupportAdminProviderContactClaimSchema = BillingSupportAdminProviderNotExecutedSchema;
export const BillingSupportAdminBackendUploadClaimSchema = BillingSupportAdminProviderNotExecutedSchema;
export const BillingSupportAdminChildActivityCustodyClaimSchema = BillingSupportAdminChildActivityCustodyExcludedSchema;

export const BillingSupportAdminBoundaryIdSchema = brandedNonEmptyStringSchema('BillingSupportAdminBoundaryId');
export const BillingSupportAdminCaseReferenceSchema = brandedNonEmptyStringSchema('BillingSupportAdminCaseReference');
export const BillingSupportAdminAuditReferenceSchema = BillingSupportAdminSharedAuditReferenceSchema;

export type BillingSupportAdminAction = Infer<typeof BillingSupportAdminActionSchema>;
export type BillingSupportAdminRuntimeState = Infer<typeof BillingSupportAdminRuntimeStateSchema>;
export type BillingSupportAdminDataClass = Infer<typeof BillingSupportAdminDataClassSchema>;
export type BillingSupportAdminNonClaim = Infer<typeof BillingSupportAdminNonClaimSchema>;

export function summarizeBillingSupportAdminActions(
  rows: ReadonlyArray<{ readonly action: BillingSupportAdminAction }>
): Record<BillingSupportAdminAction, number> {
  return summarizeBillingSupportAdminValues(BillingSupportAdminActionValues, rows, 'action');
}

export function summarizeBillingSupportAdminRuntimeStates(
  rows: ReadonlyArray<{ readonly runtimeState: BillingSupportAdminRuntimeState }>
): Record<BillingSupportAdminRuntimeState, number> {
  return summarizeBillingSupportAdminValues(BillingSupportAdminRuntimeStateValues, rows, 'runtimeState');
}


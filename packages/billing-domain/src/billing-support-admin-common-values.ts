import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  BillingFailureKindSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentResolutionSchema,
  BillingParentVisibleStateSchema,
} from './billing-entitlement-values';

export const BillingSupportAdminSharedDataClassValues = [
  'support-case-status-ref',
  'account-status-ref',
  'subscription-status-ref',
  'entitlement-snapshot-ref',
  'device-limit-decision-ref',
  'billing-failure-state-ref',
  'redaction-audit-ref',
] as const;

export const BillingSupportAdminSharedNonClaimValues = [
  'no-stripe-sdk',
  'no-provider-secrets',
  'no-entitlement-admin-override-runtime',
  'no-refund-credit-runtime',
  'no-portal-admin-ui',
  'no-support-backend-upload',
  'no-child-activity-custody',
] as const;

export const BillingSupportAdminSharedDataClassSchema = withParser(
  Schema.Literal(...BillingSupportAdminSharedDataClassValues)
);
export const BillingSupportAdminSharedNonClaimSchema = withParser(
  Schema.Literal(...BillingSupportAdminSharedNonClaimValues)
);
export const BillingSupportAdminProviderSecretCustodyAbsentSchema = withParser(Schema.Literal('not-present'));
export const BillingSupportAdminPortalUiNotImplementedSchema = withParser(Schema.Literal('not-implemented'));
export const BillingSupportAdminProviderNotExecutedSchema = withParser(Schema.Literal('not-executed'));
export const BillingSupportAdminChildActivityCustodyExcludedSchema = withParser(Schema.Literal('not-included'));
export const BillingSupportAdminEvidenceExportRetainedSchema = withParser(Schema.Literal('retained'));
export const BillingSupportAdminLocalSafetyContinuesSchema = withParser(Schema.Literal('continues'));

export const BillingSupportAdminAuditReferenceSchema = brandedNonEmptyStringSchema('BillingSupportAdminAuditReference');

export type BillingSupportAdminSharedDataClass = Infer<typeof BillingSupportAdminSharedDataClassSchema>;
export type BillingSupportAdminSharedNonClaim = Infer<typeof BillingSupportAdminSharedNonClaimSchema>;

export function summarizeBillingSupportAdminValues<const Value extends string, const Key extends string>(
  values: readonly Value[],
  rows: ReadonlyArray<{ readonly [Field in Key]: Value }>,
  key: Key
): Record<Value, number> {
  const counts = {} as Record<Value, number>;
  for (const value of values) {
    counts[value] = 0;
  }
  for (const row of rows) {
    counts[row[key]] += 1;
  }
  return counts;
}

export function buildBillingFailureStateSchema(failureLabel: string) {
  return withParser(
    Schema.Struct({
      failureKind: BillingFailureKindSchema,
      parentVisibleState: BillingParentVisibleStateSchema,
      localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
      retainEvidenceExportAccess: Schema.Boolean,
      existingLocalSafetyContinues: Schema.Boolean,
      parentResolution: BillingParentResolutionSchema,
      retryAllowed: Schema.Boolean,
      retryAfter: Schema.Union(ParentTimestampSchema, Schema.Null),
    }).pipe(
      Schema.filter(
        (failure) =>
          failure.retainEvidenceExportAccess ||
          `Expected ${failureLabel} failures to retain evidence export and safety-critical audit access`
      ),
      Schema.filter(
        (failure) =>
          failure.existingLocalSafetyContinues ||
          `Expected ${failureLabel} failures to keep existing local safety behavior explicit`
      )
    )
  );
}

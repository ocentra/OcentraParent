import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { FamilyReferenceSchema, ParentAccountReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  BillingChildActivityCustodySchema,
  BillingEvidenceExportAccessSchema,
  BillingFailureKindSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentResolutionSchema,
  BillingParentVisibleStateSchema,
  BillingProviderBoundarySchema,
} from './billing-entitlement-values';
import {
  BillingSupportAdminActionSchema,
  BillingSupportAdminAuditReferenceSchema,
  BillingSupportAdminBackendUploadClaimSchema,
  BillingSupportAdminBoundaryIdSchema,
  BillingSupportAdminCaseReferenceSchema,
  BillingSupportAdminChildActivityCustodyClaimSchema,
  BillingSupportAdminDataClassSchema,
  BillingSupportAdminNonClaimSchema,
  BillingSupportAdminPortalUiClaimSchema,
  BillingSupportAdminProviderContactClaimSchema,
  BillingSupportAdminProviderSecretCustodySchema,
  BillingSupportAdminRuntimeStateSchema,
  BillingSupportAdminSchemaVersionSchema,
  type BillingSupportAdminAction,
  type BillingSupportAdminDataClass,
  type BillingSupportAdminNonClaim,
  type BillingSupportAdminRuntimeState,
} from './billing-support-admin-boundary-values';

export * from './billing-support-admin-boundary-values';

export const BillingSupportAdminFailureStateSchema = withParser(
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
        'Expected billing support admin failures to retain evidence export and audit access'
    ),
    Schema.filter(
      (failure) =>
        failure.existingLocalSafetyContinues ||
        'Expected billing support admin failures to keep existing local safety behavior explicit'
    )
  )
);

export const BillingSupportAdminBoundaryRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSupportAdminSchemaVersionSchema,
    boundaryId: BillingSupportAdminBoundaryIdSchema,
    supportCase: BillingSupportAdminCaseReferenceSchema,
    parentAccount: ParentAccountReferenceSchema,
    family: FamilyReferenceSchema,
    action: BillingSupportAdminActionSchema,
    runtimeState: BillingSupportAdminRuntimeStateSchema,
    parentVisibleState: BillingParentVisibleStateSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    providerBoundary: BillingProviderBoundarySchema,
    providerSecretCustody: BillingSupportAdminProviderSecretCustodySchema,
    disclosedDataClasses: Schema.Array(BillingSupportAdminDataClassSchema),
    manualRequired: Schema.Boolean,
    providerContacted: Schema.Boolean,
    accountLookupExecuted: Schema.Boolean,
    entitlementOverrideApplied: Schema.Boolean,
    refundCreditIssued: Schema.Boolean,
    supportBackendUploadExecuted: Schema.Boolean,
    failureState: Schema.Union(BillingSupportAdminFailureStateSchema, Schema.Null),
    auditReference: BillingSupportAdminAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        billingSupportAdminRowIsSupportSafe(row) ||
        'Expected billing support admin rows to disclose only support-safe billing refs, require manual boundaries, and avoid provider/backend/child-data custody'
    )
  )
);

export const BillingSupportAdminBoundaryProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSupportAdminSchemaVersionSchema,
    rows: Schema.Array(BillingSupportAdminBoundaryRowSchema),
    nonClaims: Schema.Array(BillingSupportAdminNonClaimSchema),
    portalUiClaim: BillingSupportAdminPortalUiClaimSchema,
    providerContactClaim: BillingSupportAdminProviderContactClaimSchema,
    backendUploadClaim: BillingSupportAdminBackendUploadClaimSchema,
    childActivityCustodyClaim: BillingSupportAdminChildActivityCustodyClaimSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        billingSupportAdminProofIsHonest(proof) ||
        'Expected billing support admin proof to cover support/account/billing/admin manual states and non-claims'
    )
  )
);

export type BillingSupportAdminFailureState = Infer<typeof BillingSupportAdminFailureStateSchema>;
export type BillingSupportAdminBoundaryRow = Infer<typeof BillingSupportAdminBoundaryRowSchema>;
export type BillingSupportAdminBoundaryProof = Infer<typeof BillingSupportAdminBoundaryProofSchema>;

export const decodeBillingSupportAdminBoundaryProof = Schema.decodeUnknownSync(BillingSupportAdminBoundaryProofSchema);

function billingSupportAdminRowIsSupportSafe(row: {
  readonly action: BillingSupportAdminAction;
  readonly runtimeState: BillingSupportAdminRuntimeState;
  readonly providerSecretCustody: 'not-present';
  readonly disclosedDataClasses: ReadonlyArray<BillingSupportAdminDataClass>;
  readonly manualRequired: boolean;
  readonly providerContacted: boolean;
  readonly accountLookupExecuted: boolean;
  readonly entitlementOverrideApplied: boolean;
  readonly refundCreditIssued: boolean;
  readonly supportBackendUploadExecuted: boolean;
  readonly failureState: BillingSupportAdminFailureState | null;
}): boolean {
  const manualActions: ReadonlyArray<BillingSupportAdminAction> = [
    'provider-contact-manual-required',
    'entitlement-admin-override-manual-required',
    'refund-credit-manual-required',
  ];
  const needsFailureContext = row.runtimeState !== 'read-only-local-proof';
  return (
    row.providerSecretCustody === 'not-present' &&
    row.disclosedDataClasses.includes('redaction-audit-ref') &&
    (!manualActions.includes(row.action) || row.manualRequired) &&
    (!needsFailureContext || row.failureState !== null) &&
    !row.providerContacted &&
    !row.accountLookupExecuted &&
    !row.entitlementOverrideApplied &&
    !row.refundCreditIssued &&
    !row.supportBackendUploadExecuted
  );
}

function billingSupportAdminProofIsHonest(proof: {
  readonly rows: ReadonlyArray<{ readonly action: BillingSupportAdminAction; readonly manualRequired: boolean }>;
  readonly nonClaims: ReadonlyArray<BillingSupportAdminNonClaim>;
  readonly portalUiClaim: 'not-implemented';
  readonly providerContactClaim: 'not-executed';
  readonly backendUploadClaim: 'not-executed';
  readonly childActivityCustodyClaim: 'not-included';
}): boolean {
  const requiredNonClaims: ReadonlyArray<BillingSupportAdminNonClaim> = [
    'no-stripe-sdk',
    'no-provider-secrets',
    'no-billing-provider-contact',
    'no-account-backend-admin-runtime',
    'no-entitlement-admin-override-runtime',
    'no-refund-credit-runtime',
    'no-portal-admin-ui',
    'no-support-backend-upload',
    'no-child-activity-custody',
  ];
  const requiredActions: ReadonlyArray<BillingSupportAdminAction> = [
    'support-case-triage',
    'account-status-review',
    'billing-escalation-request',
    'provider-contact-manual-required',
    'entitlement-admin-override-manual-required',
    'refund-credit-manual-required',
  ];
  return (
    requiredNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    requiredActions.every((action) => proof.rows.some((row) => row.action === action)) &&
    proof.rows.some((row) => row.action === 'billing-escalation-request' && row.manualRequired) &&
    proof.portalUiClaim === 'not-implemented' &&
    proof.providerContactClaim === 'not-executed' &&
    proof.backendUploadClaim === 'not-executed' &&
    proof.childActivityCustodyClaim === 'not-included'
  );
}

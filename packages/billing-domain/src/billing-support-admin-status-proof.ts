import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  BillingChildActivityCustodySchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentResolutionSchema,
  BillingParentVisibleStateSchema,
} from './billing-entitlement-values';
import { BillingSupportAdminBoundaryProofReadModel } from './billing-support-admin-boundary-proof';
import {
  BillingSupportAdminStatusAuditReferenceSchema,
  BillingSupportAdminStatusChildActivityCustodyClaimSchema,
  BillingSupportAdminStatusDataClassSchema,
  BillingSupportAdminStatusEvidenceExportAccessSchema,
  BillingSupportAdminStatusIdSchema,
  BillingSupportAdminStatusLocalSafetyClaimSchema,
  BillingSupportAdminStatusNonClaimSchema,
  BillingSupportAdminStatusPortalClaimSchema,
  BillingSupportAdminStatusProofRefSchema,
  BillingSupportAdminStatusProviderClaimSchema,
  BillingSupportAdminStatusRowSchema,
  BillingSupportAdminStatusRuntimeStateSchema,
  BillingSupportAdminStatusSchemaVersionSchema,
  type BillingSupportAdminStatusDataClass,
  type BillingSupportAdminStatusNonClaim,
  type BillingSupportAdminStatusProofRef,
  type BillingSupportAdminStatusRow,
  type BillingSupportAdminStatusRuntimeState,
} from './billing-support-admin-status-values';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { FamilyReferenceSchema, ParentAccountReferenceSchema } from '@ocentra-parent/family-domain/references';

const decodeParentTimestamp = Schema.decodeUnknownSync(ParentTimestampSchema);
const UpdatedAt = decodeParentTimestamp('2026-06-05T17:22:00.000Z');

export const BillingSupportAdminStatusProofRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSupportAdminStatusSchemaVersionSchema,
    statusId: BillingSupportAdminStatusIdSchema,
    parentAccount: ParentAccountReferenceSchema,
    family: FamilyReferenceSchema,
    statusRow: BillingSupportAdminStatusRowSchema,
    runtimeState: BillingSupportAdminStatusRuntimeStateSchema,
    parentVisibleState: BillingParentVisibleStateSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingSupportAdminStatusEvidenceExportAccessSchema,
    localSafetyClaim: BillingSupportAdminStatusLocalSafetyClaimSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    disclosedDataClasses: Schema.Array(BillingSupportAdminStatusDataClassSchema),
    proofRefs: Schema.Array(BillingSupportAdminStatusProofRefSchema),
    manualRequired: Schema.Boolean,
    providerContactExecuted: Schema.Boolean,
    accountLookupExecuted: Schema.Boolean,
    entitlementOverrideApplied: Schema.Boolean,
    refundCreditIssued: Schema.Boolean,
    supportBackendUploadExecuted: Schema.Boolean,
    portalAdminUiClaim: BillingSupportAdminStatusPortalClaimSchema,
    providerClaim: BillingSupportAdminStatusProviderClaimSchema,
    parentResolution: BillingParentResolutionSchema,
    auditReference: BillingSupportAdminStatusAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        billingSupportAdminStatusRowIsHonest(row) ||
        'Expected billing support admin status rows to retain support-safe proof refs and avoid provider/backend/admin runtime execution'
    )
  )
);

export const BillingSupportAdminStatusProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingSupportAdminStatusSchemaVersionSchema,
    boundaryProof: Schema.Literal('billing-support-admin-boundary-proof'),
    rows: Schema.Array(BillingSupportAdminStatusProofRowSchema),
    nonClaims: Schema.Array(BillingSupportAdminStatusNonClaimSchema),
    providerClaim: BillingSupportAdminStatusProviderClaimSchema,
    portalAdminUiClaim: BillingSupportAdminStatusPortalClaimSchema,
    childActivityCustodyClaim: BillingSupportAdminStatusChildActivityCustodyClaimSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        billingSupportAdminStatusProofIsComplete(proof) ||
        'Expected billing support admin status proof to cover visible, manual-required, and not-implemented support/admin states with explicit non-claims'
    )
  )
);

export type BillingSupportAdminStatusProofRow = typeof BillingSupportAdminStatusProofRowSchema.Type;
export type BillingSupportAdminStatusProof = typeof BillingSupportAdminStatusProofSchema.Type;

export const decodeBillingSupportAdminStatusProof = Schema.decodeUnknownSync(BillingSupportAdminStatusProofSchema);

export const BillingSupportAdminStatusProofReadModel = BillingSupportAdminStatusProofSchema.parse({
  schemaVersion: 'billing-support-admin-status-proof',
  boundaryProof: 'billing-support-admin-boundary-proof',
  rows: [
    billingSupportAdminStatusRow(
      'billing-support-status-case-triage',
      'case-triage-visible',
      'source-contract-ready',
      'manual-review',
      'manual-support-review',
      false,
      ['support-case-status-ref', 'account-status-ref', 'redaction-audit-ref'],
      ['billing-support-admin-boundary-proof']
    ),
    billingSupportAdminStatusRow(
      'billing-support-status-account-review',
      'account-review-visible',
      'source-contract-ready',
      'available',
      'none',
      false,
      ['account-status-ref', 'subscription-status-ref', 'entitlement-snapshot-ref', 'redaction-audit-ref'],
      [
        'billing-support-admin-boundary-proof',
        'billing-entitlement-contract-proof',
        'billing-entitlement-runtime-proof',
      ]
    ),
    billingSupportAdminStatusRow(
      'billing-support-status-escalation',
      'billing-escalation-visible',
      'manual-required',
      'manual-review',
      'manual-support-review',
      true,
      ['support-case-status-ref', 'billing-failure-state-ref', 'manual-proof-ref', 'redaction-audit-ref'],
      ['billing-support-admin-boundary-proof', 'billing-failure-state-proof']
    ),
    billingSupportAdminStatusRow(
      'billing-support-status-provider-contact',
      'provider-contact-manual-required',
      'manual-required',
      'past-due',
      'payment-update',
      true,
      ['account-status-ref', 'billing-failure-state-ref', 'manual-proof-ref', 'redaction-audit-ref'],
      ['billing-support-admin-boundary-proof', 'billing-failure-state-proof']
    ),
    billingSupportAdminStatusRow(
      'billing-support-status-entitlement-override',
      'entitlement-override-manual-required',
      'not-implemented',
      'manual-review',
      'manual-support-review',
      true,
      ['entitlement-snapshot-ref', 'device-limit-decision-ref', 'manual-proof-ref', 'redaction-audit-ref'],
      ['billing-support-admin-boundary-proof', 'billing-subscription-device-limit-proof']
    ),
    billingSupportAdminStatusRow(
      'billing-support-status-refund-credit',
      'refund-credit-manual-required',
      'not-implemented',
      'unavailable',
      'wait-for-provider',
      true,
      ['account-status-ref', 'billing-failure-state-ref', 'manual-proof-ref', 'redaction-audit-ref'],
      ['billing-support-admin-boundary-proof', 'billing-failure-state-proof']
    ),
    billingSupportAdminStatusRow(
      'billing-support-status-resolution-update',
      'resolution-update-ready',
      'source-contract-ready',
      'grace',
      'manual-support-review',
      true,
      [
        'support-case-status-ref',
        'subscription-status-ref',
        'billing-failure-state-ref',
        'manual-proof-ref',
        'redaction-audit-ref',
      ],
      ['billing-support-admin-boundary-proof', 'billing-entitlement-runtime-proof']
    ),
  ],
  nonClaims: [
    'no-stripe-sdk',
    'no-provider-secrets',
    'no-billing-provider-contact-execution',
    'no-account-lookup-execution',
    'no-entitlement-admin-override-runtime',
    'no-refund-credit-runtime',
    'no-portal-admin-ui',
    'no-support-backend-upload',
    'no-child-activity-custody',
  ],
  providerClaim: 'not-executed',
  portalAdminUiClaim: 'not-implemented',
  childActivityCustodyClaim: 'not-included',
  updatedAt: UpdatedAt,
});

export const BillingSupportAdminStatusProof = BillingSupportAdminStatusProofReadModel;

export const BillingSupportAdminStatusKnownGaps = [
  'Billing provider contact remains manual-required and not executed.',
  'Account lookup execution and account backend admin runtime remain unimplemented.',
  'Entitlement override and refund/credit runtime remain unimplemented.',
  'Portal admin UI, support backend upload, and production billing support execution remain unimplemented.',
  'Child activity custody remains excluded from billing support admin status rows.',
] as const;

export const BillingSupportAdminStatusBoundaryProof = BillingSupportAdminBoundaryProofReadModel;

function billingSupportAdminStatusRow(
  statusId:
    | 'billing-support-status-case-triage'
    | 'billing-support-status-account-review'
    | 'billing-support-status-escalation'
    | 'billing-support-status-provider-contact'
    | 'billing-support-status-entitlement-override'
    | 'billing-support-status-refund-credit'
    | 'billing-support-status-resolution-update',
  statusRow: BillingSupportAdminStatusRow,
  runtimeState: BillingSupportAdminStatusRuntimeState,
  parentVisibleState: 'available' | 'past-due' | 'grace' | 'unavailable' | 'manual-review',
  parentResolution: 'none' | 'payment-update' | 'manual-support-review' | 'wait-for-provider',
  manualRequired: boolean,
  disclosedDataClasses: ReadonlyArray<BillingSupportAdminStatusDataClass>,
  proofRefs: ReadonlyArray<BillingSupportAdminStatusProofRef>
) {
  return {
    schemaVersion: 'billing-support-admin-status-proof',
    statusId,
    parentAccount: {
      parentAccountId: 'parent-account-billing-support-admin-status-proof-1',
    },
    family: {
      familyId: 'family-billing-support-admin-status-proof-1',
    },
    statusRow,
    runtimeState,
    parentVisibleState,
    localSafetyBehavior: parentVisibleState === 'unavailable' ? 'local-only' : 'manual-review-with-local-safety',
    evidenceExportAccess: 'retained',
    localSafetyClaim: 'continues',
    childActivityCustody: 'not-included',
    disclosedDataClasses,
    proofRefs,
    manualRequired,
    providerContactExecuted: false,
    accountLookupExecuted: false,
    entitlementOverrideApplied: false,
    refundCreditIssued: false,
    supportBackendUploadExecuted: false,
    portalAdminUiClaim: 'not-implemented',
    providerClaim: 'not-executed',
    parentResolution,
    auditReference: `audit-${statusId}`,
  } as const;
}

function billingSupportAdminStatusRowIsHonest(row: {
  readonly statusRow: BillingSupportAdminStatusRow;
  readonly runtimeState: BillingSupportAdminStatusRuntimeState;
  readonly disclosedDataClasses: ReadonlyArray<BillingSupportAdminStatusDataClass>;
  readonly proofRefs: ReadonlyArray<BillingSupportAdminStatusProofRef>;
  readonly manualRequired: boolean;
  readonly providerContactExecuted: boolean;
  readonly accountLookupExecuted: boolean;
  readonly entitlementOverrideApplied: boolean;
  readonly refundCreditIssued: boolean;
  readonly supportBackendUploadExecuted: boolean;
  readonly portalAdminUiClaim: 'not-implemented';
  readonly providerClaim: 'not-executed';
}): boolean {
  return (
    row.proofRefs.includes('billing-support-admin-boundary-proof') &&
    row.disclosedDataClasses.includes('redaction-audit-ref') &&
    billingSupportAdminStatusManualStateIsHonest(row) &&
    billingSupportAdminStatusExecutionClaimsAreAbsent(row) &&
    row.portalAdminUiClaim === 'not-implemented' &&
    row.providerClaim === 'not-executed'
  );
}

function billingSupportAdminStatusManualStateIsHonest(row: {
  readonly statusRow: BillingSupportAdminStatusRow;
  readonly runtimeState: BillingSupportAdminStatusRuntimeState;
  readonly disclosedDataClasses: ReadonlyArray<BillingSupportAdminStatusDataClass>;
  readonly manualRequired: boolean;
}): boolean {
  const manualRows: ReadonlyArray<BillingSupportAdminStatusRow> = [
    'billing-escalation-visible',
    'provider-contact-manual-required',
    'entitlement-override-manual-required',
    'refund-credit-manual-required',
    'resolution-update-ready',
  ];
  const manualRowIsHonest =
    !manualRows.includes(row.statusRow) ||
    (row.manualRequired && row.disclosedDataClasses.includes('manual-proof-ref'));
  const notImplementedRowIsHonest = row.runtimeState !== 'not-implemented' || row.manualRequired;
  return manualRowIsHonest && notImplementedRowIsHonest;
}

function billingSupportAdminStatusExecutionClaimsAreAbsent(row: {
  readonly providerContactExecuted: boolean;
  readonly accountLookupExecuted: boolean;
  readonly entitlementOverrideApplied: boolean;
  readonly refundCreditIssued: boolean;
  readonly supportBackendUploadExecuted: boolean;
}): boolean {
  return (
    !row.providerContactExecuted &&
    !row.accountLookupExecuted &&
    !row.entitlementOverrideApplied &&
    !row.refundCreditIssued &&
    !row.supportBackendUploadExecuted
  );
}

function billingSupportAdminStatusProofIsComplete(proof: {
  readonly rows: ReadonlyArray<{
    readonly statusRow: BillingSupportAdminStatusRow;
    readonly runtimeState: BillingSupportAdminStatusRuntimeState;
  }>;
  readonly nonClaims: ReadonlyArray<BillingSupportAdminStatusNonClaim>;
  readonly providerClaim: 'not-executed';
  readonly portalAdminUiClaim: 'not-implemented';
  readonly childActivityCustodyClaim: 'not-included';
}): boolean {
  const requiredRows: ReadonlyArray<BillingSupportAdminStatusRow> = [
    'case-triage-visible',
    'account-review-visible',
    'billing-escalation-visible',
    'provider-contact-manual-required',
    'entitlement-override-manual-required',
    'refund-credit-manual-required',
    'resolution-update-ready',
  ];
  const requiredNonClaims: ReadonlyArray<BillingSupportAdminStatusNonClaim> = [
    'no-stripe-sdk',
    'no-provider-secrets',
    'no-billing-provider-contact-execution',
    'no-account-lookup-execution',
    'no-entitlement-admin-override-runtime',
    'no-refund-credit-runtime',
    'no-portal-admin-ui',
    'no-support-backend-upload',
    'no-child-activity-custody',
  ];
  return (
    requiredRows.every((statusRow) => proof.rows.some((row) => row.statusRow === statusRow)) &&
    requiredNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    proof.rows.some((row) => row.runtimeState === 'source-contract-ready') &&
    proof.rows.some((row) => row.runtimeState === 'manual-required') &&
    proof.rows.some((row) => row.runtimeState === 'not-implemented') &&
    proof.providerClaim === 'not-executed' &&
    proof.portalAdminUiClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-included'
  );
}

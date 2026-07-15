/* generated from crates/schema/src/billing_invoice_tax_refund_dispute_ts.rs */

import { type Infer, Schema, withParser } from './effect';
import { FamilyReferenceSchema, ParentAccountReferenceSchema } from './family-references';
import { ParentTimestampSchema } from './family-reference-primitives';
import {
  BillingChildActivityCustodySchema,
  BillingEvidenceExportAccessSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentVisibleStateSchema,
  BillingProviderReferenceSchema,
} from './billing-entitlement-values';
import { BillingFailureStateSchema, type BillingFailureState } from './generated-billing-entitlement';
import {
  BillingCollectionRecoveryStateSchema,
  BillingDisputeLifecycleStateSchema,
  BillingEntitlementSideEffectSchema,
  BillingInvoiceCurrencyCodeSchema,
  BillingInvoiceChildActivityCustodyClaimSchema,
  BillingInvoiceHostedSurfaceClaimSchema,
  BillingInvoiceLifecycleAuditReferenceSchema,
  BillingInvoiceLifecycleBoundaryIdSchema,
  BillingInvoiceLifecycleNonClaimSchema,
  BillingInvoiceManualSupportClaimSchema,
  BillingInvoiceNumberSchema,
  BillingInvoiceProviderModeSchema,
  BillingInvoiceTaxRefundDisputeSchemaVersionSchema,
  BillingInvoiceVisibilityStateSchema,
  BillingRefundLifecycleStateSchema,
  BillingSupportAuditStateSchema,
  BillingTaxModeDecisionSchema,
  BillingTaxRegionStateSchema,
  type BillingCollectionRecoveryState,
  type BillingDisputeLifecycleState,
  type BillingEntitlementSideEffect,
  type BillingInvoiceProviderMode,
  type BillingInvoiceLifecycleNonClaim,
  type BillingInvoiceVisibilityState,
  type BillingRefundLifecycleState,
  type BillingTaxModeDecision,
  type BillingTaxRegionState,
} from './billing-invoice-tax-refund-dispute-values';

const Timestamp = '2026-06-13T09:10:00.000Z';
const GraceRetryTimestamp = '2026-06-16T09:10:00.000Z';
const InvoicePeriodStartTimestamp = '2026-06-14T00:00:00.000Z';
const InvoicePeriodEndTimestamp = '2026-07-14T00:00:00.000Z';

const BillingInvoiceTaxRefundDisputeRowStruct = Schema.Struct({
  schemaVersion: BillingInvoiceTaxRefundDisputeSchemaVersionSchema,
  boundaryId: BillingInvoiceLifecycleBoundaryIdSchema,
  parentAccount: ParentAccountReferenceSchema,
  family: FamilyReferenceSchema,
  providerMode: BillingInvoiceProviderModeSchema,
  providerReference: Schema.Union(BillingProviderReferenceSchema, Schema.Null),
  invoiceNumber: BillingInvoiceNumberSchema,
  currency: BillingInvoiceCurrencyCodeSchema,
  periodStart: ParentTimestampSchema,
  periodEnd: ParentTimestampSchema,
  invoiceVisibility: BillingInvoiceVisibilityStateSchema,
  taxMode: BillingTaxModeDecisionSchema,
  taxRegionState: BillingTaxRegionStateSchema,
  refundState: BillingRefundLifecycleStateSchema,
  disputeState: BillingDisputeLifecycleStateSchema,
  recoveryState: BillingCollectionRecoveryStateSchema,
  parentVisibleState: BillingParentVisibleStateSchema,
  localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
  evidenceExportAccess: BillingEvidenceExportAccessSchema,
  childActivityCustody: BillingChildActivityCustodySchema,
  entitlementSideEffect: BillingEntitlementSideEffectSchema,
  supportAuditState: BillingSupportAuditStateSchema,
  portalHosted: Schema.Boolean,
  failureState: Schema.Union(BillingFailureStateSchema, Schema.Null),
  auditReference: BillingInvoiceLifecycleAuditReferenceSchema,
});

type BillingInvoiceTaxRefundDisputeRowShape = Infer<typeof BillingInvoiceTaxRefundDisputeRowStruct>;

const BillingInvoiceTaxRefundDisputeRowRefinedSchema = BillingInvoiceTaxRefundDisputeRowStruct.pipe(
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.invoiceVisibility !== 'customer-portal-hosted' ||
      row.portalHosted ||
      'Expected customer-visible invoices to stay on the Stripe-hosted portal surface'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.periodStart < row.periodEnd || 'Expected invoice lifecycle rows to keep period start before period end'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.providerMode !== 'manual-invoice' ||
      (!row.portalHosted && row.providerReference === null && row.invoiceVisibility === 'manual-support-required') ||
      'Expected manual invoice rows to remain non-hosted and app-owned'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.providerMode !== 'stripe-hosted' ||
      row.providerReference !== null ||
      'Expected Stripe-hosted invoice rows to retain a backend provider reference'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.invoiceVisibility !== 'manual-support-required' ||
      row.failureState !== null ||
      'Expected manual-support invoice states to carry explicit failure or support context'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.taxMode !== 'manual-support-required' ||
      row.taxRegionState === 'manual-support-required' ||
      'Expected manual tax handling to stay behind a manual-support region decision'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.recoveryState !== 'grace' ||
      row.entitlementSideEffect === 'grace-paid-access' ||
      'Expected grace recovery to preserve only explicit grace-paid access'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      !['cancelled', 'unpaid'].includes(row.recoveryState) ||
      row.entitlementSideEffect === 'limit-paid-access' ||
      row.entitlementSideEffect === 'revoke-paid-access' ||
      row.entitlementSideEffect === 'manual-review-required' ||
      'Expected cancelled and unpaid recovery states to limit, revoke, or manually review paid access'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.refundState !== 'refund-settled' ||
      row.entitlementSideEffect === 'revoke-paid-access' ||
      'Expected settled refunds to revoke paid-access entitlement'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.disputeState !== 'dispute-opened' ||
      row.entitlementSideEffect !== 'retain-paid-access' ||
      'Expected open disputes not to silently retain paid access'
  )
).pipe(
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.disputeState !== 'dispute-lost' ||
      row.entitlementSideEffect === 'revoke-paid-access' ||
      'Expected lost disputes to revoke paid-access entitlement'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      !['refund-requested', 'refund-issued', 'refund-denied'].includes(row.refundState) ||
      row.entitlementSideEffect === 'retain-paid-access' ||
      row.entitlementSideEffect === 'manual-review-required' ||
      'Expected non-settled refunds to stay explicit without collapsing into full-refund revocation'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.disputeState !== 'dispute-won' ||
      row.entitlementSideEffect === 'retain-paid-access' ||
      row.entitlementSideEffect === 'grace-paid-access' ||
      'Expected won disputes to preserve or restore the paid-access path'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.boundaryId !== 'billing-invoice-receipt-download' ||
      row.invoiceVisibility === 'download-link-issued' ||
      'Expected explicit receipt visibility rows to use the download-link-issued state'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.boundaryId !== 'billing-invoice-partial-refund-issued' ||
      (row.refundState === 'refund-issued' &&
        row.entitlementSideEffect === 'retain-paid-access' &&
        row.recoveryState === 'active') ||
      'Expected partial refund rows to remain distinct from full-refund revocation'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.boundaryId !== 'billing-invoice-refund-denied' ||
      (row.refundState === 'refund-denied' &&
        row.entitlementSideEffect === 'retain-paid-access' &&
        row.recoveryState === 'active') ||
      'Expected denied refunds to remain explicit and auditable without revoking paid access'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.boundaryId !== 'billing-invoice-dispute-won' ||
      (row.disputeState === 'dispute-won' &&
        row.entitlementSideEffect === 'retain-paid-access' &&
        row.recoveryState === 'active') ||
      'Expected dispute-won rows to restore or preserve the active entitlement path'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.boundaryId !== 'billing-invoice-chargeback-opened' ||
      (row.disputeState === 'dispute-opened' &&
        row.invoiceVisibility === 'manual-support-required' &&
        row.entitlementSideEffect === 'manual-review-required') ||
      'Expected chargeback rows to stay distinct from ordinary refunds and require manual review'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.boundaryId !== 'billing-invoice-cancel-immediate' ||
      (row.recoveryState === 'cancelled' &&
        row.entitlementSideEffect === 'revoke-paid-access' &&
        row.parentVisibleState === 'locked') ||
      'Expected immediate cancellation rows to make revocation explicit'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.boundaryId !== 'billing-invoice-cancel-period-end' ||
      (row.recoveryState === 'grace' &&
        row.entitlementSideEffect === 'grace-paid-access' &&
        row.parentVisibleState === 'grace') ||
      'Expected period-end cancellation rows to remain distinct from immediate cancellation'
  ),
  Schema.filter(
    (row: BillingInvoiceTaxRefundDisputeRowShape) =>
      row.boundaryId !== 'billing-invoice-resume-after-past-due' ||
      (row.recoveryState === 'active' &&
        row.entitlementSideEffect === 'retain-paid-access' &&
        row.parentVisibleState === 'available') ||
      'Expected resume-after-past-due rows to restore a ledger-backed active state'
  )
);

export const BillingInvoiceTaxRefundDisputeRowSchema = withParser(BillingInvoiceTaxRefundDisputeRowRefinedSchema);
const BillingInvoiceTaxRefundDisputeProofStruct = Schema.Struct({
  schemaVersion: BillingInvoiceTaxRefundDisputeSchemaVersionSchema,
  rows: Schema.Array(BillingInvoiceTaxRefundDisputeRowSchema),
  nonClaims: Schema.Array(BillingInvoiceLifecycleNonClaimSchema),
  hostedInvoiceClaim: BillingInvoiceHostedSurfaceClaimSchema,
  manualSupportClaim: BillingInvoiceManualSupportClaimSchema,
  childActivityCustodyClaim: BillingInvoiceChildActivityCustodyClaimSchema,
  updatedAt: ParentTimestampSchema,
});

type BillingInvoiceTaxRefundDisputeProofShape = Infer<typeof BillingInvoiceTaxRefundDisputeProofStruct>;

export const BillingInvoiceTaxRefundDisputeProofSchema = withParser(
  BillingInvoiceTaxRefundDisputeProofStruct.pipe(
    Schema.filter(
      (proof: BillingInvoiceTaxRefundDisputeProofShape) =>
        billingInvoiceTaxRefundDisputeProofIsHonest(proof) ||
        'Expected billing invoice tax refund dispute proof to cover invoice visibility tax handling refunds disputes and failed-renewal grace with explicit non-claims'
    )
  )
);

export type BillingInvoiceTaxRefundDisputeRow = Infer<typeof BillingInvoiceTaxRefundDisputeRowSchema>;
export type BillingInvoiceTaxRefundDisputeProof = Infer<typeof BillingInvoiceTaxRefundDisputeProofSchema>;
type BillingLocalSafetyBehavior = Infer<typeof BillingLocalSafetyBehaviorSchema>;
type BillingParentVisibleState = Infer<typeof BillingParentVisibleStateSchema>;

const RequiredLifecycleNonClaims: ReadonlyArray<BillingInvoiceLifecycleNonClaim> = [
  'no-invoice-pdf-custody',
  'no-self-service-refund',
  'no-self-service-dispute',
  'no-child-activity-custody',
];

const RequiredLifecycleBoundaryIds = [
  'billing-invoice-cancel-immediate',
  'billing-invoice-cancel-period-end',
  'billing-invoice-resume-after-past-due',
  'billing-invoice-partial-refund-issued',
  'billing-invoice-refund-denied',
  'billing-invoice-chargeback-opened',
] as const;

const RequiredRefundStates = ['refund-settled'] as const;
const RequiredDisputeStates = ['dispute-opened', 'dispute-won', 'dispute-lost'] as const;

export const BillingInvoiceTaxRefundDisputeProofReadModel = BillingInvoiceTaxRefundDisputeProofSchema.parse({
  schemaVersion: 'billing-invoice-tax-refund-dispute',
  rows: [
    lifecycleRow(
      'billing-invoice-active',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'none',
      'active',
      'available',
      'unchanged',
      'retain-paid-access',
      true,
      null
    ),
    lifecycleRow(
      'billing-invoice-receipt-download',
      'download-link-issued',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'none',
      'active',
      'available',
      'unchanged',
      'retain-paid-access',
      false,
      null
    ),
    lifecycleRow(
      'billing-invoice-grace',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'none',
      'grace',
      'grace',
      'grace-with-local-safety',
      'grace-paid-access',
      true,
      graceFailureState()
    ),
    lifecycleRow(
      'billing-invoice-unpaid',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'none',
      'unpaid',
      'past-due',
      'local-only',
      'limit-paid-access',
      true,
      paymentFailureState()
    ),
    lifecycleRow(
      'billing-invoice-refund-settled',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'refund-settled',
      'none',
      'cancelled',
      'locked',
      'local-only',
      'revoke-paid-access',
      true,
      null
    ),
    lifecycleRow(
      'billing-invoice-partial-refund-issued',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'refund-issued',
      'none',
      'active',
      'available',
      'unchanged',
      'retain-paid-access',
      true,
      null
    ),
    lifecycleRow(
      'billing-invoice-refund-denied',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'refund-denied',
      'none',
      'active',
      'available',
      'unchanged',
      'retain-paid-access',
      true,
      null
    ),
    lifecycleRow(
      'billing-invoice-dispute-opened',
      'manual-support-required',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'dispute-opened',
      'support-required',
      'manual-review',
      'manual-review-with-local-safety',
      'manual-review-required',
      false,
      supportFailureState()
    ),
    lifecycleRow(
      'billing-invoice-dispute-lost',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'dispute-lost',
      'cancelled',
      'locked',
      'local-only',
      'revoke-paid-access',
      true,
      null
    ),
    lifecycleRow(
      'billing-invoice-dispute-won',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'dispute-won',
      'active',
      'available',
      'unchanged',
      'retain-paid-access',
      true,
      null
    ),
    lifecycleRow(
      'billing-invoice-chargeback-opened',
      'manual-support-required',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'dispute-opened',
      'support-required',
      'manual-review',
      'manual-review-with-local-safety',
      'manual-review-required',
      false,
      supportFailureState()
    ),
    lifecycleRow(
      'billing-tax-manual-support',
      'manual-support-required',
      'manual-support-required',
      'manual-support-required',
      'none',
      'none',
      'support-required',
      'manual-review',
      'manual-review-with-local-safety',
      'manual-review-required',
      false,
      supportFailureState()
    ),
    lifecycleRow(
      'billing-invoice-cancel-immediate',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'none',
      'cancelled',
      'locked',
      'local-only',
      'revoke-paid-access',
      true,
      null
    ),
    lifecycleRow(
      'billing-invoice-cancel-period-end',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'none',
      'grace',
      'grace',
      'grace-with-local-safety',
      'grace-paid-access',
      true,
      null
    ),
    lifecycleRow(
      'billing-invoice-resume-after-past-due',
      'customer-portal-hosted',
      'stripe-automatic-tax',
      'launch-supported',
      'none',
      'none',
      'active',
      'available',
      'unchanged',
      'retain-paid-access',
      true,
      null
    ),
  ],
  nonClaims: [
    'no-invoice-pdf-custody',
    'no-self-service-refund',
    'no-self-service-dispute',
    'no-child-activity-custody',
  ],
  hostedInvoiceClaim: 'customer-portal-hosted-only',
  manualSupportClaim: 'audited-required',
  childActivityCustodyClaim: 'not-included',
  updatedAt: Timestamp,
});

export {
  BillingCollectionRecoveryStateSchema,
  BillingDisputeLifecycleStateSchema,
  BillingEntitlementSideEffectSchema,
  BillingInvoiceChildActivityCustodyClaimSchema,
  BillingInvoiceHostedSurfaceClaimSchema,
  BillingInvoiceLifecycleAuditReferenceSchema,
  BillingInvoiceLifecycleBoundaryIdSchema,
  BillingInvoiceLifecycleNonClaimSchema,
  BillingInvoiceManualSupportClaimSchema,
  BillingInvoiceTaxRefundDisputeSchemaVersionSchema,
  BillingInvoiceVisibilityStateSchema,
  BillingRefundLifecycleStateSchema,
  BillingSupportAuditStateSchema,
  BillingTaxModeDecisionSchema,
  BillingTaxRegionStateSchema,
};

export type {
  BillingCollectionRecoveryState,
  BillingDisputeLifecycleState,
  BillingEntitlementSideEffect,
  BillingInvoiceLifecycleNonClaim,
  BillingInvoiceVisibilityState,
  BillingRefundLifecycleState,
  BillingTaxModeDecision,
  BillingTaxRegionState,
};

function billingInvoiceTaxRefundDisputeProofIsHonest(proof: {
  readonly rows: ReadonlyArray<{
    readonly boundaryId: string;
    readonly providerMode: BillingInvoiceProviderMode;
    readonly providerReference: string | null;
    readonly invoiceVisibility: BillingInvoiceVisibilityState;
    readonly taxMode: BillingTaxModeDecision;
    readonly taxRegionState: BillingTaxRegionState;
    readonly refundState: BillingRefundLifecycleState;
    readonly disputeState: BillingDisputeLifecycleState;
    readonly recoveryState: BillingCollectionRecoveryState;
    readonly entitlementSideEffect: BillingEntitlementSideEffect;
    readonly portalHosted: boolean;
    readonly supportAuditState: 'audited';
  }>;
  readonly nonClaims: ReadonlyArray<BillingInvoiceLifecycleNonClaim>;
  readonly hostedInvoiceClaim: 'customer-portal-hosted-only';
  readonly manualSupportClaim: 'audited-required';
  readonly childActivityCustodyClaim: 'not-included';
}): boolean {
  return (
    RequiredLifecycleNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    proofRowsCoverRequiredLifecycleStates(proof) &&
    proof.rows.some((row) => row.providerMode === 'stripe-hosted' && row.providerReference !== null) &&
    proof.rows.some((row) => row.providerMode === 'manual-invoice' && row.providerReference === null) &&
    proof.rows.every((row) => row.supportAuditState === 'audited') &&
    proof.hostedInvoiceClaim === 'customer-portal-hosted-only' &&
    proof.manualSupportClaim === 'audited-required' &&
    proof.childActivityCustodyClaim === 'not-included'
  );
}

function proofRowsCoverRequiredLifecycleStates(proof: {
  readonly rows: ReadonlyArray<{
    readonly boundaryId: string;
    readonly invoiceVisibility: BillingInvoiceVisibilityState;
    readonly taxMode: BillingTaxModeDecision;
    readonly taxRegionState: BillingTaxRegionState;
    readonly refundState: BillingRefundLifecycleState;
    readonly disputeState: BillingDisputeLifecycleState;
    readonly recoveryState: BillingCollectionRecoveryState;
    readonly portalHosted: boolean;
  }>;
}): boolean {
  return (
    proof.rows.some((row) => row.invoiceVisibility === 'customer-portal-hosted' && row.portalHosted) &&
    proof.rows.some((row) => row.invoiceVisibility === 'download-link-issued') &&
    ['grace', 'unpaid'].every((state) => proof.rows.some((row) => row.recoveryState === state)) &&
    RequiredLifecycleBoundaryIds.every((boundaryId) => proof.rows.some((row) => row.boundaryId === boundaryId)) &&
    RequiredRefundStates.every((refundState) => proof.rows.some((row) => row.refundState === refundState)) &&
    RequiredDisputeStates.every((disputeState) => proof.rows.some((row) => row.disputeState === disputeState)) &&
    proof.rows.some(
      (row) => row.taxMode === 'manual-support-required' && row.taxRegionState === 'manual-support-required'
    )
  );
}

function lifecycleRow(
  boundaryId:
    | 'billing-invoice-active'
    | 'billing-invoice-receipt-download'
    | 'billing-invoice-grace'
    | 'billing-invoice-unpaid'
    | 'billing-invoice-refund-settled'
    | 'billing-invoice-partial-refund-issued'
    | 'billing-invoice-refund-denied'
    | 'billing-invoice-dispute-opened'
    | 'billing-invoice-dispute-won'
    | 'billing-invoice-dispute-lost'
    | 'billing-invoice-chargeback-opened'
    | 'billing-tax-manual-support'
    | 'billing-invoice-cancel-immediate'
    | 'billing-invoice-cancel-period-end'
    | 'billing-invoice-resume-after-past-due',
  invoiceVisibility: BillingInvoiceVisibilityState,
  taxMode: BillingTaxModeDecision,
  taxRegionState: BillingTaxRegionState,
  refundState: BillingRefundLifecycleState,
  disputeState: BillingDisputeLifecycleState,
  recoveryState: BillingCollectionRecoveryState,
  parentVisibleState: BillingParentVisibleState,
  localSafetyBehavior: BillingLocalSafetyBehavior,
  entitlementSideEffect: BillingEntitlementSideEffect,
  portalHosted: boolean,
  failureState: BillingFailureState | null
) {
  const invoiceIdentity = invoiceLifecycleIdentity(boundaryId);
  return {
    schemaVersion: 'billing-invoice-tax-refund-dispute',
    boundaryId,
    parentAccount: { parentAccountId: 'parent-account-1' },
    family: { familyId: 'family-1' },
    ...invoiceIdentity,
    invoiceVisibility,
    taxMode,
    taxRegionState,
    refundState,
    disputeState,
    recoveryState,
    parentVisibleState,
    localSafetyBehavior,
    evidenceExportAccess: 'retained',
    childActivityCustody: 'not-included',
    entitlementSideEffect,
    supportAuditState: 'audited',
    portalHosted,
    failureState,
    auditReference: `audit-${boundaryId}`,
  } as const;
}

function invoiceLifecycleIdentity(
  boundaryId:
    | 'billing-invoice-active'
    | 'billing-invoice-receipt-download'
    | 'billing-invoice-grace'
    | 'billing-invoice-unpaid'
    | 'billing-invoice-refund-settled'
    | 'billing-invoice-partial-refund-issued'
    | 'billing-invoice-refund-denied'
    | 'billing-invoice-dispute-opened'
    | 'billing-invoice-dispute-won'
    | 'billing-invoice-dispute-lost'
    | 'billing-invoice-chargeback-opened'
    | 'billing-tax-manual-support'
    | 'billing-invoice-cancel-immediate'
    | 'billing-invoice-cancel-period-end'
    | 'billing-invoice-resume-after-past-due'
) {
  if (boundaryId === 'billing-tax-manual-support') {
    return {
      providerMode: 'manual-invoice' as const,
      providerReference: null,
      invoiceNumber: 'INV-MANUAL-1001',
      currency: 'USD' as const,
      periodStart: InvoicePeriodStartTimestamp,
      periodEnd: InvoicePeriodEndTimestamp,
    };
  }

  const suffix = boundaryId.replace(/^billing-/, '').replace(/-/g, '_');
  const invoiceNumberByBoundary = {
    'billing-invoice-active': 'INV-1001',
    'billing-invoice-receipt-download': 'INV-1001',
    'billing-invoice-grace': 'INV-1002',
    'billing-invoice-unpaid': 'INV-1003',
    'billing-invoice-refund-settled': 'INV-1004',
    'billing-invoice-partial-refund-issued': 'INV-1005',
    'billing-invoice-refund-denied': 'INV-1006',
    'billing-invoice-dispute-opened': 'INV-1007',
    'billing-invoice-dispute-lost': 'INV-1008',
    'billing-invoice-dispute-won': 'INV-1009',
    'billing-invoice-chargeback-opened': 'INV-1010',
    'billing-invoice-cancel-immediate': 'INV-1011',
    'billing-invoice-cancel-period-end': 'INV-1012',
    'billing-invoice-resume-after-past-due': 'INV-1013',
  } as const;

  return {
    providerMode: 'stripe-hosted' as const,
    providerReference: `stripe_${suffix}_backend_ref`,
    invoiceNumber: invoiceNumberByBoundary[boundaryId],
    currency: 'USD' as const,
    periodStart: InvoicePeriodStartTimestamp,
    periodEnd: InvoicePeriodEndTimestamp,
  };
}

function graceFailureState(): BillingFailureState {
  return BillingFailureStateSchema.parse({
    failureKind: 'payment-required',
    parentVisibleState: 'grace',
    localSafetyBehavior: 'grace-with-local-safety',
    retainEvidenceExportAccess: true,
    existingLocalSafetyContinues: true,
    parentResolution: 'payment-update',
    retryAllowed: true,
    retryAfter: GraceRetryTimestamp,
  });
}

function paymentFailureState(): BillingFailureState {
  return BillingFailureStateSchema.parse({
    failureKind: 'payment-required',
    parentVisibleState: 'past-due',
    localSafetyBehavior: 'local-only',
    retainEvidenceExportAccess: true,
    existingLocalSafetyContinues: true,
    parentResolution: 'payment-update',
    retryAllowed: true,
    retryAfter: GraceRetryTimestamp,
  });
}

function supportFailureState(): BillingFailureState {
  return BillingFailureStateSchema.parse({
    failureKind: 'validation-failed',
    parentVisibleState: 'manual-review',
    localSafetyBehavior: 'manual-review-with-local-safety',
    retainEvidenceExportAccess: true,
    existingLocalSafetyContinues: true,
    parentResolution: 'manual-support-review',
    retryAllowed: false,
    retryAfter: null,
  });
}

import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { FamilyReferenceSchema, ParentAccountReferenceSchema } from '@ocentra-parent/family-domain/references';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  BillingChildActivityCustodySchema,
  BillingEvidenceExportAccessSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentVisibleStateSchema,
} from './billing-entitlement-values';
import {
  BillingFailureStateSchema,
  type BillingFailureState,
} from './billing-entitlement';
import {
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
  type BillingCollectionRecoveryState,
  type BillingDisputeLifecycleState,
  type BillingEntitlementSideEffect,
  type BillingInvoiceLifecycleNonClaim,
  type BillingInvoiceVisibilityState,
  type BillingRefundLifecycleState,
  type BillingTaxModeDecision,
  type BillingTaxRegionState,
} from './billing-invoice-tax-refund-dispute-values';

export * from './billing-invoice-tax-refund-dispute-values';

const Timestamp = '2026-06-13T09:10:00.000Z';
const GraceRetryTimestamp = '2026-06-16T09:10:00.000Z';

export const BillingInvoiceTaxRefundDisputeRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingInvoiceTaxRefundDisputeSchemaVersionSchema,
    boundaryId: BillingInvoiceLifecycleBoundaryIdSchema,
    parentAccount: ParentAccountReferenceSchema,
    family: FamilyReferenceSchema,
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
  }).pipe(
    Schema.filter(
      (row) =>
        row.invoiceVisibility !== 'customer-portal-hosted' ||
        row.portalHosted ||
        'Expected customer-visible invoices to stay on the Stripe-hosted portal surface'
    ),
    Schema.filter(
      (row) =>
        row.invoiceVisibility !== 'manual-support-required' ||
        row.failureState !== null ||
        'Expected manual-support invoice states to carry explicit failure or support context'
    ),
    Schema.filter(
      (row) =>
        row.taxMode !== 'manual-support-required' ||
        row.taxRegionState === 'manual-support-required' ||
        'Expected manual tax handling to stay behind a manual-support region decision'
    ),
    Schema.filter(
      (row) =>
        row.recoveryState !== 'grace' ||
        row.entitlementSideEffect === 'grace-paid-access' ||
        'Expected grace recovery to preserve only explicit grace-paid access'
    ),
    Schema.filter(
      (row) =>
        !['cancelled', 'unpaid'].includes(row.recoveryState) ||
        row.entitlementSideEffect === 'limit-paid-access' ||
        row.entitlementSideEffect === 'revoke-paid-access' ||
        row.entitlementSideEffect === 'manual-review-required' ||
        'Expected cancelled and unpaid recovery states to limit, revoke, or manually review paid access'
    ),
    Schema.filter(
      (row) =>
        row.refundState !== 'refund-settled' ||
        row.entitlementSideEffect === 'revoke-paid-access' ||
        'Expected settled refunds to revoke paid-access entitlement'
    ),
    Schema.filter(
      (row) =>
        row.disputeState !== 'dispute-opened' ||
        row.entitlementSideEffect !== 'retain-paid-access' ||
        'Expected open disputes not to silently retain paid access'
    ),
    Schema.filter(
      (row) =>
        row.disputeState !== 'dispute-lost' ||
        row.entitlementSideEffect === 'revoke-paid-access' ||
        'Expected lost disputes to revoke paid-access entitlement'
    )
  )
);

export const BillingInvoiceTaxRefundDisputeProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingInvoiceTaxRefundDisputeSchemaVersionSchema,
    rows: Schema.Array(BillingInvoiceTaxRefundDisputeRowSchema),
    nonClaims: Schema.Array(BillingInvoiceLifecycleNonClaimSchema),
    hostedInvoiceClaim: BillingInvoiceHostedSurfaceClaimSchema,
    manualSupportClaim: BillingInvoiceManualSupportClaimSchema,
    childActivityCustodyClaim: BillingInvoiceChildActivityCustodyClaimSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        billingInvoiceTaxRefundDisputeProofIsHonest(proof) ||
        'Expected billing invoice tax refund dispute proof to cover invoice visibility tax handling refunds disputes and failed-renewal grace with explicit non-claims'
    )
  )
);

export type BillingInvoiceTaxRefundDisputeRow = Infer<
  typeof BillingInvoiceTaxRefundDisputeRowSchema
>;
export type BillingInvoiceTaxRefundDisputeProof = Infer<
  typeof BillingInvoiceTaxRefundDisputeProofSchema
>;
type BillingLocalSafetyBehavior = Infer<typeof BillingLocalSafetyBehaviorSchema>;
type BillingParentVisibleState = Infer<typeof BillingParentVisibleStateSchema>;

export const decodeBillingInvoiceTaxRefundDisputeProof = Schema.decodeUnknownSync(
  BillingInvoiceTaxRefundDisputeProofSchema
);

export const BillingInvoiceTaxRefundDisputeProofReadModel =
  BillingInvoiceTaxRefundDisputeProofSchema.parse({
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

export const BillingInvoiceTaxRefundDisputeProof =
  BillingInvoiceTaxRefundDisputeProofReadModel;

function billingInvoiceTaxRefundDisputeProofIsHonest(proof: {
  readonly rows: ReadonlyArray<{
    readonly invoiceVisibility: BillingInvoiceVisibilityState;
    readonly taxMode: BillingTaxModeDecision;
    readonly taxRegionState: BillingTaxRegionState;
    readonly refundState: BillingRefundLifecycleState;
    readonly disputeState: BillingDisputeLifecycleState;
    readonly recoveryState: BillingCollectionRecoveryState;
    readonly portalHosted: boolean;
    readonly supportAuditState: 'audited';
  }>;
  readonly nonClaims: ReadonlyArray<BillingInvoiceLifecycleNonClaim>;
  readonly hostedInvoiceClaim: 'customer-portal-hosted-only';
  readonly manualSupportClaim: 'audited-required';
  readonly childActivityCustodyClaim: 'not-included';
}): boolean {
  const requiredNonClaims: ReadonlyArray<BillingInvoiceLifecycleNonClaim> = [
    'no-invoice-pdf-custody',
    'no-self-service-refund',
    'no-self-service-dispute',
    'no-child-activity-custody',
  ];
  return (
    requiredNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    proof.rows.some(
      (row) => row.invoiceVisibility === 'customer-portal-hosted' && row.portalHosted
    ) &&
    proof.rows.some((row) => row.recoveryState === 'grace') &&
    proof.rows.some((row) => row.recoveryState === 'unpaid') &&
    proof.rows.some((row) => row.refundState === 'refund-settled') &&
    proof.rows.some((row) => row.disputeState === 'dispute-opened') &&
    proof.rows.some((row) => row.disputeState === 'dispute-lost') &&
    proof.rows.some(
      (row) =>
        row.taxMode === 'manual-support-required' &&
        row.taxRegionState === 'manual-support-required'
    ) &&
    proof.rows.every((row) => row.supportAuditState === 'audited') &&
    proof.hostedInvoiceClaim === 'customer-portal-hosted-only' &&
    proof.manualSupportClaim === 'audited-required' &&
    proof.childActivityCustodyClaim === 'not-included'
  );
}

function lifecycleRow(
  boundaryId:
    | 'billing-invoice-active'
    | 'billing-invoice-grace'
    | 'billing-invoice-unpaid'
    | 'billing-invoice-refund-settled'
    | 'billing-invoice-dispute-opened'
    | 'billing-invoice-dispute-lost'
    | 'billing-tax-manual-support',
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
  return {
    schemaVersion: 'billing-invoice-tax-refund-dispute',
    boundaryId,
    parentAccount: { parentAccountId: 'parent-account-1' },
    family: { familyId: 'family-1' },
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

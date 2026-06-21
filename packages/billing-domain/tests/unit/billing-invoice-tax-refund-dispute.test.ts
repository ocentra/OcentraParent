import { describe, expect, it } from 'vitest';
import {
  BillingInvoiceTaxRefundDisputeProofReadModel,
  BillingInvoiceTaxRefundDisputeProofSchema,
  BillingInvoiceTaxRefundDisputeRowSchema,
} from '@ocentra-parent/schema-domain/billing-invoice-tax-refund-dispute';

describe('billing invoice tax refund dispute lifecycle', () => {
  provesInvoiceVisibility();
  provesReceiptVisibility();
  provesManualTaxMode();
  provesFullRefundState();
  provesPartialRefundState();
  provesRefundDeniedState();
  provesDisputeOpenedState();
  provesDisputeWonState();
  provesChargebackState();
  provesFailedRenewalGrace();
  provesCancellationStates();
  provesResumeAfterPastDue();
  provesSupportAdminAuditEnvelope();
  provesNoDataDeleteOnRefund();
});

function provesInvoiceVisibility(): void {
  it('billing.invoice-visible keeps provider identity, invoice period, and manual-support ownership explicit', () => {
    const proof = BillingInvoiceTaxRefundDisputeProofSchema.parse(
      BillingInvoiceTaxRefundDisputeProofReadModel
    );
    const activeRow = requiredRow('billing-invoice-active');
    const manualInvoiceRow = requiredRow('billing-tax-manual-support');

    expect(
      proof.rows.filter((row) => row.invoiceVisibility === 'customer-portal-hosted').every((row) => row.portalHosted)
    ).toBe(true);
    expect(
      proof.rows.some(
        (row) =>
          row.invoiceVisibility === 'manual-support-required' &&
          row.failureState !== null &&
          row.supportAuditState === 'audited'
      )
    ).toBe(true);
    expect(activeRow.providerMode).toBe('stripe-hosted');
    expect(activeRow.providerReference).toBe('stripe_invoice_active_backend_ref');
    expect(activeRow.invoiceNumber).toBe('INV-1001');
    expect(activeRow.currency).toBe('USD');
    expect(activeRow.periodStart).toBe('2026-06-14T00:00:00.000Z');
    expect(activeRow.periodEnd).toBe('2026-07-14T00:00:00.000Z');
    expect(manualInvoiceRow.providerMode).toBe('manual-invoice');
    expect(manualInvoiceRow.providerReference).toBeNull();
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...manualInvoiceRow,
        portalHosted: true,
      }).success
    ).toBe(false);
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...activeRow,
        providerReference: null,
      }).success
    ).toBe(false);
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...activeRow,
        periodEnd: activeRow.periodStart,
      }).success
    ).toBe(false);
  });
}

function provesReceiptVisibility(): void {
  it('billing.receipt-visible keeps receipt download visibility explicit and separate from raw provider payloads', () => {
    const receiptRow = requiredRow('billing-invoice-receipt-download');

    expect(receiptRow.invoiceVisibility).toBe('download-link-issued');
    expect(receiptRow.auditReference).toBe('audit-billing-invoice-receipt-download');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...receiptRow,
        invoiceVisibility: 'customer-portal-hosted',
      }).success
    ).toBe(false);
  });
}

function provesManualTaxMode(): void {
  it('billing.tax-mode-decision rejects manual tax handling without a manual-support region state', () => {
    const manualTaxRow = requiredRow('billing-tax-manual-support');

    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...manualTaxRow,
        taxRegionState: 'launch-supported',
      }).success
    ).toBe(false);
  });
}

function provesFullRefundState(): void {
  it('billing.refund-state requires settled refunds to revoke paid access', () => {
    const refundRow = requiredRow('billing-invoice-refund-settled');

    expect(refundRow.entitlementSideEffect).toBe('revoke-paid-access');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...refundRow,
        entitlementSideEffect: 'retain-paid-access',
      }).success
    ).toBe(false);
  });
}

function provesPartialRefundState(): void {
  it('billing.partial-refund-state keeps partial refunds distinct from full refund revocation', () => {
    const partialRefundRow = requiredRow('billing-invoice-partial-refund-issued');

    expect(partialRefundRow.refundState).toBe('refund-issued');
    expect(partialRefundRow.entitlementSideEffect).toBe('retain-paid-access');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...partialRefundRow,
        entitlementSideEffect: 'revoke-paid-access',
      }).success
    ).toBe(false);
  });
}

function provesRefundDeniedState(): void {
  it('billing.refund-failed-state keeps denied refunds explicit and auditable', () => {
    const deniedRefundRow = requiredRow('billing-invoice-refund-denied');

    expect(deniedRefundRow.refundState).toBe('refund-denied');
    expect(deniedRefundRow.supportAuditState).toBe('audited');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...deniedRefundRow,
        entitlementSideEffect: 'revoke-paid-access',
      }).success
    ).toBe(false);
  });
}

function provesDisputeOpenedState(): void {
  it('billing.dispute-state rejects disputes that silently retain paid access', () => {
    const disputeRow = requiredRow('billing-invoice-dispute-opened');

    expect(disputeRow.disputeState).toBe('dispute-opened');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...disputeRow,
        entitlementSideEffect: 'retain-paid-access',
      }).success
    ).toBe(false);
  });
}

function provesDisputeWonState(): void {
  it('billing.dispute-won restores the active paid-access path', () => {
    const disputeWonRow = requiredRow('billing-invoice-dispute-won');

    expect(disputeWonRow.disputeState).toBe('dispute-won');
    expect(disputeWonRow.entitlementSideEffect).toBe('retain-paid-access');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...disputeWonRow,
        entitlementSideEffect: 'manual-review-required',
      }).success
    ).toBe(false);
  });
}

function provesChargebackState(): void {
  it('billing.chargeback-state stays distinct from ordinary refunds and remains manual-review owned', () => {
    const chargebackRow = requiredRow('billing-invoice-chargeback-opened');

    expect(chargebackRow.disputeState).toBe('dispute-opened');
    expect(chargebackRow.invoiceVisibility).toBe('manual-support-required');
    expect(chargebackRow.entitlementSideEffect).toBe('manual-review-required');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...chargebackRow,
        invoiceVisibility: 'customer-portal-hosted',
      }).success
    ).toBe(false);
  });
}

function provesFailedRenewalGrace(): void {
  it('billing.failed-renewal-grace requires an explicit grace paid-access side effect', () => {
    const graceRow = requiredRow('billing-invoice-grace');

    expect(graceRow.entitlementSideEffect).toBe('grace-paid-access');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...graceRow,
        entitlementSideEffect: 'limit-paid-access',
      }).success
    ).toBe(false);
  });
}

function provesCancellationStates(): void {
  it('billing.cancel states keep immediate and period-end semantics distinct', () => {
    const cancelImmediateRow = requiredRow('billing-invoice-cancel-immediate');
    const cancelPeriodEndRow = requiredRow('billing-invoice-cancel-period-end');

    expect(cancelImmediateRow.recoveryState).toBe('cancelled');
    expect(cancelImmediateRow.entitlementSideEffect).toBe('revoke-paid-access');
    expect(cancelPeriodEndRow.recoveryState).toBe('grace');
    expect(cancelPeriodEndRow.entitlementSideEffect).toBe('grace-paid-access');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...cancelPeriodEndRow,
        entitlementSideEffect: 'revoke-paid-access',
      }).success
    ).toBe(false);
  });
}

function provesResumeAfterPastDue(): void {
  it('billing.resume-after-past-due restores an explicit active ledger-backed state', () => {
    const resumedRow = requiredRow('billing-invoice-resume-after-past-due');

    expect(resumedRow.recoveryState).toBe('active');
    expect(resumedRow.parentVisibleState).toBe('available');
    expect(resumedRow.entitlementSideEffect).toBe('retain-paid-access');
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...resumedRow,
        entitlementSideEffect: 'limit-paid-access',
      }).success
    ).toBe(false);
  });
}

function provesSupportAdminAuditEnvelope(): void {
  it('billing.support-admin-audited rejects proof rows that drop the audited back-office envelope', () => {
    const proof = BillingInvoiceTaxRefundDisputeProofReadModel;

    expect(
      BillingInvoiceTaxRefundDisputeProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-self-service-dispute'),
      }).success
    ).toBe(false);
    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...requiredRow('billing-invoice-dispute-lost'),
        supportAuditState: 'missing',
      }).success
    ).toBe(false);
  });
}

function provesNoDataDeleteOnRefund(): void {
  it('billing.no-data-delete-on-refund keeps refund and dispute history audited without child custody claims', () => {
    const proof = BillingInvoiceTaxRefundDisputeProofSchema.parse(
      BillingInvoiceTaxRefundDisputeProofReadModel
    );
    const refundAndDisputeRows = proof.rows.filter(
      (row) => row.refundState !== 'none' || row.disputeState !== 'none'
    );

    expect(proof.nonClaims).toContain('no-child-activity-custody');
    expect(proof.nonClaims).toContain('no-invoice-pdf-custody');
    expect(refundAndDisputeRows.every((row) => row.auditReference.startsWith('audit-'))).toBe(true);
    expect(refundAndDisputeRows.every((row) => row.childActivityCustody === 'not-included')).toBe(true);
  });
}

function requiredRow(
  boundaryId:
    | 'billing-invoice-active'
    | 'billing-tax-manual-support'
    | 'billing-invoice-receipt-download'
    | 'billing-invoice-grace'
    | 'billing-invoice-partial-refund-issued'
    | 'billing-invoice-refund-denied'
    | 'billing-invoice-refund-settled'
    | 'billing-invoice-dispute-opened'
    | 'billing-invoice-dispute-won'
    | 'billing-invoice-dispute-lost'
    | 'billing-invoice-chargeback-opened'
    | 'billing-invoice-cancel-immediate'
    | 'billing-invoice-cancel-period-end'
    | 'billing-invoice-resume-after-past-due'
) {
  const row = BillingInvoiceTaxRefundDisputeProofReadModel.rows.find(
    (entry) => entry.boundaryId === boundaryId
  );
  if (row === undefined) {
    throw new Error(`missing billing invoice lifecycle row: ${boundaryId}`);
  }
  return row;
}

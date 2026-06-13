import { describe, expect, it } from 'vitest';
import {
  BillingInvoiceTaxRefundDisputeProofReadModel,
  BillingInvoiceTaxRefundDisputeProofSchema,
  BillingInvoiceTaxRefundDisputeRowSchema,
} from '../../src/billing-invoice-tax-refund-dispute';

describe('billing invoice tax refund dispute lifecycle', () => {
  it('billing.invoice-visible accepts customer-portal and manual-support invoice visibility with explicit audit context', () => {
    const proof = BillingInvoiceTaxRefundDisputeProofSchema.parse(
      BillingInvoiceTaxRefundDisputeProofReadModel
    );

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
  });

  it('billing.tax-mode-decision rejects manual tax handling without a manual-support region state', () => {
    const manualTaxRow = requiredRow('billing-tax-manual-support');

    expect(
      BillingInvoiceTaxRefundDisputeRowSchema.safeParse({
        ...manualTaxRow,
        taxRegionState: 'launch-supported',
      }).success
    ).toBe(false);
  });

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
});

function requiredRow(
  boundaryId:
    | 'billing-invoice-grace'
    | 'billing-invoice-refund-settled'
    | 'billing-invoice-dispute-opened'
    | 'billing-invoice-dispute-lost'
    | 'billing-tax-manual-support'
) {
  const row = BillingInvoiceTaxRefundDisputeProofReadModel.rows.find(
    (entry) => entry.boundaryId === boundaryId
  );
  if (row === undefined) {
    throw new Error(`missing billing invoice lifecycle row: ${boundaryId}`);
  }
  return row;
}

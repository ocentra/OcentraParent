import { describe, expect, it } from 'vitest';
import {
  BillingSupportAdminReconciliationSummarySchema,
  BillingSupportAdminRefundResultSchema,
} from '@ocentra-parent/schema-domain/billing-support-admin-runtime-boundary';

describe('billing support admin runtime boundary', () => {
  it('accepts partial refund and queued reconciliation summaries', () => {
    const refund = BillingSupportAdminRefundResultSchema.parse({
      requestId: 'refund-partial-proof',
      status: 'accepted',
      invoiceId: 'invoice-parent-demo-active-current',
      refundState: 'refund-requested',
      amountCents: 500,
      auditReference: 'audit:billing:demo-active:refund',
      rejectionReason: null,
    });

    const reconciliation = BillingSupportAdminReconciliationSummarySchema.parse({
      requestId: 'reconciliation-proof',
      status: 'accepted',
      queued: true,
      driftFamiliesVisible: 2,
      retryBacklogVisible: 1,
      deadLetterVisible: 0,
      auditReference: 'audit:billing:reconciliation',
    });

    expect(refund.amountCents).toBe(500);
    expect(reconciliation.queued).toBe(true);
  });

  it('rejects refund rejections that drop the manual-review-required state or reason', () => {
    expect(
      BillingSupportAdminRefundResultSchema.safeParse({
        requestId: 'refund-invalid-rejection',
        status: 'rejected',
        invoiceId: 'invoice-missing',
        refundState: 'refund-requested',
        amountCents: null,
        auditReference: 'audit:refund:rejected',
        rejectionReason: 'invoice-not-found',
      }).success
    ).toBe(false);

    expect(
      BillingSupportAdminRefundResultSchema.safeParse({
        requestId: 'refund-invalid-accepted',
        status: 'accepted',
        invoiceId: 'invoice-parent-demo-active-current',
        refundState: 'refund-settled',
        amountCents: null,
        auditReference: 'audit:billing:demo-active:refund',
        rejectionReason: null,
      }).success
    ).toBe(false);
  });
});

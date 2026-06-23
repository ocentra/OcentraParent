import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { BillingAuditReferenceSchema, NonNegativeBillingCountSchema } from './billing-entitlement-values';

export const BillingSupportAdminRequestIdSchema = brandedNonEmptyStringSchema('BillingSupportAdminRequestId');

export const BillingSupportAdminRefundStatusSchema = withParser(Schema.Literal('accepted', 'rejected'));

export const BillingSupportAdminRefundStateSchema = withParser(
  Schema.Literal('refund-requested', 'refund-settled', 'manual-review-required')
);

export const BillingSupportAdminRefundRejectionReasonSchema = withParser(Schema.Literal('invoice-not-found'));

export const BillingSupportAdminInvoiceIdSchema = brandedNonEmptyStringSchema('BillingSupportAdminInvoiceId');

export const BillingSupportAdminRefundResultSchema = withParser(
  Schema.Struct({
    requestId: BillingSupportAdminRequestIdSchema,
    status: BillingSupportAdminRefundStatusSchema,
    invoiceId: Schema.Union(BillingSupportAdminInvoiceIdSchema, Schema.Null),
    refundState: BillingSupportAdminRefundStateSchema,
    amountCents: Schema.Union(NonNegativeBillingCountSchema, Schema.Null),
    auditReference: BillingAuditReferenceSchema,
    rejectionReason: Schema.Union(BillingSupportAdminRefundRejectionReasonSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (result) =>
        result.status !== 'accepted' ||
        (result.invoiceId !== null && result.amountCents !== null && result.rejectionReason === null) ||
        'Expected accepted refund results to carry an invoice id and amount without a rejection reason'
    ),
    Schema.filter(
      (result) =>
        result.status !== 'rejected' ||
        (result.refundState === 'manual-review-required' &&
          result.amountCents === null &&
          result.rejectionReason === 'invoice-not-found') ||
        'Expected rejected refund results to stay manual-review-required with an explicit invoice-not-found reason'
    )
  )
);

export const BillingSupportAdminReconciliationSummarySchema = withParser(
  Schema.Struct({
    requestId: BillingSupportAdminRequestIdSchema,
    status: Schema.Literal('accepted'),
    queued: Schema.Boolean,
    driftFamiliesVisible: NonNegativeBillingCountSchema,
    retryBacklogVisible: NonNegativeBillingCountSchema,
    deadLetterVisible: NonNegativeBillingCountSchema,
    auditReference: BillingAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.queued ||
        summary.driftFamiliesVisible > 0 ||
        summary.retryBacklogVisible > 0 ||
        summary.deadLetterVisible > 0 ||
        'Expected reconciliation summaries to keep queue or backlog visibility explicit'
    )
  )
);

export type BillingSupportAdminRefundResult = Infer<typeof BillingSupportAdminRefundResultSchema>;
export type BillingSupportAdminReconciliationSummary = Infer<typeof BillingSupportAdminReconciliationSummarySchema>;

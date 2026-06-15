import {
  type Infer,
  NonEmptyStringSchema,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
import {
  FamilyIdSchema,
  ParentAccountIdSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  BillingChildActivityCustodySchema,
  BillingEvidenceExportAccessSchema,
  BillingFailureKindSchema,
  BillingPlanIdSchema,
  NonNegativeBillingCountSchema,
} from './billing-entitlement-values';
import {
  BillingEntitlementSideEffectSchema,
  BillingInvoiceCurrencyCodeSchema,
  BillingInvoiceNumberSchema,
  BillingInvoiceVisibilityStateSchema,
} from './billing-invoice-tax-refund-dispute-values';
import {
  BillingSupportAdminAuditReferenceSchema,
  BillingSupportAdminNonClaimSchema,
  BillingSupportAdminProviderSecretCustodySchema,
} from './billing-support-admin-boundary-values';
import {
  BillingReferralCodeSchema,
  BillingReferralSubjectSchema,
} from './billing-referral-boundary';

export const BillingSupportAdminReadActorRoleSchema = withParser(
  Schema.Literal('support', 'admin')
);

export const BillingSupportAdminParentVisibleStateSchema = withParser(
  Schema.Literal('available', 'grace', 'manual-review')
);

export const BillingSupportAdminInvoiceIdSchema = brandedNonEmptyStringSchema(
  'BillingSupportAdminInvoiceId'
);
export const BillingSupportAdminDisputeIdSchema = brandedNonEmptyStringSchema(
  'BillingSupportAdminDisputeId'
);
export const BillingSupportAdminAuditEventIdSchema =
  brandedNonEmptyStringSchema('BillingSupportAdminAuditEventId');
export const BillingSupportAdminAuditEventTypeSchema =
  brandedNonEmptyStringSchema('BillingSupportAdminAuditEventType');

export const BillingSupportAdminInvoiceProviderSchema = withParser(
  Schema.Literal('stripe', 'manual-invoice')
);

export const BillingSupportAdminInvoicePaymentStateSchema = withParser(
  Schema.Literal('paid', 'grace', 'unpaid', 'refunded')
);

export const BillingSupportAdminReferralAbuseReviewStateSchema = withParser(
  Schema.Literal('clear', 'review-required')
);

export const BillingSupportAdminAuditActorRoleSchema = withParser(
  Schema.Literal('parent', 'guardian', 'support', 'admin', 'system')
);

export const BillingSupportAdminAccountSummarySchema = withParser(
  Schema.Struct({
    parentAccountRef: ParentAccountIdSchema,
    familyRef: FamilyIdSchema,
    parentVisibleState: BillingSupportAdminParentVisibleStateSchema,
    subscriptionStatus: Schema.Union(
      Schema.Literal('active'),
      Schema.Literal('grace'),
      Schema.Literal('past-due')
    ),
    planId: BillingPlanIdSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    providerSecretCustody: BillingSupportAdminProviderSecretCustodySchema,
    manualRequired: Schema.Boolean,
    failureKind: Schema.Union(BillingFailureKindSchema, Schema.Null),
    auditReference: BillingSupportAdminAuditReferenceSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.providerSecretCustody === 'not-present' &&
          summary.childActivityCustody === 'not-included' ||
        'Expected support-admin account summaries to stay free of provider secrets and child activity custody'
    )
  )
);

export const BillingSupportAdminInvoiceSummarySchema = withParser(
  Schema.Struct({
    invoiceId: BillingSupportAdminInvoiceIdSchema,
    invoiceNumber: BillingInvoiceNumberSchema,
    parentAccountRef: ParentAccountIdSchema,
    familyRef: FamilyIdSchema,
    planId: BillingPlanIdSchema,
    currency: BillingInvoiceCurrencyCodeSchema,
    subtotalCents: NonNegativeBillingCountSchema,
    taxCents: NonNegativeBillingCountSchema,
    totalCents: NonNegativeBillingCountSchema,
    invoiceVisibility: BillingInvoiceVisibilityStateSchema,
    paymentState: BillingSupportAdminInvoicePaymentStateSchema,
    provider: BillingSupportAdminInvoiceProviderSchema,
    hostedUrl: Schema.Union(NonEmptyStringSchema, Schema.Null),
    periodStart: ParentTimestampSchema,
    periodEnd: ParentTimestampSchema,
    updatedAt: ParentTimestampSchema,
    auditReference: BillingSupportAdminAuditReferenceSchema,
    manualRequired: Schema.Boolean,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.totalCents === summary.subtotalCents + summary.taxCents ||
        'Expected support-admin invoice totals to equal subtotal plus tax'
    ),
    Schema.filter(
      (summary) =>
        summary.manualRequired ===
          (summary.invoiceVisibility === 'manual-support-required') ||
        'Expected support-admin invoice manualRequired to match manual invoice visibility'
    )
  )
);

export const BillingSupportAdminDisputeSummarySchema = withParser(
  Schema.Struct({
    disputeId: BillingSupportAdminDisputeIdSchema,
    parentAccountRef: ParentAccountIdSchema,
    familyRef: FamilyIdSchema,
    invoiceId: BillingSupportAdminInvoiceIdSchema,
    disputeState: Schema.Union(
      Schema.Literal('dispute-opened'),
      Schema.Literal('dispute-won'),
      Schema.Literal('dispute-lost')
    ),
    entitlementEffect: BillingEntitlementSideEffectSchema,
    manualRequired: Schema.Boolean,
    auditReference: BillingSupportAdminAuditReferenceSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.disputeState !== 'dispute-opened' ||
          summary.entitlementEffect === 'manual-review-required' ||
        'Expected open disputes to retain manual-review-required entitlement effect'
    )
  )
);

export const BillingSupportAdminReferralSummarySchema = withParser(
  Schema.Struct({
    referralCode: BillingReferralCodeSchema,
    ownerSubject: BillingReferralSubjectSchema,
    creditedFamilies: NonNegativeBillingCountSchema,
    invitedFamilies: NonNegativeBillingCountSchema,
    abuseReviewState: BillingSupportAdminReferralAbuseReviewStateSchema,
    auditReference: BillingSupportAdminAuditReferenceSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.invitedFamilies >= summary.creditedFamilies ||
        'Expected support-admin referral summaries not to over-credit more families than were invited'
    )
  )
);

export const BillingSupportAdminAuditEventSummarySchema = withParser(
  Schema.Struct({
    eventId: BillingSupportAdminAuditEventIdSchema,
    eventType: BillingSupportAdminAuditEventTypeSchema,
    actorRole: BillingSupportAdminAuditActorRoleSchema,
    parentAccountRef: ParentAccountIdSchema,
    familyRef: FamilyIdSchema,
    auditReference: BillingSupportAdminAuditReferenceSchema,
    createdAt: ParentTimestampSchema,
  })
);

export const BillingSupportAdminAccountsResponseSchema = withParser(
  Schema.Struct({
    status: Schema.Literal('ok'),
    actorRole: BillingSupportAdminReadActorRoleSchema,
    resultCount: NonNegativeBillingCountSchema,
    manualActionsPending: NonNegativeBillingCountSchema,
    nonClaims: Schema.Array(BillingSupportAdminNonClaimSchema),
    results: Schema.Array(BillingSupportAdminAccountSummarySchema),
  }).pipe(
    Schema.filter(
      (response) =>
        response.resultCount === response.results.length ||
        'Expected support-admin account response resultCount to match the result rows'
    ),
    Schema.filter(
      (response) =>
        response.manualActionsPending ===
          response.results.filter((row) => row.manualRequired).length ||
        'Expected support-admin account response manualActionsPending to match manualRequired rows'
    )
  )
);

export const BillingSupportAdminInvoicesResponseSchema = withParser(
  Schema.Struct({
    status: Schema.Literal('ok'),
    actorRole: BillingSupportAdminReadActorRoleSchema,
    resultCount: NonNegativeBillingCountSchema,
    results: Schema.Array(BillingSupportAdminInvoiceSummarySchema),
  }).pipe(
    Schema.filter(
      (response) =>
        response.resultCount === response.results.length ||
        'Expected support-admin invoice response resultCount to match the result rows'
    )
  )
);

export const BillingSupportAdminDisputesResponseSchema = withParser(
  Schema.Struct({
    status: Schema.Literal('ok'),
    actorRole: BillingSupportAdminReadActorRoleSchema,
    resultCount: NonNegativeBillingCountSchema,
    results: Schema.Array(BillingSupportAdminDisputeSummarySchema),
  }).pipe(
    Schema.filter(
      (response) =>
        response.resultCount === response.results.length ||
        'Expected support-admin dispute response resultCount to match the result rows'
    )
  )
);

export const BillingSupportAdminReferralsResponseSchema = withParser(
  Schema.Struct({
    status: Schema.Literal('ok'),
    actorRole: BillingSupportAdminReadActorRoleSchema,
    resultCount: NonNegativeBillingCountSchema,
    results: Schema.Array(BillingSupportAdminReferralSummarySchema),
  }).pipe(
    Schema.filter(
      (response) =>
        response.resultCount === response.results.length ||
        'Expected support-admin referral response resultCount to match the result rows'
    )
  )
);

export const BillingSupportAdminAuditEventsResponseSchema = withParser(
  Schema.Struct({
    status: Schema.Literal('ok'),
    actorRole: BillingSupportAdminReadActorRoleSchema,
    resultCount: NonNegativeBillingCountSchema,
    results: Schema.Array(BillingSupportAdminAuditEventSummarySchema),
  }).pipe(
    Schema.filter(
      (response) =>
        response.resultCount === response.results.length ||
        'Expected support-admin audit response resultCount to match the result rows'
    )
  )
);

export type BillingSupportAdminReadActorRole = Infer<
  typeof BillingSupportAdminReadActorRoleSchema
>;
export type BillingSupportAdminAccountSummary = Infer<
  typeof BillingSupportAdminAccountSummarySchema
>;
export type BillingSupportAdminInvoiceSummary = Infer<
  typeof BillingSupportAdminInvoiceSummarySchema
>;
export type BillingSupportAdminDisputeSummary = Infer<
  typeof BillingSupportAdminDisputeSummarySchema
>;
export type BillingSupportAdminReferralSummary = Infer<
  typeof BillingSupportAdminReferralSummarySchema
>;
export type BillingSupportAdminAuditEventSummary = Infer<
  typeof BillingSupportAdminAuditEventSummarySchema
>;
export type BillingSupportAdminAccountsResponse = Infer<
  typeof BillingSupportAdminAccountsResponseSchema
>;
export type BillingSupportAdminInvoicesResponse = Infer<
  typeof BillingSupportAdminInvoicesResponseSchema
>;
export type BillingSupportAdminDisputesResponse = Infer<
  typeof BillingSupportAdminDisputesResponseSchema
>;
export type BillingSupportAdminReferralsResponse = Infer<
  typeof BillingSupportAdminReferralsResponseSchema
>;
export type BillingSupportAdminAuditEventsResponse = Infer<
  typeof BillingSupportAdminAuditEventsResponseSchema
>;

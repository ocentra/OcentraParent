import { describe, expect, it } from 'vitest';
import {
  BillingSupportAdminAccountsResponseSchema,
  BillingSupportAdminAuditEventsResponseSchema,
  BillingSupportAdminDisputesResponseSchema,
  BillingSupportAdminInvoicesResponseSchema,
  BillingSupportAdminReferralsResponseSchema,
} from '@ocentra-parent/schema-domain/billing-support-admin-api-boundary';

const validAccountsResponse = {
  status: 'ok',
  actorRole: 'support',
  resultCount: 1,
  manualActionsPending: 1,
  nonClaims: [
    'no-provider-secrets',
    'no-billing-provider-contact',
    'no-support-backend-upload',
    'no-child-activity-custody',
  ],
  results: [
    {
      parentAccountRef: 'parent-account:demo-review',
      familyRef: 'family:demo-review',
      parentVisibleState: 'manual-review',
      subscriptionStatus: 'past-due',
      planId: 'family-plus-monthly',
      evidenceExportAccess: 'retained',
      childActivityCustody: 'not-included',
      providerSecretCustody: 'not-present',
      manualRequired: true,
      failureKind: 'validation-failed',
      auditReference: 'audit:billing:account-1',
      updatedAt: '2026-06-14T00:00:00.000Z',
    },
  ],
} as const;

const validInvoicesResponse = {
  status: 'ok',
  actorRole: 'support',
  resultCount: 1,
  results: [
    {
      invoiceId: 'invoice-1',
      invoiceNumber: 'INV-1001',
      parentAccountRef: 'parent-account:demo-active',
      familyRef: 'family:demo-active',
      planId: 'family-plus-monthly',
      currency: 'USD',
      subtotalCents: 1000,
      taxCents: 100,
      totalCents: 1100,
      invoiceVisibility: 'customer-portal-hosted',
      paymentState: 'paid',
      provider: 'stripe',
      hostedUrl: 'https://billing.stripe.com/invoice/invoice-1',
      periodStart: '2026-06-01T00:00:00.000Z',
      periodEnd: '2026-07-01T00:00:00.000Z',
      updatedAt: '2026-06-14T00:00:00.000Z',
      auditReference: 'audit:billing:invoice-1',
      manualRequired: false,
    },
  ],
} as const;

const validDisputesResponse = {
  status: 'ok',
  actorRole: 'admin',
  resultCount: 1,
  results: [
    {
      disputeId: 'dispute-1',
      parentAccountRef: 'parent-account:demo-review',
      familyRef: 'family:demo-review',
      invoiceId: 'invoice-1',
      disputeState: 'dispute-opened',
      entitlementEffect: 'manual-review-required',
      manualRequired: true,
      auditReference: 'audit:billing:dispute-1',
      updatedAt: '2026-06-14T00:00:00.000Z',
    },
  ],
} as const;

const validReferralsResponse = {
  status: 'ok',
  actorRole: 'admin',
  resultCount: 1,
  results: [
    {
      referralCode: 'REF-FAMILY-CORE',
      ownerSubject: 'parent:demo-active',
      creditedFamilies: 1,
      invitedFamilies: 2,
      abuseReviewState: 'review-required',
      auditReference: 'audit:referral:ref-family-core',
      updatedAt: '2026-06-14T00:00:00.000Z',
    },
  ],
} as const;

const validAuditEventsResponse = {
  status: 'ok',
  actorRole: 'admin',
  resultCount: 1,
  results: [
    {
      eventId: 'audit-event-1',
      eventType: 'billing.invoice.visible',
      actorRole: 'support',
      parentAccountRef: 'parent-account:demo-active',
      familyRef: 'family:demo-active',
      auditReference: 'audit:billing:event-1',
      createdAt: '2026-06-14T00:00:00.000Z',
    },
  ],
} as const;

const invalidAccountsResponse = {
  ...validAccountsResponse,
  manualActionsPending: 0,
} as const;

const invalidInvoicesResponse = {
  ...validInvoicesResponse,
  results: [
    {
      ...validInvoicesResponse.results[0],
      invoiceVisibility: 'manual-support-required',
      manualRequired: false,
    },
  ],
} as const;

describe('billing support admin api boundary', () => {
  it('accepts support/admin read envelopes with explicit counts and redacted result rows', () => {
    const accounts =
      BillingSupportAdminAccountsResponseSchema.parse(validAccountsResponse);
    const invoices =
      BillingSupportAdminInvoicesResponseSchema.parse(validInvoicesResponse);
    const disputes =
      BillingSupportAdminDisputesResponseSchema.parse(validDisputesResponse);
    const referrals =
      BillingSupportAdminReferralsResponseSchema.parse(validReferralsResponse);
    const audit = BillingSupportAdminAuditEventsResponseSchema.parse(
      validAuditEventsResponse
    );

    expect(accounts.manualActionsPending).toBe(1);
    expect(invoices.results[0]?.totalCents).toBe(1100);
    expect(disputes.results[0]?.entitlementEffect).toBe(
      'manual-review-required'
    );
    expect(referrals.results[0]?.invitedFamilies).toBe(2);
    expect(audit.results[0]?.actorRole).toBe('support');
  });

  it('rejects mismatched counts and inconsistent manual/support-safe fields', () => {
    expect(
      BillingSupportAdminAccountsResponseSchema.safeParse(
        invalidAccountsResponse
      ).success
    ).toBe(false);

    expect(
      BillingSupportAdminInvoicesResponseSchema.safeParse(
        invalidInvoicesResponse
      ).success
    ).toBe(false);
  });
});

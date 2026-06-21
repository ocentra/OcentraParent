import { describe, expect, it } from 'vitest';
import {
  BillingReferralInviteResultSchema,
  BillingReferralSummarySchema,
} from '@ocentra-parent/schema-domain/billing-referral-boundary';

describe('billing referral boundary', () => {
  it('accepts referral summaries with explicit invite and credit counts', () => {
    const summary = BillingReferralSummarySchema.parse({
      subject: 'parent:demo-active',
      referralCode: 'REF-FAMILY-CORE',
      availableCredits: 2,
      activeReferredParents: 2,
      pendingInvites: 1,
      invites: [
        {
          inviteId: 'REF-FAMILY-CORE-invite-1',
          inviteState: 'invite-created',
          referralCode: 'REF-FAMILY-CORE',
          invitedIdentifier: 'invite+family-core@example.com',
          auditReference: 'audit:billing:demo-active:referral-invite',
          updatedAt: '2026-06-14T00:00:00.000Z',
        },
      ],
      auditReference: 'audit:billing:demo-active:referrals',
    });

    expect(summary.availableCredits).toBe(2);
    expect(summary.invites).toHaveLength(1);
  });

  it('rejects referral summaries when active referred parents drift from available credits', () => {
    expect(
      BillingReferralSummarySchema.safeParse({
        subject: 'parent:demo-active',
        referralCode: 'REF-FAMILY-CORE',
        availableCredits: 2,
        activeReferredParents: 1,
        pendingInvites: 1,
        invites: [],
        auditReference: 'audit:billing:demo-active:referrals',
      }).success
    ).toBe(false);
  });

  it('accepts explicit device-farm rejection and payment-method manual-review outcomes', () => {
    const rejected = BillingReferralInviteResultSchema.parse({
      requestId: 'referral-device-farm',
      status: 'rejected',
      inviteState: 'fraud-review',
      referralCode: 'REF-FAMILY-CORE',
      rejectionReason: 'same-device-farm-rejected',
      auditReference: 'audit:billing:demo-active:invite-device-farm-review',
    });

    const manualReview = BillingReferralInviteResultSchema.parse({
      requestId: 'referral-same-payment-method',
      status: 'manual-review',
      inviteState: 'fraud-review',
      referralCode: 'REF-FAMILY-CORE',
      rejectionReason: 'same-payment-method-manual-review',
      auditReference: 'audit:billing:demo-active:invite-payment-method-review',
    });

    expect(rejected.status).toBe('rejected');
    expect(manualReview.status).toBe('manual-review');
  });

  it('rejects manual-review outcomes that drop the fraud-review state or use a rejected-only reason', () => {
    expect(
      BillingReferralInviteResultSchema.safeParse({
        requestId: 'referral-invalid-manual-review',
        status: 'manual-review',
        inviteState: null,
        referralCode: 'REF-FAMILY-CORE',
        rejectionReason: 'same-payment-method-manual-review',
        auditReference: 'audit:billing:demo-active:invite-payment-method-review',
      }).success
    ).toBe(false);

    expect(
      BillingReferralInviteResultSchema.safeParse({
        requestId: 'referral-invalid-rejected',
        status: 'rejected',
        inviteState: 'fraud-review',
        referralCode: 'REF-FAMILY-CORE',
        rejectionReason: 'same-payment-method-manual-review',
        auditReference: 'audit:billing:demo-active:invite-payment-method-review',
      }).success
    ).toBe(false);
  });
});

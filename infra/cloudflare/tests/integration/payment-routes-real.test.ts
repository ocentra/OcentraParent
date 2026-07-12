import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  BillingReferralInviteResultSchema,
  BillingReferralSummarySchema,
  type BillingReferralSummary,
  BillingSupportAdminAccountsResponseSchema,
  type BillingSupportAdminAccountsResponse,
  BillingSupportAdminAuditEventsResponseSchema,
  type BillingSupportAdminAuditEventsResponse,
  BillingSupportAdminDisputesResponseSchema,
  BillingSupportAdminInvoicesResponseSchema,
  BillingSupportAdminReferralsResponseSchema,
  BillingSupportAdminReconciliationSummarySchema,
  BillingSupportAdminRefundResultSchema,
} from '../../src/generated/billing-contracts.js';
import { createTestHarness, executeRequest, readJson } from '../../src/testing.js';

interface BillingInvoicesResponse {
  status: string;
  invoices: Array<{
    paymentState: string;
    provider?: string;
  }>;
}

interface BillingStatusResponse {
  accountStatus?: string;
  manualInvoiceState?: {
    visible: boolean;
    invoiceState: string | null;
  };
  plan: {
    planId: string;
  };
  parentVisibleState?: string;
  providerMode?: string;
  subscriptionStatus?: string;
  deviceUsage: {
    limit: number;
  };
  warnings: ReadonlyArray<string>;
}

interface BillingEntitlementSnapshotResponse {
  snapshot: {
    planId: string;
    deviceLimit: number;
    signatureState: string;
    subscriptionStatus: string;
    source: string;
    availableDeviceSlots: number;
  };
}

interface BillingLicenseResponse {
  decision: string;
  reasonCode: string;
  deviceActivationBehavior: string;
  requestedDeviceAlreadyTrusted: boolean;
  currentActiveDevices: number;
  limit: number;
}

interface BillingPlanChangeResponse {
  status: 'accepted' | 'rejected';
  changeKind: 'upgrade' | 'downgrade' | 'invalid';
  hostedUrl: string | null;
  rejectionReason: 'invalid-plan' | null;
}

interface BillingCancelResponse {
  status: string;
  cancellationState: string;
}

interface ManualInvoiceResponse {
  invoiceState: string;
}

interface ManualInvoiceQueuedResponse extends ManualInvoiceResponse {
  queued: boolean;
  region: string;
}

const interactiveHeaders = {
  origin: 'http://localhost:3000',
  authorization: 'Bearer parent:demo-active',
  'x-ocentra-csrf': 'interactive-parent-session',
};

describe('real payment worker routes', () => {
  it('returns billing-safe invoice summaries', async () => {
    const { response } = await executeRequest({
      path: '/auth/billing/invoices',
      headers: {
        authorization: 'Bearer parent:demo-grace',
      },
    });

    const body = await readJson<BillingInvoicesResponse>(response);
    assert.equal(response.status, 200);
    assert.equal(body.status, 'ok');
    assert.ok(body.invoices.some((invoice) => invoice.paymentState === 'grace'));
  });

  it('returns referral state and supports explicit invite creation', async () => {
    const harness = createTestHarness();

    const referrals = await executeRequest({
      path: '/auth/billing/referrals',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const referralsBody = BillingReferralSummarySchema.parse(await readJson<unknown>(referrals.response));
    assert.equal(referrals.response.status, 200);
    assert.ok(typeof referralsBody.referralCode === 'string');

    const invite = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'invite-accepted',
        invitee: 'new-family@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const inviteBody = BillingReferralInviteResultSchema.parse(await readJson<unknown>(invite.response));
    assert.equal(invite.response.status, 200);
    assert.equal(inviteBody.status, 'accepted');
    assert.equal(inviteBody.inviteState, 'invite-created');

    const updatedReferrals = await executeRequest({
      path: '/auth/billing/referrals',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const updatedReferralsBody = BillingReferralSummarySchema.parse(await readJson<unknown>(updatedReferrals.response));
    assert.equal(updatedReferrals.response.status, 200);
    assert.equal(updatedReferralsBody.pendingInvites, referralsBody.pendingInvites + 1);
    assert.ok(
      updatedReferralsBody.invites.some(
        (inviteSummary: (typeof updatedReferralsBody.invites)[number]) =>
          inviteSummary.invitedIdentifier === 'new-family@example.com' && inviteSummary.inviteState === 'invite-created'
      )
    );

    const adminReferrals = await executeRequest({
      path: '/admin/billing/referrals',
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const adminReferralsBody = BillingSupportAdminReferralsResponseSchema.parse(
      await readJson<unknown>(adminReferrals.response)
    );
    const adminReferral = adminReferralsBody.results.find(
      (row: (typeof adminReferralsBody.results)[number]) => row.referralCode === updatedReferralsBody.referralCode
    );
    assert.equal(adminReferrals.response.status, 200);
    assert.equal(adminReferral?.invitedFamilies, 5);
    assert.equal(adminReferral?.creditedFamilies, 2);
  });

  it('keeps referral invite rejections explicit and non-queued for same-household and self-referral cases', async () => {
    const harness = createTestHarness();

    const sameHouseholdInvite = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'invite-same-household',
        invitee: 'same-household@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const sameHouseholdBody = BillingReferralInviteResultSchema.parse(
      await readJson<unknown>(sameHouseholdInvite.response)
    );
    assert.equal(sameHouseholdInvite.response.status, 200);
    assert.equal(sameHouseholdBody.status, 'rejected');
    assert.equal(sameHouseholdBody.inviteState, null);
    assert.equal(sameHouseholdBody.rejectionReason, 'same-household-rejected');
    assert.equal(harness.queueMessages.length, 0);

    const selfReferralInvite = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'invite-self-referral',
        invitee: 'parentdemoactive@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const selfReferralBody = BillingReferralInviteResultSchema.parse(
      await readJson<unknown>(selfReferralInvite.response)
    );
    assert.equal(selfReferralInvite.response.status, 200);
    assert.equal(selfReferralBody.status, 'rejected');
    assert.equal(selfReferralBody.inviteState, null);
    assert.equal(selfReferralBody.rejectionReason, 'self-referral-rejected');
    assert.equal(harness.queueMessages.length, 0);
  });

  it('keeps referral abuse outcomes explicit and non-queued for device-farm, payment-method, and generic fraud-review cases', async () => {
    const harness = createTestHarness();

    const deviceFarmInvite = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'invite-device-farm',
        invitee: 'device-farm@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const deviceFarmBody = BillingReferralInviteResultSchema.parse(await readJson<unknown>(deviceFarmInvite.response));
    assert.equal(deviceFarmInvite.response.status, 200);
    assert.equal(deviceFarmBody.status, 'rejected');
    assert.equal(deviceFarmBody.inviteState, 'fraud-review');
    assert.equal(deviceFarmBody.rejectionReason, 'same-device-farm-rejected');
    assert.equal(harness.queueMessages.length, 0);

    const paymentMethodInvite = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'invite-same-payment-method',
        invitee: 'same-payment-method@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const paymentMethodBody = BillingReferralInviteResultSchema.parse(
      await readJson<unknown>(paymentMethodInvite.response)
    );
    assert.equal(paymentMethodInvite.response.status, 200);
    assert.equal(paymentMethodBody.status, 'manual-review');
    assert.equal(paymentMethodBody.inviteState, 'fraud-review');
    assert.equal(paymentMethodBody.rejectionReason, 'same-payment-method-manual-review');
    assert.equal(harness.queueMessages.length, 0);

    const fraudReviewInvite = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'invite-fraud-review',
        invitee: 'fraud-review@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const fraudReviewBody = BillingReferralInviteResultSchema.parse(
      await readJson<unknown>(fraudReviewInvite.response)
    );
    assert.equal(fraudReviewInvite.response.status, 200);
    assert.equal(fraudReviewBody.status, 'manual-review');
    assert.equal(fraudReviewBody.inviteState, 'fraud-review');
    assert.equal(fraudReviewBody.rejectionReason, 'fraud-review');
    assert.equal(harness.queueMessages.length, 0);
  });

  it('returns signed entitlement snapshot capacity fields and a grace license decision for trusted devices', async () => {
    const snapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      headers: {
        authorization: 'Bearer parent:demo-grace',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const snapshotBody = await readJson<BillingEntitlementSnapshotResponse>(snapshot.response);
    assert.equal(snapshot.response.status, 200);
    assert.equal(snapshotBody.snapshot.signatureState, 'signed');
    assert.equal(snapshotBody.snapshot.subscriptionStatus, 'grace');
    assert.equal(snapshotBody.snapshot.source, 'signed-local-snapshot');
    assert.equal(snapshotBody.snapshot.availableDeviceSlots, 1);

    const license = await executeRequest({
      path: '/auth/billing/license-check',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-grace',
        'x-ocentra-trusted-device': 'true',
      },
      body: {
        requestId: 'license-grace',
        deviceId: 'device-1',
        requestedNewDevice: true,
      },
    });
    const licenseBody = await readJson<BillingLicenseResponse>(license.response);
    assert.equal(license.response.status, 200);
    assert.equal(licenseBody.decision, 'grace');
    assert.equal(licenseBody.reasonCode, 'payment-required');
    assert.equal(licenseBody.deviceActivationBehavior, 'grace-existing-devices');
    assert.equal(licenseBody.requestedDeviceAlreadyTrusted, false);
    assert.equal(licenseBody.currentActiveDevices, 9);
    assert.equal(licenseBody.limit, 10);
  });

  it('denies new trusted-device activation when the effective device limit is already reached', async () => {
    const snapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const snapshotBody = await readJson<BillingEntitlementSnapshotResponse>(snapshot.response);
    assert.equal(snapshot.response.status, 200);
    assert.equal(snapshotBody.snapshot.availableDeviceSlots, 0);

    const license = await executeRequest({
      path: '/auth/billing/license-check',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        'x-ocentra-trusted-device': 'true',
      },
      body: {
        requestId: 'license-maxed',
        deviceId: 'device-6',
        requestedNewDevice: true,
      },
    });
    const licenseBody = await readJson<BillingLicenseResponse>(license.response);
    assert.equal(license.response.status, 200);
    assert.equal(licenseBody.decision, 'denied');
    assert.equal(licenseBody.reasonCode, 'limit-exceeded');
    assert.equal(licenseBody.deviceActivationBehavior, 'deny-new-device');
    assert.equal(licenseBody.requestedDeviceAlreadyTrusted, false);
    assert.equal(licenseBody.currentActiveDevices, 5);
    assert.equal(licenseBody.limit, 5);
  });

  it('allows already-trusted device checks at the plan limit without granting new capacity', async () => {
    const license = await executeRequest({
      path: '/auth/billing/license-check',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        'x-ocentra-trusted-device': 'true',
      },
      body: {
        requestId: 'license-maxed-existing',
        deviceId: 'device-5',
        requestedNewDevice: false,
      },
    });
    const licenseBody = await readJson<BillingLicenseResponse>(license.response);
    assert.equal(license.response.status, 200);
    assert.equal(licenseBody.decision, 'allowed');
    assert.equal(licenseBody.reasonCode, 'within-plan');
    assert.equal(licenseBody.deviceActivationBehavior, 'allow-new-device');
    assert.equal(licenseBody.requestedDeviceAlreadyTrusted, true);
    assert.equal(licenseBody.currentActiveDevices, 5);
    assert.equal(licenseBody.limit, 5);
  });

  it('marks manual-admin entitlement snapshots as manual-required instead of signed', async () => {
    const snapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      headers: {
        authorization: 'Bearer parent:demo-review',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const snapshotBody = await readJson<BillingEntitlementSnapshotResponse>(snapshot.response);
    assert.equal(snapshot.response.status, 200);
    assert.equal(snapshotBody.snapshot.signatureState, 'manual-required');
    assert.equal(snapshotBody.snapshot.subscriptionStatus, 'past-due');
    assert.equal(snapshotBody.snapshot.source, 'manual-admin-review');
    assert.equal(snapshotBody.snapshot.availableDeviceSlots, 0);
  });

  it('queues accepted change-plan and cancel reconciliation actions with real route summaries', async () => {
    const harness = createTestHarness();

    const changePlan = await executeRequest({
      path: '/auth/billing/change-plan',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'change-plan-upgrade',
        planId: 'family-max',
        abuseGateState: 'passed-turnstile',
      },
    });
    const changePlanBody = await readJson<BillingPlanChangeResponse>(changePlan.response);
    assert.equal(changePlan.response.status, 200);
    assert.equal(changePlanBody.status, 'accepted');
    assert.equal(changePlanBody.changeKind, 'upgrade');
    if (changePlanBody.hostedUrl === null) {
      assert.fail('expected accepted change-plan requests to produce a hostedUrl');
    }
    assert.match(changePlanBody.hostedUrl, /^https:\/\/checkout\.stripe\.com\//);
    assert.equal(harness.queueMessages.length, 1);
    assert.deepEqual(harness.queueMessages[0], {
      action: 'change-plan',
      requestId: 'change-plan-upgrade',
      subject: 'parent:demo-active',
      targetPlanId: 'family-max',
    });

    const updatedStatus = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const updatedStatusBody = await readJson<BillingStatusResponse>(updatedStatus.response);
    assert.equal(updatedStatus.response.status, 200);
    assert.equal(updatedStatusBody.plan.planId, 'family-max');
    assert.equal(updatedStatusBody.deviceUsage.limit, 10);
    assert.ok(updatedStatusBody.warnings.includes('plan-change-pending-provider-sync'));

    const updatedSnapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const updatedSnapshotBody = await readJson<BillingEntitlementSnapshotResponse>(updatedSnapshot.response);
    assert.equal(updatedSnapshot.response.status, 200);
    assert.equal(updatedSnapshotBody.snapshot.planId, 'family-max');
    assert.equal(updatedSnapshotBody.snapshot.deviceLimit, 10);

    const rejectedChangePlan = await executeRequest({
      path: '/auth/billing/change-plan',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'change-plan-invalid',
        planId: 'family-free',
        abuseGateState: 'passed-turnstile',
      },
    });
    const rejectedChangePlanBody = await readJson<BillingPlanChangeResponse>(rejectedChangePlan.response);
    assert.equal(rejectedChangePlan.response.status, 200);
    assert.equal(rejectedChangePlanBody.status, 'rejected');
    assert.equal(rejectedChangePlanBody.rejectionReason, 'invalid-plan');
    assert.equal(harness.queueMessages.length, 1);

    const cancel = await executeRequest({
      path: '/auth/billing/cancel',
      method: 'POST',
      harness,
      headers: {
        origin: 'http://localhost:3000',
        authorization: 'Bearer parent:demo-grace',
        'x-ocentra-csrf': 'interactive-parent-session',
      },
      body: {
        requestId: 'cancel-grace-subscription',
        abuseGateState: 'trusted-authenticated-session',
      },
    });
    const cancelBody = await readJson<BillingCancelResponse>(cancel.response);
    assert.equal(cancel.response.status, 200);
    assert.equal(cancelBody.status, 'accepted');
    assert.equal(cancelBody.cancellationState, 'already-in-grace');
    assert.equal(harness.queueMessages.length, 2);
    assert.deepEqual(harness.queueMessages[1], {
      action: 'cancel',
      requestId: 'cancel-grace-subscription',
      subject: 'parent:demo-grace',
      cancellationState: 'already-in-grace',
    });

    const cancelledStatus = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-grace',
      },
    });
    const cancelledStatusBody = await readJson<BillingStatusResponse>(cancelledStatus.response);
    assert.equal(cancelledStatus.response.status, 200);
    assert.ok(cancelledStatusBody.warnings.includes('cancellation-confirmed-in-grace'));
  });

  it('deduplicates repeated billing write requests through durable-object idempotency', async () => {
    const harness = createTestHarness();

    const firstChangePlan = await executeRequest({
      path: '/auth/billing/change-plan',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'change-plan-dedupe',
        planId: 'family-max',
        abuseGateState: 'passed-turnstile',
      },
    });
    const secondChangePlan = await executeRequest({
      path: '/auth/billing/change-plan',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'change-plan-dedupe',
        planId: 'family-max',
        abuseGateState: 'passed-turnstile',
      },
    });

    assert.deepEqual(
      await readJson<BillingPlanChangeResponse>(firstChangePlan.response),
      await readJson<BillingPlanChangeResponse>(secondChangePlan.response)
    );
    assert.deepEqual(harness.queueMessages, [
      {
        action: 'change-plan',
        requestId: 'change-plan-dedupe',
        subject: 'parent:demo-active',
        targetPlanId: 'family-max',
      },
    ]);

    const firstInvite = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'referral-dedupe',
        invitee: 'dedupe-family@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const secondInvite = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'referral-dedupe',
        invitee: 'dedupe-family@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });

    assert.deepEqual(
      BillingReferralInviteResultSchema.parse(await readJson<unknown>(firstInvite.response)),
      BillingReferralInviteResultSchema.parse(await readJson<unknown>(secondInvite.response))
    );
    assert.deepEqual(harness.queueMessages, [
      {
        action: 'change-plan',
        requestId: 'change-plan-dedupe',
        subject: 'parent:demo-active',
        targetPlanId: 'family-max',
      },
      {
        action: 'referral-invite',
        requestId: 'referral-dedupe',
        subject: 'parent:demo-active',
        referralCode: 'REF-FAMILY-CORE',
      },
    ]);

    const referralSummary = BillingReferralSummarySchema.parse(
      await readJson<unknown>(
        (
          await executeRequest({
            path: '/auth/billing/referrals',
            harness,
            headers: {
              authorization: 'Bearer parent:demo-active',
            },
          })
        ).response
      )
    );
    assert.equal(referralSummary.pendingInvites, 3);
    assert.equal(
      referralSummary.invites.filter(
        (inviteSummary: BillingReferralSummary['invites'][number]) =>
          inviteSummary.invitedIdentifier === 'dedupe-family@example.com'
      ).length,
      1
    );
  });

  it('queues referral, manual-invoice, refund, and reconciliation actions with billing-safe payloads', async () => {
    const harness = createTestHarness();

    const referralInvite = await executeRequest({
      path: '/auth/billing/referral-invite',
      method: 'POST',
      harness,
      headers: interactiveHeaders,
      body: {
        requestId: 'invite-queued',
        invitee: 'queued-family@example.com',
        abuseGateState: 'passed-turnstile',
      },
    });
    const referralInviteBody = BillingReferralInviteResultSchema.parse(
      await readJson<unknown>(referralInvite.response)
    );
    assert.equal(referralInvite.response.status, 200);
    assert.equal(referralInviteBody.status, 'accepted');

    const manualInvoice = await executeRequest({
      path: '/auth/billing/manual-invoice',
      method: 'POST',
      harness,
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
      body: {
        requestId: 'manual-invoice-queued',
        region: 'pakistan',
      },
    });
    const manualInvoiceBody = await readJson<ManualInvoiceQueuedResponse>(manualInvoice.response);
    assert.equal(manualInvoice.response.status, 202);
    assert.equal(manualInvoiceBody.invoiceState, 'manual-support-required');
    assert.equal(manualInvoiceBody.queued, true);

    const manualInvoiceStatus = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:support-agent',
      },
    });
    const manualInvoiceStatusBody = await readJson<BillingStatusResponse>(manualInvoiceStatus.response);
    assert.equal(manualInvoiceStatus.response.status, 200);
    assert.equal(manualInvoiceStatusBody.accountStatus, 'manual-review');
    assert.equal(manualInvoiceStatusBody.parentVisibleState, 'manual-review');
    assert.equal(manualInvoiceStatusBody.providerMode, 'manual-invoice');
    assert.equal(manualInvoiceStatusBody.manualInvoiceState?.visible, true);
    assert.equal(manualInvoiceStatusBody.manualInvoiceState?.invoiceState, 'manual-support-required');

    const manualInvoiceInvoices = await executeRequest({
      path: '/auth/billing/invoices',
      harness,
      headers: {
        authorization: 'Bearer parent:support-agent',
      },
    });
    const manualInvoiceInvoicesBody = await readJson<BillingInvoicesResponse>(manualInvoiceInvoices.response);
    assert.equal(manualInvoiceInvoices.response.status, 200);
    assert.ok(
      manualInvoiceInvoicesBody.invoices.some(
        (invoice) => invoice.provider === 'manual-invoice' && invoice.paymentState === 'unpaid'
      )
    );

    const refund = await executeRequest({
      path: '/admin/billing/refunds',
      method: 'POST',
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
      body: {
        requestId: 'refund-queued',
        invoiceId: 'parent-demo-active-invoice-current',
      },
    });
    const refundBody = BillingSupportAdminRefundResultSchema.parse(await readJson<unknown>(refund.response));
    assert.equal(refund.response.status, 200);
    assert.equal(refundBody.status, 'accepted');
    assert.equal(refundBody.refundState, 'refund-settled');

    const refundedInvoices = await executeRequest({
      path: '/auth/billing/invoices',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const refundedInvoicesBody = await readJson<BillingInvoicesResponse>(refundedInvoices.response);
    assert.equal(refundedInvoices.response.status, 200);
    assert.equal(refundedInvoicesBody.invoices[0]?.paymentState, 'refunded');

    const refundedStatus = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const refundedStatusBody = await readJson<BillingStatusResponse>(refundedStatus.response);
    assert.equal(refundedStatus.response.status, 200);
    assert.equal(refundedStatusBody.accountStatus, 'manual-review');
    assert.equal(refundedStatusBody.subscriptionStatus, 'past-due');
    assert.equal(refundedStatusBody.parentVisibleState, 'manual-review');
    assert.ok(refundedStatusBody.warnings.includes('refund-settled-manual-review'));

    const refundedSnapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const refundedSnapshotBody = await readJson<BillingEntitlementSnapshotResponse>(refundedSnapshot.response);
    assert.equal(refundedSnapshot.response.status, 200);
    assert.equal(refundedSnapshotBody.snapshot.signatureState, 'manual-required');
    assert.equal(refundedSnapshotBody.snapshot.subscriptionStatus, 'past-due');
    assert.equal(refundedSnapshotBody.snapshot.source, 'manual-admin-review');

    const reconciliation = await executeRequest({
      path: '/admin/billing/reconciliation',
      method: 'POST',
      harness,
      headers: {
        'x-ocentra-internal-call': 'true',
        'x-ocentra-internal-secret': 'internal-test-secret',
      },
      body: {
        requestId: 'reconciliation-queued',
      },
    });
    const reconciliationBody = BillingSupportAdminReconciliationSummarySchema.parse(
      await readJson<unknown>(reconciliation.response)
    );
    assert.equal(reconciliation.response.status, 202);
    assert.equal(reconciliationBody.status, 'accepted');
    assert.equal(reconciliationBody.queued, true);
    assert.equal(reconciliationBody.driftFamiliesVisible, 2);
    assert.equal(reconciliationBody.retryBacklogVisible, 1);
    assert.equal(reconciliationBody.deadLetterVisible, 0);

    const reconciliationReplay = await executeRequest({
      path: '/admin/billing/reconciliation',
      method: 'POST',
      harness,
      headers: {
        'x-ocentra-internal-call': 'true',
        'x-ocentra-internal-secret': 'internal-test-secret',
      },
      body: {
        requestId: 'reconciliation-queued',
      },
    });
    const reconciliationReplayBody = BillingSupportAdminReconciliationSummarySchema.parse(
      await readJson<unknown>(reconciliationReplay.response)
    );
    assert.equal(reconciliationReplay.response.status, 202);
    assert.deepEqual(reconciliationReplayBody, reconciliationBody);

    const reconciliationAudit = await executeRequest({
      path: '/admin/billing/audit?q=reconciliation-queued',
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const reconciliationAuditBody = BillingSupportAdminAuditEventsResponseSchema.parse(
      await readJson<unknown>(reconciliationAudit.response)
    );
    const reconciliationEvents = reconciliationAuditBody.results.filter(
      (event: BillingSupportAdminAuditEventsResponse['results'][number]) =>
        event.eventId === 'billing-reconciliation:reconciliation-queued'
    );
    assert.equal(reconciliationAudit.response.status, 200);
    assert.equal(reconciliationEvents.length, 1);
    assert.equal(reconciliationEvents[0]?.eventType, 'billing.reconciliation.accepted');
    assert.equal(reconciliationEvents[0]?.actorRole, 'system');

    assert.deepEqual(harness.queueMessages, [
      {
        action: 'referral-invite',
        requestId: 'invite-queued',
        subject: 'parent:demo-active',
        referralCode: 'REF-FAMILY-CORE',
      },
      {
        action: 'manual-invoice',
        requestId: 'manual-invoice-queued',
        subject: 'parent:support-agent',
        region: 'pakistan',
        actorRole: 'support',
      },
      {
        action: 'admin-refund',
        requestId: 'refund-queued',
        invoiceId: 'parent-demo-active-invoice-current',
        amountCents: 1403,
        actorRole: 'admin',
      },
      {
        action: 'reconciliation',
        requestId: 'reconciliation-queued',
        actorRole: 'internal',
      },
    ]);
  });

  it('keeps partial refund, invoice-not-found rejection, and reconciliation visibility explicit', async () => {
    const harness = createTestHarness();

    const partialRefund = await executeRequest({
      path: '/admin/billing/refunds',
      method: 'POST',
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
      body: {
        requestId: 'refund-partial',
        invoiceId: 'parent-demo-active-invoice-current',
        amountCents: 500,
      },
    });
    const partialRefundBody = BillingSupportAdminRefundResultSchema.parse(
      await readJson<unknown>(partialRefund.response)
    );
    assert.equal(partialRefund.response.status, 200);
    assert.equal(partialRefundBody.status, 'accepted');
    assert.equal(partialRefundBody.refundState, 'refund-requested');
    assert.equal(partialRefundBody.amountCents, 500);

    const partialRefundInvoices = await executeRequest({
      path: '/auth/billing/invoices',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const partialRefundInvoicesBody = await readJson<BillingInvoicesResponse>(partialRefundInvoices.response);
    assert.equal(partialRefundInvoices.response.status, 200);
    assert.equal(partialRefundInvoicesBody.invoices[0]?.paymentState, 'paid');

    const missingInvoiceRefund = await executeRequest({
      path: '/admin/billing/refunds',
      method: 'POST',
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
      body: {
        requestId: 'refund-missing-invoice',
        invoiceId: 'missing-invoice',
      },
    });
    const missingInvoiceRefundBody = BillingSupportAdminRefundResultSchema.parse(
      await readJson<unknown>(missingInvoiceRefund.response)
    );
    assert.equal(missingInvoiceRefund.response.status, 200);
    assert.equal(missingInvoiceRefundBody.status, 'rejected');
    assert.equal(missingInvoiceRefundBody.refundState, 'manual-review-required');
    assert.equal(missingInvoiceRefundBody.amountCents, null);
    assert.equal(missingInvoiceRefundBody.rejectionReason, 'invoice-not-found');

    const reconciliation = await executeRequest({
      path: '/admin/billing/reconciliation',
      method: 'POST',
      harness,
      headers: {
        'x-ocentra-internal-call': 'true',
        'x-ocentra-internal-secret': 'internal-test-secret',
      },
      body: {
        requestId: 'reconciliation-visibility',
      },
    });
    const reconciliationBody = BillingSupportAdminReconciliationSummarySchema.parse(
      await readJson<unknown>(reconciliation.response)
    );
    assert.equal(reconciliation.response.status, 202);
    assert.equal(reconciliationBody.queued, true);
    assert.equal(reconciliationBody.driftFamiliesVisible, 2);
    assert.equal(reconciliationBody.retryBacklogVisible, 1);
    assert.equal(reconciliationBody.deadLetterVisible, 0);
  });

  it('supports manual invoice, admin invoice search, disputes, referrals, refunds, reconciliation, and audit surfaces', async () => {
    const accounts = await executeRequest({
      path: '/admin/billing/accounts?q=review',
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
    });
    const accountsBody = BillingSupportAdminAccountsResponseSchema.parse(await readJson<unknown>(accounts.response));
    assert.equal(accounts.response.status, 200);
    assert.equal(accountsBody.actorRole, 'support');
    assert.equal(
      accountsBody.manualActionsPending,
      accountsBody.results.filter((row: BillingSupportAdminAccountsResponse['results'][number]) => row.manualRequired)
        .length
    );

    const manualInvoice = await executeRequest({
      path: '/auth/billing/manual-invoice',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
      body: {
        requestId: 'manual-invoice-1',
        region: 'pakistan',
      },
    });
    const manualInvoiceBody = await readJson<ManualInvoiceResponse>(manualInvoice.response);
    assert.equal(manualInvoice.response.status, 202);
    assert.equal(manualInvoiceBody.invoiceState, 'manual-support-required');

    const adminInvoices = await executeRequest({
      path: '/admin/billing/invoices?q=INV-',
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
    });
    const adminInvoicesBody = BillingSupportAdminInvoicesResponseSchema.parse(
      await readJson<unknown>(adminInvoices.response)
    );
    assert.equal(adminInvoices.response.status, 200);
    assert.equal(adminInvoicesBody.actorRole, 'support');
    assert.ok(adminInvoicesBody.resultCount >= 1);

    const disputes = await executeRequest({
      path: '/admin/billing/disputes',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const disputesBody = BillingSupportAdminDisputesResponseSchema.parse(await readJson<unknown>(disputes.response));
    assert.equal(disputes.response.status, 200);
    assert.equal(disputesBody.actorRole, 'admin');
    assert.ok(disputesBody.resultCount >= 1);

    const referrals = await executeRequest({
      path: '/admin/billing/referrals',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const referralsBody = BillingSupportAdminReferralsResponseSchema.parse(await readJson<unknown>(referrals.response));
    assert.equal(referrals.response.status, 200);
    assert.equal(referralsBody.actorRole, 'admin');
    assert.ok(referralsBody.resultCount >= 1);

    const refund = await executeRequest({
      path: '/admin/billing/refunds',
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
      body: {
        requestId: 'refund-1',
        invoiceId: 'parent-demo-active-invoice-current',
      },
    });
    const refundBody = BillingSupportAdminRefundResultSchema.parse(await readJson<unknown>(refund.response));
    assert.equal(refund.response.status, 200);
    assert.equal(refundBody.status, 'accepted');

    const reconciliation = await executeRequest({
      path: '/admin/billing/reconciliation',
      method: 'POST',
      headers: {
        'x-ocentra-internal-call': 'true',
        'x-ocentra-internal-secret': 'internal-test-secret',
      },
      body: {
        requestId: 'reconciliation-1',
      },
    });
    const reconciliationBody = BillingSupportAdminReconciliationSummarySchema.parse(
      await readJson<unknown>(reconciliation.response)
    );
    assert.equal(reconciliation.response.status, 202);
    assert.equal(reconciliationBody.status, 'accepted');

    const audit = await executeRequest({
      path: '/admin/billing/audit',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const auditBody = BillingSupportAdminAuditEventsResponseSchema.parse(await readJson<unknown>(audit.response));
    assert.equal(audit.response.status, 200);
    assert.equal(auditBody.actorRole, 'admin');
    assert.ok(auditBody.resultCount >= 1);
  });
});

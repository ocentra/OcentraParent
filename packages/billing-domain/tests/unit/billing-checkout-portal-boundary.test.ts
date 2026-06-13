import { describe, expect, it } from 'vitest';
import {
  BillingCheckoutSessionRequestSchema,
  BillingCheckoutSessionResponseSchema,
  BillingHostedReturnRoute,
  BillingPortalSessionRequestSchema,
  BillingPortalSessionResponseSchema,
  billingHostedReturnRoutePath,
} from '../../src/billing-checkout-portal-boundary';

describe('billing checkout and portal boundary', () => {
  it('accepts authenticated checkout session requests with allowlisted success and cancel routes', () => {
    const request = BillingCheckoutSessionRequestSchema.parse({
      schemaVersion: 'billing-checkout-portal-boundary',
      requestId: 'billing-checkout-request-1',
      kind: 'checkout-session-create',
      actor: { actorId: 'parent-actor-1', role: 'parent' },
      parentAccount: { parentAccountId: 'parent-account-1' },
      family: { familyId: 'family-1' },
      planId: 'family-plus-monthly',
      successRoute: BillingHostedReturnRoute.CheckoutSuccess,
      cancelRoute: BillingHostedReturnRoute.CheckoutCancel,
      abuseGateState: 'passed-turnstile',
    });

    expect(request.actor.role).toBe('parent');
    expect(billingHostedReturnRoutePath(request.successRoute.routeId)).toBe('/family/billing/checkout/success');
    expect(billingHostedReturnRoutePath(request.cancelRoute.routeId)).toBe('/family/billing/checkout/cancel');
  });

  it('rejects checkout session requests from non-interactive system actors', () => {
    expect(
      BillingCheckoutSessionRequestSchema.safeParse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId: 'billing-checkout-request-system',
        kind: 'checkout-session-create',
        actor: { actorId: 'system-actor-1', role: 'system' },
        parentAccount: { parentAccountId: 'parent-account-1' },
        family: { familyId: 'family-1' },
        planId: 'family-plus-monthly',
        successRoute: BillingHostedReturnRoute.CheckoutSuccess,
        cancelRoute: BillingHostedReturnRoute.CheckoutCancel,
        abuseGateState: 'trusted-authenticated-session',
      }).success
    ).toBe(false);
  });

  it('rejects checkout requests that drift off the redirect allowlist', () => {
    expect(
      BillingCheckoutSessionRequestSchema.safeParse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId: 'billing-checkout-request-bad-route',
        kind: 'checkout-session-create',
        actor: { actorId: 'guardian-actor-1', role: 'guardian' },
        parentAccount: { parentAccountId: 'parent-account-1' },
        family: { familyId: 'family-1' },
        planId: 'family-plus-monthly',
        successRoute: {
          routeId: 'family-billing-checkout-success',
          relativePath: '/family/billing/checkout/cancel',
        },
        cancelRoute: BillingHostedReturnRoute.CheckoutCancel,
        abuseGateState: 'passed-turnstile',
      }).success
    ).toBe(false);
  });

  it('rejects portal session requests that try to use checkout redirect routes', () => {
    expect(
      BillingPortalSessionRequestSchema.safeParse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId: 'billing-portal-request-bad-return',
        kind: 'billing-portal-session-create',
        actor: { actorId: 'parent-actor-1', role: 'parent' },
        parentAccount: { parentAccountId: 'parent-account-1' },
        family: { familyId: 'family-1' },
        returnRoute: BillingHostedReturnRoute.CheckoutSuccess,
        abuseGateState: 'trusted-authenticated-session',
      }).success
    ).toBe(false);
  });

  it('accepts Stripe-hosted checkout responses and rejects client-secret leakage', () => {
    expect(
      BillingCheckoutSessionResponseSchema.safeParse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId: 'billing-checkout-request-1',
        kind: 'checkout-session-create',
        status: 'accepted',
        hostedSessionId: 'checkout-session-1',
        hostedUrl: 'https://checkout.stripe.com/c/pay/cs_test_a',
        expiresAt: '2026-06-13T09:00:00.000Z',
        rejectionReason: null,
      }).success
    ).toBe(true);

    expect(
      BillingCheckoutSessionResponseSchema.safeParse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId: 'billing-checkout-request-1',
        kind: 'checkout-session-create',
        status: 'accepted',
        hostedSessionId: 'checkout-session-1',
        hostedUrl: 'https://checkout.stripe.com/c/pay/cs_test_a?client_secret=leak',
        expiresAt: '2026-06-13T09:00:00.000Z',
        rejectionReason: null,
      }).success
    ).toBe(false);
  });

  it('accepts explicit invalid-plan rejections and portal short-lived Stripe sessions', () => {
    expect(
      BillingCheckoutSessionResponseSchema.safeParse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId: 'billing-checkout-request-invalid-plan',
        kind: 'checkout-session-create',
        status: 'rejected',
        hostedSessionId: null,
        hostedUrl: null,
        expiresAt: null,
        rejectionReason: 'invalid-plan',
      }).success
    ).toBe(true);

    expect(
      BillingPortalSessionResponseSchema.safeParse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId: 'billing-portal-request-1',
        kind: 'billing-portal-session-create',
        status: 'accepted',
        hostedSessionId: 'portal-session-1',
        hostedUrl: 'https://billing.stripe.com/p/session/test_123',
        expiresAt: '2026-06-13T09:00:00.000Z',
        rejectionReason: null,
      }).success
    ).toBe(true);
  });
});

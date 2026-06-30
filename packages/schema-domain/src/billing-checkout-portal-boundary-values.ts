import { type Infer, NonEmptyStringSchema, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { BillingPlanIdSchema } from './billing-entitlement-values';

export const BillingCheckoutPortalBoundarySchemaVersionSchema = withParser(
  Schema.Literal('billing-checkout-portal-boundary')
);

export const BillingHostedSessionKindSchema = withParser(
  Schema.Literal('checkout-session-create', 'billing-portal-session-create')
);

export const BillingHostedSessionStatusSchema = withParser(Schema.Literal('accepted', 'rejected'));

export const BillingCheckoutAbuseGateStateSchema = withParser(
  Schema.Literal('passed-turnstile', 'trusted-authenticated-session')
);

export const BillingHostedCheckoutPlanIdSchema = withParser(
  BillingPlanIdSchema.pipe(
    Schema.filter(
      (value) =>
        value === 'family-plus-monthly' ||
        value === 'family-monitor-core' ||
        value === 'family-monitor-plus' ||
        'Expected hosted checkout requests to reject unknown or non-billable plan ids'
    )
  )
);

export const BillingHostedOriginGateStateSchema = withParser(Schema.Literal('same-origin-verified'));

export const BillingHostedCsrfStateSchema = withParser(Schema.Literal('csrf-token-verified'));

export const BillingHostedSurfaceSecretCustodySchema = withParser(Schema.Literal('not-present'));

export const BillingHostedSessionRejectionReasonSchema = withParser(
  Schema.Literal(
    'auth-required',
    'unauthorized-role',
    'invalid-plan',
    'origin-csrf-rejected',
    'redirect-not-allowlisted',
    'abuse-gate-required',
    'provider-unavailable'
  )
);

export const BillingHostedReturnRouteIdSchema = withParser(
  Schema.Literal('family-billing-checkout-success', 'family-billing-checkout-cancel', 'family-billing-portal-return')
);

export const BillingHostedReturnPathSchema = withParser(
  NonEmptyStringSchema.pipe(
    Schema.filter(
      (value) =>
        value === '/family/billing/checkout/success' ||
        value === '/family/billing/checkout/cancel' ||
        value === '/family/billing/manage' ||
        'Expected billing checkout and portal paths to stay inside the allowlisted family billing routes'
    )
  )
);

export const BillingHostedReturnResolutionSchema = withParser(
  Schema.Literal(
    'awaiting-provider-webhook',
    'cancelled-before-provider-confirmation',
    'portal-management-only'
  )
);

export const BillingHostedSessionRequestIdSchema = brandedNonEmptyStringSchema('BillingHostedSessionRequestId');
export const BillingHostedSessionIdSchema = brandedNonEmptyStringSchema('BillingHostedSessionId');

export const BillingHostedCheckoutUrlSchema = withParser(
  NonEmptyStringSchema.pipe(
    Schema.filter(
      (value) =>
        value.startsWith('https://checkout.stripe.com/') ||
        'Expected checkout session redirects to stay on Stripe-hosted checkout'
    ),
    Schema.filter(
      (value) =>
        !value.includes('client_secret=') ||
        'Expected checkout session redirects not to leak client_secret values into the browser boundary'
    )
  )
);

export const BillingHostedPortalUrlSchema = withParser(
  NonEmptyStringSchema.pipe(
    Schema.filter(
      (value) =>
        value.startsWith('https://billing.stripe.com/') ||
        'Expected billing portal redirects to stay on Stripe-hosted billing portal'
    ),
    Schema.filter(
      (value) =>
        !value.includes('client_secret=') ||
        'Expected billing portal redirects not to leak client_secret values into the browser boundary'
    )
  )
);

export type BillingHostedSessionKind = Infer<typeof BillingHostedSessionKindSchema>;
export type BillingHostedSessionStatus = Infer<typeof BillingHostedSessionStatusSchema>;
export type BillingCheckoutAbuseGateState = Infer<typeof BillingCheckoutAbuseGateStateSchema>;
export type BillingHostedCheckoutPlanId = Infer<typeof BillingHostedCheckoutPlanIdSchema>;
export type BillingHostedOriginGateState = Infer<typeof BillingHostedOriginGateStateSchema>;
export type BillingHostedCsrfState = Infer<typeof BillingHostedCsrfStateSchema>;
export type BillingHostedSurfaceSecretCustody = Infer<typeof BillingHostedSurfaceSecretCustodySchema>;
export type BillingHostedSessionRejectionReason = Infer<typeof BillingHostedSessionRejectionReasonSchema>;
export type BillingHostedReturnRouteId = Infer<typeof BillingHostedReturnRouteIdSchema>;
export type BillingHostedReturnResolution = Infer<typeof BillingHostedReturnResolutionSchema>;

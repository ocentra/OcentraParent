/* generated from crates/schema/src/billing_checkout_portal_boundary_values_ts.rs */

export const GeneratedBillingCheckoutPortalBoundarySchemaVersion = 'billing-checkout-portal-boundary' as const;

export const GeneratedBillingHostedSessionKinds = ['checkout-session-create', 'billing-portal-session-create'] as const;

export const GeneratedBillingHostedSessionStatuses = ['accepted', 'rejected'] as const;

export const GeneratedBillingCheckoutAbuseGateStates = ['passed-turnstile', 'trusted-authenticated-session'] as const;

export const GeneratedBillingHostedCheckoutPlanIds = [
  'family-plus-monthly',
  'family-monitor-core',
  'family-monitor-plus',
] as const;

export const GeneratedBillingHostedOriginGateStates = ['same-origin-verified'] as const;

export const GeneratedBillingHostedCsrfStates = ['csrf-token-verified'] as const;

export const GeneratedBillingHostedSurfaceSecretCustodies = ['not-present'] as const;

export const GeneratedBillingHostedSessionRejectionReasons = [
  'auth-required',
  'unauthorized-role',
  'invalid-plan',
  'origin-csrf-rejected',
  'redirect-not-allowlisted',
  'abuse-gate-required',
  'provider-unavailable',
] as const;

export const GeneratedBillingHostedReturnRouteIds = [
  'family-billing-checkout-success',
  'family-billing-checkout-cancel',
  'family-billing-portal-return',
] as const;

export const GeneratedBillingHostedReturnPaths = [
  '/family/billing/checkout/success',
  '/family/billing/checkout/cancel',
  '/family/billing/manage',
] as const;

export const GeneratedBillingHostedReturnResolutions = [
  'awaiting-provider-webhook',
  'cancelled-before-provider-confirmation',
  'portal-management-only',
] as const;

export const GeneratedBillingHostedRouteContractById = {
  'family-billing-checkout-success': {
    relativePath: '/family/billing/checkout/success',
    resolution: 'awaiting-provider-webhook',
  },
  'family-billing-checkout-cancel': {
    relativePath: '/family/billing/checkout/cancel',
    resolution: 'cancelled-before-provider-confirmation',
  },
  'family-billing-portal-return': {
    relativePath: '/family/billing/manage',
    resolution: 'portal-management-only',
  },
} as const;

export const GeneratedBillingHostedReturnRoute = {
  CheckoutSuccess: {
    routeId: 'family-billing-checkout-success',
    relativePath: '/family/billing/checkout/success',
    resolution: 'awaiting-provider-webhook',
  },
  CheckoutCancel: {
    routeId: 'family-billing-checkout-cancel',
    relativePath: '/family/billing/checkout/cancel',
    resolution: 'cancelled-before-provider-confirmation',
  },
  PortalReturn: {
    routeId: 'family-billing-portal-return',
    relativePath: '/family/billing/manage',
    resolution: 'portal-management-only',
  },
} as const;

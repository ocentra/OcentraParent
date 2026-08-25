import type { AuthState, RouteAuditRule } from './auth/model.js';
import {
  BillingCheckoutSessionRequestSchema,
  BillingCheckoutSessionResponseSchema,
  BillingPortalSessionRequestSchema,
  BillingPortalSessionResponseSchema,
  BillingReferralInviteResultSchema,
  BillingSupportAdminAccountsResponseSchema,
  BillingSupportAdminAuditEventsResponseSchema,
  BillingSupportAdminDisputesResponseSchema,
  BillingSupportAdminInvoicesResponseSchema,
  BillingSupportAdminReferralsResponseSchema,
  BillingSupportAdminReconciliationSummarySchema,
  BillingSupportAdminRefundResultSchema,
} from './generated/billing-contracts.js';

export type RouteMethod = 'GET' | 'POST';

/** Route family metadata is derived from and validated against the manifest path. */
export type RouteGroup = 'health' | 'public' | 'session' | 'billing' | 'webhook' | 'admin';
export type RouteBoundary = 'public' | 'session-login' | 'private' | 'support-exception' | 'webhook' | 'internal-queue';
export type RouteWebhookProvider = 'stripe' | 'razorpay' | 'paypal' | 'apple' | 'google';

export type RouteContractCodec = {
  readonly parse: (value: unknown) => unknown;
  readonly safeParse: (value: unknown) => { readonly success: boolean; readonly data?: unknown };
};

export type RouteContractBinding =
  | {
      readonly state: 'bound';
      readonly requestCodec: RouteContractCodec | null;
      readonly responseCodec: RouteContractCodec;
    }
  | {
      readonly state: 'response-bound';
      readonly requestCodec: RouteContractCodec | null;
      readonly responseCodec: RouteContractCodec;
      readonly blocker: string;
    }
  | {
      readonly state: 'manual-required';
      readonly requestCodec: RouteContractCodec | null;
      readonly responseCodec: RouteContractCodec | null;
      readonly blocker: string;
    }
  | {
      readonly state: 'unbound';
      readonly requestCodec: null;
      readonly responseCodec: null;
      readonly blocker: string;
    };

type RouteManifestSeed = {
  readonly path: string;
  readonly method: RouteMethod;
  readonly authState: AuthState;
  readonly handlerKey: string;
  readonly requestModel: string;
  readonly responseModel: string;
  readonly auditRule: RouteAuditRule;
  readonly auditEvent: string;
  readonly proofIdFamily: string;
};

const ROUTE_MANIFEST_SOURCE = [
  {
    path: '/health',
    method: 'GET',
    authState: 'public',
    handlerKey: 'health',
    requestModel: 'none',
    responseModel: 'HealthStatusResponse',
    auditRule: 'public-observability',
    auditEvent: 'cloudflare.health.read',
    proofIdFamily: 'cloudflare-control.worker-entrypoint',
  },
  {
    path: '/auth/session/login',
    method: 'POST',
    authState: 'public',
    handlerKey: 'account-session-login',
    requestModel: 'AccountSessionLoginRequest',
    responseModel: 'AccountSessionResponse',
    auditRule: 'public-observability',
    auditEvent: 'account.session.login',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  },
  {
    path: '/auth/session/refresh',
    method: 'POST',
    authState: 'browser-refresh-required',
    handlerKey: 'account-session-refresh',
    requestModel: 'AccountSessionRefreshRequest',
    responseModel: 'AccountSessionResponse',
    auditRule: 'parent-session-write',
    auditEvent: 'account.session.refresh',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  },
  {
    path: '/auth/session/logout',
    method: 'POST',
    authState: 'browser-refresh-required',
    handlerKey: 'account-session-logout',
    requestModel: 'AccountSessionLogoutRequest',
    responseModel: 'AccountSessionLogoutResponse',
    auditRule: 'parent-session-write',
    auditEvent: 'account.session.logout',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  },
  {
    path: '/auth/session/revoke',
    method: 'POST',
    authState: 'browser-refresh-required',
    handlerKey: 'account-session-revoke',
    requestModel: 'AccountSessionRevokeRequest',
    responseModel: 'AccountSessionLogoutResponse',
    auditRule: 'parent-session-write',
    auditEvent: 'account.session.global-revoke',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  },
  {
    path: '/public/pricing',
    method: 'GET',
    authState: 'public',
    handlerKey: 'pricing-public',
    requestModel: 'BillingPricingPublicRequest',
    responseModel: 'BillingPricingPublicResponse',
    auditRule: 'public-observability',
    auditEvent: 'billing.pricing.read',
    proofIdFamily: 'payment-route.cloudflare-prerequisite',
  },
  {
    path: '/auth/billing/status',
    method: 'GET',
    authState: 'parent-session-required',
    handlerKey: 'billing-status',
    requestModel: 'BillingStatusRequest',
    responseModel: 'BillingStatusResponse',
    auditRule: 'parent-session-read',
    auditEvent: 'billing.status.read',
    proofIdFamily: 'cloudflare-control.portal-to-worker-smoke',
  },
  {
    path: '/auth/billing/checkout',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-checkout',
    requestModel: 'BillingCheckoutRequest',
    responseModel: 'BillingCheckoutResponse',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.checkout.create',
    proofIdFamily: 'payment-route.checkout',
  },
  {
    path: '/auth/billing/portal',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-portal',
    requestModel: 'BillingPortalRequest',
    responseModel: 'BillingPortalResponse',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.portal.open',
    proofIdFamily: 'payment-route.portal',
  },
  {
    path: '/auth/billing/invoices',
    method: 'GET',
    authState: 'parent-session-required',
    handlerKey: 'billing-invoices',
    requestModel: 'BillingInvoicesRequest',
    responseModel: 'BillingInvoicesResponse',
    auditRule: 'parent-session-read',
    auditEvent: 'billing.invoices.read',
    proofIdFamily: 'payment-route.invoices',
  },
  {
    path: '/auth/billing/change-plan',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-change-plan',
    requestModel: 'BillingChangePlanRequest',
    responseModel: 'BillingChangePlanResponse',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.plan.change',
    proofIdFamily: 'payment-route.plan-change',
  },
  {
    path: '/auth/billing/cancel',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-cancel',
    requestModel: 'BillingCancelRequest',
    responseModel: 'BillingCancelResponse',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.subscription.cancel',
    proofIdFamily: 'payment-route.cancellation',
  },
  {
    path: '/auth/billing/referrals',
    method: 'GET',
    authState: 'parent-session-required',
    handlerKey: 'billing-referrals',
    requestModel: 'BillingReferralsRequest',
    responseModel: 'BillingReferralsResponse',
    auditRule: 'parent-session-read',
    auditEvent: 'billing.referrals.read',
    proofIdFamily: 'payment-route.referrals',
  },
  {
    path: '/auth/billing/referral-invite',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-referral-invite',
    requestModel: 'BillingReferralInviteRequest',
    responseModel: 'BillingReferralInviteResponse',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.referrals.invite',
    proofIdFamily: 'payment-route.referrals',
  },
  {
    path: '/auth/billing/entitlement-snapshot',
    method: 'GET',
    authState: 'trusted-parent-device-required',
    handlerKey: 'billing-entitlement-snapshot',
    requestModel: 'BillingEntitlementSnapshotRequest',
    responseModel: 'BillingEntitlementSnapshotResponse',
    auditRule: 'trusted-parent-device-read',
    auditEvent: 'billing.entitlement.snapshot',
    proofIdFamily: 'payment-route.entitlement-snapshot',
  },
  {
    path: '/auth/billing/license-check',
    method: 'POST',
    authState: 'trusted-parent-device-required',
    handlerKey: 'billing-license-check',
    requestModel: 'BillingLicenseCheckRequest',
    responseModel: 'BillingLicenseCheckResponse',
    auditRule: 'trusted-parent-device-write',
    auditEvent: 'billing.license.check',
    proofIdFamily: 'payment-route.license-check',
  },
  {
    path: '/auth/billing/manual-invoice',
    method: 'POST',
    authState: 'support-required',
    handlerKey: 'billing-manual-invoice',
    requestModel: 'BillingManualInvoiceRequest',
    responseModel: 'BillingManualInvoiceResponse',
    auditRule: 'support-write',
    auditEvent: 'billing.manual-invoice.create',
    proofIdFamily: 'payment-route.support-admin',
  },
  {
    path: '/webhooks/stripe',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'stripe-webhook',
    requestModel: 'StripeWebhookRequest',
    responseModel: 'WebhookAckResponse',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.stripe',
    proofIdFamily: 'payment-route.webhook-stripe',
  },
  {
    path: '/webhooks/razorpay',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'razorpay-webhook',
    requestModel: 'RazorpayWebhookRequest',
    responseModel: 'WebhookAckResponse',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.razorpay',
    proofIdFamily: 'payment-route.webhook-razorpay',
  },
  {
    path: '/webhooks/paypal',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'paypal-webhook',
    requestModel: 'PayPalWebhookRequest',
    responseModel: 'WebhookAckResponse',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.paypal',
    proofIdFamily: 'payment-route.webhook-paypal',
  },
  {
    path: '/webhooks/apple',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'apple-webhook',
    requestModel: 'AppleWebhookRequest',
    responseModel: 'WebhookAckResponse',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.apple',
    proofIdFamily: 'payment-route.webhook-apple',
  },
  {
    path: '/webhooks/google',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'google-webhook',
    requestModel: 'GoogleWebhookRequest',
    responseModel: 'WebhookAckResponse',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.google',
    proofIdFamily: 'payment-route.webhook-google',
  },
  {
    path: '/admin/billing/accounts',
    method: 'GET',
    authState: 'support-required',
    handlerKey: 'admin-billing-accounts',
    requestModel: 'AdminBillingAccountsRequest',
    responseModel: 'AdminBillingAccountsResponse',
    auditRule: 'support-read',
    auditEvent: 'billing.admin.accounts.read',
    proofIdFamily: 'payment-route.support-admin',
  },
  {
    path: '/admin/billing/invoices',
    method: 'GET',
    authState: 'support-required',
    handlerKey: 'admin-billing-invoices',
    requestModel: 'AdminBillingInvoicesRequest',
    responseModel: 'AdminBillingInvoicesResponse',
    auditRule: 'support-read',
    auditEvent: 'billing.admin.invoices.read',
    proofIdFamily: 'payment-route.support-admin',
  },
  {
    path: '/admin/billing/refunds',
    method: 'POST',
    authState: 'admin-required',
    handlerKey: 'admin-billing-refunds',
    requestModel: 'AdminBillingRefundRequest',
    responseModel: 'AdminBillingRefundResponse',
    auditRule: 'admin-write',
    auditEvent: 'billing.admin.refund.create',
    proofIdFamily: 'payment-route.refunds',
  },
  {
    path: '/admin/billing/disputes',
    method: 'GET',
    authState: 'admin-required',
    handlerKey: 'admin-billing-disputes',
    requestModel: 'AdminBillingDisputesRequest',
    responseModel: 'AdminBillingDisputesResponse',
    auditRule: 'admin-read',
    auditEvent: 'billing.admin.disputes.read',
    proofIdFamily: 'payment-route.disputes',
  },
  {
    path: '/admin/billing/referrals',
    method: 'GET',
    authState: 'admin-required',
    handlerKey: 'admin-billing-referrals',
    requestModel: 'AdminBillingReferralsRequest',
    responseModel: 'AdminBillingReferralsResponse',
    auditRule: 'admin-read',
    auditEvent: 'billing.admin.referrals.read',
    proofIdFamily: 'payment-route.referrals',
  },
  {
    path: '/admin/billing/reconciliation',
    method: 'POST',
    authState: 'internal-queue-only',
    handlerKey: 'admin-billing-reconciliation',
    requestModel: 'AdminBillingReconciliationRequest',
    responseModel: 'AdminBillingReconciliationResponse',
    auditRule: 'internal-queue',
    auditEvent: 'billing.admin.reconciliation.run',
    proofIdFamily: 'payment-route.reconciliation',
  },
  {
    path: '/admin/billing/audit',
    method: 'GET',
    authState: 'admin-required',
    handlerKey: 'admin-billing-audit',
    requestModel: 'AdminBillingAuditRequest',
    responseModel: 'AdminBillingAuditResponse',
    auditRule: 'admin-read',
    auditEvent: 'billing.admin.audit.read',
    proofIdFamily: 'payment-route.audit',
  },
] as const satisfies readonly RouteManifestSeed[];

type RouteManifestSourceEntry = (typeof ROUTE_MANIFEST_SOURCE)[number];

/** Every route key and contract identifier is derived from this source. */
export type RouteHandlerKey = RouteManifestSourceEntry['handlerKey'];
export type RouteRequestModel = RouteManifestSourceEntry['requestModel'];
export type RouteResponseModel = RouteManifestSourceEntry['responseModel'];

export type RouteManifestEntry = RouteManifestSourceEntry & {
  readonly routeGroup: RouteGroup;
  readonly routeBoundary: RouteBoundary;
  readonly contract: RouteContractBinding;
};

const ROUTE_CONTRACT_BINDINGS = {
  health: {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'health-response-contract-not-generated',
  },
  'account-session-login': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'account-session-contract-owned-by-account-identity',
  },
  'account-session-refresh': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'account-session-contract-owned-by-account-identity',
  },
  'account-session-logout': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'account-session-contract-owned-by-account-identity',
  },
  'account-session-revoke': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'account-session-contract-owned-by-account-identity',
  },
  'pricing-public': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'pricing-contract-owned-by-billing-domain',
  },
  'billing-status': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'billing-status-contract-not-generated',
  },
  'billing-checkout': {
    state: 'manual-required',
    requestCodec: BillingCheckoutSessionRequestSchema,
    responseCodec: BillingCheckoutSessionResponseSchema,
    blocker: 'payment-provider-execution-owner-missing',
  },
  'billing-portal': {
    state: 'manual-required',
    requestCodec: BillingPortalSessionRequestSchema,
    responseCodec: BillingPortalSessionResponseSchema,
    blocker: 'payment-provider-execution-owner-missing',
  },
  'billing-invoices': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'billing-invoice-contract-not-generated',
  },
  'billing-change-plan': {
    state: 'manual-required',
    requestCodec: null,
    responseCodec: null,
    blocker: 'payment-provider-execution-owner-missing',
  },
  'billing-cancel': {
    state: 'manual-required',
    requestCodec: null,
    responseCodec: null,
    blocker: 'payment-provider-execution-owner-missing',
  },
  'billing-referrals': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'billing-referral-route-wrapper-contract-not-generated',
  },
  'billing-referral-invite': {
    state: 'response-bound',
    requestCodec: null,
    responseCodec: BillingReferralInviteResultSchema,
    blocker: 'billing-referral-invite-request-contract-not-generated',
  },
  'billing-entitlement-snapshot': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'billing-entitlement-contract-not-bound-at-worker-route',
  },
  'billing-license-check': {
    state: 'unbound',
    requestCodec: null,
    responseCodec: null,
    blocker: 'billing-license-contract-not-bound-at-worker-route',
  },
  'billing-manual-invoice': {
    state: 'manual-required',
    requestCodec: null,
    responseCodec: null,
    blocker: 'manual-invoice-owner-adapter-missing',
  },
  'stripe-webhook': {
    state: 'manual-required',
    requestCodec: null,
    responseCodec: null,
    blocker: 'stripe-provider-event-contract-owner-missing',
  },
  'razorpay-webhook': {
    state: 'manual-required',
    requestCodec: null,
    responseCodec: null,
    blocker: 'razorpay-provider-event-contract-owner-missing',
  },
  'paypal-webhook': {
    state: 'manual-required',
    requestCodec: null,
    responseCodec: null,
    blocker: 'paypal-provider-event-contract-owner-missing',
  },
  'apple-webhook': {
    state: 'manual-required',
    requestCodec: null,
    responseCodec: null,
    blocker: 'apple-provider-event-contract-owner-missing',
  },
  'google-webhook': {
    state: 'manual-required',
    requestCodec: null,
    responseCodec: null,
    blocker: 'google-provider-event-contract-owner-missing',
  },
  'admin-billing-accounts': {
    state: 'bound',
    requestCodec: null,
    responseCodec: BillingSupportAdminAccountsResponseSchema,
  },
  'admin-billing-invoices': {
    state: 'bound',
    requestCodec: null,
    responseCodec: BillingSupportAdminInvoicesResponseSchema,
  },
  'admin-billing-refunds': {
    state: 'manual-required',
    requestCodec: null,
    responseCodec: BillingSupportAdminRefundResultSchema,
    blocker: 'billing-refund-owner-adapter-missing',
  },
  'admin-billing-disputes': {
    state: 'bound',
    requestCodec: null,
    responseCodec: BillingSupportAdminDisputesResponseSchema,
  },
  'admin-billing-referrals': {
    state: 'bound',
    requestCodec: null,
    responseCodec: BillingSupportAdminReferralsResponseSchema,
  },
  'admin-billing-reconciliation': {
    state: 'response-bound',
    requestCodec: null,
    responseCodec: BillingSupportAdminReconciliationSummarySchema,
    blocker: 'reconciliation-request-contract-not-generated',
  },
  'admin-billing-audit': {
    state: 'bound',
    requestCodec: null,
    responseCodec: BillingSupportAdminAuditEventsResponseSchema,
  },
} satisfies { [K in RouteHandlerKey]: RouteContractBinding };

function routeMetadata(entry: RouteManifestSourceEntry): {
  routeGroup: RouteGroup;
  routeBoundary: RouteBoundary;
} {
  const { path, method, authState, auditRule, handlerKey } = entry;
  const fail = (reason: string): never => {
    throw new Error(`Invalid Cloudflare route manifest entry ${method} ${path}: ${reason}`);
  };
  const requireCondition = (condition: boolean, reason: string): void => {
    if (!condition) {
      fail(reason);
    }
  };

  let routeGroup: RouteGroup;
  let routeBoundary: RouteBoundary;

  if (path === '/health') {
    requireCondition(method === 'GET' && authState === 'public', 'health must be GET/public');
    routeGroup = 'health';
    routeBoundary = 'public';
  } else if (path === '/public/pricing') {
    requireCondition(method === 'GET' && authState === 'public', 'public pricing must be GET/public');
    routeGroup = 'public';
    routeBoundary = 'public';
  } else if (path === '/auth/session/login') {
    requireCondition(method === 'POST' && authState === 'public', 'session login must be POST/public');
    routeGroup = 'session';
    routeBoundary = 'session-login';
  } else if (path.startsWith('/auth/session/')) {
    requireCondition(
      method === 'POST' && !(['public'] as readonly AuthState[]).includes(authState),
      'session mutations must be POST/private'
    );
    routeGroup = 'session';
    routeBoundary = 'private';
  } else if (path.startsWith('/webhooks/')) {
    requireCondition(method === 'POST', 'provider webhooks must be POST');
    requireCondition(
      authState === 'provider-webhook-signature-required' && auditRule === 'provider-webhook',
      'provider webhooks require signature auth and audit'
    );
    routeGroup = 'webhook';
    routeBoundary = 'webhook';
  } else if (path === '/auth/billing/manual-invoice') {
    requireCondition(
      method === 'POST' && authState === 'support-required' && auditRule === 'support-write',
      'manual invoice must remain the support-owned POST exception'
    );
    routeGroup = 'billing';
    routeBoundary = 'support-exception';
  } else if (path.startsWith('/auth/billing/')) {
    requireCondition(
      (authState === 'parent-session-required' || authState === 'trusted-parent-device-required') &&
        (auditRule === 'parent-session-read' ||
          auditRule === 'parent-session-write' ||
          auditRule === 'trusted-parent-device-read' ||
          auditRule === 'trusted-parent-device-write'),
      'billing routes require parent or trusted-device auth with matching audit'
    );
    routeGroup = 'billing';
    routeBoundary = 'private';
  } else if (path === '/admin/billing/reconciliation') {
    requireCondition(
      method === 'POST' && authState === 'internal-queue-only' && auditRule === 'internal-queue',
      'reconciliation must remain an internal queue POST'
    );
    routeGroup = 'admin';
    routeBoundary = 'internal-queue';
  } else if (path.startsWith('/admin/')) {
    requireCondition(
      (['support-required', 'admin-required'] as readonly AuthState[]).includes(authState) &&
        (
          [
            'support-read',
            'support-write',
            'admin-read',
            'admin-write',
          ] as readonly RouteAuditRule[]
        ).includes(auditRule),
      'admin routes require elevated auth with matching audit'
    );
    routeGroup = 'admin';
    routeBoundary = 'private';
  } else {
    return fail('path is outside the public manifest groups');
  }

  const handlerMatchesGroup =
    (routeGroup === 'health' && handlerKey === 'health') ||
    (routeGroup === 'public' && handlerKey === 'pricing-public') ||
    (routeGroup === 'session' && handlerKey.startsWith('account-session-')) ||
    (routeGroup === 'billing' && handlerKey.startsWith('billing-')) ||
    (routeGroup === 'webhook' && handlerKey.endsWith('-webhook')) ||
    (routeGroup === 'admin' && handlerKey.startsWith('admin-billing-'));
  requireCondition(handlerMatchesGroup, 'handler key does not match path-derived route group');

  return { routeGroup, routeBoundary };
}

export const ROUTE_MANIFEST: readonly RouteManifestEntry[] = ROUTE_MANIFEST_SOURCE.map((entry) => ({
  ...entry,
  ...routeMetadata(entry),
  contract: ROUTE_CONTRACT_BINDINGS[entry.handlerKey],
}));

/** The manifest is the only source of the worker handler-key inventory. */
export const ROUTE_HANDLER_KEYS: readonly RouteHandlerKey[] = [
  ...new Set(ROUTE_MANIFEST.map((entry) => entry.handlerKey)),
];

export type RouteHandlerMap<Handler> = {
  [K in RouteHandlerKey]: Handler;
};

/**
 * Provider webhook identity is derived from the manifest handler key.  Auth
 * code must not recover provider ownership by parsing an arbitrary pathname;
 * an unknown or unregistered path has no provider identity.
 */
export function webhookProviderForPath(pathname: string): RouteWebhookProvider | null {
  const route = ROUTE_MANIFEST.find((entry) => entry.path === pathname && entry.routeGroup === 'webhook');
  switch (route?.handlerKey) {
    case 'stripe-webhook':
      return 'stripe';
    case 'razorpay-webhook':
      return 'razorpay';
    case 'paypal-webhook':
      return 'paypal';
    case 'apple-webhook':
      return 'apple';
    case 'google-webhook':
      return 'google';
    default:
      return null;
  }
}

export function routeKey(route: Pick<RouteManifestEntry, 'path' | 'method'>): string {
  return `${route.method} ${route.path}`;
}

export function findRoute(path: string, method: string): RouteManifestEntry | null {
  const normalizedMethod = method.toUpperCase();
  if (normalizedMethod !== 'GET' && normalizedMethod !== 'POST') {
    return null;
  }
  return ROUTE_MANIFEST.find((entry) => entry.path === path && entry.method === normalizedMethod) ?? null;
}

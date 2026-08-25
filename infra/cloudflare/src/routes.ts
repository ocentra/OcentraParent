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
export type RouteClass =
  | 'health'
  | 'public-pricing'
  | 'session-login'
  | 'session-refresh'
  | 'session-logout'
  | 'session-revoke'
  | 'billing-parent-read'
  | 'billing-parent-write'
  | 'billing-trusted-read'
  | 'billing-trusted-write'
  | 'billing-support-write'
  | 'provider-webhook'
  | 'admin-support-read'
  | 'admin-read'
  | 'admin-write'
  | 'internal-queue';

export type RouteContractCodec = {
  readonly parse: (value: unknown) => unknown;
  readonly safeParse: (value: unknown) => { readonly success: boolean; readonly data?: unknown };
};

const GENERATED_ROUTE_CODECS = Object.freeze({
  BillingCheckoutSessionRequest: BillingCheckoutSessionRequestSchema,
  BillingCheckoutSessionResponse: BillingCheckoutSessionResponseSchema,
  BillingPortalSessionRequest: BillingPortalSessionRequestSchema,
  BillingPortalSessionResponse: BillingPortalSessionResponseSchema,
  BillingReferralInviteResult: BillingReferralInviteResultSchema,
  BillingSupportAdminAccountsResponse: BillingSupportAdminAccountsResponseSchema,
  BillingSupportAdminInvoicesResponse: BillingSupportAdminInvoicesResponseSchema,
  BillingSupportAdminRefundResult: BillingSupportAdminRefundResultSchema,
  BillingSupportAdminDisputesResponse: BillingSupportAdminDisputesResponseSchema,
  BillingSupportAdminReferralsResponse: BillingSupportAdminReferralsResponseSchema,
  BillingSupportAdminReconciliationSummary: BillingSupportAdminReconciliationSummarySchema,
  BillingSupportAdminAuditEventsResponse: BillingSupportAdminAuditEventsResponseSchema,
} as const satisfies Record<string, RouteContractCodec>);

type GeneratedRouteCodecRegistry = typeof GENERATED_ROUTE_CODECS;
type GeneratedRouteModel = keyof GeneratedRouteCodecRegistry;

export type RouteContractCodecDescriptor<Model extends GeneratedRouteModel = GeneratedRouteModel> =
  Model extends GeneratedRouteModel
    ? {
        readonly model: Model;
        readonly codec: GeneratedRouteCodecRegistry[Model];
      }
    : never;

type RouteContractCodecDescriptorRegistry = {
  readonly [Model in GeneratedRouteModel]: RouteContractCodecDescriptor<Model>;
};

function bindGeneratedCodecModels(): RouteContractCodecDescriptorRegistry {
  const descriptors = Object.fromEntries(
    Object.entries(GENERATED_ROUTE_CODECS).map(([model, codec]) => [model, Object.freeze({ model, codec })])
  );
  return Object.freeze(descriptors) as RouteContractCodecDescriptorRegistry;
}

const GENERATED_ROUTE_CODEC_DESCRIPTORS = bindGeneratedCodecModels();

export type RouteRequestContract =
  | {
      readonly state: 'bound';
      readonly transport: 'json-body';
      readonly descriptor: RouteContractCodecDescriptor;
    }
  | {
      readonly state: 'none';
      readonly model: 'none';
      readonly transport: 'none';
      readonly codec: null;
    }
  | {
      readonly state: 'unbound';
      readonly model: string;
      readonly transport: 'json-body' | 'query' | 'provider-body';
      readonly codec: null;
      readonly blocker: string;
    };

export type RouteResponseContract =
  | {
      readonly state: 'bound';
      readonly descriptor: RouteContractCodecDescriptor;
    }
  | {
      readonly state: 'unbound';
      readonly model: string;
      readonly codec: null;
      readonly blocker: string;
    };

export type RouteContractBinding = {
  readonly request: RouteRequestContract;
  readonly response: RouteResponseContract;
  readonly execution: { readonly state: 'ready' } | { readonly state: 'manual-required'; readonly blocker: string };
};

export type RouteContractReadiness =
  | {
      readonly ready: true;
    }
  | {
      readonly ready: false;
      readonly side: 'request' | 'response' | 'execution';
      readonly blocker: string;
    };

type RouteIdentityTuple = {
  readonly path: string;
  readonly method: RouteMethod;
  readonly authState: AuthState;
  readonly routeClass: RouteClass;
  readonly webhookProvider?: RouteWebhookProvider;
  readonly auditRule: RouteAuditRule;
  readonly auditEvent: string;
  readonly proofIdFamily: string;
};
const ROUTE_BINDINGS_BY_HANDLER = Object.freeze({
  health: Object.freeze({
    path: '/health',
    method: 'GET',
    authState: 'public',
    routeClass: 'health',
    auditRule: 'public-observability',
    auditEvent: 'cloudflare.health.read',
    proofIdFamily: 'cloudflare-control.worker-entrypoint',
  }),
  'account-session-login': Object.freeze({
    path: '/auth/session/login',
    method: 'POST',
    authState: 'public',
    routeClass: 'session-login',
    auditRule: 'public-observability',
    auditEvent: 'account.session.login',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  }),
  'account-session-refresh': Object.freeze({
    path: '/auth/session/refresh',
    method: 'POST',
    authState: 'browser-refresh-required',
    routeClass: 'session-refresh',
    auditRule: 'parent-session-write',
    auditEvent: 'account.session.refresh',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  }),
  'account-session-logout': Object.freeze({
    path: '/auth/session/logout',
    method: 'POST',
    authState: 'browser-refresh-required',
    routeClass: 'session-logout',
    auditRule: 'parent-session-write',
    auditEvent: 'account.session.logout',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  }),
  'account-session-revoke': Object.freeze({
    path: '/auth/session/revoke',
    method: 'POST',
    authState: 'browser-refresh-required',
    routeClass: 'session-revoke',
    auditRule: 'parent-session-write',
    auditEvent: 'account.session.global-revoke',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  }),
  'pricing-public': Object.freeze({
    path: '/public/pricing',
    method: 'GET',
    authState: 'public',
    routeClass: 'public-pricing',
    auditRule: 'public-observability',
    auditEvent: 'billing.pricing.read',
    proofIdFamily: 'payment-route.cloudflare-prerequisite',
  }),
  'billing-status': Object.freeze({
    path: '/auth/billing/status',
    method: 'GET',
    authState: 'parent-session-required',
    routeClass: 'billing-parent-read',
    auditRule: 'parent-session-read',
    auditEvent: 'billing.status.read',
    proofIdFamily: 'cloudflare-control.portal-to-worker-smoke',
  }),
  'billing-checkout': Object.freeze({
    path: '/auth/billing/checkout',
    method: 'POST',
    authState: 'parent-session-required',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.checkout.create',
    proofIdFamily: 'payment-route.checkout',
  }),
  'billing-portal': Object.freeze({
    path: '/auth/billing/portal',
    method: 'POST',
    authState: 'parent-session-required',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.portal.open',
    proofIdFamily: 'payment-route.portal',
  }),
  'billing-invoices': Object.freeze({
    path: '/auth/billing/invoices',
    method: 'GET',
    authState: 'parent-session-required',
    routeClass: 'billing-parent-read',
    auditRule: 'parent-session-read',
    auditEvent: 'billing.invoices.read',
    proofIdFamily: 'payment-route.invoices',
  }),
  'billing-change-plan': Object.freeze({
    path: '/auth/billing/change-plan',
    method: 'POST',
    authState: 'parent-session-required',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.plan.change',
    proofIdFamily: 'payment-route.plan-change',
  }),
  'billing-cancel': Object.freeze({
    path: '/auth/billing/cancel',
    method: 'POST',
    authState: 'parent-session-required',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.subscription.cancel',
    proofIdFamily: 'payment-route.cancellation',
  }),
  'billing-referrals': Object.freeze({
    path: '/auth/billing/referrals',
    method: 'GET',
    authState: 'parent-session-required',
    routeClass: 'billing-parent-read',
    auditRule: 'parent-session-read',
    auditEvent: 'billing.referrals.read',
    proofIdFamily: 'payment-route.referrals',
  }),
  'billing-referral-invite': Object.freeze({
    path: '/auth/billing/referral-invite',
    method: 'POST',
    authState: 'parent-session-required',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.referrals.invite',
    proofIdFamily: 'payment-route.referrals',
  }),
  'billing-entitlement-snapshot': Object.freeze({
    path: '/auth/billing/entitlement-snapshot',
    method: 'GET',
    authState: 'trusted-parent-device-required',
    routeClass: 'billing-trusted-read',
    auditRule: 'trusted-parent-device-read',
    auditEvent: 'billing.entitlement.snapshot',
    proofIdFamily: 'payment-route.entitlement-snapshot',
  }),
  'billing-license-check': Object.freeze({
    path: '/auth/billing/license-check',
    method: 'POST',
    authState: 'trusted-parent-device-required',
    routeClass: 'billing-trusted-write',
    auditRule: 'trusted-parent-device-write',
    auditEvent: 'billing.license.check',
    proofIdFamily: 'payment-route.license-check',
  }),
  'billing-manual-invoice': Object.freeze({
    path: '/auth/billing/manual-invoice',
    method: 'POST',
    authState: 'support-required',
    routeClass: 'billing-support-write',
    auditRule: 'support-write',
    auditEvent: 'billing.manual-invoice.create',
    proofIdFamily: 'payment-route.support-admin',
  }),
  'stripe-webhook': Object.freeze({
    path: '/webhooks/stripe',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    routeClass: 'provider-webhook',
    webhookProvider: 'stripe',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.stripe',
    proofIdFamily: 'payment-route.webhook-stripe',
  }),
  'razorpay-webhook': Object.freeze({
    path: '/webhooks/razorpay',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    routeClass: 'provider-webhook',
    webhookProvider: 'razorpay',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.razorpay',
    proofIdFamily: 'payment-route.webhook-razorpay',
  }),
  'paypal-webhook': Object.freeze({
    path: '/webhooks/paypal',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    routeClass: 'provider-webhook',
    webhookProvider: 'paypal',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.paypal',
    proofIdFamily: 'payment-route.webhook-paypal',
  }),
  'apple-webhook': Object.freeze({
    path: '/webhooks/apple',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    routeClass: 'provider-webhook',
    webhookProvider: 'apple',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.apple',
    proofIdFamily: 'payment-route.webhook-apple',
  }),
  'google-webhook': Object.freeze({
    path: '/webhooks/google',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    routeClass: 'provider-webhook',
    webhookProvider: 'google',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.google',
    proofIdFamily: 'payment-route.webhook-google',
  }),
  'admin-billing-accounts': Object.freeze({
    path: '/admin/billing/accounts',
    method: 'GET',
    authState: 'support-required',
    routeClass: 'admin-support-read',
    auditRule: 'support-read',
    auditEvent: 'billing.admin.accounts.read',
    proofIdFamily: 'payment-route.support-admin',
  }),
  'admin-billing-invoices': Object.freeze({
    path: '/admin/billing/invoices',
    method: 'GET',
    authState: 'support-required',
    routeClass: 'admin-support-read',
    auditRule: 'support-read',
    auditEvent: 'billing.admin.invoices.read',
    proofIdFamily: 'payment-route.support-admin',
  }),
  'admin-billing-refunds': Object.freeze({
    path: '/admin/billing/refunds',
    method: 'POST',
    authState: 'admin-required',
    routeClass: 'admin-write',
    auditRule: 'admin-write',
    auditEvent: 'billing.admin.refund.create',
    proofIdFamily: 'payment-route.refunds',
  }),
  'admin-billing-disputes': Object.freeze({
    path: '/admin/billing/disputes',
    method: 'GET',
    authState: 'admin-required',
    routeClass: 'admin-read',
    auditRule: 'admin-read',
    auditEvent: 'billing.admin.disputes.read',
    proofIdFamily: 'payment-route.disputes',
  }),
  'admin-billing-referrals': Object.freeze({
    path: '/admin/billing/referrals',
    method: 'GET',
    authState: 'admin-required',
    routeClass: 'admin-read',
    auditRule: 'admin-read',
    auditEvent: 'billing.admin.referrals.read',
    proofIdFamily: 'payment-route.referrals',
  }),
  'admin-billing-reconciliation': Object.freeze({
    path: '/admin/billing/reconciliation',
    method: 'POST',
    authState: 'internal-queue-only',
    routeClass: 'internal-queue',
    auditRule: 'internal-queue',
    auditEvent: 'billing.admin.reconciliation.run',
    proofIdFamily: 'payment-route.reconciliation',
  }),
  'admin-billing-audit': Object.freeze({
    path: '/admin/billing/audit',
    method: 'GET',
    authState: 'admin-required',
    routeClass: 'admin-read',
    auditRule: 'admin-read',
    auditEvent: 'billing.admin.audit.read',
    proofIdFamily: 'payment-route.audit',
  }),
} as const satisfies Record<string, RouteIdentityTuple>);

export type RouteHandlerKey = keyof typeof ROUTE_BINDINGS_BY_HANDLER;
type RouteIdentityBinding = (typeof ROUTE_BINDINGS_BY_HANDLER)[RouteHandlerKey];

/** Every executable handler is bound to one immutable manifest tuple. */
function noRequest(): Extract<RouteRequestContract, { readonly state: 'none' }> {
  return { state: 'none', model: 'none', transport: 'none', codec: null };
}

function boundRequest<const Descriptor extends RouteContractCodecDescriptor>(
  descriptor: Descriptor
): Omit<Extract<RouteRequestContract, { readonly state: 'bound' }>, 'descriptor'> & {
  readonly descriptor: Descriptor;
} {
  return { state: 'bound', transport: 'json-body', descriptor };
}

function unboundRequest<const Model extends string>(
  model: Model,
  transport: 'json-body' | 'query' | 'provider-body',
  blocker: string
): Omit<Extract<RouteRequestContract, { readonly state: 'unbound' }>, 'model'> & { readonly model: Model } {
  return { state: 'unbound', model, transport, codec: null, blocker };
}

function boundResponse<const Descriptor extends RouteContractCodecDescriptor>(
  descriptor: Descriptor
): Omit<Extract<RouteResponseContract, { readonly state: 'bound' }>, 'descriptor'> & {
  readonly descriptor: Descriptor;
} {
  return { state: 'bound', descriptor };
}

function unboundResponse<const Model extends string>(
  model: Model,
  blocker: string
): Omit<Extract<RouteResponseContract, { readonly state: 'unbound' }>, 'model'> & { readonly model: Model } {
  return { state: 'unbound', model, codec: null, blocker };
}

const EXECUTION_READY = { state: 'ready' } as const;

function manualExecution(
  blocker: string
): Extract<RouteContractBinding['execution'], { readonly state: 'manual-required' }> {
  return { state: 'manual-required', blocker };
}

const ROUTE_CONTRACT_BINDINGS = {
  health: {
    request: noRequest(),
    response: unboundResponse('HealthStatusResponse', 'health-response-contract-not-generated'),
    execution: EXECUTION_READY,
  },
  'account-session-login': {
    request: unboundRequest(
      'AccountSessionLoginRequest',
      'json-body',
      'account-session-request-contract-owned-by-account-identity'
    ),
    response: unboundResponse('AccountSessionResponse', 'account-session-response-contract-owned-by-account-identity'),
    execution: EXECUTION_READY,
  },
  'account-session-refresh': {
    request: unboundRequest(
      'AccountSessionRefreshRequest',
      'json-body',
      'account-session-request-contract-owned-by-account-identity'
    ),
    response: unboundResponse('AccountSessionResponse', 'account-session-response-contract-owned-by-account-identity'),
    execution: EXECUTION_READY,
  },
  'account-session-logout': {
    request: unboundRequest(
      'AccountSessionLogoutRequest',
      'json-body',
      'account-session-request-contract-owned-by-account-identity'
    ),
    response: unboundResponse(
      'AccountSessionLogoutResponse',
      'account-session-response-contract-owned-by-account-identity'
    ),
    execution: EXECUTION_READY,
  },
  'account-session-revoke': {
    request: unboundRequest(
      'AccountSessionRevokeRequest',
      'json-body',
      'account-session-request-contract-owned-by-account-identity'
    ),
    response: unboundResponse(
      'AccountSessionLogoutResponse',
      'account-session-response-contract-owned-by-account-identity'
    ),
    execution: EXECUTION_READY,
  },
  'pricing-public': {
    request: noRequest(),
    response: unboundResponse('BillingPricingPublicResponse', 'pricing-contract-owned-by-billing-domain'),
    execution: EXECUTION_READY,
  },
  'billing-status': {
    request: noRequest(),
    response: unboundResponse('BillingStatusResponse', 'billing-status-contract-not-generated'),
    execution: EXECUTION_READY,
  },
  'billing-checkout': {
    request: boundRequest(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingCheckoutSessionRequest),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingCheckoutSessionResponse),
    execution: manualExecution('payment-provider-execution-owner-missing'),
  },
  'billing-portal': {
    request: boundRequest(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingPortalSessionRequest),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingPortalSessionResponse),
    execution: manualExecution('payment-provider-execution-owner-missing'),
  },
  'billing-invoices': {
    request: noRequest(),
    response: unboundResponse('BillingInvoicesResponse', 'billing-invoice-contract-not-generated'),
    execution: EXECUTION_READY,
  },
  'billing-change-plan': {
    request: unboundRequest('BillingChangePlanRequest', 'json-body', 'billing-change-plan-request-not-generated'),
    response: unboundResponse('BillingChangePlanResponse', 'billing-change-plan-response-not-generated'),
    execution: manualExecution('payment-provider-execution-owner-missing'),
  },
  'billing-cancel': {
    request: unboundRequest('BillingCancelRequest', 'json-body', 'billing-cancel-request-not-generated'),
    response: unboundResponse('BillingCancelResponse', 'billing-cancel-response-not-generated'),
    execution: manualExecution('payment-provider-execution-owner-missing'),
  },
  'billing-referrals': {
    request: noRequest(),
    response: unboundResponse('BillingReferralsResponse', 'billing-referral-route-wrapper-contract-not-generated'),
    execution: EXECUTION_READY,
  },
  'billing-referral-invite': {
    request: unboundRequest(
      'BillingReferralInviteRequest',
      'json-body',
      'billing-referral-invite-request-contract-not-generated'
    ),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingReferralInviteResult),
    execution: EXECUTION_READY,
  },
  'billing-entitlement-snapshot': {
    request: noRequest(),
    response: unboundResponse(
      'BillingEntitlementSnapshotResponse',
      'billing-entitlement-contract-not-bound-at-worker-route'
    ),
    execution: EXECUTION_READY,
  },
  'billing-license-check': {
    request: unboundRequest(
      'BillingLicenseCheckRequest',
      'json-body',
      'billing-license-request-contract-not-bound-at-worker-route'
    ),
    response: unboundResponse(
      'BillingLicenseCheckResponse',
      'billing-license-response-contract-not-bound-at-worker-route'
    ),
    execution: EXECUTION_READY,
  },
  'billing-manual-invoice': {
    request: unboundRequest('BillingManualInvoiceRequest', 'json-body', 'manual-invoice-request-not-generated'),
    response: unboundResponse('BillingManualInvoiceResponse', 'manual-invoice-response-not-generated'),
    execution: manualExecution('manual-invoice-owner-adapter-missing'),
  },
  'stripe-webhook': {
    request: unboundRequest('StripeWebhookRequest', 'provider-body', 'stripe-provider-event-contract-owner-missing'),
    response: unboundResponse('WebhookAckResponse', 'provider-webhook-ack-contract-not-generated'),
    execution: manualExecution('stripe-provider-event-contract-owner-missing'),
  },
  'razorpay-webhook': {
    request: unboundRequest(
      'RazorpayWebhookRequest',
      'provider-body',
      'razorpay-provider-event-contract-owner-missing'
    ),
    response: unboundResponse('WebhookAckResponse', 'provider-webhook-ack-contract-not-generated'),
    execution: manualExecution('razorpay-provider-event-contract-owner-missing'),
  },
  'paypal-webhook': {
    request: unboundRequest('PayPalWebhookRequest', 'provider-body', 'paypal-provider-event-contract-owner-missing'),
    response: unboundResponse('WebhookAckResponse', 'provider-webhook-ack-contract-not-generated'),
    execution: manualExecution('paypal-provider-event-contract-owner-missing'),
  },
  'apple-webhook': {
    request: unboundRequest('AppleWebhookRequest', 'provider-body', 'apple-provider-event-contract-owner-missing'),
    response: unboundResponse('WebhookAckResponse', 'provider-webhook-ack-contract-not-generated'),
    execution: manualExecution('apple-provider-event-contract-owner-missing'),
  },
  'google-webhook': {
    request: unboundRequest('GoogleWebhookRequest', 'provider-body', 'google-provider-event-contract-owner-missing'),
    response: unboundResponse('WebhookAckResponse', 'provider-webhook-ack-contract-not-generated'),
    execution: manualExecution('google-provider-event-contract-owner-missing'),
  },
  'admin-billing-accounts': {
    request: unboundRequest('AdminBillingAccountsRequest', 'query', 'admin-accounts-query-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingSupportAdminAccountsResponse),
    execution: EXECUTION_READY,
  },
  'admin-billing-invoices': {
    request: unboundRequest('AdminBillingInvoicesRequest', 'query', 'admin-invoices-query-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingSupportAdminInvoicesResponse),
    execution: EXECUTION_READY,
  },
  'admin-billing-refunds': {
    request: unboundRequest('AdminBillingRefundRequest', 'json-body', 'admin-refund-request-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingSupportAdminRefundResult),
    execution: manualExecution('billing-refund-owner-adapter-missing'),
  },
  'admin-billing-disputes': {
    request: unboundRequest('AdminBillingDisputesRequest', 'query', 'admin-disputes-query-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingSupportAdminDisputesResponse),
    execution: EXECUTION_READY,
  },
  'admin-billing-referrals': {
    request: unboundRequest('AdminBillingReferralsRequest', 'query', 'admin-referrals-query-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingSupportAdminReferralsResponse),
    execution: EXECUTION_READY,
  },
  'admin-billing-reconciliation': {
    request: unboundRequest(
      'AdminBillingReconciliationRequest',
      'json-body',
      'reconciliation-request-contract-not-generated'
    ),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingSupportAdminReconciliationSummary),
    execution: EXECUTION_READY,
  },
  'admin-billing-audit': {
    request: unboundRequest('AdminBillingAuditRequest', 'query', 'admin-audit-query-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.BillingSupportAdminAuditEventsResponse),
    execution: EXECUTION_READY,
  },
} satisfies { [K in RouteHandlerKey]: RouteContractBinding };

type ContractModel<Contract> = Contract extends {
  readonly descriptor: RouteContractCodecDescriptor<infer Model>;
}
  ? Model
  : Contract extends { readonly model: infer Model extends string }
    ? Model
    : never;

function contractModel<const Contract extends RouteRequestContract | RouteResponseContract>(
  contract: Contract
): ContractModel<Contract> {
  return (contract.state === 'bound' ? contract.descriptor.model : contract.model) as ContractModel<Contract>;
}

export type RouteManifestEntry = {
  readonly path: RouteIdentityBinding['path'];
  readonly method: RouteMethod;
  readonly authState: AuthState;
  readonly handlerKey: RouteHandlerKey;
  readonly routeClass: RouteClass;
  readonly webhookProvider: RouteWebhookProvider | null;
  readonly auditRule: RouteAuditRule;
  readonly auditEvent: string;
  readonly proofIdFamily: string;
  readonly routeGroup: RouteGroup;
  readonly routeBoundary: RouteBoundary;
  readonly contract: RouteContractBinding;
};

const ROUTE_CLASS_METADATA = Object.freeze({
  health: Object.freeze({ routeGroup: 'health', routeBoundary: 'public' }),
  'public-pricing': Object.freeze({ routeGroup: 'public', routeBoundary: 'public' }),
  'session-login': Object.freeze({ routeGroup: 'session', routeBoundary: 'session-login' }),
  'session-refresh': Object.freeze({ routeGroup: 'session', routeBoundary: 'private' }),
  'session-logout': Object.freeze({ routeGroup: 'session', routeBoundary: 'private' }),
  'session-revoke': Object.freeze({ routeGroup: 'session', routeBoundary: 'private' }),
  'billing-parent-read': Object.freeze({ routeGroup: 'billing', routeBoundary: 'private' }),
  'billing-parent-write': Object.freeze({ routeGroup: 'billing', routeBoundary: 'private' }),
  'billing-trusted-read': Object.freeze({ routeGroup: 'billing', routeBoundary: 'private' }),
  'billing-trusted-write': Object.freeze({ routeGroup: 'billing', routeBoundary: 'private' }),
  'billing-support-write': Object.freeze({ routeGroup: 'billing', routeBoundary: 'support-exception' }),
  'provider-webhook': Object.freeze({ routeGroup: 'webhook', routeBoundary: 'webhook' }),
  'admin-support-read': Object.freeze({ routeGroup: 'admin', routeBoundary: 'private' }),
  'admin-read': Object.freeze({ routeGroup: 'admin', routeBoundary: 'private' }),
  'admin-write': Object.freeze({ routeGroup: 'admin', routeBoundary: 'private' }),
  'internal-queue': Object.freeze({ routeGroup: 'admin', routeBoundary: 'internal-queue' }),
} as const satisfies Record<RouteClass, { readonly routeGroup: RouteGroup; readonly routeBoundary: RouteBoundary }>);

function routeMetadata(entry: RouteIdentityBinding): {
  readonly routeGroup: RouteGroup;
  readonly routeBoundary: RouteBoundary;
} {
  return ROUTE_CLASS_METADATA[entry.routeClass];
}

function buildRouteManifest(): readonly RouteManifestEntry[] {
  const routeKeys = new Set<string>();
  const handlerKeys = new Set<RouteHandlerKey>();
  const webhookProviders = new Set<RouteWebhookProvider>();
  const webhookPaths = new Set<string>();
  const webhookHandlers = new Set<RouteHandlerKey>();
  const entries: RouteManifestEntry[] = [];

  const manifestHandlerKeys = Object.keys(ROUTE_BINDINGS_BY_HANDLER) as RouteHandlerKey[];
  for (const handlerKey of manifestHandlerKeys) {
    const source = ROUTE_BINDINGS_BY_HANDLER[handlerKey];
    if (source.proofIdFamily.trim().length === 0) {
      throw new Error(`Cloudflare route proof family is empty: ${handlerKey}`);
    }
    const key = `${source.method} ${source.path}`;
    if (routeKeys.has(key)) {
      throw new Error(`Duplicate Cloudflare route identity: ${key}`);
    }
    if (handlerKeys.has(handlerKey)) {
      throw new Error(`Duplicate Cloudflare handler identity: ${handlerKey}`);
    }
    routeKeys.add(key);
    handlerKeys.add(handlerKey);

    const webhookProvider = 'webhookProvider' in source ? source.webhookProvider : null;
    if (webhookProvider !== null) {
      if (webhookProviders.has(webhookProvider) || webhookPaths.has(source.path) || webhookHandlers.has(handlerKey)) {
        throw new Error(`Duplicate provider webhook identity: ${webhookProvider}/${source.path}/${handlerKey}`);
      }
      webhookProviders.add(webhookProvider);
      webhookPaths.add(source.path);
      webhookHandlers.add(handlerKey);
    }

    const contract = ROUTE_CONTRACT_BINDINGS[handlerKey];
    entries.push(
      Object.freeze({
        path: source.path,
        method: source.method,
        authState: source.authState,
        handlerKey,
        routeClass: source.routeClass,
        webhookProvider,
        auditRule: source.auditRule,
        auditEvent: source.auditEvent,
        proofIdFamily: source.proofIdFamily,
        ...routeMetadata(source),
        contract,
      })
    );
  }

  const contractHandlerKeys = Object.keys(ROUTE_CONTRACT_BINDINGS) as RouteHandlerKey[];
  const missingHandler = contractHandlerKeys.find((handlerKey) => !handlerKeys.has(handlerKey));
  if (missingHandler !== undefined || handlerKeys.size !== contractHandlerKeys.length) {
    throw new Error(`Cloudflare route manifest is not exhaustive: ${missingHandler ?? 'unexpected-handler-count'}`);
  }

  return Object.freeze(entries);
}

export const ROUTE_MANIFEST = buildRouteManifest();

/** The manifest is the only source of the worker handler-key inventory. */
export const ROUTE_HANDLER_KEYS: readonly RouteHandlerKey[] = ROUTE_MANIFEST.map((entry) => entry.handlerKey);

export type RouteHandlerMap<Handler> = {
  [K in RouteHandlerKey]: Handler;
};

/**
 * Provider identity is a manifest-owned tuple. Auth code never derives it
 * from an arbitrary pathname or a separately maintained handler map.
 */
export function webhookProviderForPath(pathname: string): RouteWebhookProvider | null {
  const route = ROUTE_MANIFEST.find((entry) => entry.path === pathname && entry.routeGroup === 'webhook');
  return route?.webhookProvider ?? null;
}

export function routeContractReadiness(route: RouteManifestEntry): RouteContractReadiness {
  if (route.contract.request.state === 'unbound') {
    return { ready: false, side: 'request', blocker: route.contract.request.blocker };
  }
  if (route.contract.response.state === 'unbound') {
    return { ready: false, side: 'response', blocker: route.contract.response.blocker };
  }
  if (route.contract.execution.state === 'manual-required') {
    return { ready: false, side: 'execution', blocker: route.contract.execution.blocker };
  }
  return { ready: true };
}

export function routeRequestModel(route: RouteManifestEntry): string {
  return contractModel(route.contract.request);
}

export function routeResponseModel(route: RouteManifestEntry): string {
  return contractModel(route.contract.response);
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

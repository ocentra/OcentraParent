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

export type RouteContractCodecDescriptor<Model extends string = string> = {
  readonly model: Model;
  readonly codec: RouteContractCodec;
};

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

type RouteManifestSeed = {
  readonly path: string;
  readonly method: RouteMethod;
  readonly authState: AuthState;
  readonly handlerKey: string;
  readonly routeClass: RouteClass;
  readonly webhookProvider?: RouteWebhookProvider;
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
    routeClass: 'health',
    auditRule: 'public-observability',
    auditEvent: 'cloudflare.health.read',
    proofIdFamily: 'cloudflare-control.worker-entrypoint',
  },
  {
    path: '/auth/session/login',
    method: 'POST',
    authState: 'public',
    handlerKey: 'account-session-login',
    routeClass: 'session-login',
    auditRule: 'public-observability',
    auditEvent: 'account.session.login',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  },
  {
    path: '/auth/session/refresh',
    method: 'POST',
    authState: 'browser-refresh-required',
    handlerKey: 'account-session-refresh',
    routeClass: 'session-refresh',
    auditRule: 'parent-session-write',
    auditEvent: 'account.session.refresh',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  },
  {
    path: '/auth/session/logout',
    method: 'POST',
    authState: 'browser-refresh-required',
    handlerKey: 'account-session-logout',
    routeClass: 'session-logout',
    auditRule: 'parent-session-write',
    auditEvent: 'account.session.logout',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  },
  {
    path: '/auth/session/revoke',
    method: 'POST',
    authState: 'browser-refresh-required',
    handlerKey: 'account-session-revoke',
    routeClass: 'session-revoke',
    auditRule: 'parent-session-write',
    auditEvent: 'account.session.global-revoke',
    proofIdFamily: 'account-identity-family-plan.wp03-session-runtime',
  },
  {
    path: '/public/pricing',
    method: 'GET',
    authState: 'public',
    handlerKey: 'pricing-public',
    routeClass: 'public-pricing',
    auditRule: 'public-observability',
    auditEvent: 'billing.pricing.read',
    proofIdFamily: 'payment-route.cloudflare-prerequisite',
  },
  {
    path: '/auth/billing/status',
    method: 'GET',
    authState: 'parent-session-required',
    handlerKey: 'billing-status',
    routeClass: 'billing-parent-read',
    auditRule: 'parent-session-read',
    auditEvent: 'billing.status.read',
    proofIdFamily: 'cloudflare-control.portal-to-worker-smoke',
  },
  {
    path: '/auth/billing/checkout',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-checkout',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.checkout.create',
    proofIdFamily: 'payment-route.checkout',
  },
  {
    path: '/auth/billing/portal',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-portal',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.portal.open',
    proofIdFamily: 'payment-route.portal',
  },
  {
    path: '/auth/billing/invoices',
    method: 'GET',
    authState: 'parent-session-required',
    handlerKey: 'billing-invoices',
    routeClass: 'billing-parent-read',
    auditRule: 'parent-session-read',
    auditEvent: 'billing.invoices.read',
    proofIdFamily: 'payment-route.invoices',
  },
  {
    path: '/auth/billing/change-plan',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-change-plan',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.plan.change',
    proofIdFamily: 'payment-route.plan-change',
  },
  {
    path: '/auth/billing/cancel',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-cancel',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.subscription.cancel',
    proofIdFamily: 'payment-route.cancellation',
  },
  {
    path: '/auth/billing/referrals',
    method: 'GET',
    authState: 'parent-session-required',
    handlerKey: 'billing-referrals',
    routeClass: 'billing-parent-read',
    auditRule: 'parent-session-read',
    auditEvent: 'billing.referrals.read',
    proofIdFamily: 'payment-route.referrals',
  },
  {
    path: '/auth/billing/referral-invite',
    method: 'POST',
    authState: 'parent-session-required',
    handlerKey: 'billing-referral-invite',
    routeClass: 'billing-parent-write',
    auditRule: 'parent-session-write',
    auditEvent: 'billing.referrals.invite',
    proofIdFamily: 'payment-route.referrals',
  },
  {
    path: '/auth/billing/entitlement-snapshot',
    method: 'GET',
    authState: 'trusted-parent-device-required',
    handlerKey: 'billing-entitlement-snapshot',
    routeClass: 'billing-trusted-read',
    auditRule: 'trusted-parent-device-read',
    auditEvent: 'billing.entitlement.snapshot',
    proofIdFamily: 'payment-route.entitlement-snapshot',
  },
  {
    path: '/auth/billing/license-check',
    method: 'POST',
    authState: 'trusted-parent-device-required',
    handlerKey: 'billing-license-check',
    routeClass: 'billing-trusted-write',
    auditRule: 'trusted-parent-device-write',
    auditEvent: 'billing.license.check',
    proofIdFamily: 'payment-route.license-check',
  },
  {
    path: '/auth/billing/manual-invoice',
    method: 'POST',
    authState: 'support-required',
    handlerKey: 'billing-manual-invoice',
    routeClass: 'billing-support-write',
    auditRule: 'support-write',
    auditEvent: 'billing.manual-invoice.create',
    proofIdFamily: 'payment-route.support-admin',
  },
  {
    path: '/webhooks/stripe',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'stripe-webhook',
    routeClass: 'provider-webhook',
    webhookProvider: 'stripe',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.stripe',
    proofIdFamily: 'payment-route.webhook-stripe',
  },
  {
    path: '/webhooks/razorpay',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'razorpay-webhook',
    routeClass: 'provider-webhook',
    webhookProvider: 'razorpay',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.razorpay',
    proofIdFamily: 'payment-route.webhook-razorpay',
  },
  {
    path: '/webhooks/paypal',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'paypal-webhook',
    routeClass: 'provider-webhook',
    webhookProvider: 'paypal',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.paypal',
    proofIdFamily: 'payment-route.webhook-paypal',
  },
  {
    path: '/webhooks/apple',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'apple-webhook',
    routeClass: 'provider-webhook',
    webhookProvider: 'apple',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.apple',
    proofIdFamily: 'payment-route.webhook-apple',
  },
  {
    path: '/webhooks/google',
    method: 'POST',
    authState: 'provider-webhook-signature-required',
    handlerKey: 'google-webhook',
    routeClass: 'provider-webhook',
    webhookProvider: 'google',
    auditRule: 'provider-webhook',
    auditEvent: 'billing.webhook.google',
    proofIdFamily: 'payment-route.webhook-google',
  },
  {
    path: '/admin/billing/accounts',
    method: 'GET',
    authState: 'support-required',
    handlerKey: 'admin-billing-accounts',
    routeClass: 'admin-support-read',
    auditRule: 'support-read',
    auditEvent: 'billing.admin.accounts.read',
    proofIdFamily: 'payment-route.support-admin',
  },
  {
    path: '/admin/billing/invoices',
    method: 'GET',
    authState: 'support-required',
    handlerKey: 'admin-billing-invoices',
    routeClass: 'admin-support-read',
    auditRule: 'support-read',
    auditEvent: 'billing.admin.invoices.read',
    proofIdFamily: 'payment-route.support-admin',
  },
  {
    path: '/admin/billing/refunds',
    method: 'POST',
    authState: 'admin-required',
    handlerKey: 'admin-billing-refunds',
    routeClass: 'admin-write',
    auditRule: 'admin-write',
    auditEvent: 'billing.admin.refund.create',
    proofIdFamily: 'payment-route.refunds',
  },
  {
    path: '/admin/billing/disputes',
    method: 'GET',
    authState: 'admin-required',
    handlerKey: 'admin-billing-disputes',
    routeClass: 'admin-read',
    auditRule: 'admin-read',
    auditEvent: 'billing.admin.disputes.read',
    proofIdFamily: 'payment-route.disputes',
  },
  {
    path: '/admin/billing/referrals',
    method: 'GET',
    authState: 'admin-required',
    handlerKey: 'admin-billing-referrals',
    routeClass: 'admin-read',
    auditRule: 'admin-read',
    auditEvent: 'billing.admin.referrals.read',
    proofIdFamily: 'payment-route.referrals',
  },
  {
    path: '/admin/billing/reconciliation',
    method: 'POST',
    authState: 'internal-queue-only',
    handlerKey: 'admin-billing-reconciliation',
    routeClass: 'internal-queue',
    auditRule: 'internal-queue',
    auditEvent: 'billing.admin.reconciliation.run',
    proofIdFamily: 'payment-route.reconciliation',
  },
  {
    path: '/admin/billing/audit',
    method: 'GET',
    authState: 'admin-required',
    handlerKey: 'admin-billing-audit',
    routeClass: 'admin-read',
    auditRule: 'admin-read',
    auditEvent: 'billing.admin.audit.read',
    proofIdFamily: 'payment-route.audit',
  },
] as const satisfies readonly RouteManifestSeed[];

type RouteManifestSourceEntry = (typeof ROUTE_MANIFEST_SOURCE)[number];

/** Every handler key is derived from the single manifest source. */
export type RouteHandlerKey = RouteManifestSourceEntry['handlerKey'];

function noRequest(): Extract<RouteRequestContract, { readonly state: 'none' }> {
  return { state: 'none', model: 'none', transport: 'none', codec: null };
}

function codecDescriptor<const Model extends string>(
  model: Model,
  codec: RouteContractCodec
): RouteContractCodecDescriptor<Model> {
  return Object.freeze({ model, codec });
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

const GENERATED_ROUTE_CODEC_DESCRIPTORS = {
  billingCheckoutRequest: codecDescriptor('BillingCheckoutSessionRequest', BillingCheckoutSessionRequestSchema),
  billingCheckoutResponse: codecDescriptor('BillingCheckoutSessionResponse', BillingCheckoutSessionResponseSchema),
  billingPortalRequest: codecDescriptor('BillingPortalSessionRequest', BillingPortalSessionRequestSchema),
  billingPortalResponse: codecDescriptor('BillingPortalSessionResponse', BillingPortalSessionResponseSchema),
  billingReferralInviteResult: codecDescriptor('BillingReferralInviteResult', BillingReferralInviteResultSchema),
  adminAccountsResponse: codecDescriptor(
    'BillingSupportAdminAccountsResponse',
    BillingSupportAdminAccountsResponseSchema
  ),
  adminInvoicesResponse: codecDescriptor(
    'BillingSupportAdminInvoicesResponse',
    BillingSupportAdminInvoicesResponseSchema
  ),
  adminRefundResult: codecDescriptor('BillingSupportAdminRefundResult', BillingSupportAdminRefundResultSchema),
  adminDisputesResponse: codecDescriptor(
    'BillingSupportAdminDisputesResponse',
    BillingSupportAdminDisputesResponseSchema
  ),
  adminReferralsResponse: codecDescriptor(
    'BillingSupportAdminReferralsResponse',
    BillingSupportAdminReferralsResponseSchema
  ),
  adminReconciliationSummary: codecDescriptor(
    'BillingSupportAdminReconciliationSummary',
    BillingSupportAdminReconciliationSummarySchema
  ),
  adminAuditEventsResponse: codecDescriptor(
    'BillingSupportAdminAuditEventsResponse',
    BillingSupportAdminAuditEventsResponseSchema
  ),
} as const;

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
    request: boundRequest(GENERATED_ROUTE_CODEC_DESCRIPTORS.billingCheckoutRequest),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.billingCheckoutResponse),
    execution: manualExecution('payment-provider-execution-owner-missing'),
  },
  'billing-portal': {
    request: boundRequest(GENERATED_ROUTE_CODEC_DESCRIPTORS.billingPortalRequest),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.billingPortalResponse),
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
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.billingReferralInviteResult),
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
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.adminAccountsResponse),
    execution: EXECUTION_READY,
  },
  'admin-billing-invoices': {
    request: unboundRequest('AdminBillingInvoicesRequest', 'query', 'admin-invoices-query-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.adminInvoicesResponse),
    execution: EXECUTION_READY,
  },
  'admin-billing-refunds': {
    request: unboundRequest('AdminBillingRefundRequest', 'json-body', 'admin-refund-request-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.adminRefundResult),
    execution: manualExecution('billing-refund-owner-adapter-missing'),
  },
  'admin-billing-disputes': {
    request: unboundRequest('AdminBillingDisputesRequest', 'query', 'admin-disputes-query-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.adminDisputesResponse),
    execution: EXECUTION_READY,
  },
  'admin-billing-referrals': {
    request: unboundRequest('AdminBillingReferralsRequest', 'query', 'admin-referrals-query-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.adminReferralsResponse),
    execution: EXECUTION_READY,
  },
  'admin-billing-reconciliation': {
    request: unboundRequest(
      'AdminBillingReconciliationRequest',
      'json-body',
      'reconciliation-request-contract-not-generated'
    ),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.adminReconciliationSummary),
    execution: EXECUTION_READY,
  },
  'admin-billing-audit': {
    request: unboundRequest('AdminBillingAuditRequest', 'query', 'admin-audit-query-contract-not-generated'),
    response: boundResponse(GENERATED_ROUTE_CODEC_DESCRIPTORS.adminAuditEventsResponse),
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

export type RouteRequestModel = ContractModel<(typeof ROUTE_CONTRACT_BINDINGS)[RouteHandlerKey]['request']>;
export type RouteResponseModel = ContractModel<(typeof ROUTE_CONTRACT_BINDINGS)[RouteHandlerKey]['response']>;

function contractModel<const Contract extends RouteRequestContract | RouteResponseContract>(
  contract: Contract
): ContractModel<Contract> {
  return (contract.state === 'bound' ? contract.descriptor.model : contract.model) as ContractModel<Contract>;
}

export type RouteManifestEntry = {
  readonly path: RouteManifestSourceEntry['path'];
  readonly method: RouteMethod;
  readonly authState: AuthState;
  readonly handlerKey: RouteHandlerKey;
  readonly routeClass: RouteClass;
  readonly webhookProvider: RouteWebhookProvider | null;
  readonly requestModel: RouteRequestModel;
  readonly responseModel: RouteResponseModel;
  readonly auditRule: RouteAuditRule;
  readonly auditEvent: string;
  readonly proofIdFamily: string;
  readonly routeGroup: RouteGroup;
  readonly routeBoundary: RouteBoundary;
  readonly contract: RouteContractBinding;
};

const WEBHOOK_ROUTE_IDENTITIES = {
  stripe: { path: '/webhooks/stripe', handlerKey: 'stripe-webhook' },
  razorpay: { path: '/webhooks/razorpay', handlerKey: 'razorpay-webhook' },
  paypal: { path: '/webhooks/paypal', handlerKey: 'paypal-webhook' },
  apple: { path: '/webhooks/apple', handlerKey: 'apple-webhook' },
  google: { path: '/webhooks/google', handlerKey: 'google-webhook' },
} as const satisfies Record<
  RouteWebhookProvider,
  { readonly path: RouteManifestSourceEntry['path']; readonly handlerKey: RouteHandlerKey }
>;

function expectedPathForHandler(handlerKey: RouteHandlerKey): string {
  if (handlerKey === 'health') return '/health';
  if (handlerKey === 'pricing-public') return '/public/pricing';
  if (handlerKey.startsWith('account-session-')) {
    return `/auth/session/${handlerKey.slice('account-session-'.length)}`;
  }
  if (handlerKey.endsWith('-webhook')) {
    return `/webhooks/${handlerKey.slice(0, -'-webhook'.length)}`;
  }
  if (handlerKey.startsWith('admin-billing-')) {
    return `/admin/billing/${handlerKey.slice('admin-billing-'.length)}`;
  }
  if (handlerKey.startsWith('billing-')) {
    return `/auth/billing/${handlerKey.slice('billing-'.length)}`;
  }
  throw new Error(`No canonical path derivation for Cloudflare handler: ${handlerKey}`);
}

function routeMetadata(entry: RouteManifestSourceEntry): {
  readonly routeGroup: RouteGroup;
  readonly routeBoundary: RouteBoundary;
} {
  const { path, method, authState, auditRule, handlerKey, routeClass } = entry;
  const fail = (reason: string): never => {
    throw new Error(`Invalid Cloudflare route manifest entry ${method} ${path}: ${reason}`);
  };
  const requireCondition = (condition: boolean, reason: string): void => {
    if (!condition) {
      fail(reason);
    }
  };

  requireCondition(entry.auditEvent.trim().length > 0, 'audit event is required');
  requireCondition(entry.proofIdFamily.trim().length > 0, 'proof ID family is required');
  requireCondition(path === expectedPathForHandler(handlerKey), 'handler/path identity mismatch');
  if (routeClass !== 'provider-webhook') {
    requireCondition(!('webhookProvider' in entry), 'non-webhook route cannot declare a provider identity');
  }

  switch (routeClass) {
    case 'health':
      requireCondition(
        path === '/health' &&
          method === 'GET' &&
          authState === 'public' &&
          auditRule === 'public-observability' &&
          handlerKey === 'health',
        'health tuple mismatch'
      );
      return { routeGroup: 'health', routeBoundary: 'public' };
    case 'public-pricing':
      requireCondition(
        path === '/public/pricing' &&
          method === 'GET' &&
          authState === 'public' &&
          auditRule === 'public-observability' &&
          handlerKey === 'pricing-public',
        'public pricing tuple mismatch'
      );
      return { routeGroup: 'public', routeBoundary: 'public' };
    case 'session-login':
      requireCondition(
        path === '/auth/session/login' &&
          method === 'POST' &&
          authState === 'public' &&
          auditRule === 'public-observability' &&
          handlerKey === 'account-session-login',
        'session login tuple mismatch'
      );
      return { routeGroup: 'session', routeBoundary: 'session-login' };
    case 'session-refresh':
      requireCondition(
        path === '/auth/session/refresh' &&
          method === 'POST' &&
          authState === 'browser-refresh-required' &&
          auditRule === 'parent-session-write' &&
          handlerKey === 'account-session-refresh',
        'session refresh tuple mismatch'
      );
      return { routeGroup: 'session', routeBoundary: 'private' };
    case 'session-logout':
      requireCondition(
        path === '/auth/session/logout' &&
          method === 'POST' &&
          authState === 'browser-refresh-required' &&
          auditRule === 'parent-session-write' &&
          handlerKey === 'account-session-logout',
        'session logout tuple mismatch'
      );
      return { routeGroup: 'session', routeBoundary: 'private' };
    case 'session-revoke':
      requireCondition(
        path === '/auth/session/revoke' &&
          method === 'POST' &&
          authState === 'browser-refresh-required' &&
          auditRule === 'parent-session-write' &&
          handlerKey === 'account-session-revoke',
        'session revoke tuple mismatch'
      );
      return { routeGroup: 'session', routeBoundary: 'private' };
    case 'billing-parent-read':
      requireCondition(
        path.startsWith('/auth/billing/') &&
          method === 'GET' &&
          authState === 'parent-session-required' &&
          auditRule === 'parent-session-read' &&
          handlerKey.startsWith('billing-'),
        'parent billing read tuple mismatch'
      );
      return { routeGroup: 'billing', routeBoundary: 'private' };
    case 'billing-parent-write':
      requireCondition(
        path.startsWith('/auth/billing/') &&
          method === 'POST' &&
          authState === 'parent-session-required' &&
          auditRule === 'parent-session-write' &&
          handlerKey.startsWith('billing-'),
        'parent billing write tuple mismatch'
      );
      return { routeGroup: 'billing', routeBoundary: 'private' };
    case 'billing-trusted-read':
      requireCondition(
        path.startsWith('/auth/billing/') &&
          method === 'GET' &&
          authState === 'trusted-parent-device-required' &&
          auditRule === 'trusted-parent-device-read' &&
          handlerKey.startsWith('billing-'),
        'trusted-device billing read tuple mismatch'
      );
      return { routeGroup: 'billing', routeBoundary: 'private' };
    case 'billing-trusted-write':
      requireCondition(
        path.startsWith('/auth/billing/') &&
          method === 'POST' &&
          authState === 'trusted-parent-device-required' &&
          auditRule === 'trusted-parent-device-write' &&
          handlerKey.startsWith('billing-'),
        'trusted-device billing write tuple mismatch'
      );
      return { routeGroup: 'billing', routeBoundary: 'private' };
    case 'billing-support-write':
      requireCondition(
        path === '/auth/billing/manual-invoice' &&
          method === 'POST' &&
          authState === 'support-required' &&
          auditRule === 'support-write' &&
          handlerKey === 'billing-manual-invoice',
        'support billing write tuple mismatch'
      );
      return { routeGroup: 'billing', routeBoundary: 'support-exception' };
    case 'provider-webhook': {
      const provider = 'webhookProvider' in entry ? entry.webhookProvider : undefined;
      requireCondition(provider !== undefined, 'webhook provider is required');
      const identity = provider === undefined ? undefined : WEBHOOK_ROUTE_IDENTITIES[provider];
      requireCondition(
        identity !== undefined &&
          path === identity.path &&
          handlerKey === identity.handlerKey &&
          method === 'POST' &&
          authState === 'provider-webhook-signature-required' &&
          auditRule === 'provider-webhook',
        'provider webhook identity tuple mismatch'
      );
      return { routeGroup: 'webhook', routeBoundary: 'webhook' };
    }
    case 'admin-support-read':
      requireCondition(
        path.startsWith('/admin/billing/') &&
          method === 'GET' &&
          authState === 'support-required' &&
          auditRule === 'support-read' &&
          handlerKey.startsWith('admin-billing-'),
        'support admin read tuple mismatch'
      );
      return { routeGroup: 'admin', routeBoundary: 'private' };
    case 'admin-read':
      requireCondition(
        path.startsWith('/admin/billing/') &&
          method === 'GET' &&
          authState === 'admin-required' &&
          auditRule === 'admin-read' &&
          handlerKey.startsWith('admin-billing-'),
        'admin read tuple mismatch'
      );
      return { routeGroup: 'admin', routeBoundary: 'private' };
    case 'admin-write':
      requireCondition(
        path.startsWith('/admin/billing/') &&
          method === 'POST' &&
          authState === 'admin-required' &&
          auditRule === 'admin-write' &&
          handlerKey.startsWith('admin-billing-'),
        'admin write tuple mismatch'
      );
      return { routeGroup: 'admin', routeBoundary: 'private' };
    case 'internal-queue':
      requireCondition(
        path === '/admin/billing/reconciliation' &&
          method === 'POST' &&
          authState === 'internal-queue-only' &&
          auditRule === 'internal-queue' &&
          handlerKey === 'admin-billing-reconciliation',
        'internal queue tuple mismatch'
      );
      return { routeGroup: 'admin', routeBoundary: 'internal-queue' };
  }
}

function buildRouteManifest(): readonly RouteManifestEntry[] {
  const routeKeys = new Set<string>();
  const handlerKeys = new Set<RouteHandlerKey>();
  const webhookProviders = new Set<RouteWebhookProvider>();
  const webhookPaths = new Set<string>();
  const webhookHandlers = new Set<RouteHandlerKey>();
  const entries: RouteManifestEntry[] = [];

  for (const source of ROUTE_MANIFEST_SOURCE) {
    const key = `${source.method} ${source.path}`;
    if (routeKeys.has(key)) {
      throw new Error(`Duplicate Cloudflare route identity: ${key}`);
    }
    if (handlerKeys.has(source.handlerKey)) {
      throw new Error(`Duplicate Cloudflare handler identity: ${source.handlerKey}`);
    }
    routeKeys.add(key);
    handlerKeys.add(source.handlerKey);

    const webhookProvider = 'webhookProvider' in source ? source.webhookProvider : null;
    if (webhookProvider !== null) {
      if (
        webhookProviders.has(webhookProvider) ||
        webhookPaths.has(source.path) ||
        webhookHandlers.has(source.handlerKey)
      ) {
        throw new Error(`Duplicate provider webhook identity: ${webhookProvider}/${source.path}/${source.handlerKey}`);
      }
      webhookProviders.add(webhookProvider);
      webhookPaths.add(source.path);
      webhookHandlers.add(source.handlerKey);
    }

    const contract = ROUTE_CONTRACT_BINDINGS[source.handlerKey];
    entries.push({
      path: source.path,
      method: source.method,
      authState: source.authState,
      handlerKey: source.handlerKey,
      routeClass: source.routeClass,
      webhookProvider,
      requestModel: contractModel(contract.request),
      responseModel: contractModel(contract.response),
      auditRule: source.auditRule,
      auditEvent: source.auditEvent,
      proofIdFamily: source.proofIdFamily,
      ...routeMetadata(source),
      contract,
    });
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

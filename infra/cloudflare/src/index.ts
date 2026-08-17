import type { DurableObjectState } from '@cloudflare/workers-types';
import {
  buildAdminRefundResult,
  buildBillingCancellationSummaryFromStatus,
  buildBillingPlanChangeSummaryFromStatus,
  buildManualInvoiceResult,
  buildReconciliationSummary,
  buildReferralInviteResult,
} from './fixtures.js';
import {
  applyBillingStateMutation,
  BillingReadModelUnavailableError,
  findBillingInvoiceSubject,
  loadAdminBillingAccounts,
  loadAdminBillingDisputes,
  loadAdminBillingInvoices,
  loadAdminBillingReferrals,
  loadBillingAuditEvents,
  loadBillingEntitlementSnapshot,
  loadBillingInvoices,
  loadBillingLicenseDecision,
  loadBillingReferralSummary,
  loadBillingStatusSummary,
  loadLocalSeedSummary,
  loadPricingPlans,
  type BillingStateMutation,
} from './billing-binding-read-model.js';
import {
  BillingHostedReturnRoute,
  BillingCheckoutSessionRequestSchema,
  BillingCheckoutSessionResponseSchema,
  BillingPortalSessionRequestSchema,
  BillingPortalSessionResponseSchema,
  BillingSupportAdminAccountsResponseSchema,
  BillingSupportAdminAuditEventsResponseSchema,
  BillingSupportAdminDisputesResponseSchema,
  BillingSupportAdminInvoicesResponseSchema,
  BillingSupportAdminReferralsResponseSchema,
} from './generated/billing-contracts.js';
import {
  signatureHeaderName,
  verifyAuthState,
  verifyStripeWebhookSignature,
  type VerifiedIdentity,
} from './auth/verifier.js';
import { createFirebaseProviderVerificationPort } from './providers/firebase-auth.js';
import { validateAuthBoundaryRoute } from './auth/model.js';
import {
  isVerifiedAccountIdentityAuthorityCapability,
  type VerifiedAccountIdentityAuthorityCapability,
} from './storage/account-identity-authority-store.js';
import {
  getMissingBindings,
  isRouteKillSwitchEnabled,
  parseAllowedOrigins,
  parseRequestMaxBytes,
  resolveAuthAdapterMode,
  validateEnv,
  type Env,
} from './env.js';
import { findRoute, ROUTE_MANIFEST, type RouteManifestEntry } from './routes.js';
import { redactHeaders } from './security/redaction.js';

const STATE_CHANGING_METHODS = new Set(['POST', 'PUT', 'PATCH', 'DELETE']);
const INTERACTIVE_CSRF_HEADER = 'x-ocentra-csrf';

function billingReadModelUnavailableResponse(
  route: RouteManifestEntry,
  error: BillingReadModelUnavailableError,
  identity?: VerifiedIdentity
): Response {
  return json(503, {
    status: 'manual-required',
    handlerKey: route.handlerKey,
    authState: route.authState,
    proofIdFamily: route.proofIdFamily,
    actorRole: identity?.role ?? null,
    blocker: error.code,
    scope: error.scope,
    message: 'Durable billing read-model data is unavailable; fixture fallback is disabled outside local-safe mode.',
  });
}

function interactiveCsrfToken(env: Env): string | null {
  if (env.INTERACTIVE_CSRF_TOKEN) {
    return env.INTERACTIVE_CSRF_TOKEN;
  }

  return env.ENVIRONMENT === 'test' ? ['interactive', 'parent', 'session'].join('-') : null;
}

export const IMPLEMENTED_HANDLER_KEYS = [
  'health',
  'pricing-public',
  'billing-status',
  'billing-checkout',
  'billing-portal',
  'billing-invoices',
  'billing-change-plan',
  'billing-cancel',
  'billing-referrals',
  'billing-referral-invite',
  'billing-entitlement-snapshot',
  'billing-license-check',
  'billing-manual-invoice',
  'stripe-webhook',
  'razorpay-webhook',
  'paypal-webhook',
  'apple-webhook',
  'google-webhook',
  'admin-billing-accounts',
  'admin-billing-invoices',
  'admin-billing-refunds',
  'admin-billing-disputes',
  'admin-billing-referrals',
  'admin-billing-reconciliation',
  'admin-billing-audit',
] as const;

type HandlerContext = {
  request: Request;
  env: Env;
  route: RouteManifestEntry;
  identity?: VerifiedIdentity;
};

type RouteHandler = (context: HandlerContext) => Promise<Response>;
type HostedSessionKind = 'checkout-session-create' | 'billing-portal-session-create';
type HostedSessionRejectionReason =
  | 'auth-required'
  | 'unauthorized-role'
  | 'invalid-plan'
  | 'redirect-not-allowlisted'
  | 'abuse-gate-required'
  | 'provider-unavailable';
type CheckoutRequestBody = {
  requestId?: unknown;
  planId?: unknown;
  successPath?: unknown;
  cancelPath?: unknown;
  abuseGateState?: unknown;
  providerPreference?: unknown;
  referralCode?: unknown;
};
type PortalRequestBody = {
  requestId?: unknown;
  returnPath?: unknown;
  abuseGateState?: unknown;
};
type ChangePlanRequestBody = {
  requestId?: unknown;
  planId?: unknown;
  abuseGateState?: unknown;
};
type CancelRequestBody = {
  requestId?: unknown;
  abuseGateState?: unknown;
};
type ReferralInviteRequestBody = {
  requestId?: unknown;
  invitee?: unknown;
  abuseGateState?: unknown;
};
type LicenseCheckRequestBody = {
  requestId?: unknown;
  requestedNewDevice?: unknown;
};
type ManualInvoiceRequestBody = {
  requestId?: unknown;
  region?: unknown;
};
type AdminRefundRequestBody = {
  requestId?: unknown;
  invoiceId?: unknown;
  amountCents?: unknown;
};
type ReconciliationRequestBody = {
  requestId?: unknown;
};
type IdempotentWriteEnvelope = {
  requestKey?: unknown;
  requestFingerprint?: unknown;
  responseStatus?: unknown;
  responseBody?: unknown;
  queueMessage?: unknown;
  stateMutation?: unknown;
  conflictResponseStatus?: unknown;
  conflictResponseBody?: unknown;
};
type IdempotentWriteResult = {
  replayed: boolean;
  responseStatus: number;
  responseBody: unknown;
  queued: boolean;
};
type QueueFailureReason = 'reconciliation-queue-missing' | 'reconciliation-queue-send-failed';

const CHECKOUT_SUCCESS_PATH = BillingHostedReturnRoute.CheckoutSuccess.relativePath;
const CHECKOUT_CANCEL_PATH = BillingHostedReturnRoute.CheckoutCancel.relativePath;
const PORTAL_RETURN_PATH = BillingHostedReturnRoute.PortalReturn.relativePath;
const ALLOWLISTED_RETURN_PATHS: ReadonlySet<string> = new Set([
  CHECKOUT_SUCCESS_PATH,
  CHECKOUT_CANCEL_PATH,
  PORTAL_RETURN_PATH,
]);
const ACCEPTED_ABUSE_GATE_STATES = new Set(['passed-turnstile', 'trusted-authenticated-session']);

function json(status: number, body: unknown, headers: HeadersInit = {}): Response {
  return new Response(JSON.stringify(body, null, 2), {
    status,
    headers: {
      'content-type': 'application/json; charset=utf-8',
      ...headers,
    },
  });
}

function requireSupportAdminReadIdentity(identity: VerifiedIdentity | undefined): VerifiedIdentity {
  if (
    !identity ||
    !isVerifiedAccountIdentityAuthorityCapability(identity.authority) ||
    identity.authority.role !== 'support-admin'
  ) {
    throw new Error('support-admin-read-identity-required');
  }

  return identity;
}

type VerifiedAuthority = VerifiedAccountIdentityAuthorityCapability;

function requireVerifiedAuthority(identity: VerifiedIdentity | undefined): VerifiedAuthority | Response {
  if (!identity || !isVerifiedAccountIdentityAuthorityCapability(identity.authority)) {
    return json(503, {
      status: 'manual-required',
      blocker: 'account-identity-authority-capability-missing',
    });
  }
  return identity.authority;
}

function requireVerifiedParentAuthority(identity: VerifiedIdentity | undefined): VerifiedAuthority | Response {
  const authority = requireVerifiedAuthority(identity);
  if (authority instanceof Response) {
    return authority;
  }
  if (authority.role !== 'parent-owner' && authority.role !== 'co-parent-guardian') {
    return json(403, {
      error: 'parent-role-capability-required',
    });
  }
  return authority;
}

function requireVerifiedSupportAuthority(identity: VerifiedIdentity | undefined): VerifiedAuthority | Response {
  const authority = requireVerifiedAuthority(identity);
  if (authority instanceof Response) {
    return authority;
  }
  if (authority.role !== 'support-admin') {
    return json(403, {
      error: 'support-admin-capability-required',
    });
  }
  return authority;
}

function withCors(response: Response, request: Request, env: Env): Response {
  const headers = new Headers(response.headers);
  const origin = request.headers.get('origin');
  headers.set('access-control-allow-methods', 'GET,POST,OPTIONS');
  headers.set(
    'access-control-allow-headers',
    'authorization,content-type,stripe-signature,paypal-transmission-id,paypal-transmission-sig,x-razorpay-signature,x-goog-signature,x-ocentra-internal-call,x-ocentra-internal-secret,x-ocentra-csrf'
  );
  headers.set('access-control-max-age', '86400');
  headers.set('vary', 'origin');
  headers.set('access-control-allow-origin', resolveResponseOrigin(origin, env));

  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers,
  });
}

function resolveResponseOrigin(origin: string | null, env: Env): string {
  const allowedOrigins = parseAllowedOrigins(env);
  if (!origin) {
    return allowedOrigins[0] ?? env.APP_ORIGIN;
  }
  return allowedOrigins.includes(origin) ? origin : env.APP_ORIGIN;
}

function isAllowedOrigin(origin: string | null, env: Env): boolean {
  if (!origin) {
    return true;
  }
  return parseAllowedOrigins(env).includes(origin);
}

function parseContentLengthHeader(request: Request): { ok: true; value: number } | { ok: false; response: Response } {
  const transferEncoding = request.headers.get('transfer-encoding');
  if (transferEncoding && transferEncoding.trim().length > 0) {
    return {
      ok: false,
      response: json(400, {
        error: 'unsupported-transfer-encoding',
      }),
    };
  }

  const headerValue = request.headers.get('content-length');
  if (!headerValue) {
    if (request.body !== null && STATE_CHANGING_METHODS.has(request.method)) {
      return {
        ok: false,
        response: json(400, {
          error: 'missing-content-length',
        }),
      };
    }

    return {
      ok: true,
      value: 0,
    };
  }

  const trimmed = headerValue.trim();
  if (trimmed.includes(',')) {
    return {
      ok: false,
      response: json(400, {
        error: 'ambiguous-content-length',
      }),
    };
  }

  if (!/^\d+$/.test(trimmed)) {
    return {
      ok: false,
      response: json(400, {
        error: 'invalid-content-length',
      }),
    };
  }

  return {
    ok: true,
    value: Number(trimmed),
  };
}

function manualRequiredResponse(route: RouteManifestEntry, identity?: VerifiedIdentity): Response {
  return json(501, {
    status: 'manual-required',
    handlerKey: route.handlerKey,
    authState: route.authState,
    proofIdFamily: route.proofIdFamily,
    actorRole: identity?.role ?? null,
    message:
      'This route is contract-shaped, but provider-backed behavior is still owned by the active payment workpacks.',
    nextStep: `Implement ${route.handlerKey} behind the shared billing contract before calling this route production-ready.`,
  });
}

function sanitizeIdFragment(value: string): string {
  return value.replace(/[^A-Za-z0-9_-]/g, '-').slice(0, 48);
}

function requestIdFor(kind: string, subject: string, providedRequestId: unknown): string {
  if (typeof providedRequestId === 'string' && providedRequestId.trim().length > 0) {
    return sanitizeIdFragment(providedRequestId.trim());
  }
  return `${sanitizeIdFragment(kind)}-${sanitizeIdFragment(subject)}`;
}

function rejectionResponse(
  kind: HostedSessionKind,
  requestId: string,
  rejectionReason: HostedSessionRejectionReason
): Response {
  const pendingEntitlementConfirmation = kind === 'checkout-session-create';
  if (kind === 'checkout-session-create') {
    return json(
      200,
      BillingCheckoutSessionResponseSchema.parse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId,
        kind,
        status: 'rejected',
        hostedSessionId: null,
        hostedUrl: null,
        expiresAt: null,
        rejectionReason,
        provider: 'stripe',
        ownerSubject: 'unknown',
        pendingEntitlementConfirmation,
      })
    );
  }

  return json(
    200,
    BillingPortalSessionResponseSchema.parse({
      schemaVersion: 'billing-checkout-portal-boundary',
      requestId,
      kind,
      status: 'rejected',
      hostedSessionId: null,
      hostedUrl: null,
      expiresAt: null,
      rejectionReason,
      provider: 'stripe',
      ownerSubject: 'unknown',
      pendingEntitlementConfirmation,
    })
  );
}

function hostedSessionPrefix(kind: HostedSessionKind): 'checkout-session' | 'portal-session' {
  return kind === 'checkout-session-create' ? 'checkout-session' : 'portal-session';
}

function hostedSessionIdFor(kind: HostedSessionKind, requestId: string): string {
  return `${hostedSessionPrefix(kind)}-${sanitizeIdFragment(requestId)}`;
}

function hostedSessionUrlFor(kind: HostedSessionKind, requestId: string): string {
  return kind === 'checkout-session-create'
    ? `https://checkout.stripe.com/c/pay/${sanitizeIdFragment(requestId)}`
    : `https://billing.stripe.com/p/session/${sanitizeIdFragment(requestId)}`;
}

function hostedSessionAuditReference(kind: HostedSessionKind, subject: string, requestId: string): string {
  return `audit:${hostedSessionPrefix(kind)}:${sanitizeIdFragment(subject)}:${sanitizeIdFragment(requestId)}`;
}

function acceptedResponseBody(kind: HostedSessionKind, requestId: string, subject: string): Record<string, unknown> {
  const hostedUrl = hostedSessionUrlFor(kind, requestId);
  const pendingEntitlementConfirmation = kind === 'checkout-session-create';
  const contractResponse =
    kind === 'checkout-session-create'
      ? BillingCheckoutSessionResponseSchema.parse({
          schemaVersion: 'billing-checkout-portal-boundary',
          requestId,
          kind,
          status: 'accepted',
          hostedSessionId: hostedSessionIdFor(kind, requestId),
          hostedUrl,
          expiresAt: '2026-06-14T01:00:00.000Z',
          rejectionReason: null,
          provider: 'stripe',
          ownerSubject: subject,
          pendingEntitlementConfirmation,
        })
      : BillingPortalSessionResponseSchema.parse({
          schemaVersion: 'billing-checkout-portal-boundary',
          requestId,
          kind,
          status: 'accepted',
          hostedSessionId: hostedSessionIdFor(kind, requestId),
          hostedUrl,
          expiresAt: '2026-06-14T01:00:00.000Z',
          rejectionReason: null,
          provider: 'stripe',
          ownerSubject: subject,
          pendingEntitlementConfirmation,
        });

  return contractResponse;
}

async function readJsonObject<T extends Record<string, unknown>>(request: Request): Promise<T | null> {
  const body = await request.text();
  if (body.trim().length === 0) {
    return {} as T;
  }

  try {
    const parsed = JSON.parse(body) as unknown;
    if (typeof parsed === 'object' && parsed !== null && !Array.isArray(parsed)) {
      return parsed as T;
    }
  } catch {
    return null;
  }

  return null;
}

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && value.constructor === Object;
}

function stringOrNull(value: unknown): string | null {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : null;
}

function numberOrNull(value: unknown): number | null {
  return typeof value === 'number' && Number.isFinite(value) ? value : null;
}

function booleanFromUnknown(value: unknown): boolean {
  return value === true;
}

function extractAbuseGateState(value: unknown): string {
  return typeof value === 'string' ? value : 'trusted-authenticated-session';
}

function cloneJsonValue<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

function withQueuedFlag(responseBody: unknown, queued: boolean): unknown {
  if (!isPlainObject(responseBody) || !Object.prototype.hasOwnProperty.call(responseBody, 'queued')) {
    return responseBody;
  }

  return {
    ...responseBody,
    queued,
  };
}

function queueFailureMessage(error: unknown): string | null {
  if (!(error instanceof Error)) {
    return null;
  }

  return error.message.replace(/\s+/gu, ' ').trim().slice(0, 160) || null;
}

function deadLetterPayload(
  payload: Record<string, unknown>,
  reason: QueueFailureReason,
  error: unknown
): Record<string, unknown> {
  return {
    disposition: 'dead-letter',
    sourceQueue: 'BILLING_RECONCILIATION_QUEUE',
    reason,
    payload: cloneJsonValue(payload),
    failedAt: new Date().toISOString(),
    errorMessage: queueFailureMessage(error),
  };
}

function explicitProviderHint(payload: unknown): string | null {
  if (!isPlainObject(payload)) {
    return null;
  }

  const directHint =
    stringOrNull(payload.provider) ?? stringOrNull(payload.providerName) ?? stringOrNull(payload.providerRoute);
  if (directHint) {
    return directHint;
  }

  const metadata = isPlainObject(payload.metadata) ? payload.metadata : null;
  const metadataHint =
    stringOrNull(metadata?.provider) ?? stringOrNull(metadata?.providerName) ?? stringOrNull(metadata?.providerRoute);
  if (metadataHint) {
    return metadataHint;
  }

  const data = isPlainObject(payload.data) ? payload.data : null;
  const object = isPlainObject(data?.object) ? data.object : null;
  const objectMetadata = isPlainObject(object?.metadata) ? object.metadata : null;
  return (
    stringOrNull(object?.provider) ??
    stringOrNull(objectMetadata?.provider) ??
    stringOrNull(objectMetadata?.providerName) ??
    stringOrNull(objectMetadata?.providerRoute)
  );
}

function parseBillingStateMutation(value: unknown): BillingStateMutation | null {
  if (!isPlainObject(value)) {
    return null;
  }

  const kind = stringOrNull(value.kind);
  const subject = stringOrNull(value.subject);
  if (!kind || !subject) {
    return null;
  }

  if (kind === 'hosted-session') {
    const requestId = stringOrNull(value.requestId);
    const sessionKind = stringOrNull(value.sessionKind);
    const auditReference = stringOrNull(value.auditReference);
    const actorRole = stringOrNull(value.actorRole);
    if (
      !requestId ||
      !auditReference ||
      (sessionKind !== 'checkout-session-create' && sessionKind !== 'billing-portal-session-create') ||
      (actorRole !== 'parent' && actorRole !== 'guardian')
    ) {
      return null;
    }
    return {
      kind,
      subject,
      requestId,
      sessionKind,
      auditReference,
      actorRole,
    };
  }

  if (kind === 'change-plan') {
    const requestId = stringOrNull(value.requestId);
    const auditReference = stringOrNull(value.auditReference);
    const targetPlanId = stringOrNull(value.targetPlanId);
    if (!requestId || !auditReference || !targetPlanId) {
      return null;
    }
    return {
      kind,
      subject,
      requestId,
      targetPlanId,
      auditReference,
    };
  }

  if (kind === 'cancel') {
    const requestId = stringOrNull(value.requestId);
    const auditReference = stringOrNull(value.auditReference);
    const cancellationState = stringOrNull(value.cancellationState);
    if (
      !requestId ||
      !auditReference ||
      (cancellationState !== 'scheduled-period-end' &&
        cancellationState !== 'already-in-grace' &&
        cancellationState !== 'manual-review-required')
    ) {
      return null;
    }
    return {
      kind,
      subject,
      requestId,
      cancellationState,
      auditReference,
    };
  }

  if (kind === 'referral-invite') {
    const requestId = stringOrNull(value.requestId);
    const invitedIdentifier = stringOrNull(value.invitedIdentifier);
    const referralCode = stringOrNull(value.referralCode);
    const auditReference = stringOrNull(value.auditReference);
    const actorRole = stringOrNull(value.actorRole);
    if (
      !requestId ||
      !invitedIdentifier ||
      !referralCode ||
      !auditReference ||
      (actorRole !== 'parent' && actorRole !== 'guardian')
    ) {
      return null;
    }
    return {
      kind,
      subject,
      requestId,
      invitedIdentifier,
      referralCode,
      auditReference,
      actorRole,
    };
  }

  if (kind === 'manual-invoice') {
    const requestId = stringOrNull(value.requestId);
    const auditReference = stringOrNull(value.auditReference);
    const region = stringOrNull(value.region);
    const actorRole = stringOrNull(value.actorRole);
    if (!requestId || !auditReference || !region || (actorRole !== 'support' && actorRole !== 'admin')) {
      return null;
    }
    return {
      kind,
      subject,
      requestId,
      region,
      auditReference,
      actorRole,
    };
  }

  if (kind === 'admin-refund') {
    const requestId = stringOrNull(value.requestId);
    const invoiceId = stringOrNull(value.invoiceId);
    const auditReference = stringOrNull(value.auditReference);
    const refundState = stringOrNull(value.refundState);
    const amountCents = numberOrNull(value.amountCents);
    const actorRole = stringOrNull(value.actorRole);
    if (
      !requestId ||
      !invoiceId ||
      !auditReference ||
      amountCents === null ||
      (refundState !== 'refund-requested' && refundState !== 'refund-settled') ||
      (actorRole !== 'support' && actorRole !== 'admin')
    ) {
      return null;
    }
    return {
      kind,
      subject,
      requestId,
      invoiceId,
      refundState,
      amountCents,
      auditReference,
      actorRole,
    };
  }

  if (kind === 'reconciliation') {
    const requestId = stringOrNull(value.requestId);
    const auditReference = stringOrNull(value.auditReference);
    const actorRole = stringOrNull(value.actorRole);
    if (!requestId || !auditReference || (actorRole !== 'support' && actorRole !== 'admin' && actorRole !== 'system')) {
      return null;
    }
    return {
      kind,
      subject,
      requestId,
      auditReference,
      actorRole,
    };
  }

  if (kind === 'provider-webhook') {
    const provider = stringOrNull(value.provider);
    const eventId = stringOrNull(value.eventId);
    const eventType = stringOrNull(value.eventType);
    if (!provider || !eventId || !eventType) {
      return null;
    }
    return {
      kind,
      provider,
      subject,
      eventId,
      eventType,
      disputeId: stringOrNull(value.disputeId),
      invoiceId: stringOrNull(value.invoiceId),
    };
  }

  return null;
}

function durableWriteKey(action: string, subject: string, requestId: string): string {
  return `${action}:${subject}:${requestId}`;
}

function webhookIdempotencyKey(provider: string, eventId: string): string {
  return durableWriteKey('provider-webhook', provider, eventId);
}

function webhookRequestFingerprint(provider: string, body: string): string {
  return `${provider}:${body}`;
}

function acceptedWebhookResponse(
  provider: string,
  proofIdFamily: string,
  event: { eventId: string; eventType: string }
): {
  status: 'accepted';
  provider: string;
  queued: boolean;
  proofIdFamily: string;
  eventId: string;
  eventType: string;
} {
  return {
    status: 'accepted',
    provider,
    queued: true,
    proofIdFamily,
    ...event,
  };
}

function conflictingWebhookResponse(
  provider: string,
  proofIdFamily: string,
  event: { eventId: string; eventType: string }
): {
  status: 'manual-review';
  provider: string;
  queued: boolean;
  proofIdFamily: string;
  eventId: string;
  eventType: string;
  conflictReason: 'event-id-payload-mismatch';
} {
  return {
    status: 'manual-review',
    provider,
    queued: false,
    proofIdFamily,
    ...event,
    conflictReason: 'event-id-payload-mismatch',
  };
}

async function executeIdempotentWrite(
  namespace: Env['BILLING_DO'] | Env['REFERRAL_DO'],
  objectName: string,
  envelope: {
    requestKey: string;
    requestFingerprint?: string;
    responseStatus: number;
    responseBody: unknown;
    queueMessage: Record<string, unknown> | null;
    stateMutation?: BillingStateMutation | null;
    conflictResponseStatus?: number;
    conflictResponseBody?: unknown;
  },
  env: Env
): Promise<Response> {
  if (!namespace) {
    if (envelope.stateMutation) {
      await applyBillingStateMutation(env, envelope.stateMutation);
    }
    const queued = envelope.queueMessage ? await queueReconciliationEvent(env, envelope.queueMessage) : false;
    const responseBody = envelope.queueMessage
      ? withQueuedFlag(cloneJsonValue(envelope.responseBody), queued)
      : envelope.responseBody;
    return json(envelope.responseStatus, responseBody);
  }

  const durableObject = namespace.get(namespace.idFromName(objectName));
  const doResponse = await durableObject.fetch(
    new Request('https://durable-object.local/idempotency/execute', {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
      },
      body: JSON.stringify(envelope),
    })
  );
  const result = (await doResponse.json()) as IdempotentWriteResult;
  return json(result.responseStatus, result.responseBody);
}

function billingActorRoleForAuthority(authority: VerifiedAuthority): 'parent' | 'guardian' {
  return authority.role === 'parent-owner' ? 'parent' : 'guardian';
}

async function billingHostedRouteContext(
  authority: VerifiedAuthority
): Promise<{
  actor: {
    actorId: string;
    role: 'parent' | 'guardian';
  };
  parentAccount: {
    parentAccountId: string;
  };
  family: {
    familyId: string;
  };
}> {
  const actorRole = billingActorRoleForAuthority(authority);
  return {
    actor: {
      actorId: `actor-${sanitizeIdFragment(authority.memberId)}`,
      role: actorRole,
    },
    parentAccount: {
      parentAccountId: authority.accountId,
    },
    family: {
      familyId: authority.householdId,
    },
  };
}

function checkoutHostedRouteForPath(path: string) {
  if (path === CHECKOUT_SUCCESS_PATH) {
    return BillingHostedReturnRoute.CheckoutSuccess;
  }
  if (path === CHECKOUT_CANCEL_PATH) {
    return BillingHostedReturnRoute.CheckoutCancel;
  }
  return null;
}

function requireInteractiveRequestBoundary(
  request: Request,
  env: Env,
  identity: VerifiedIdentity,
  requestId: string
): Response | null {
  const authority = requireVerifiedParentAuthority(identity);
  if (authority instanceof Response) {
    return authority;
  }

  const origin = request.headers.get('origin');
  if (!origin || !parseAllowedOrigins(env).includes(origin)) {
    return json(403, {
      error: 'origin-validation-failed',
    });
  }

  const expectedCsrfToken = interactiveCsrfToken(env);
  if (!expectedCsrfToken || request.headers.get(INTERACTIVE_CSRF_HEADER) !== expectedCsrfToken) {
    return json(403, {
      error: 'csrf-validation-failed',
    });
  }

  return null;
}

async function computeHexHmac(payload: string, secret: string): Promise<string> {
  const key = await crypto.subtle.importKey(
    'raw',
    new TextEncoder().encode(secret),
    {
      name: 'HMAC',
      hash: 'SHA-256',
    },
    false,
    ['sign']
  );
  const signed = await crypto.subtle.sign('HMAC', key, new TextEncoder().encode(payload));
  return Array.from(new Uint8Array(signed), (value) => value.toString(16).padStart(2, '0')).join('');
}

function safeEqual(left: string, right: string): boolean {
  if (left.length !== right.length) {
    return false;
  }
  let diff = 0;
  for (let index = 0; index < left.length; index += 1) {
    diff |= left.charCodeAt(index) ^ right.charCodeAt(index);
  }
  return diff === 0;
}

async function verifyHexHmac(payload: string, signature: string, secret: string): Promise<boolean> {
  const expected = await computeHexHmac(payload, secret);
  return safeEqual(signature.toLowerCase(), expected.toLowerCase());
}

function providerEventDetails(provider: string, payload: unknown): { eventId: string; eventType: string } {
  const fallbackId = `${provider}_evt_local_unknown`;
  const fallbackType = `${provider}.unknown`;
  if (typeof payload !== 'object' || payload === null) {
    return {
      eventId: fallbackId,
      eventType: fallbackType,
    };
  }

  const payloadRecord = payload as Record<string, unknown>;
  return {
    eventId: typeof payloadRecord.id === 'string' ? payloadRecord.id : fallbackId,
    eventType: typeof payloadRecord.type === 'string' ? payloadRecord.type : fallbackType,
  };
}

function providerWebhookSubject(payload: unknown): string | null {
  if (!isPlainObject(payload)) {
    return null;
  }

  const directSubject =
    stringOrNull(payload.subject) ?? stringOrNull(payload.ownerSubject) ?? stringOrNull(payload.parentSubject);
  if (directSubject) {
    return directSubject;
  }

  const metadata = isPlainObject(payload.metadata) ? payload.metadata : null;
  const metadataSubject =
    stringOrNull(metadata?.subject) ??
    stringOrNull(metadata?.ownerSubject) ??
    stringOrNull(metadata?.ocentraParentSubject);
  if (metadataSubject) {
    return metadataSubject;
  }

  const data = isPlainObject(payload.data) ? payload.data : null;
  const object = isPlainObject(data?.object) ? data.object : null;
  const objectMetadata = isPlainObject(object?.metadata) ? object.metadata : null;
  return (
    stringOrNull(object?.subject) ??
    stringOrNull(objectMetadata?.subject) ??
    stringOrNull(objectMetadata?.ownerSubject) ??
    stringOrNull(objectMetadata?.ocentraParentSubject)
  );
}

function providerWebhookInvoiceId(payload: unknown): string | null {
  if (!isPlainObject(payload)) {
    return null;
  }

  const directInvoiceId = stringOrNull(payload.invoiceId) ?? stringOrNull(payload.invoice_id);
  if (directInvoiceId) {
    return directInvoiceId;
  }

  const data = isPlainObject(payload.data) ? payload.data : null;
  const object = isPlainObject(data?.object) ? data.object : null;
  return stringOrNull(object?.invoice) ?? stringOrNull(object?.invoiceId) ?? stringOrNull(object?.invoice_id);
}

function providerWebhookDisputeId(payload: unknown, eventType: string, fallbackDisputeId: string): string | null {
  if (!eventType.includes('dispute')) {
    return null;
  }
  if (!isPlainObject(payload)) {
    return fallbackDisputeId;
  }

  const directDisputeId = stringOrNull(payload.disputeId) ?? stringOrNull(payload.dispute_id);
  if (directDisputeId) {
    return directDisputeId;
  }

  const data = isPlainObject(payload.data) ? payload.data : null;
  const object = isPlainObject(data?.object) ? data.object : null;
  return stringOrNull(object?.id) ?? fallbackDisputeId;
}

async function acceptProviderWebhook(
  provider: string,
  body: string,
  proofIdFamily: string,
  env: Env
): Promise<Response> {
  let payload: unknown;
  try {
    payload = body.length > 0 ? JSON.parse(body) : {};
  } catch {
    return json(400, {
      error: 'invalid-webhook-payload',
      provider,
    });
  }

  if (!isPlainObject(payload)) {
    return json(400, {
      error: 'invalid-webhook-payload',
      provider,
    });
  }

  const providerHint = explicitProviderHint(payload);
  if (providerHint && providerHint !== provider) {
    return json(400, {
      error: 'provider-route-mismatch',
      provider,
    });
  }

  const event = providerEventDetails(provider, payload);
  const subject = providerWebhookSubject(payload);
  const invoiceId = providerWebhookInvoiceId(payload);
  const disputeId = providerWebhookDisputeId(payload, event.eventType, `dispute-${provider}-${event.eventId}`);
  return executeIdempotentWrite(
    env.BILLING_DO,
    subject ? `billing-control:${subject}` : `billing-control:webhook:${provider}`,
    {
      requestKey: webhookIdempotencyKey(provider, event.eventId),
      requestFingerprint: webhookRequestFingerprint(provider, body),
      responseStatus: 202,
      responseBody: acceptedWebhookResponse(provider, proofIdFamily, event),
      queueMessage: {
        provider,
        ...event,
        subject,
        receivedAt: new Date().toISOString(),
      },
      stateMutation: subject
        ? {
            kind: 'provider-webhook',
            provider,
            subject,
            eventId: event.eventId,
            eventType: event.eventType,
            disputeId,
            invoiceId,
          }
        : null,
      conflictResponseStatus: 409,
      conflictResponseBody: conflictingWebhookResponse(provider, proofIdFamily, event),
    },
    env
  );
}

async function queueReconciliationEvent(env: Env, payload: Record<string, unknown>): Promise<boolean> {
  if (!env.BILLING_RECONCILIATION_QUEUE) {
    try {
      await env.BILLING_DEAD_LETTER_QUEUE?.send(deadLetterPayload(payload, 'reconciliation-queue-missing', null));
    } catch {
      return false;
    }
    return false;
  }

  try {
    await env.BILLING_RECONCILIATION_QUEUE.send(payload);
    return true;
  } catch (error) {
    try {
      await env.BILLING_DEAD_LETTER_QUEUE?.send(deadLetterPayload(payload, 'reconciliation-queue-send-failed', error));
    } catch {
      return false;
    }

    return false;
  }
}

async function routeHandlerMap(): Promise<Record<string, RouteHandler>> {
  return {
    async health({ env }): Promise<Response> {
      const missingBindings = getMissingBindings(env);
      return json(200, {
        status: 'ok',
        service: 'cloudflare-control-plane',
        environment: env.ENVIRONMENT,
        authAdapterMode: resolveAuthAdapterMode(env),
        routeCount: ROUTE_MANIFEST.length,
        implementedHandlerCount: IMPLEMENTED_HANDLER_KEYS.length,
        bindingStatus: missingBindings.length === 0 ? 'ready' : 'degraded',
        missingBindingCount: missingBindings.length,
        seedSummary: await loadLocalSeedSummary(env),
      });
    },

    async 'pricing-public'({ env }): Promise<Response> {
      return json(200, {
        status: 'ok',
        plans: await loadPricingPlans(env),
        updatedAt: '2026-06-14T00:00:00.000Z',
      });
    },

    async 'billing-status'({ env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      return json(200, await loadBillingStatusSummary(env, subject));
    },

    async 'billing-checkout'({ request, env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      const body = await readJsonObject<CheckoutRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('checkout', subject, body.requestId);
      const boundaryFailure = requireInteractiveRequestBoundary(
        request,
        env,
        identity,
        requestId
      );
      if (boundaryFailure) {
        return boundaryFailure;
      }

      const planId = stringOrNull(body.planId);
      const pricingPlans = await loadPricingPlans(env);
      if (!planId || !pricingPlans.some((plan) => plan.planId === planId && plan.priceCents > 0)) {
        return rejectionResponse('checkout-session-create', requestId, 'invalid-plan');
      }

      const successPath = stringOrNull(body.successPath) ?? CHECKOUT_SUCCESS_PATH;
      const cancelPath = stringOrNull(body.cancelPath) ?? CHECKOUT_CANCEL_PATH;
      if (successPath !== CHECKOUT_SUCCESS_PATH || cancelPath !== CHECKOUT_CANCEL_PATH) {
        return rejectionResponse('checkout-session-create', requestId, 'redirect-not-allowlisted');
      }

      const abuseGateState = extractAbuseGateState(body.abuseGateState);
      if (!ACCEPTED_ABUSE_GATE_STATES.has(abuseGateState)) {
        return rejectionResponse('checkout-session-create', requestId, 'abuse-gate-required');
      }

      BillingCheckoutSessionRequestSchema.parse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId,
        kind: 'checkout-session-create',
        ...(await billingHostedRouteContext(authority)),
        planId,
        successRoute: checkoutHostedRouteForPath(successPath),
        cancelRoute: checkoutHostedRouteForPath(cancelPath),
        abuseGateState,
      });

      const actorRole = billingActorRoleForAuthority(authority);
      return executeIdempotentWrite(
        env.BILLING_DO,
        `billing-control:${subject}`,
        {
          requestKey: durableWriteKey('checkout-session-create', subject, requestId),
          requestFingerprint: `checkout-session-create:${planId}:${successPath}:${cancelPath}`,
          responseStatus: 200,
          responseBody: acceptedResponseBody('checkout-session-create', requestId, subject),
          queueMessage: null,
          stateMutation: actorRole
            ? {
                kind: 'hosted-session',
                subject,
                requestId,
                sessionKind: 'checkout-session-create',
                auditReference: hostedSessionAuditReference('checkout-session-create', subject, requestId),
                actorRole,
              }
            : null,
        },
        env
      );
    },

    async 'billing-portal'({ request, env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      const body = await readJsonObject<PortalRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('portal', subject, body.requestId);
      const boundaryFailure = requireInteractiveRequestBoundary(
        request,
        env,
        identity,
        requestId
      );
      if (boundaryFailure) {
        return boundaryFailure;
      }

      const returnPath = stringOrNull(body.returnPath) ?? PORTAL_RETURN_PATH;
      if (!ALLOWLISTED_RETURN_PATHS.has(returnPath) || returnPath !== PORTAL_RETURN_PATH) {
        return rejectionResponse('billing-portal-session-create', requestId, 'redirect-not-allowlisted');
      }

      const abuseGateState = extractAbuseGateState(body.abuseGateState);
      if (!ACCEPTED_ABUSE_GATE_STATES.has(abuseGateState)) {
        return rejectionResponse('billing-portal-session-create', requestId, 'abuse-gate-required');
      }

      BillingPortalSessionRequestSchema.parse({
        schemaVersion: 'billing-checkout-portal-boundary',
        requestId,
        kind: 'billing-portal-session-create',
        ...(await billingHostedRouteContext(authority)),
        returnRoute: BillingHostedReturnRoute.PortalReturn,
        abuseGateState,
      });

      const actorRole = billingActorRoleForAuthority(authority);
      return executeIdempotentWrite(
        env.BILLING_DO,
        `billing-control:${subject}`,
        {
          requestKey: durableWriteKey('billing-portal-session-create', subject, requestId),
          requestFingerprint: `billing-portal-session-create:${returnPath}`,
          responseStatus: 200,
          responseBody: acceptedResponseBody('billing-portal-session-create', requestId, subject),
          queueMessage: null,
          stateMutation: actorRole
            ? {
                kind: 'hosted-session',
                subject,
                requestId,
                sessionKind: 'billing-portal-session-create',
                auditReference: hostedSessionAuditReference(
                  'billing-portal-session-create',
                  subject,
                  requestId
                ),
                actorRole,
              }
            : null,
        },
        env
      );
    },

    async 'billing-invoices'({ env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      const invoices = await loadBillingInvoices(env, subject);
      return json(200, {
        status: 'ok',
        subject,
        invoiceCount: invoices.length,
        invoices,
      });
    },

    async 'billing-change-plan'({ request, env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      const body = await readJsonObject<ChangePlanRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('change-plan', subject, body.requestId);
      const boundaryFailure = requireInteractiveRequestBoundary(request, env, identity, requestId);
      if (boundaryFailure) {
        return boundaryFailure;
      }

      const abuseGateState = extractAbuseGateState(body.abuseGateState);
      if (!ACCEPTED_ABUSE_GATE_STATES.has(abuseGateState)) {
        return json(403, {
          error: 'abuse-gate-required',
        });
      }

      const summary = buildBillingPlanChangeSummaryFromStatus(
        await loadBillingStatusSummary(env, subject),
        requestId,
        stringOrNull(body.planId),
        await loadPricingPlans(env)
      );
      const targetPlanId = summary.targetPlanId;
      return executeIdempotentWrite(
        env.BILLING_DO,
        `billing-control:${subject}`,
        {
          requestKey: durableWriteKey('change-plan', subject, requestId),
          requestFingerprint: `change-plan:${targetPlanId ?? 'invalid'}`,
          responseStatus: 200,
          responseBody: summary,
          queueMessage:
            summary.status === 'accepted'
              ? {
                  action: 'change-plan',
                  requestId,
                  subject,
                  targetPlanId,
                }
              : null,
          stateMutation:
            summary.status === 'accepted' && targetPlanId !== null
              ? {
                  kind: 'change-plan',
                  subject,
                  requestId,
                  targetPlanId,
                  auditReference: summary.auditReference,
                }
              : null,
        },
        env
      );
    },

    async 'billing-cancel'({ request, env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      const body = await readJsonObject<CancelRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('cancel', subject, body.requestId);
      const boundaryFailure = requireInteractiveRequestBoundary(request, env, identity, requestId);
      if (boundaryFailure) {
        return boundaryFailure;
      }

      const abuseGateState = extractAbuseGateState(body.abuseGateState);
      if (!ACCEPTED_ABUSE_GATE_STATES.has(abuseGateState)) {
        return json(403, {
          error: 'abuse-gate-required',
        });
      }

      const summary = buildBillingCancellationSummaryFromStatus(
        await loadBillingStatusSummary(env, subject),
        requestId
      );
      return executeIdempotentWrite(
        env.BILLING_DO,
        `billing-control:${subject}`,
        {
          requestKey: durableWriteKey('cancel', subject, requestId),
          requestFingerprint: `cancel:${summary.cancellationState}`,
          responseStatus: 200,
          responseBody: summary,
          queueMessage: {
            action: 'cancel',
            requestId,
            subject,
            cancellationState: summary.cancellationState,
          },
          stateMutation: {
            kind: 'cancel',
            subject,
            requestId,
            cancellationState: summary.cancellationState,
            auditReference: summary.auditReference,
          },
        },
        env
      );
    },

    async 'billing-referrals'({ env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      return json(200, {
        status: 'ok',
        ...(await loadBillingReferralSummary(env, subject)),
      });
    },

    async 'billing-referral-invite'({ request, env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      const body = await readJsonObject<ReferralInviteRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('referral-invite', subject, body.requestId);
      const boundaryFailure = requireInteractiveRequestBoundary(request, env, identity, requestId);
      if (boundaryFailure) {
        return boundaryFailure;
      }

      const abuseGateState = extractAbuseGateState(body.abuseGateState);
      if (!ACCEPTED_ABUSE_GATE_STATES.has(abuseGateState)) {
        return json(403, {
          error: 'abuse-gate-required',
        });
      }

      const result = buildReferralInviteResult(subject, requestId, stringOrNull(body.invitee));
      if (result.status === 'accepted') {
        const invitedIdentifier = stringOrNull(body.invitee)?.trim().toLowerCase();
        const actorRole = billingActorRoleForAuthority(authority);
        return executeIdempotentWrite(
          env.BILLING_DO,
          `billing-control:${subject}`,
          {
            requestKey: durableWriteKey('referral-invite', subject, requestId),
            responseStatus: 200,
            responseBody: result,
            queueMessage: {
              action: 'referral-invite',
              requestId,
              subject,
              referralCode: result.referralCode,
            },
            stateMutation:
              invitedIdentifier && result.referralCode && actorRole
                ? {
                    kind: 'referral-invite',
                    subject,
                    requestId,
                    invitedIdentifier,
                    referralCode: result.referralCode,
                    auditReference: result.auditReference,
                    actorRole,
                  }
                : null,
          },
          env
        );
      }
      return json(200, result);
    },

    async 'billing-entitlement-snapshot'({ env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      return json(200, {
        status: 'ok',
        snapshot: await loadBillingEntitlementSnapshot(env, subject),
      });
    },

    async 'billing-license-check'({ request, env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedParentAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      const body = await readJsonObject<LicenseCheckRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('license-check', subject, body.requestId);
      const deviceId = authority.deviceId;
      return json(
        200,
        await loadBillingLicenseDecision(
          env,
          subject,
          requestId,
          deviceId,
          booleanFromUnknown(body.requestedNewDevice)
        )
      );
    },

    async 'billing-manual-invoice'({ request, env, identity }): Promise<Response> {
      if (!identity) {
        return json(500, {
          error: 'identity-missing',
        });
      }

      const authority = requireVerifiedSupportAuthority(identity);
      if (authority instanceof Response) {
        return authority;
      }
      const subject = authority.providerSubject;

      const body = await readJsonObject<ManualInvoiceRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('manual-invoice', subject, body.requestId);
      const result = buildManualInvoiceResult(subject, requestId, stringOrNull(body.region));
      return executeIdempotentWrite(
        env.BILLING_DO,
        `billing-control:${subject}`,
        {
          requestKey: durableWriteKey('manual-invoice', subject, requestId),
          responseStatus: 202,
          responseBody: {
            ...result,
            queued: true,
          },
          queueMessage: {
            action: 'manual-invoice',
            requestId,
            subject,
            region: result.region,
            actorRole: 'support',
          },
          stateMutation: {
            kind: 'manual-invoice',
            subject,
            requestId,
            region: result.region,
            auditReference: result.auditReference,
            actorRole: 'support',
          },
        },
        env
      );
    },

    async 'stripe-webhook'({ request, env, route }): Promise<Response> {
      const signatureHeader = request.headers.get(signatureHeaderName(new URL(request.url).pathname));
      if (!signatureHeader) {
        return json(401, {
          error: 'authentication-required',
          authState: route.authState,
          missingHeader: 'stripe-signature',
        });
      }

      if (!env.STRIPE_WEBHOOK_SECRET) {
        return json(503, {
          error: 'manual-required',
          authState: route.authState,
          blocker: 'stripe-webhook-secret-missing',
        });
      }

      const body = await request.text();
      if (!(await verifyStripeWebhookSignature(body, signatureHeader, env.STRIPE_WEBHOOK_SECRET))) {
        return json(400, {
          error: 'invalid-stripe-signature',
        });
      }

      return acceptProviderWebhook('stripe', body, route.proofIdFamily, env);
    },

    async 'razorpay-webhook'({ request, env, route }): Promise<Response> {
      if (!env.RAZORPAY_KEY_SECRET) {
        return json(503, {
          error: 'manual-required',
          authState: route.authState,
          blocker: 'razorpay-webhook-secret-missing',
        });
      }

      const signature = request.headers.get('x-razorpay-signature');
      if (!signature) {
        return json(401, {
          error: 'authentication-required',
          authState: route.authState,
          missingHeader: 'x-razorpay-signature',
        });
      }

      const body = await request.text();
      if (!(await verifyHexHmac(body, signature, env.RAZORPAY_KEY_SECRET))) {
        return json(400, {
          error: 'invalid-razorpay-signature',
        });
      }

      return acceptProviderWebhook('razorpay', body, route.proofIdFamily, env);
    },

    async 'paypal-webhook'({ request, env, route }): Promise<Response> {
      if (!env.PAYPAL_CLIENT_SECRET) {
        return json(503, {
          error: 'manual-required',
          authState: route.authState,
          blocker: 'paypal-webhook-secret-missing',
        });
      }

      const transmissionId = request.headers.get('paypal-transmission-id');
      const transmissionSig = request.headers.get('paypal-transmission-sig');
      if (!transmissionId || !transmissionSig) {
        return json(401, {
          error: 'authentication-required',
          authState: route.authState,
          missingHeader: !transmissionId ? 'paypal-transmission-id' : 'paypal-transmission-sig',
        });
      }

      const body = await request.text();
      if (!(await verifyHexHmac(`${transmissionId}.${body}`, transmissionSig, env.PAYPAL_CLIENT_SECRET))) {
        return json(400, {
          error: 'invalid-paypal-signature',
        });
      }

      return acceptProviderWebhook('paypal', body, route.proofIdFamily, env);
    },

    async 'apple-webhook'({ request, env, route }): Promise<Response> {
      if (!env.APPLE_STORE_KEY_REF) {
        return json(503, {
          error: 'manual-required',
          authState: route.authState,
          blocker: 'apple-store-key-ref-missing',
        });
      }

      const authorization = request.headers.get('authorization');
      if (authorization !== `Bearer ${env.APPLE_STORE_KEY_REF}`) {
        return json(400, {
          error: 'invalid-apple-authorization',
        });
      }

      const body = (await request.text()) || '';
      return acceptProviderWebhook('apple', body, route.proofIdFamily, env);
    },

    async 'google-webhook'({ request, env, route }): Promise<Response> {
      if (!env.GOOGLE_PLAY_SERVICE_ACCOUNT_REF) {
        return json(503, {
          error: 'manual-required',
          authState: route.authState,
          blocker: 'google-play-service-account-ref-missing',
        });
      }

      const signature = request.headers.get('x-goog-signature');
      if (!signature) {
        return json(401, {
          error: 'authentication-required',
          authState: route.authState,
          missingHeader: 'x-goog-signature',
        });
      }

      const body = await request.text();
      if (!(await verifyHexHmac(body, signature, env.GOOGLE_PLAY_SERVICE_ACCOUNT_REF))) {
        return json(400, {
          error: 'invalid-google-signature',
        });
      }

      return acceptProviderWebhook('google', body, route.proofIdFamily, env);
    },

    async 'admin-billing-accounts'({ request, env, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadAdminBillingAccounts(env, query);
      const response = BillingSupportAdminAccountsResponseSchema.parse({
        status: 'ok',
        actorRole: verifiedIdentity.role,
        resultCount: results.length,
        manualActionsPending: results.filter((row) => row.manualRequired).length,
        nonClaims: [
          'no-provider-secrets',
          'no-child-activity-custody',
          'no-billing-provider-contact',
          'no-support-backend-upload',
        ],
        results,
      });

      return json(200, response);
    },

    async 'admin-billing-invoices'({ request, env, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadAdminBillingInvoices(env, query);
      const response = BillingSupportAdminInvoicesResponseSchema.parse({
        status: 'ok',
        actorRole: verifiedIdentity.role,
        resultCount: results.length,
        results,
      });

      return json(200, response);
    },

    async 'admin-billing-refunds'({ request, env, identity }): Promise<Response> {
      const body = await readJsonObject<AdminRefundRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('admin-refund', identity?.subject ?? 'admin', body.requestId);
      const result = buildAdminRefundResult(requestId, stringOrNull(body.invoiceId), numberOrNull(body.amountCents));
      if (result.status === 'accepted') {
        const refundSubject = result.invoiceId ? await findBillingInvoiceSubject(env, result.invoiceId) : null;
        return executeIdempotentWrite(
          env.BILLING_DO,
          `billing-control:${refundSubject}`,
          {
            requestKey: durableWriteKey('admin-refund', identity?.subject ?? 'admin', requestId),
            responseStatus: 200,
            responseBody: result,
            queueMessage: {
              action: 'admin-refund',
              requestId,
              invoiceId: result.invoiceId,
              amountCents: result.amountCents,
              actorRole: identity?.role ?? null,
            },
            stateMutation:
              refundSubject &&
              result.invoiceId &&
              result.refundState !== 'manual-review-required' &&
              result.amountCents !== null
                ? {
                    kind: 'admin-refund',
                    subject: refundSubject,
                    requestId,
                    invoiceId: result.invoiceId,
                    refundState: result.refundState,
                    amountCents: result.amountCents,
                    auditReference: result.auditReference,
                    actorRole: identity?.role === 'support' ? 'support' : 'admin',
                  }
                : null,
          },
          env
        );
      }
      return json(200, result);
    },

    async 'admin-billing-disputes'({ request, env, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadAdminBillingDisputes(env, query);
      const response = BillingSupportAdminDisputesResponseSchema.parse({
        status: 'ok',
        actorRole: verifiedIdentity.role,
        resultCount: results.length,
        results,
      });

      return json(200, response);
    },

    async 'admin-billing-referrals'({ request, env, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadAdminBillingReferrals(env, query);
      const response = BillingSupportAdminReferralsResponseSchema.parse({
        status: 'ok',
        actorRole: verifiedIdentity.role,
        resultCount: results.length,
        results,
      });

      return json(200, response);
    },

    async 'admin-billing-reconciliation'({ request, env, identity }): Promise<Response> {
      const body = await readJsonObject<ReconciliationRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('reconciliation', identity?.subject ?? 'internal', body.requestId);
      const summary = buildReconciliationSummary(requestId, true);
      const actorRole = identity?.role === 'support' ? 'support' : identity?.role === 'admin' ? 'admin' : 'system';
      return executeIdempotentWrite(
        env.BILLING_DO,
        `billing-control:${identity?.subject ?? 'internal'}`,
        {
          requestKey: durableWriteKey('reconciliation', identity?.subject ?? 'internal', requestId),
          responseStatus: 202,
          responseBody: summary,
          queueMessage: {
            action: 'reconciliation',
            requestId,
            actorRole: identity?.role ?? null,
          },
          stateMutation: {
            kind: 'reconciliation',
            subject: identity?.subject ?? 'internal',
            requestId,
            auditReference: summary.auditReference,
            actorRole,
          },
        },
        env
      );
    },

    async 'admin-billing-audit'({ request, env, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadBillingAuditEvents(env, query);
      const response = BillingSupportAdminAuditEventsResponseSchema.parse({
        status: 'ok',
        actorRole: verifiedIdentity.role,
        resultCount: results.length,
        results,
      });

      return json(200, response);
    },
  };
}

async function handleRequest(request: Request, env: Env): Promise<Response> {
  const validationErrors = validateEnv(env);
  if (validationErrors.length > 0) {
    return json(500, {
      error: 'environment-validation-failed',
      validationErrors,
    });
  }

  if (request.method === 'OPTIONS') {
    return new Response(null, { status: 204 });
  }

  if (!isAllowedOrigin(request.headers.get('origin'), env)) {
    return json(403, {
      error: 'cors-origin-rejected',
      allowedOrigins: parseAllowedOrigins(env),
    });
  }

  const contentLengthResult = parseContentLengthHeader(request);
  if (!contentLengthResult.ok) {
    return contentLengthResult.response;
  }

  const contentLength = contentLengthResult.value;
  if (contentLength > parseRequestMaxBytes(env)) {
    return json(413, {
      error: 'payload-too-large',
      maxBytes: parseRequestMaxBytes(env),
    });
  }

  if (isRouteKillSwitchEnabled(env) && STATE_CHANGING_METHODS.has(request.method)) {
    return json(503, {
      error: 'billing-route-kill-switch-enabled',
      status: 'manual-required',
    });
  }

  const route = findRoute(new URL(request.url).pathname, request.method);
  if (!route) {
    return json(404, {
      error: 'route-not-found',
    });
  }

  const boundaryViolation = validateAuthBoundaryRoute(route);
  if (boundaryViolation) {
    return json(500, {
      error: 'route-auth-boundary-invalid',
      routeKey: `${route.method} ${route.path}`,
      authState: route.authState,
      auditRule: route.auditRule,
      reason: boundaryViolation,
    });
  }

  const handlers = await routeHandlerMap();
  const handler = handlers[route.handlerKey];

  if (route.authState === 'public') {
    if (!handler) {
      return manualRequiredResponse(route);
    }
    try {
      return await handler({
        request,
        env,
        route,
      });
    } catch (error) {
      if (error instanceof BillingReadModelUnavailableError) {
        return billingReadModelUnavailableResponse(route, error);
      }
      return json(500, {
        error: 'worker-unhandled-error',
        handlerKey: route.handlerKey,
        message: error instanceof Error ? error.message : 'unknown-error',
        requestHeaders: redactHeaders(request.headers),
      });
    }
  }

  const providerVerifier = createFirebaseProviderVerificationPort(env);
  const authResult = await verifyAuthState(route.authState, request, env, providerVerifier);
  if (!authResult.ok) {
    return authResult.response;
  }

  if (!handler) {
    return manualRequiredResponse(route, authResult.identity);
  }

  try {
    return await handler({
      request,
      env,
      route,
      identity: authResult.identity,
    });
  } catch (error) {
    if (error instanceof BillingReadModelUnavailableError) {
      return billingReadModelUnavailableResponse(route, error, authResult.identity);
    }
    return json(500, {
      error: 'worker-unhandled-error',
      handlerKey: route.handlerKey,
      message: error instanceof Error ? error.message : 'unknown-error',
      requestHeaders: redactHeaders(request.headers),
    });
  }
}

class BasePlaceholderDO {
  constructor(
    protected readonly state: DurableObjectState,
    protected readonly env: Env
  ) {}

  async fetch(_request?: Request): Promise<Response> {
    void _request;
    void this.state;
    return json(200, {
      status: 'not-wired',
      durableObject: this.constructor.name,
      missingBindings: getMissingBindings(this.env),
      message: 'Durable Object contracts exist, but stateful runtime behavior is still deferred to later workpacks.',
    });
  }
}

class IdempotentWriteDO extends BasePlaceholderDO {
  private readonly writes = new Map<
    string,
    { requestFingerprint: string | null; responseStatus: number; responseBody: unknown }
  >();

  override async fetch(request: Request): Promise<Response> {
    const url = new URL(request.url);
    if (request.method !== 'POST' || url.pathname !== '/idempotency/execute') {
      return super.fetch();
    }

    const envelope = await readJsonObject<IdempotentWriteEnvelope>(request);
    if (!envelope) {
      return json(400, {
        error: 'invalid-json',
      });
    }

    const requestKey = stringOrNull(envelope.requestKey);
    const requestFingerprint = stringOrNull(envelope.requestFingerprint);
    const responseStatus = numberOrNull(envelope.responseStatus);
    if (requestKey === null || responseStatus === null) {
      return json(400, {
        error: 'invalid-idempotency-envelope',
      });
    }

    const existing = this.writes.get(requestKey);
    if (existing !== undefined) {
      if (
        existing.requestFingerprint !== null &&
        requestFingerprint !== null &&
        existing.requestFingerprint !== requestFingerprint
      ) {
        const conflictResponseStatus = numberOrNull(envelope.conflictResponseStatus) ?? 409;
        const conflictResponseBody = envelope.conflictResponseBody ?? {
          error: 'idempotency-conflict',
        };
        return json(200, {
          replayed: true,
          responseStatus: conflictResponseStatus,
          responseBody: cloneJsonValue(conflictResponseBody),
          queued: false,
        } satisfies IdempotentWriteResult);
      }

      return json(200, {
        replayed: true,
        responseStatus: existing.responseStatus,
        responseBody: cloneJsonValue(existing.responseBody),
        queued: false,
      } satisfies IdempotentWriteResult);
    }

    const queueMessage = isPlainObject(envelope.queueMessage) ? envelope.queueMessage : null;
    const stateMutation = parseBillingStateMutation(envelope.stateMutation);
    if (stateMutation) {
      await applyBillingStateMutation(this.env, stateMutation);
    }
    const queued = queueMessage ? await queueReconciliationEvent(this.env, queueMessage) : false;
    const responseBody = queueMessage ? withQueuedFlag(envelope.responseBody, queued) : envelope.responseBody;
    const stored = {
      requestFingerprint,
      responseStatus,
      responseBody: cloneJsonValue(responseBody),
    };
    this.writes.set(requestKey, stored);

    return json(200, {
      replayed: false,
      responseStatus,
      responseBody,
      queued,
    } satisfies IdempotentWriteResult);
  }
}

export class BillingControlDO extends IdempotentWriteDO {}

export class ReferralControlDO extends IdempotentWriteDO {}

export class EntitlementSnapshotDO extends BasePlaceholderDO {}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    return withCors(await handleRequest(request, env), request, env);
  },

  async scheduled(): Promise<void> {
    return;
  },
};

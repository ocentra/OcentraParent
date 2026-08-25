import type { DurableObjectState, MessageBatch } from '@cloudflare/workers-types';
import { NonNegativeBillingCountSchema } from '@ocentra-parent/schema-domain/billing-entitlement-values';
import {
  appendBillingAuditEventAtOwner,
  applyBillingStateMutation,
  BILLING_AUDIT_APPEND_PATH,
  BillingReadModelUnavailableError,
  buildBillingReferralInviteResultFromD1,
  buildManualInvoiceResultFromD1,
  buildReconciliationSummaryFromD1,
  drainPendingBillingMutationOutbox,
  loadAdminBillingAccounts,
  loadAdminBillingDisputes,
  loadAdminBillingInvoices,
  loadAdminBillingReferrals,
  loadBillingAuditEvents,
  loadBillingEntitlementSnapshot,
  loadBillingInvoices,
  loadBillingLicenseDecision,
  loadBillingProviderEventReceipt,
  loadBillingProviderEventCursor,
  loadBillingReferralSummary,
  loadBillingStatusSummary,
  loadLocalSeedSummary,
  loadPricingPlans,
  isIgnoredProviderWebhookEvent,
  markBillingProviderEventQueue,
  registerBillingProviderEvent,
  type BillingStateMutation,
  type ProviderEventReceipt,
} from './billing-binding-read-model.js';
import {
  resolveProviderBillingAuthority,
  type ProviderBillingReferenceHints,
  type ProviderBillingAuthority,
} from './storage/account-identity-billing-store.js';
import { signatureHeaderName, verifyAuthState, type VerifiedIdentity } from './auth/verifier.js';
import { PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS, verifyStripeWebhookSignature } from './auth/provider-webhook.js';
import { createFirebaseProviderVerificationPort } from './providers/firebase-auth.js';
import { validateAuthBoundaryRoute } from './auth/model.js';
import {
  isVerifiedAccountIdentityAuthorityCapability,
  type VerifiedAccountIdentityAuthorityCapability,
} from './storage/account-identity-authority-store.js';
import {
  getMissingBindings,
  isLocalFixtureEnvironment,
  isRouteKillSwitchEnabled,
  parseAllowedOrigins,
  parseRequestMaxBytes,
  resolveAuthAdapterMode,
  validateEnv,
  type Env,
} from './env.js';
import {
  findRoute,
  routeContractReadiness,
  routeRequestModel,
  routeResponseModel,
  ROUTE_HANDLER_KEYS,
  ROUTE_MANIFEST,
  type RouteHandlerMap,
  type RouteManifestEntry,
} from './routes.js';
import { redactHeaders } from './security/redaction.js';
import {
  loginBrowserSession,
  logoutBrowserSession,
  refreshBrowserSession,
  revokeBrowserSessions,
} from './auth/browser-session-routes.js';
import { browserSessionCookieNames, readCookie } from './storage/account-browser-session-codec.js';
import { createBrowserSessionStore } from './storage/account-browser-session-store.js';

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

/** Kept as a compatibility export; its contents are derived from the manifest. */
export const IMPLEMENTED_HANDLER_KEYS = ROUTE_HANDLER_KEYS;

type HandlerContext = {
  request: Request;
  env: Env;
  route: RouteManifestEntry;
  identity?: VerifiedIdentity;
};

type RouteHandler = (context: HandlerContext) => Promise<Response>;
type ReferralInviteRequestBody = {
  requestId?: unknown;
  invitee?: unknown;
};
type LicenseCheckRequestBody = {
  requestId?: unknown;
  requestedNewDevice?: unknown;
};
type ManualInvoiceRequestBody = {
  requestId?: unknown;
  region?: unknown;
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
type DurableIdempotencyRecord = {
  state: 'pending' | 'completed' | 'manual-required';
  requestFingerprint: string;
  responseStatus: number;
  responseBody: unknown;
  stateVersion: number;
  attemptCount: number;
  leaseToken: string | null;
  leaseExpiresAt: string | null;
  retryAt: string | null;
  lastError: string | null;
};
type QueueFailureReason =
  | 'reconciliation-queue-missing'
  | 'reconciliation-queue-send-failed'
  | 'queue-consumer-invalid-message'
  | 'queue-consumer-manual-required';

const IDEMPOTENCY_LEASE_MS = 60_000;
const IDEMPOTENCY_RETRY_BASE_MS = 1_000;
const IDEMPOTENCY_RETRY_MAX_MS = 60_000;
const IDEMPOTENCY_MAX_ATTEMPTS = 5;

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
    identity.authority.role !== 'support-admin' ||
    identity.authority.supportScope === null ||
    identity.authority.supportIssuer === null ||
    identity.authority.supportAuditIdentity === null
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
  if (authority.supportScope === null || authority.supportIssuer === null || authority.supportAuditIdentity === null) {
    return json(503, {
      status: 'manual-required',
      blocker: 'support-receipt-provenance-missing',
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
    'authorization,content-type,cookie,stripe-signature,paypal-transmission-id,paypal-transmission-sig,x-razorpay-signature,x-goog-signature,x-ocentra-internal-call,x-ocentra-internal-secret,x-ocentra-csrf,sec-fetch-site'
  );
  headers.set('access-control-allow-credentials', 'true');
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

function manualRequiredResponse(
  route: RouteManifestEntry,
  identity?: VerifiedIdentity,
  contractReadiness = routeContractReadiness(route)
): Response {
  return json(501, {
    status: 'manual-required',
    handlerKey: route.handlerKey,
    authState: route.authState,
    requestModel: routeRequestModel(route),
    responseModel: routeResponseModel(route),
    contractState: contractReadiness.ready ? 'ready' : 'manual-required',
    contractSide: contractReadiness.ready ? null : contractReadiness.side,
    contractBlocker: contractReadiness.ready ? 'handler-unavailable' : contractReadiness.blocker,
    proofIdFamily: route.proofIdFamily,
    actorRole: identity?.role ?? null,
    message: 'Route dispatch is disabled until its request, response, and owner execution contracts are bound.',
  });
}

function parseManifestResponse(route: RouteManifestEntry, value: unknown): unknown | Response {
  const responseContract = route.contract.response;
  if (responseContract.state !== 'bound') return manualRequiredResponse(route);

  try {
    return responseContract.descriptor.codec.parse(value);
  } catch {
    return json(500, {
      error: 'route-response-contract-rejected',
      handlerKey: route.handlerKey,
      responseModel: routeResponseModel(route),
    });
  }
}

async function validateManifestRequest(route: RouteManifestEntry, request: Request): Promise<Response | null> {
  const requestContract = route.contract.request;
  if (requestContract.state === 'unbound') return manualRequiredResponse(route);
  if (requestContract.state === 'none') {
    if (request.body !== null || new URL(request.url).search.length > 0) {
      return json(400, {
        error: 'route-request-contract-rejected',
        handlerKey: route.handlerKey,
        requestModel: routeRequestModel(route),
        reason: 'request-input-not-allowed',
      });
    }
    return null;
  }

  let payload: unknown;
  try {
    payload = await request.clone().json();
    requestContract.descriptor.codec.parse(payload);
  } catch {
    return json(400, {
      error: 'route-request-contract-rejected',
      handlerKey: route.handlerKey,
      requestModel: routeRequestModel(route),
      reason: 'request-codec-rejected',
    });
  }
  return null;
}

async function validateManifestResponse(route: RouteManifestEntry, response: Response): Promise<Response> {
  if (response.status < 200 || response.status >= 300) return response;
  const responseContract = route.contract.response;
  if (responseContract.state !== 'bound') return manualRequiredResponse(route);

  try {
    responseContract.descriptor.codec.parse(await response.clone().json());
    return response;
  } catch {
    return json(500, {
      error: 'route-response-contract-rejected',
      handlerKey: route.handlerKey,
      responseModel: routeResponseModel(route),
    });
  }
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

function parseRefundAmount(value: unknown): { valid: true; value: number | null } | { valid: false; value: null } {
  if (value === undefined || value === null) {
    return { valid: true, value: null };
  }
  try {
    return { valid: true, value: NonNegativeBillingCountSchema.parse(value) };
  } catch (_error) {
    return { valid: false, value: null };
  }
}

function booleanFromUnknown(value: unknown): boolean {
  return value === true;
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
    const actorSubject = stringOrNull(value.actorSubject);
    const currency = stringOrNull(value.currency);
    const auditReference = stringOrNull(value.auditReference);
    const refundState = stringOrNull(value.refundState);
    const parsedAmount = parseRefundAmount(value.amountCents);
    const amountCents = parsedAmount.value;
    const actorRole = stringOrNull(value.actorRole);
    if (
      !requestId ||
      !invoiceId ||
      !actorSubject ||
      !currency ||
      !auditReference ||
      !parsedAmount.valid ||
      amountCents === null ||
      (refundState !== 'refund-requested' && refundState !== 'refund-settled') ||
      (actorRole !== 'support' && actorRole !== 'admin')
    ) {
      return null;
    }
    return {
      kind,
      subject,
      actorSubject,
      requestId,
      invoiceId,
      currency,
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
    const providerOccurredAt = value.providerOccurredAt === null ? null : stringOrNull(value.providerOccurredAt);
    const providerSequence = value.providerSequence === null ? null : numberOrNull(value.providerSequence);
    const providerCursorExpectedVersion = numberOrNull(value.providerCursorExpectedVersion);
    if (
      !provider ||
      !eventId ||
      !eventType ||
      (value.providerOccurredAt !== null && providerOccurredAt === null) ||
      (value.providerSequence !== null && providerSequence === null) ||
      providerCursorExpectedVersion === null ||
      !Number.isInteger(providerCursorExpectedVersion)
    ) {
      return null;
    }
    return {
      kind,
      provider,
      subject,
      eventId,
      eventType,
      providerOccurredAt,
      providerSequence,
      providerCursorExpectedVersion,
      disputeId: stringOrNull(value.disputeId),
      invoiceId: stringOrNull(value.invoiceId),
      parentAccountRef: stringOrNull(value.parentAccountRef),
      familyRef: stringOrNull(value.familyRef),
    };
  }

  return null;
}

function durableWriteKey(action: string, subject: string, requestId: string): string {
  return `${action}:${subject}:${requestId}`;
}

function canonicalRequestFingerprint(
  action: string,
  subject: string | null,
  requestId: string,
  details: Readonly<Record<string, unknown>> = {}
): string {
  return JSON.stringify({ ...details, action, subject, requestId });
}

function webhookIdempotencyKey(provider: string, eventId: string, billingSubject: string | null): string {
  return durableWriteKey('provider-webhook', billingSubject ?? 'unresolved', `${provider}:${eventId}`);
}

function webhookRequestFingerprint(
  provider: string,
  body: string,
  event: { eventId: string; eventType: string },
  authority: ProviderBillingAuthority | null
): string {
  return canonicalRequestFingerprint('provider-webhook', authority?.billingSubject ?? null, event.eventId, {
    provider,
    eventType: event.eventType,
    body,
    accountId: authority?.accountId ?? null,
    providerCustomerId: authority?.providerCustomerId ?? null,
    providerSubscriptionId: authority?.providerSubscriptionId ?? null,
    providerInvoiceId: authority?.providerInvoiceId ?? null,
    billingInvoiceId: authority?.billingInvoiceId ?? null,
  });
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
    requestFingerprint: string;
    responseStatus: number;
    responseBody: unknown;
    queueMessage: Record<string, unknown> | null;
    stateMutation?: BillingStateMutation | null;
    conflictResponseStatus?: number;
    conflictResponseBody?: unknown;
  },
  env: Env
): Promise<Response> {
  void env;
  if (!namespace) {
    throw new BillingReadModelUnavailableError('billing-control-do-binding-missing');
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
  if (!doResponse.ok) {
    throw new BillingReadModelUnavailableError('billing-control-do-unavailable');
  }
  const result = (await doResponse.json()) as IdempotentWriteResult;
  if (
    typeof result.responseStatus !== 'number' ||
    typeof result.replayed !== 'boolean' ||
    typeof result.queued !== 'boolean'
  ) {
    throw new BillingReadModelUnavailableError('billing-control-do-response-invalid');
  }
  return json(result.responseStatus, result.responseBody);
}

function billingActorRoleForAuthority(authority: VerifiedAuthority): 'parent' | 'guardian' {
  return authority.role === 'parent-owner' ? 'parent' : 'guardian';
}

async function requireInteractiveRequestBoundary(
  request: Request,
  env: Env,
  identity: VerifiedIdentity,
  requestId: string
): Promise<Response | null> {
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

  const fetchSite = request.headers.get('sec-fetch-site');
  const cookieNames = browserSessionCookieNames(!isLocalFixtureEnvironment(env));
  const sessionToken = readCookie(request, cookieNames.session);
  if (sessionToken !== null && fetchSite !== 'same-origin' && fetchSite !== 'same-site') {
    return json(403, { error: 'fetch-metadata-validation-failed' });
  }

  if (sessionToken !== null) {
    const csrfValid = await createBrowserSessionStore(env.ACCOUNT_IDENTITY_D1).verifyCsrf(
      sessionToken,
      request.headers.get(INTERACTIVE_CSRF_HEADER)
    );
    if (!csrfValid) return json(403, { error: 'csrf-validation-failed' });
    return null;
  }

  const expectedCsrfToken = interactiveCsrfToken(env);
  if (!expectedCsrfToken || request.headers.get(INTERACTIVE_CSRF_HEADER) !== expectedCsrfToken) {
    return json(403, {
      error: 'csrf-validation-failed',
    });
  }

  return null;
}

function requireServerVerifiedAbuseGate(
  request: Request,
  env: Env,
  identity: VerifiedIdentity,
  route: RouteManifestEntry
): Response | null {
  const cookieNames = browserSessionCookieNames(!isLocalFixtureEnvironment(env));
  const sessionToken = readCookie(request, cookieNames.session);
  if (
    identity.state !== 'parent-session-required' ||
    sessionToken === null ||
    !isVerifiedAccountIdentityAuthorityCapability(identity.authority)
  ) {
    return manualRequiredResponse(route, identity);
  }

  return null;
}

function providerEventDetails(
  provider: string,
  payload: unknown,
  fallbackId: string
): { eventId: string; eventType: string } {
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

async function providerWebhookBodyId(provider: string, body: string): Promise<string> {
  const digest = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(body));
  const hex = Array.from(new Uint8Array(digest), (value) => value.toString(16).padStart(2, '0')).join('');
  return `${provider}_evt_body_${hex}`;
}

function providerWebhookReferences(payload: unknown, eventType: string): ProviderBillingReferenceHints {
  if (!isPlainObject(payload)) {
    return { customerId: null, subscriptionId: null, invoiceId: null };
  }
  const data = isPlainObject(payload.data) ? payload.data : null;
  const object = isPlainObject(data?.object) ? data.object : payload;
  const normalizedEventType = eventType.toLowerCase();
  const objectId = stringOrNull(object.id);
  const customerId =
    (normalizedEventType.includes('customer') && !normalizedEventType.includes('subscription') ? objectId : null) ??
    stringOrNull(object.customer) ??
    stringOrNull(object.customer_id) ??
    stringOrNull(payload.customer);
  const subscriptionId =
    (normalizedEventType.includes('subscription') ? objectId : null) ??
    stringOrNull(object.subscription) ??
    stringOrNull(object.subscription_id) ??
    stringOrNull(payload.subscription);
  const invoiceId =
    (normalizedEventType.includes('invoice') ? objectId : null) ??
    stringOrNull(object.invoice) ??
    stringOrNull(object.invoice_id) ??
    stringOrNull(payload.invoice) ??
    stringOrNull(payload.invoice_id);
  return { customerId, subscriptionId, invoiceId };
}

function providerOccurrenceTimestamp(value: unknown): string | null {
  const numericValue =
    typeof value === 'number'
      ? value
      : typeof value === 'string' && /^\d+(?:\.\d+)?$/.test(value.trim())
        ? Number(value.trim())
        : null;
  if (numericValue !== null && Number.isFinite(numericValue) && numericValue >= 0) {
    const milliseconds = numericValue < 1_000_000_000_000 ? numericValue * 1000 : numericValue;
    const timestamp = new Date(milliseconds);
    return Number.isFinite(timestamp.getTime()) ? timestamp.toISOString() : null;
  }
  if (typeof value === 'string' && value.trim().length > 0) {
    const timestamp = new Date(value.trim());
    return Number.isFinite(timestamp.getTime()) ? timestamp.toISOString() : null;
  }
  return null;
}

function providerWebhookOccurrence(payload: unknown): {
  occurredAt: string | null;
  sequence: number | null;
  valid: boolean;
} {
  if (!isPlainObject(payload)) {
    return { occurredAt: null, sequence: null, valid: false };
  }
  const data = isPlainObject(payload.data) ? payload.data : null;
  const object = isPlainObject(data?.object) ? data.object : payload;
  const occurredAt =
    [
      payload.occurredAt,
      payload.occurred_at,
      payload.createdAt,
      payload.created_at,
      payload.timestamp,
      payload.created,
      object.occurredAt,
      object.occurred_at,
      object.createdAt,
      object.created_at,
      object.timestamp,
      object.created,
    ]
      .map(providerOccurrenceTimestamp)
      .find((value): value is string => value !== null) ?? null;
  const sequenceCandidate = [
    payload.sequence,
    payload.sequence_number,
    payload.event_sequence,
    payload.version,
    object.sequence,
    object.sequence_number,
    object.event_sequence,
    object.version,
  ].find((value) => value !== null && value !== undefined);
  let sequence: number | null = null;
  let valid = true;
  if (sequenceCandidate !== undefined) {
    try {
      sequence = NonNegativeBillingCountSchema.parse(sequenceCandidate);
    } catch (_error) {
      valid = false;
    }
  }
  return { occurredAt, sequence, valid };
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

  const event = providerEventDetails(provider, payload, await providerWebhookBodyId(provider, body));
  const providerReferences = providerWebhookReferences(payload, event.eventType);
  const occurrence = providerWebhookOccurrence(payload);
  const authority = await resolveProviderBillingAuthority(env.ACCOUNT_IDENTITY_D1, provider, providerReferences);
  const trustedAuthority = authority.status === 'trusted' ? authority.authority : null;
  const subject = trustedAuthority?.billingSubject ?? null;
  const invoiceId = trustedAuthority?.billingInvoiceId ?? null;
  const disputeId = providerWebhookDisputeId(payload, event.eventType, `dispute-${provider}-${event.eventId}`);
  const requestFingerprint = webhookRequestFingerprint(provider, body, event, trustedAuthority);
  const processingState =
    trustedAuthority !== null && occurrence.valid && (occurrence.occurredAt !== null || occurrence.sequence !== null)
      ? isIgnoredProviderWebhookEvent(event.eventType)
        ? 'ignored'
        : 'received'
      : 'manual-required';
  const registration = await registerBillingProviderEvent(env, {
    provider,
    eventId: event.eventId,
    eventFingerprint: requestFingerprint,
    eventType: event.eventType,
    providerOccurredAt: occurrence.occurredAt,
    providerSequence: occurrence.sequence,
    accountId: trustedAuthority?.accountId ?? null,
    providerCustomerId: trustedAuthority?.providerCustomerId ?? providerReferences.customerId,
    providerSubscriptionId: trustedAuthority?.providerSubscriptionId ?? providerReferences.subscriptionId,
    providerInvoiceId: trustedAuthority?.providerInvoiceId ?? providerReferences.invoiceId,
    billingSubject: subject,
    parentAccountRef: trustedAuthority?.parentAccountRef ?? null,
    familyRef: trustedAuthority?.familyRef ?? null,
    billingInvoiceId: invoiceId,
    processingState,
  });
  if (registration.status === 'conflict') {
    return json(409, conflictingWebhookResponse(provider, proofIdFamily, event));
  }
  const receipt = registration.receipt;
  if (
    registration.status === 'replay' &&
    (receipt.queueState === 'queued' ||
      receipt.queueState === 'delivered' ||
      receipt.queueState === 'manual-required' ||
      receipt.queueState === 'dead-letter')
  ) {
    return json(202, {
      ...acceptedWebhookResponse(provider, proofIdFamily, event),
      queued: receipt.queueState === 'queued' || receipt.queueState === 'delivered',
    });
  }
  if (!env.BILLING_DO) {
    await markBillingProviderEventQueue(
      env,
      receipt.provider,
      receipt.eventId,
      'manual-required',
      receipt.processingState === 'ignored' || receipt.processingState === 'applied'
        ? receipt.processingState
        : 'manual-required',
      'billing-control-do-binding-missing',
      receipt
    );
    throw new BillingReadModelUnavailableError('billing-control-do-binding-missing');
  }
  const queuePayload = {
    action: 'provider-webhook',
    provider,
    ...event,
    disputeId,
    receivedAt: new Date().toISOString(),
  } satisfies Record<string, unknown>;
  let response: Response;
  try {
    response = await executeIdempotentWrite(
      env.BILLING_DO,
      subject ? `billing-control:${subject}` : `billing-control:webhook:${provider}`,
      {
        requestKey: webhookIdempotencyKey(provider, event.eventId, subject),
        requestFingerprint,
        responseStatus: 202,
        responseBody: acceptedWebhookResponse(provider, proofIdFamily, event),
        queueMessage: queuePayload,
        stateMutation: null,
        conflictResponseStatus: 409,
        conflictResponseBody: conflictingWebhookResponse(provider, proofIdFamily, event),
      },
      env
    );
  } catch (error) {
    if (error instanceof BillingReadModelUnavailableError) {
      const current = await loadBillingProviderEventReceipt(env, receipt.provider, receipt.eventId);
      if (current?.queueState === 'pending') {
        await markBillingProviderEventQueue(
          env,
          current.provider,
          current.eventId,
          'manual-required',
          current.processingState === 'ignored' || current.processingState === 'applied'
            ? current.processingState
            : 'manual-required',
          error.message,
          current
        );
      }
    }
    throw error;
  }
  const responseBody = await response.json();
  const queued = isPlainObject(responseBody) && responseBody.queued === true;
  await recordProviderWebhookQueueOutcome(
    env,
    receipt,
    queued,
    receipt.billingSubject !== null && receipt.processingState !== 'manual-required'
  );
  return json(response.status, responseBody);
}

async function recordProviderWebhookQueueOutcome(
  env: Env,
  receipt: ProviderEventReceipt,
  queued: boolean,
  hasTrustedAuthority: boolean
): Promise<boolean> {
  const failedQueueState = 'manual-required' as const;
  const failedProcessingState =
    receipt.processingState === 'ignored' || receipt.processingState === 'applied'
      ? receipt.processingState
      : failedQueueState;
  const queuedProcessingState =
    receipt.processingState === 'ignored'
      ? 'ignored'
      : receipt.processingState === 'manual-required'
        ? 'manual-required'
        : hasTrustedAuthority
          ? 'queued'
          : 'manual-required';
  try {
    const current = await markBillingProviderEventQueue(
      env,
      receipt.provider,
      receipt.eventId,
      queued ? 'queued' : failedQueueState,
      queued ? queuedProcessingState : failedProcessingState,
      queued ? null : 'provider-event-queue-delivery-failed',
      receipt
    );
    return current.queueState === 'queued' || current.queueState === 'delivered';
  } catch (error) {
    if (!(error instanceof BillingReadModelUnavailableError)) {
      throw error;
    }
    const current = await loadBillingProviderEventReceipt(env, receipt.provider, receipt.eventId);
    if (!current) {
      throw error;
    }
    if (queued && (current.queueState === 'queued' || current.queueState === 'delivered')) {
      return true;
    }
    if (!queued && (current.queueState === 'manual-required' || current.queueState === 'dead-letter')) {
      return false;
    }
    throw error;
  }
}

async function markProviderWebhookDelivered(
  env: Env,
  receipt: ProviderEventReceipt,
  processingState: Extract<ProviderEventReceipt['processingState'], 'applied' | 'ignored'>
): Promise<void> {
  try {
    await markBillingProviderEventQueue(
      env,
      receipt.provider,
      receipt.eventId,
      'delivered',
      processingState,
      null,
      receipt
    );
  } catch (error) {
    if (!(error instanceof BillingReadModelUnavailableError)) {
      throw error;
    }
    const current = await loadBillingProviderEventReceipt(env, receipt.provider, receipt.eventId);
    if (current?.queueState === 'delivered' && current.processingState === processingState) {
      return;
    }
    throw error;
  }
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

async function sendBillingDeadLetter(
  env: Env,
  payload: Record<string, unknown>,
  reason: QueueFailureReason,
  error: unknown
): Promise<boolean> {
  if (!env.BILLING_DEAD_LETTER_QUEUE) {
    return false;
  }
  try {
    await env.BILLING_DEAD_LETTER_QUEUE.send(deadLetterPayload(payload, reason, error));
    return true;
  } catch {
    return false;
  }
}

async function routeHandlerMap(): Promise<RouteHandlerMap<RouteHandler>> {
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

    async 'account-session-login'({ request, env }): Promise<Response> {
      return loginBrowserSession(request, env, createFirebaseProviderVerificationPort(env));
    },

    async 'account-session-refresh'({ request, env, identity }): Promise<Response> {
      if (!identity) return json(500, { error: 'identity-missing' });
      return refreshBrowserSession(request, env, identity);
    },

    async 'account-session-logout'({ request, env, identity }): Promise<Response> {
      if (!identity) return json(500, { error: 'identity-missing' });
      return logoutBrowserSession(request, env, identity);
    },

    async 'account-session-revoke'({ request, env, identity }): Promise<Response> {
      if (!identity) return json(500, { error: 'identity-missing' });
      return revokeBrowserSessions(request, env, identity);
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

    async 'billing-checkout'({ route, identity }): Promise<Response> {
      return manualRequiredResponse(route, identity);
    },

    async 'billing-portal'({ route, identity }): Promise<Response> {
      return manualRequiredResponse(route, identity);
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

    async 'billing-change-plan'({ route, identity }): Promise<Response> {
      return manualRequiredResponse(route, identity);
    },

    async 'billing-cancel'({ route, identity }): Promise<Response> {
      return manualRequiredResponse(route, identity);
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

    async 'billing-referral-invite'({ request, env, route, identity }): Promise<Response> {
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
      const boundaryFailure = await requireInteractiveRequestBoundary(request, env, identity, requestId);
      if (boundaryFailure) {
        return boundaryFailure;
      }

      const abuseGateFailure = requireServerVerifiedAbuseGate(request, env, identity, route);
      if (abuseGateFailure) {
        return abuseGateFailure;
      }

      const invitee = stringOrNull(body.invitee)?.trim();
      if (!invitee) {
        return json(400, {
          error: 'invitee-required',
        });
      }
      const result = await buildBillingReferralInviteResultFromD1(env, subject, requestId, invitee);
      const contractResult = parseManifestResponse(route, result);
      if (contractResult instanceof Response) return contractResult;
      if (result.status === 'accepted') {
        const invitedIdentifier = stringOrNull(body.invitee)?.trim().toLowerCase();
        const actorRole = billingActorRoleForAuthority(authority);
        return executeIdempotentWrite(
          env.BILLING_DO,
          `billing-control:${subject}`,
          {
            requestKey: durableWriteKey('referral-invite', subject, requestId),
            requestFingerprint: canonicalRequestFingerprint('referral-invite', subject, requestId, {
              invitedIdentifier,
              referralCode: result.referralCode,
            }),
            responseStatus: 200,
            responseBody: contractResult,
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
      return json(200, contractResult);
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

    async 'billing-license-check'({ request, env, route, identity }): Promise<Response> {
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
        await loadBillingLicenseDecision(env, subject, requestId, deviceId, booleanFromUnknown(body.requestedNewDevice))
      );
    },

    async 'billing-manual-invoice'({ request, env, route, identity }): Promise<Response> {
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
      const region = stringOrNull(body.region)?.trim();
      if (!region) {
        return json(400, {
          error: 'region-required',
        });
      }
      const result = await buildManualInvoiceResultFromD1(env, subject, requestId, region);
      return executeIdempotentWrite(
        env.BILLING_DO,
        `billing-control:${subject}`,
        {
          requestKey: durableWriteKey('manual-invoice', subject, requestId),
          requestFingerprint: canonicalRequestFingerprint('manual-invoice', subject, requestId, {
            region: result.region,
          }),
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
      if (
        !(await verifyStripeWebhookSignature(
          body,
          signatureHeader,
          env.STRIPE_WEBHOOK_SECRET,
          env.STRIPE_WEBHOOK_TOLERANCE_SECONDS
        ))
      ) {
        return json(400, {
          error: 'invalid-stripe-signature',
        });
      }

      return acceptProviderWebhook('stripe', body, route.proofIdFamily, env);
    },

    async 'razorpay-webhook'({ request, env, route }): Promise<Response> {
      void request;
      void env;
      return json(503, {
        error: 'manual-required',
        authState: route.authState,
        blocker: PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.razorpayVerifierUnavailable,
      });
    },

    async 'paypal-webhook'({ request, env, route }): Promise<Response> {
      void request;
      void env;
      return json(503, {
        error: 'manual-required',
        authState: route.authState,
        blocker: PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.paypalVerifierUnavailable,
      });
    },

    async 'apple-webhook'({ request, env, route }): Promise<Response> {
      void request;
      void env;
      return json(503, {
        error: 'manual-required',
        authState: route.authState,
        blocker: PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.appleVerifierUnavailable,
      });
    },

    async 'google-webhook'({ request, env, route }): Promise<Response> {
      void request;
      void env;
      return json(503, {
        error: 'manual-required',
        authState: route.authState,
        blocker: PROVIDER_WEBHOOK_UNAVAILABLE_BLOCKERS.googleVerifierUnavailable,
      });
    },

    async 'admin-billing-accounts'({ request, env, route, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadAdminBillingAccounts(env, query);
      const response = parseManifestResponse(route, {
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
      if (response instanceof Response) return response;

      return json(200, response);
    },

    async 'admin-billing-invoices'({ request, env, route, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadAdminBillingInvoices(env, query);
      const response = parseManifestResponse(route, {
        status: 'ok',
        actorRole: verifiedIdentity.role,
        resultCount: results.length,
        results,
      });
      if (response instanceof Response) return response;

      return json(200, response);
    },

    async 'admin-billing-refunds'({ route, identity }): Promise<Response> {
      return manualRequiredResponse(route, identity);
    },

    async 'admin-billing-disputes'({ request, env, route, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadAdminBillingDisputes(env, query);
      const response = parseManifestResponse(route, {
        status: 'ok',
        actorRole: verifiedIdentity.role,
        resultCount: results.length,
        results,
      });
      if (response instanceof Response) return response;

      return json(200, response);
    },

    async 'admin-billing-referrals'({ request, env, route, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadAdminBillingReferrals(env, query);
      const response = parseManifestResponse(route, {
        status: 'ok',
        actorRole: verifiedIdentity.role,
        resultCount: results.length,
        results,
      });
      if (response instanceof Response) return response;

      return json(200, response);
    },

    async 'admin-billing-reconciliation'({ request, env, route, identity }): Promise<Response> {
      const body = await readJsonObject<ReconciliationRequestBody>(request);
      if (!body) {
        return json(400, {
          error: 'invalid-json',
        });
      }

      const requestId = requestIdFor('reconciliation', identity?.subject ?? 'internal', body.requestId);
      const summary = await buildReconciliationSummaryFromD1(env, requestId);
      const contractSummary = parseManifestResponse(route, summary);
      if (contractSummary instanceof Response) return contractSummary;
      const actorRole = identity?.role === 'support' ? 'support' : identity?.role === 'admin' ? 'admin' : 'system';
      return executeIdempotentWrite(
        env.BILLING_DO,
        `billing-control:${identity?.subject ?? 'internal'}`,
        {
          requestKey: durableWriteKey('reconciliation', identity?.subject ?? 'internal', requestId),
          requestFingerprint: canonicalRequestFingerprint(
            'reconciliation',
            identity?.subject ?? 'internal',
            requestId,
            { actorRole }
          ),
          responseStatus: 202,
          responseBody: contractSummary,
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

    async 'admin-billing-audit'({ request, env, route, identity }): Promise<Response> {
      const verifiedIdentity = requireSupportAdminReadIdentity(identity);
      const query = new URL(request.url).searchParams.get('q');
      const results = await loadBillingAuditEvents(env, query);
      const response = parseManifestResponse(route, {
        status: 'ok',
        actorRole: verifiedIdentity.role,
        resultCount: results.length,
        results,
      });
      if (response instanceof Response) return response;

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

  const route = findRoute(new URL(request.url).pathname, request.method);
  if (!route) {
    return json(404, {
      error: 'route-not-found',
    });
  }

  if (isRouteKillSwitchEnabled(env) && STATE_CHANGING_METHODS.has(request.method) && route.routeGroup !== 'session') {
    return json(503, {
      error: 'billing-route-kill-switch-enabled',
      status: 'manual-required',
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

  if (route.authState === 'public') {
    const contractReadiness = routeContractReadiness(route);
    if (!contractReadiness.ready) {
      return manualRequiredResponse(route, undefined, contractReadiness);
    }
    const requestContractFailure = await validateManifestRequest(route, request);
    if (requestContractFailure) return requestContractFailure;

    const handlers = await routeHandlerMap();
    const handler = handlers[route.handlerKey];
    if (!handler) {
      return manualRequiredResponse(route, undefined, contractReadiness);
    }
    try {
      return validateManifestResponse(
        route,
        await handler({
          request,
          env,
          route,
        })
      );
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

  const contractReadiness = routeContractReadiness(route);
  if (!contractReadiness.ready) {
    return manualRequiredResponse(route, authResult.identity, contractReadiness);
  }
  const requestContractFailure = await validateManifestRequest(route, request);
  if (requestContractFailure) return requestContractFailure;

  const handlers = await routeHandlerMap();
  const handler = handlers[route.handlerKey];
  if (!handler) {
    return manualRequiredResponse(route, authResult.identity, contractReadiness);
  }

  try {
    return validateManifestResponse(
      route,
      await handler({
        request,
        env,
        route,
        identity: authResult.identity,
      })
    );
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

function storedIdempotencyCounter(value: unknown, fallback: number | null): number | null {
  if (value === undefined) {
    return fallback;
  }
  try {
    return NonNegativeBillingCountSchema.parse(value);
  } catch (_error) {
    return null;
  }
}

function storedIdempotencyTimestamp(value: unknown, fallback: string | null): string | null {
  if (value === undefined) {
    return fallback;
  }
  if (value === null) {
    return null;
  }
  return typeof value === 'string' && Number.isFinite(Date.parse(value)) ? value : null;
}

function normalizeDurableIdempotencyRecord(value: unknown): DurableIdempotencyRecord | null {
  if (!isPlainObject(value)) {
    return null;
  }
  const state = stringOrNull(value.state);
  const requestFingerprint = stringOrNull(value.requestFingerprint);
  const responseStatus = numberOrNull(value.responseStatus);
  const stateVersion = storedIdempotencyCounter(value.stateVersion, 0);
  const attemptCount = storedIdempotencyCounter(value.attemptCount, 0);
  const leaseToken =
    value.leaseToken === undefined || value.leaseToken === null ? null : stringOrNull(value.leaseToken);
  const leaseExpiresAt = storedIdempotencyTimestamp(value.leaseExpiresAt, null);
  const retryAt = storedIdempotencyTimestamp(value.retryAt, null);
  const lastError = value.lastError === undefined || value.lastError === null ? null : stringOrNull(value.lastError);
  if (
    (state !== 'pending' && state !== 'completed' && state !== 'manual-required') ||
    requestFingerprint === null ||
    responseStatus === null ||
    !Number.isInteger(responseStatus) ||
    stateVersion === null ||
    attemptCount === null ||
    (value.leaseToken !== undefined && value.leaseToken !== null && leaseToken === null) ||
    (value.lastError !== undefined && value.lastError !== null && lastError === null) ||
    (value.leaseExpiresAt !== undefined && value.leaseExpiresAt !== null && leaseExpiresAt === null) ||
    (value.retryAt !== undefined && value.retryAt !== null && retryAt === null) ||
    !Object.prototype.hasOwnProperty.call(value, 'responseBody') ||
    (leaseToken === null && leaseExpiresAt !== null) ||
    (leaseToken !== null && leaseExpiresAt === null) ||
    (state !== 'pending' && (leaseToken !== null || leaseExpiresAt !== null || retryAt !== null))
  ) {
    return null;
  }
  return {
    state,
    requestFingerprint,
    responseStatus,
    responseBody: cloneJsonValue(value.responseBody),
    stateVersion,
    attemptCount,
    leaseToken,
    leaseExpiresAt,
    retryAt,
    lastError,
  };
}

function idempotencyRetryDelayMs(attemptCount: number): number {
  return Math.min(
    IDEMPOTENCY_RETRY_MAX_MS,
    IDEMPOTENCY_RETRY_BASE_MS * 2 ** Math.max(0, Math.min(attemptCount - 1, 6))
  );
}

function idempotencyRetryResponse(blocker: string, retryAt: string | null = null): Response {
  return json(503, {
    status: 'manual-required',
    blocker,
    retryAt,
  });
}

class IdempotentWriteDO extends BasePlaceholderDO {
  private auditAppendTail: Promise<void> = Promise.resolve();

  override async fetch(request: Request): Promise<Response> {
    const url = new URL(request.url);
    if (request.method === 'POST' && url.pathname === BILLING_AUDIT_APPEND_PATH) {
      const auditEvent = await readJsonObject<Record<string, unknown>>(request);
      if (!auditEvent) {
        return json(400, { error: 'invalid-billing-audit-event' });
      }
      const append = this.auditAppendTail.then(() => appendBillingAuditEventAtOwner(this.env, auditEvent));
      this.auditAppendTail = append.then(
        () => undefined,
        () => undefined
      );
      await append;
      return json(200, { status: 'delivered' });
    }
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
    if (
      requestKey === null ||
      requestFingerprint === null ||
      responseStatus === null ||
      !Number.isInteger(responseStatus) ||
      !Object.prototype.hasOwnProperty.call(envelope, 'responseBody')
    ) {
      return json(400, {
        error: 'invalid-idempotency-envelope',
      });
    }

    const stateMutationProvided = envelope.stateMutation !== undefined && envelope.stateMutation !== null;
    const stateMutation = parseBillingStateMutation(envelope.stateMutation);
    if (stateMutationProvided && stateMutation === null) {
      return json(400, {
        error: 'invalid-state-mutation',
      });
    }

    const storageKey = `idempotency:${requestKey}`;
    let existing: DurableIdempotencyRecord | undefined;
    try {
      const stored = await this.state.storage.get<unknown>(storageKey);
      if (stored !== undefined) {
        existing = normalizeDurableIdempotencyRecord(stored) ?? undefined;
        if (!existing) {
          return idempotencyRetryResponse('billing-control-do-idempotency-state-invalid');
        }
      }
    } catch {
      return json(503, { status: 'manual-required', blocker: 'billing-control-do-storage-unavailable' });
    }
    if (existing !== undefined) {
      if (existing.requestFingerprint !== requestFingerprint) {
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

      if (existing.state === 'manual-required') {
        return idempotencyRetryResponse('billing-control-do-idempotency-manual-required');
      }

      if (existing.state !== 'completed') {
        const now = Date.now();
        const leaseActive = existing.leaseExpiresAt !== null && Date.parse(existing.leaseExpiresAt) > now;
        if (leaseActive) {
          return idempotencyRetryResponse('billing-control-do-idempotency-in-flight', existing.leaseExpiresAt);
        }
        if (existing.retryAt !== null && Date.parse(existing.retryAt) > now) {
          return idempotencyRetryResponse('billing-control-do-idempotency-retry-backoff', existing.retryAt);
        }
        if (existing.attemptCount >= IDEMPOTENCY_MAX_ATTEMPTS && existing.lastError !== null) {
          const exhausted = {
            ...existing,
            state: 'manual-required' as const,
            stateVersion: existing.stateVersion + 1,
            leaseToken: null,
            leaseExpiresAt: null,
            retryAt: null,
            lastError: existing.lastError ?? 'idempotency-attempt-limit-exhausted',
          } satisfies DurableIdempotencyRecord;
          try {
            await this.state.storage.put(storageKey, exhausted);
          } catch {
            return json(503, { status: 'manual-required', blocker: 'billing-control-do-storage-unavailable' });
          }
          return idempotencyRetryResponse('billing-control-do-idempotency-attempt-limit-exhausted');
        }
      }

      if (existing.state === 'completed') {
        return json(200, {
          replayed: true,
          responseStatus: existing.responseStatus,
          responseBody: cloneJsonValue(existing.responseBody),
          queued: false,
        } satisfies IdempotentWriteResult);
      }
    }

    const leaseToken = crypto.randomUUID();
    const leaseExpiresAt = new Date(Date.now() + IDEMPOTENCY_LEASE_MS).toISOString();
    const pending: DurableIdempotencyRecord = existing
      ? {
          ...existing,
          state: 'pending',
          stateVersion: existing.stateVersion + 1,
          attemptCount: existing.attemptCount + 1,
          leaseToken,
          leaseExpiresAt,
          retryAt: null,
          lastError: null,
        }
      : {
          state: 'pending',
          requestFingerprint,
          responseStatus,
          responseBody: cloneJsonValue(envelope.responseBody),
          stateVersion: 1,
          attemptCount: 1,
          leaseToken,
          leaseExpiresAt,
          retryAt: null,
          lastError: null,
        };
    try {
      await this.state.storage.put(storageKey, pending);
    } catch {
      return json(503, { status: 'manual-required', blocker: 'billing-control-do-storage-unavailable' });
    }

    const queueMessage = isPlainObject(envelope.queueMessage) ? envelope.queueMessage : null;
    let queued = false;
    let responseBody: unknown = envelope.responseBody;
    try {
      if (stateMutation) {
        await applyBillingStateMutation(this.env, stateMutation);
      }
      queued = queueMessage ? await queueReconciliationEvent(this.env, queueMessage) : false;
      responseBody = queueMessage ? withQueuedFlag(envelope.responseBody, queued) : envelope.responseBody;
    } catch (error) {
      try {
        const current = normalizeDurableIdempotencyRecord(await this.state.storage.get<unknown>(storageKey));
        if (
          current === null ||
          current === undefined ||
          current.state !== 'pending' ||
          current.stateVersion !== pending.stateVersion ||
          current.leaseToken !== leaseToken
        ) {
          return idempotencyRetryResponse('billing-control-do-idempotency-lease-lost');
        }
        const exhausted = current.attemptCount >= IDEMPOTENCY_MAX_ATTEMPTS;
        const retryAt = exhausted
          ? null
          : new Date(Date.now() + idempotencyRetryDelayMs(current.attemptCount)).toISOString();
        const failed = {
          ...current,
          state: exhausted ? ('manual-required' as const) : ('pending' as const),
          stateVersion: current.stateVersion + 1,
          leaseToken: null,
          leaseExpiresAt: null,
          retryAt,
          lastError: queueFailureMessage(error) ?? 'billing-control-do-mutation-unavailable',
        } satisfies DurableIdempotencyRecord;
        await this.state.storage.put(storageKey, failed);
        return idempotencyRetryResponse(
          exhausted
            ? 'billing-control-do-idempotency-attempt-limit-exhausted'
            : 'billing-control-do-idempotency-retryable-failure',
          retryAt
        );
      } catch {
        return json(503, { status: 'manual-required', blocker: 'billing-control-do-storage-unavailable' });
      }
    }
    try {
      const current = normalizeDurableIdempotencyRecord(await this.state.storage.get<unknown>(storageKey));
      if (
        current === null ||
        current === undefined ||
        current.state !== 'pending' ||
        current.stateVersion !== pending.stateVersion ||
        current.leaseToken !== leaseToken
      ) {
        return idempotencyRetryResponse('billing-control-do-idempotency-lease-lost');
      }
      const stored = {
        ...current,
        state: 'completed' as const,
        stateVersion: current.stateVersion + 1,
        responseBody: cloneJsonValue(responseBody),
        leaseToken: null,
        leaseExpiresAt: null,
        retryAt: null,
        lastError: null,
      } satisfies DurableIdempotencyRecord;
      await this.state.storage.put(storageKey, stored);
    } catch {
      return json(503, { status: 'manual-required', blocker: 'billing-control-do-storage-unavailable' });
    }

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

async function processBillingQueueMessage(env: Env, payload: Record<string, unknown>): Promise<void> {
  const action = stringOrNull(payload.action);
  if (!action) {
    throw new Error('queue-consumer-invalid-message');
  }
  if (action === 'provider-webhook') {
    const provider = stringOrNull(payload.provider);
    const eventId = stringOrNull(payload.eventId);
    const eventType = stringOrNull(payload.eventType);
    if (!provider || !eventId || !eventType) {
      throw new Error('queue-consumer-invalid-provider-event');
    }
    const receipt = await loadBillingProviderEventReceipt(env, provider, eventId);
    if (!receipt) {
      throw new Error(`queue-consumer-provider-receipt-missing:${provider}:${eventId}`);
    }
    if (!receipt.billingSubject) {
      await markBillingProviderEventQueue(
        env,
        provider,
        eventId,
        'manual-required',
        'manual-required',
        'provider-event-authority-unresolved',
        receipt
      );
      return;
    }
    if (receipt.providerOccurredAt === null && receipt.providerSequence === null) {
      await markBillingProviderEventQueue(
        env,
        provider,
        eventId,
        'manual-required',
        'manual-required',
        'provider-event-order-metadata-missing',
        receipt
      );
      return;
    }
    const authority = await resolveProviderBillingAuthority(env.ACCOUNT_IDENTITY_D1, provider, {
      customerId: receipt.providerCustomerId,
      subscriptionId: receipt.providerSubscriptionId,
      invoiceId: receipt.providerInvoiceId,
    });
    if (
      authority.status !== 'trusted' ||
      authority.authority.billingSubject !== receipt.billingSubject ||
      authority.authority.accountId !== receipt.accountId ||
      authority.authority.providerCustomerId !== receipt.providerCustomerId ||
      authority.authority.providerSubscriptionId !== receipt.providerSubscriptionId ||
      authority.authority.providerInvoiceId !== receipt.providerInvoiceId ||
      authority.authority.parentAccountRef !== receipt.parentAccountRef ||
      authority.authority.familyRef !== receipt.familyRef ||
      authority.authority.billingInvoiceId !== receipt.billingInvoiceId
    ) {
      await markBillingProviderEventQueue(
        env,
        provider,
        eventId,
        'manual-required',
        'manual-required',
        'provider-event-authority-revoked-or-changed',
        receipt
      );
      return;
    }
    const cursor = await loadBillingProviderEventCursor(env, provider, receipt.billingSubject);
    await applyBillingStateMutation(env, {
      kind: 'provider-webhook',
      provider,
      subject: receipt.billingSubject,
      eventId,
      eventType: receipt.eventType,
      providerOccurredAt: receipt.providerOccurredAt,
      providerSequence: receipt.providerSequence,
      providerCursorExpectedVersion: cursor?.stateVersion ?? 0,
      disputeId: stringOrNull(payload.disputeId),
      invoiceId: receipt.billingInvoiceId,
      parentAccountRef: receipt.parentAccountRef,
      familyRef: receipt.familyRef,
    });
    await markProviderWebhookDelivered(
      env,
      receipt,
      isIgnoredProviderWebhookEvent(receipt.eventType) ? 'ignored' : 'applied'
    );
    return;
  }
  if (
    action === 'reconciliation' ||
    action === 'referral-invite' ||
    action === 'manual-invoice' ||
    action === 'admin-refund' ||
    action === 'change-plan' ||
    action === 'cancel'
  ) {
    const summary = await drainPendingBillingMutationOutbox(env);
    if (summary.failed > 0) {
      throw new Error(`queue-consumer-outbox-failures:${summary.failed}`);
    }
    return;
  }
  throw new Error(`queue-consumer-unsupported-action:${action}`);
}

async function consumeBillingQueue(batch: MessageBatch<unknown>, env: Env): Promise<void> {
  for (const message of batch.messages) {
    const payload = isPlainObject(message.body) ? message.body : null;
    if (!payload) {
      if (message.attempts >= 5) {
        if (
          await sendBillingDeadLetter(
            env,
            { body: cloneJsonValue(message.body) },
            'queue-consumer-invalid-message',
            null
          )
        ) {
          message.ack();
        } else {
          message.retry({ delaySeconds: 900 });
        }
      } else {
        message.retry({ delaySeconds: Math.min(900, 30 * (message.attempts + 1)) });
      }
      continue;
    }
    try {
      await processBillingQueueMessage(env, payload);
      message.ack();
    } catch (error) {
      if (message.attempts >= 5) {
        const provider = stringOrNull(payload.provider);
        const eventId = stringOrNull(payload.eventId);
        if (payload.action === 'provider-webhook' && provider && eventId) {
          const receipt = await loadBillingProviderEventReceipt(env, provider, eventId);
          if (!receipt) {
            throw new Error(`queue-consumer-provider-receipt-missing:${provider}:${eventId}`);
          }
          if (
            receipt.queueState !== 'delivered' &&
            receipt.queueState !== 'manual-required' &&
            receipt.queueState !== 'dead-letter'
          ) {
            const terminalProcessingState =
              receipt.processingState === 'ignored' ||
              receipt.processingState === 'applied' ||
              receipt.processingState === 'manual-required'
                ? receipt.processingState
                : 'dead-letter';
            await markBillingProviderEventQueue(
              env,
              provider,
              eventId,
              'dead-letter',
              terminalProcessingState,
              queueFailureMessage(error) ?? 'queue-consumer-manual-required',
              receipt
            );
          }
        }
        if (await sendBillingDeadLetter(env, payload, 'queue-consumer-manual-required', error)) {
          message.ack();
        } else {
          message.retry({ delaySeconds: 900 });
        }
      } else {
        message.retry({ delaySeconds: Math.min(900, 30 * (message.attempts + 1)) });
      }
    }
  }
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    return withCors(await handleRequest(request, env), request, env);
  },

  async scheduled(_controller: unknown, env: Env): Promise<void> {
    void _controller;
    try {
      await drainPendingBillingMutationOutbox(env);
    } catch (_error) {
      env.ANALYTICS?.writeDataPoint({
        indexes: ['billing-mutation-outbox-drain-failed'],
        doubles: [1],
      });
    }
  },

  async queue(batch: MessageBatch<unknown>, env: Env): Promise<void> {
    await consumeBillingQueue(batch, env);
  },
};

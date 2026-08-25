import type {
  AnalyticsEngineDataset,
  D1Database,
  DurableObjectNamespace,
  KVNamespace,
  Queue,
  R2Bucket,
} from '@cloudflare/workers-types';

export interface Env {
  ENVIRONMENT: string;
  APP_ORIGIN: string;
  CORS_ALLOWED_ORIGINS: string;
  REQUEST_MAX_BYTES?: string;
  BILLING_ROUTE_KILL_SWITCH?: string;
  AUTH_ADAPTER_MODE?: string;
  FIREBASE_PROJECT_ID?: string;
  FIREBASE_CLOCK_SKEW_SECONDS?: string;
  FIREBASE_JWKS_CACHE_SECONDS?: string;
  INTERACTIVE_CSRF_TOKEN?: string;
  INTERNAL_QUEUE_SHARED_SECRET?: string;
  STRIPE_SECRET_KEY?: string;
  STRIPE_WEBHOOK_SECRET?: string;
  STRIPE_WEBHOOK_TOLERANCE_SECONDS?: string;
  RAZORPAY_KEY_ID?: string;
  RAZORPAY_KEY_SECRET?: string;
  PAYPAL_CLIENT_ID?: string;
  PAYPAL_CLIENT_SECRET?: string;
  APPLE_STORE_KEY_REF?: string;
  GOOGLE_PLAY_SERVICE_ACCOUNT_REF?: string;
  ENTITLEMENT_SIGNING_KEY_REF?: string;
  BILLING_D1?: D1Database;
  ACCOUNT_IDENTITY_D1?: D1Database;
  BILLING_DO?: DurableObjectNamespace;
  REFERRAL_DO?: DurableObjectNamespace;
  ENTITLEMENT_SNAPSHOT_DO?: DurableObjectNamespace;
  BILLING_RECONCILIATION_QUEUE?: Queue;
  BILLING_DEAD_LETTER_QUEUE?: Queue;
  BILLING_RATE_LIMIT_KV?: KVNamespace;
  BILLING_CONFIG_KV?: KVNamespace;
  BILLING_AUDIT_R2?: R2Bucket;
  ANALYTICS?: AnalyticsEngineDataset;
}

export const DEFAULT_REQUEST_MAX_BYTES = 1024 * 1024;
export const FIREBASE_PROJECT_ID_PATTERN = /^[a-z][a-z0-9-]{4,28}[a-z0-9]$/;

const REQUIRED_ENV_KEYS = ['ENVIRONMENT', 'APP_ORIGIN', 'CORS_ALLOWED_ORIGINS'] as const;
export const REQUIRED_BINDING_KEYS = [
  'BILLING_D1',
  'BILLING_DO',
  'REFERRAL_DO',
  'ENTITLEMENT_SNAPSHOT_DO',
  'BILLING_RECONCILIATION_QUEUE',
  'BILLING_DEAD_LETTER_QUEUE',
  'BILLING_RATE_LIMIT_KV',
  'BILLING_CONFIG_KV',
] as const;
export const OPTIONAL_BINDING_KEYS = ['ACCOUNT_IDENTITY_D1', 'BILLING_AUDIT_R2', 'ANALYTICS'] as const;
const OPTIONAL_ENV_KEYS = [
  'REQUEST_MAX_BYTES',
  'BILLING_ROUTE_KILL_SWITCH',
  'AUTH_ADAPTER_MODE',
  'FIREBASE_PROJECT_ID',
  'FIREBASE_CLOCK_SKEW_SECONDS',
  'FIREBASE_JWKS_CACHE_SECONDS',
  'INTERACTIVE_CSRF_TOKEN',
  'INTERNAL_QUEUE_SHARED_SECRET',
  'STRIPE_SECRET_KEY',
  'STRIPE_WEBHOOK_SECRET',
  'STRIPE_WEBHOOK_TOLERANCE_SECONDS',
  'RAZORPAY_KEY_ID',
  'RAZORPAY_KEY_SECRET',
  'PAYPAL_CLIENT_ID',
  'PAYPAL_CLIENT_SECRET',
  'APPLE_STORE_KEY_REF',
  'GOOGLE_PLAY_SERVICE_ACCOUNT_REF',
  'ENTITLEMENT_SIGNING_KEY_REF',
] as const;
const TRACKED_BINDING_KEYS = [...REQUIRED_BINDING_KEYS, ...OPTIONAL_BINDING_KEYS] as const;
const KNOWN_ENV_KEYS = [...REQUIRED_ENV_KEYS, ...OPTIONAL_ENV_KEYS, ...TRACKED_BINDING_KEYS] as const;

export type RequiredBindingKey = (typeof REQUIRED_BINDING_KEYS)[number];
export type OptionalBindingKey = (typeof OPTIONAL_BINDING_KEYS)[number];
export type TrackedBindingKey = (typeof TRACKED_BINDING_KEYS)[number];
export type BindingFamily = 'analytics' | 'd1' | 'durable-object' | 'kv' | 'queue' | 'r2';
export type BindingReadinessState = 'manual-required' | 'optional' | 'required';

export interface BindingOwnership {
  owner: string;
  purpose: string;
  bindingFamily: BindingFamily;
  privacyBoundary: string;
  childDataStorage: 'forbidden';
  readinessState: BindingReadinessState;
  queueRole?: 'dead-letter' | 'producer';
  pairedQueueBinding?: TrackedBindingKey;
  rejectedUse?: string;
}

export const BINDING_OWNERSHIP = {
  BILLING_D1: {
    owner: 'billing-ledger-read-model',
    purpose: 'queryable billing ledgers, support/admin views, and reconciliation read models',
    bindingFamily: 'd1',
    privacyBoundary: 'billing, support, and reconciliation records only; no child telemetry or raw child data',
    childDataStorage: 'forbidden',
    readinessState: 'required',
  },
  ACCOUNT_IDENTITY_D1: {
    owner: 'account-identity-authority',
    purpose: 'durable provider-subject to current account/member/role/device/session authority mapping',
    bindingFamily: 'd1',
    privacyBoundary:
      'provider subject, account/member/device identifiers, current household target binding, session provenance, and support receipt metadata only; no child telemetry or raw claims',
    childDataStorage: 'forbidden',
    readinessState: 'manual-required',
    rejectedUse: 'must not accept caller-supplied household, child, device, role, session, or receipt authority',
  },
  BILLING_DO: {
    owner: 'billing-control-do',
    purpose: 'serialized billing-state writes and idempotency coordination',
    bindingFamily: 'durable-object',
    privacyBoundary: 'billing write coordination only; no child telemetry or raw child data',
    childDataStorage: 'forbidden',
    readinessState: 'required',
  },
  REFERRAL_DO: {
    owner: 'referral-control-do',
    purpose: 'serialized referral qualification, abuse review, and credit lifecycle coordination',
    bindingFamily: 'durable-object',
    privacyBoundary: 'referral coordination only; no child telemetry or raw child data',
    childDataStorage: 'forbidden',
    readinessState: 'required',
  },
  ENTITLEMENT_SNAPSHOT_DO: {
    owner: 'entitlement-snapshot-do',
    purpose: 'snapshot issuance coordination and replay-safe signing workflow',
    bindingFamily: 'durable-object',
    privacyBoundary: 'entitlement snapshot coordination only; no child telemetry or raw child data',
    childDataStorage: 'forbidden',
    readinessState: 'required',
  },
  BILLING_RECONCILIATION_QUEUE: {
    owner: 'billing-reconciliation-jobs',
    purpose: 'retry, provider polling, and reconciliation jobs',
    bindingFamily: 'queue',
    privacyBoundary: 'redacted reconciliation payloads only; no child telemetry or raw child data',
    childDataStorage: 'forbidden',
    readinessState: 'required',
    queueRole: 'producer',
    pairedQueueBinding: 'BILLING_DEAD_LETTER_QUEUE',
  },
  BILLING_DEAD_LETTER_QUEUE: {
    owner: 'billing-dead-letter-ops',
    purpose: 'dead-letter capture and operator replay workflow for reconciliation failures',
    bindingFamily: 'queue',
    privacyBoundary: 'redacted dead-letter payloads only; no child telemetry or raw child data',
    childDataStorage: 'forbidden',
    readinessState: 'required',
    queueRole: 'dead-letter',
    pairedQueueBinding: 'BILLING_RECONCILIATION_QUEUE',
  },
  BILLING_RATE_LIMIT_KV: {
    owner: 'billing-rate-limit-guard',
    purpose: 'rate limits and lightweight abuse counters',
    bindingFamily: 'kv',
    privacyBoundary: 'rate-limit counters and low-risk abuse state only; no child telemetry or raw child data',
    childDataStorage: 'forbidden',
    readinessState: 'required',
  },
  BILLING_CONFIG_KV: {
    owner: 'billing-runtime-config',
    purpose: 'low-risk runtime flags and rollout config',
    bindingFamily: 'kv',
    privacyBoundary:
      'rollout flags and non-secret config only; no child telemetry, raw child data, or provider secrets',
    childDataStorage: 'forbidden',
    readinessState: 'required',
  },
  BILLING_AUDIT_R2: {
    owner: 'support-audit-export',
    purpose: 'support-safe audit and export bundles only',
    bindingFamily: 'r2',
    privacyBoundary: 'support-safe audit/export artifacts only; no child telemetry or raw child data',
    childDataStorage: 'forbidden',
    readinessState: 'manual-required',
    rejectedUse: 'must not become a telemetry dump or general-purpose child-data storage',
  },
  ANALYTICS: {
    owner: 'redacted-ops-analytics',
    purpose: 'redacted operational metrics and audit event counters',
    bindingFamily: 'analytics',
    privacyBoundary: 'redacted operational metrics only; no child telemetry, raw child data, or provider secrets',
    childDataStorage: 'forbidden',
    readinessState: 'optional',
  },
} as const satisfies Record<TrackedBindingKey, BindingOwnership>;

export function parseAllowedOrigins(env: Env): string[] {
  const configuredOrigins: unknown = env.CORS_ALLOWED_ORIGINS;
  if (typeof configuredOrigins !== 'string') {
    return [];
  }

  return configuredOrigins
    .split(',')
    .map((entry) => entry.trim())
    .filter(Boolean);
}

export function parseRequestMaxBytes(env: Env): number {
  if (!env.REQUEST_MAX_BYTES) {
    return DEFAULT_REQUEST_MAX_BYTES;
  }
  const parsed = Number(env.REQUEST_MAX_BYTES);
  return Number.isInteger(parsed) && parsed > 0 ? parsed : DEFAULT_REQUEST_MAX_BYTES;
}

export function isRouteKillSwitchEnabled(env: Env): boolean {
  return env.BILLING_ROUTE_KILL_SWITCH === 'true';
}

export function resolveAuthAdapterMode(env: Env): string {
  return env.AUTH_ADAPTER_MODE?.trim() || 'local-safe-fixture';
}

export function isLocalFixtureEnvironment(env: Pick<Env, 'ENVIRONMENT'>): boolean {
  const configuredEnvironment: unknown = env.ENVIRONMENT;
  const environment = typeof configuredEnvironment === 'string' ? configuredEnvironment.trim().toLowerCase() : '';
  return environment === 'local' || environment === 'test' || environment === 'development';
}

function isProductionEnvironment(env: Pick<Env, 'ENVIRONMENT'>): boolean {
  const configuredEnvironment: unknown = env.ENVIRONMENT;
  return typeof configuredEnvironment === 'string' && configuredEnvironment.trim().toLowerCase() === 'production';
}

function hasWildcardOrigin(origin: unknown): boolean {
  return typeof origin === 'string' && origin.trim().includes('*');
}

export function getMissingBindings(env: Env): ReadonlyArray<RequiredBindingKey> {
  return REQUIRED_BINDING_KEYS.filter((key) => !env[key]);
}

export function getBindingHealth(env: Env): Record<TrackedBindingKey, 'configured' | 'missing'> {
  return Object.fromEntries(TRACKED_BINDING_KEYS.map((key) => [key, env[key] ? 'configured' : 'missing'])) as Record<
    TrackedBindingKey,
    'configured' | 'missing'
  >;
}

export function getBindingOwnership(): Record<TrackedBindingKey, BindingOwnership> {
  return BINDING_OWNERSHIP;
}

export function validateEnv(env: Env): string[] {
  const errors: string[] = [];

  for (const key of Object.keys(env)) {
    if (!(KNOWN_ENV_KEYS as readonly string[]).includes(key)) {
      errors.push(`unknown env key: ${key}`);
    }
  }

  for (const key of REQUIRED_ENV_KEYS) {
    const configuredValue: unknown = env[key];
    if (typeof configuredValue !== 'string' || configuredValue.trim() === '') {
      errors.push(`missing required env: ${key}`);
    }
  }

  const allowedOrigins = parseAllowedOrigins(env);
  if (allowedOrigins.length === 0) {
    errors.push('CORS_ALLOWED_ORIGINS must include at least one origin');
  }

  if (isProductionEnvironment(env)) {
    if (hasWildcardOrigin(env.APP_ORIGIN)) {
      errors.push('APP_ORIGIN must not contain a wildcard in production');
    }
    if (allowedOrigins.some(hasWildcardOrigin)) {
      errors.push('CORS_ALLOWED_ORIGINS must not contain a wildcard in production');
    }
  }

  if (env.REQUEST_MAX_BYTES && (!/^\d+$/.test(env.REQUEST_MAX_BYTES) || Number(env.REQUEST_MAX_BYTES) <= 0)) {
    errors.push('REQUEST_MAX_BYTES must be a positive integer when provided');
  }

  if (
    !env.STRIPE_WEBHOOK_TOLERANCE_SECONDS ||
    !/^\d+$/.test(env.STRIPE_WEBHOOK_TOLERANCE_SECONDS) ||
    Number(env.STRIPE_WEBHOOK_TOLERANCE_SECONDS) <= 0 ||
    Number(env.STRIPE_WEBHOOK_TOLERANCE_SECONDS) > 86_400
  ) {
    errors.push('STRIPE_WEBHOOK_TOLERANCE_SECONDS must be a positive integer no greater than 86400');
  }

  if (!env.ENTITLEMENT_SIGNING_KEY_REF) {
    errors.push('missing required env: ENTITLEMENT_SIGNING_KEY_REF');
  }

  if (!isLocalFixtureEnvironment(env) && resolveAuthAdapterMode(env) === 'local-safe-fixture') {
    errors.push('AUTH_ADAPTER_MODE local-safe-fixture is not permitted outside local/test/development');
  }

  if (resolveAuthAdapterMode(env) === 'provider-verified') {
    const projectId = env.FIREBASE_PROJECT_ID?.trim();
    if (!projectId) errors.push('missing required env: FIREBASE_PROJECT_ID');
    else if (!FIREBASE_PROJECT_ID_PATTERN.test(projectId)) {
      errors.push(
        'FIREBASE_PROJECT_ID must be 6-30 characters, start with a lowercase letter, and end with a lowercase letter or digit'
      );
    }
    for (const [name, maximum] of [
      ['FIREBASE_CLOCK_SKEW_SECONDS', 300],
      ['FIREBASE_JWKS_CACHE_SECONDS', 3600],
    ] as const) {
      const value = env[name];
      if (value !== undefined && (!/^\d+$/.test(value) || Number(value) <= 0 || Number(value) > maximum)) {
        errors.push(`${name} must be a positive integer no greater than ${maximum}`);
      }
    }
  }

  if (isProductionEnvironment(env) && !env.INTERNAL_QUEUE_SHARED_SECRET?.trim()) {
    errors.push('missing required env: INTERNAL_QUEUE_SHARED_SECRET');
  }

  if (env.ENVIRONMENT !== 'test' && !env.INTERACTIVE_CSRF_TOKEN) {
    errors.push('missing required env: INTERACTIVE_CSRF_TOKEN');
  }

  return errors;
}

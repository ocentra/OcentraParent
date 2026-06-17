import type {
  AnalyticsEngineDataset,
  D1Database,
  DurableObjectNamespace,
  KVNamespace,
  Queue,
  R2Bucket,
} from "@cloudflare/workers-types";

export interface Env {
  ENVIRONMENT: string;
  APP_ORIGIN: string;
  CORS_ALLOWED_ORIGINS: string;
  REQUEST_MAX_BYTES?: string;
  BILLING_ROUTE_KILL_SWITCH?: string;
  AUTH_ADAPTER_MODE?: string;
  INTERNAL_QUEUE_SHARED_SECRET?: string;
  STRIPE_SECRET_KEY?: string;
  STRIPE_WEBHOOK_SECRET?: string;
  RAZORPAY_KEY_ID?: string;
  RAZORPAY_KEY_SECRET?: string;
  PAYPAL_CLIENT_ID?: string;
  PAYPAL_CLIENT_SECRET?: string;
  APPLE_STORE_KEY_REF?: string;
  GOOGLE_PLAY_SERVICE_ACCOUNT_REF?: string;
  ENTITLEMENT_SIGNING_KEY_REF?: string;
  BILLING_D1?: D1Database;
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

const REQUIRED_ENV_KEYS = ["ENVIRONMENT", "APP_ORIGIN", "CORS_ALLOWED_ORIGINS"] as const;
export const REQUIRED_BINDING_KEYS = [
  "BILLING_D1",
  "BILLING_DO",
  "REFERRAL_DO",
  "ENTITLEMENT_SNAPSHOT_DO",
  "BILLING_RECONCILIATION_QUEUE",
  "BILLING_DEAD_LETTER_QUEUE",
  "BILLING_RATE_LIMIT_KV",
  "BILLING_CONFIG_KV",
] as const;
export const OPTIONAL_BINDING_KEYS = [
  "BILLING_AUDIT_R2",
  "ANALYTICS",
] as const;
const OPTIONAL_ENV_KEYS = [
  "REQUEST_MAX_BYTES",
  "BILLING_ROUTE_KILL_SWITCH",
  "AUTH_ADAPTER_MODE",
  "INTERNAL_QUEUE_SHARED_SECRET",
  "STRIPE_SECRET_KEY",
  "STRIPE_WEBHOOK_SECRET",
  "RAZORPAY_KEY_ID",
  "RAZORPAY_KEY_SECRET",
  "PAYPAL_CLIENT_ID",
  "PAYPAL_CLIENT_SECRET",
  "APPLE_STORE_KEY_REF",
  "GOOGLE_PLAY_SERVICE_ACCOUNT_REF",
  "ENTITLEMENT_SIGNING_KEY_REF",
] as const;
const TRACKED_BINDING_KEYS = [...REQUIRED_BINDING_KEYS, ...OPTIONAL_BINDING_KEYS] as const;
const KNOWN_ENV_KEYS = [...REQUIRED_ENV_KEYS, ...OPTIONAL_ENV_KEYS, ...TRACKED_BINDING_KEYS] as const;

export type RequiredBindingKey = (typeof REQUIRED_BINDING_KEYS)[number];
export type TrackedBindingKey = (typeof TRACKED_BINDING_KEYS)[number];

export function parseAllowedOrigins(env: Env): string[] {
  return env.CORS_ALLOWED_ORIGINS.split(",")
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
  return env.BILLING_ROUTE_KILL_SWITCH === "true";
}

export function resolveAuthAdapterMode(env: Env): string {
  return env.AUTH_ADAPTER_MODE?.trim() || "local-safe-fixture";
}

export function getMissingBindings(env: Env): ReadonlyArray<RequiredBindingKey> {
  return REQUIRED_BINDING_KEYS.filter((key) => !env[key]);
}

export function getBindingHealth(env: Env): Record<TrackedBindingKey, "configured" | "missing"> {
  return Object.fromEntries(
    TRACKED_BINDING_KEYS.map((key) => [key, env[key] ? "configured" : "missing"]),
  ) as Record<TrackedBindingKey, "configured" | "missing">;
}

export function validateEnv(env: Env): string[] {
  const errors: string[] = [];

  for (const key of Object.keys(env)) {
    if (!(KNOWN_ENV_KEYS as readonly string[]).includes(key)) {
      errors.push(`unknown env key: ${key}`);
    }
  }

  for (const key of REQUIRED_ENV_KEYS) {
    if (!env[key] || String(env[key]).trim() === "") {
      errors.push(`missing required env: ${key}`);
      }
  }

  if (parseAllowedOrigins(env).length === 0) {
    errors.push("CORS_ALLOWED_ORIGINS must include at least one origin");
  }

  if (
    env.REQUEST_MAX_BYTES &&
    (!/^\d+$/.test(env.REQUEST_MAX_BYTES) || Number(env.REQUEST_MAX_BYTES) <= 0)
  ) {
    errors.push("REQUEST_MAX_BYTES must be a positive integer when provided");
  }

  if (!env.ENTITLEMENT_SIGNING_KEY_REF) {
    errors.push("missing required env: ENTITLEMENT_SIGNING_KEY_REF");
  }

  return errors;
}

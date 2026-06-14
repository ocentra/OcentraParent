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
  return Number.isFinite(parsed) && parsed > 0 ? parsed : DEFAULT_REQUEST_MAX_BYTES;
}

export function isRouteKillSwitchEnabled(env: Env): boolean {
  return env.BILLING_ROUTE_KILL_SWITCH === "true";
}

export function validateEnv(env: Env): string[] {
  const errors: string[] = [];

  for (const key of REQUIRED_ENV_KEYS) {
    if (!env[key] || String(env[key]).trim() === "") {
      errors.push(`missing required env: ${key}`);
    }
  }

  if (parseAllowedOrigins(env).length === 0) {
    errors.push("CORS_ALLOWED_ORIGINS must include at least one origin");
  }

  if (!env.ENTITLEMENT_SIGNING_KEY_REF) {
    errors.push("missing required env: ENTITLEMENT_SIGNING_KEY_REF");
  }

  return errors;
}

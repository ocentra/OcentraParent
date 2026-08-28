import type {
  DurableObjectId,
  DurableObjectNamespace,
  DurableObjectState,
  DurableObjectStub,
  Queue,
} from '@cloudflare/workers-types';
import {
  buildDefaultBillingBindingSeed,
  createLocalBillingBindings,
  type LocalBillingBindingState,
} from './billing-binding-read-model.js';
import worker, { BillingControlDO, EntitlementSnapshotDO, ReferralControlDO } from './index.js';
import type { Env } from './env.js';

export interface CloudflareTestHarness {
  env: Env;
  queueMessages: unknown[];
  deadLetterMessages: unknown[];
  bindingState: LocalBillingBindingState;
}

export interface ExecuteRequestOptions {
  path: string;
  method?: string;
  headers?: HeadersInit;
  body?: BodyInit | Record<string, unknown>;
  autoContentLength?: boolean;
  harness?: CloudflareTestHarness;
  envOverrides?: Partial<Env>;
}

function createQueueRecorder(target: unknown[]): Queue {
  return {
    send: async (message: unknown): Promise<void> => {
      target.push(message);
    },
    sendBatch: async (messages: ReadonlyArray<{ body: unknown }>): Promise<void> => {
      for (const message of messages) {
        target.push(message.body);
      }
    },
  } as unknown as Queue;
}

type DurableObjectConstructor = new (state: DurableObjectState, env: Env) => DurableObjectStub;

type LocalDurableObjectId = DurableObjectId & {
  readonly name: string;
};

function createDurableObjectNamespace(ctor: DurableObjectConstructor, env: Env): DurableObjectNamespace {
  const instances = new Map<string, DurableObjectStub>();

  return {
    idFromName(name: string): DurableObjectId {
      return {
        name,
      } as LocalDurableObjectId;
    },
    get(id: DurableObjectId): DurableObjectStub {
      const localId = id as LocalDurableObjectId;
      const existing = instances.get(localId.name);
      if (existing !== undefined) {
        return existing;
      }

      const instance = new ctor({} as DurableObjectState, env);
      instances.set(localId.name, instance);
      return instance;
    },
  };
}

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && value.constructor === Object;
}

function createLocalFixtureValue(separator: string, segments: readonly string[]): string {
  return segments.join(separator);
}

export function createTestHarness(overrides: Partial<Env> = {}): CloudflareTestHarness {
  const queueMessages: unknown[] = [];
  const deadLetterMessages: unknown[] = [];
  const localBindings = createLocalBillingBindings();

  const env = {
    ENVIRONMENT: 'test',
    APP_ORIGIN: 'http://localhost:3000',
    CORS_ALLOWED_ORIGINS: 'http://localhost:3000,http://127.0.0.1:3000',
    REQUEST_MAX_BYTES: '2048',
    BILLING_ROUTE_KILL_SWITCH: 'false',
    AUTH_ADAPTER_MODE: 'local-safe-fixture',
    INTERNAL_QUEUE_SHARED_SECRET: createLocalFixtureValue('-', ['internal', 'test', 'secret']),
    STRIPE_SECRET_KEY: 'sk_test_not_exposed',
    STRIPE_WEBHOOK_SECRET: createLocalFixtureValue('_', ['whsec', 'test', 'secret']),
    STRIPE_WEBHOOK_TOLERANCE_SECONDS: '300',
    PAYPAL_CLIENT_SECRET: createLocalFixtureValue('-', ['paypal', 'test', 'secret']),
    RAZORPAY_KEY_SECRET: createLocalFixtureValue('-', ['razorpay', 'test', 'secret']),
    APPLE_STORE_KEY_REF: 'apple-store-key-test-ref',
    GOOGLE_PLAY_SERVICE_ACCOUNT_REF: 'google-play-service-account-test-ref',
    ENTITLEMENT_SIGNING_KEY_REF: 'signing-key-test-ref',
    BILLING_D1: localBindings.BILLING_D1,
    BILLING_RECONCILIATION_QUEUE: createQueueRecorder(queueMessages),
    BILLING_DEAD_LETTER_QUEUE: createQueueRecorder(deadLetterMessages),
    BILLING_RATE_LIMIT_KV: localBindings.BILLING_RATE_LIMIT_KV,
    BILLING_CONFIG_KV: localBindings.BILLING_CONFIG_KV,
    BILLING_AUDIT_R2: localBindings.BILLING_AUDIT_R2,
    ANALYTICS: localBindings.ANALYTICS,
  } as Env;

  env.BILLING_DO = createDurableObjectNamespace(BillingControlDO, env);
  env.REFERRAL_DO = createDurableObjectNamespace(ReferralControlDO, env);
  env.ENTITLEMENT_SNAPSHOT_DO = createDurableObjectNamespace(EntitlementSnapshotDO, env);
  localBindings.state.replaceSeed(buildDefaultBillingBindingSeed(env));
  Object.assign(env, overrides);

  return {
    env,
    queueMessages,
    deadLetterMessages,
    bindingState: localBindings.state,
  };
}

function normalizeBody(body: ExecuteRequestOptions['body']): BodyInit | undefined {
  if (body === undefined) {
    return undefined;
  }
  if (isPlainObject(body)) {
    return JSON.stringify(body);
  }
  return body;
}

function bodyByteLength(body: BodyInit): number | null {
  if (typeof body === 'string') {
    return new TextEncoder().encode(body).length;
  }
  if (body instanceof URLSearchParams) {
    return new TextEncoder().encode(body.toString()).length;
  }
  if (body instanceof Blob) {
    return body.size;
  }
  if (body instanceof ArrayBuffer) {
    return body.byteLength;
  }
  if (ArrayBuffer.isView(body)) {
    return body.byteLength;
  }
  return null;
}

export async function executeRequest(
  options: ExecuteRequestOptions
): Promise<{ response: Response; harness: CloudflareTestHarness }> {
  const harness = options.harness ?? createTestHarness(options.envOverrides);
  if (options.harness && options.envOverrides) {
    Object.assign(harness.env, options.envOverrides);
  }

  const headers = new Headers(options.headers);
  const body = normalizeBody(options.body);
  if (body !== undefined && !headers.has('content-type')) {
    headers.set('content-type', 'application/json');
  }
  if ((options.autoContentLength ?? true) && body !== undefined && !headers.has('content-length')) {
    const byteLength = bodyByteLength(body);
    if (byteLength !== null) {
      headers.set('content-length', String(byteLength));
    }
  }

  const response = await worker.fetch(
    new Request(`https://cloudflare.local${options.path}`, {
      method: options.method ?? 'GET',
      headers,
      body,
    }),
    harness.env
  );

  return {
    response,
    harness,
  };
}

export async function readJson<T>(response: Response): Promise<T> {
  return JSON.parse(await response.text()) as T;
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes, (value) => value.toString(16).padStart(2, '0')).join('');
}

export async function createStripeSignature(
  payload: string,
  secret: string,
  timestamp = 1_710_000_000
): Promise<string> {
  return `t=${timestamp},v1=${await createHmacSignature(`${timestamp}.${payload}`, secret)}`;
}

export async function createHmacSignature(payload: string, secret: string): Promise<string> {
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
  return bytesToHex(new Uint8Array(signed));
}

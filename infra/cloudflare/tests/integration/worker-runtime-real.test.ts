import assert from 'node:assert/strict';
import { spawn, spawnSync, type ChildProcess } from 'node:child_process';
import { readFileSync, rmSync, writeFileSync } from 'node:fs';
import { createServer } from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { after, describe, it } from 'node:test';
import { fileURLToPath } from 'node:url';
import { acquireLocalWranglerRuntimeLease, LOCAL_WEBHOOK_FIXTURE_INVENTORY } from '../../scripts/local-seed-runtime.js';

interface RuntimeHandle {
  baseUrl: string;
  stop: () => Promise<void>;
}

interface HealthResponse {
  status: 'ok';
  service: string;
  bindingStatus: string;
  missingBindingCount: number;
  seedSummary: {
    pricingPlanCount: number;
    adminAccountCount: number;
    referralFixtureCount: number;
    persistence: {
      d1StatusRows: number;
      d1AdminAccountRows: number;
      d1ReferralRows: number;
      kvPricingPlanRows: number;
      r2AuditEventRows: number;
    };
  };
}

interface PricingResponse {
  status: 'ok';
  plans: Array<{
    planId: string;
  }>;
}

interface BillingStatusResponse {
  status: 'ok';
  parentAccountRef: string;
  accountStatus?: string;
  subscriptionStatus?: string;
  portalVisibleState: string;
  parentVisibleState?: string;
  providerMode: string;
  plan: {
    planId: string;
  };
  deviceUsage: {
    limit: number;
  };
  warnings: ReadonlyArray<string>;
}

interface BillingAuditResponse {
  status: 'ok';
  actorRole: string;
  resultCount: number;
  results?: Array<{
    eventId: string;
    eventType: string;
    manualRequired?: boolean;
    disputeState?: string;
  }>;
}

interface BillingPlanChangeResponse {
  status: 'accepted' | 'rejected';
  changeKind: 'upgrade' | 'downgrade' | 'invalid';
  targetPlanId: string | null;
}

interface HostedSessionResponse {
  status: 'accepted' | 'rejected';
  kind: string;
  hostedUrl: string | null;
  ownerSubject: string;
  pendingEntitlementConfirmation?: boolean;
}

interface BillingEntitlementSnapshotResponse {
  snapshot: {
    planId: string;
    deviceLimit: number;
    subscriptionStatus?: string;
    signatureState?: string;
    parentVisibleState?: string;
  };
}

interface ProviderWebhookRuntimeResponse {
  status: string;
  provider: string;
  eventId: string;
  eventType: string;
}

interface BillingInvoicesResponse {
  status: 'ok';
  invoices: Array<{
    invoiceId: string;
    paymentState: string;
  }>;
}

interface ManualInvoiceRuntimeResponse {
  status: string;
  invoiceState: string;
  queued: boolean;
  region: string;
}

interface RefundRuntimeResponse {
  status: string;
  refundState: string;
  invoiceId: string | null;
}

interface ReconciliationRuntimeResponse {
  status: string;
  queued: boolean;
  driftFamiliesVisible: number;
  retryBacklogVisible: number;
  deadLetterVisible: number;
}

interface AdminAccountsResponse {
  status: 'ok';
  actorRole: string;
  resultCount: number;
  manualActionsPending: number;
}

interface AdminInvoicesResponse {
  status: 'ok';
  actorRole: string;
  resultCount: number;
  results: Array<{
    invoiceId: string;
    paymentState: string;
    manualRequired: boolean;
  }>;
}

interface ReferralInviteRuntimeResponse {
  status: string;
  inviteState: string | null;
  referralCode: string | null;
}

interface BillingReferralSummaryResponse {
  status: 'ok';
  referralCode: string;
  pendingInvites: number;
  invites: Array<{
    invitedIdentifier: string;
    inviteState: string;
  }>;
}

interface AdminReferralsResponse {
  status: 'ok';
  actorRole: string;
  resultCount: number;
  results: Array<{
    referralCode: string;
    invitedFamilies: number;
    creditedFamilies: number;
  }>;
}

interface LicenseCheckRuntimeResponse {
  decision: string;
  reasonCode: string;
  currentActiveDevices: number;
  limit: number;
}

const testDir = path.dirname(fileURLToPath(import.meta.url));
const cloudflareDir = path.resolve(testDir, '..', '..');
const wranglerCommand = process.platform === 'win32' ? 'npx.cmd' : 'npx';
const runtimeDevVarsPath = path.join(cloudflareDir, '.dev.vars');

function localFixtureValue(parts: readonly string[], separator = '_'): string {
  return parts.join(separator);
}

const localRuntimeSecrets = {
  interactiveCsrfToken: localFixtureValue(['interactive', 'parent', 'session'], '-'),
  internalQueueSharedSecret: localFixtureValue(['local', 'runtime', 'internal', 'secret']),
  stripeSecretKey: localFixtureValue(['sk', 'local', 'runtime', 'secret']),
  stripeWebhookSecret: localFixtureValue(['whsec', 'local', 'runtime', 'secret']),
  razorpayKeyId: localFixtureValue(['rzp', 'local', 'runtime', 'key', 'id']),
  razorpayKeySecret: localFixtureValue(['rzp', 'local', 'runtime', 'key', 'secret']),
  paypalClientId: localFixtureValue(['paypal', 'local', 'runtime', 'client', 'id']),
  paypalClientSecret: localFixtureValue(['paypal', 'local', 'runtime', 'client', 'secret']),
  appleStoreKeyRef: localFixtureValue(['apple', 'local', 'runtime', 'key', 'ref']),
  googlePlayServiceAccountRef: localFixtureValue(['google', 'local', 'runtime', 'service', 'account', 'ref']),
  entitlementSigningKeyRef: localFixtureValue(['entitlement', 'local', 'runtime', 'signing', 'ref']),
} as const;

function quoteWindowsArgument(argument: string): string {
  return argument.includes(' ') ? `"${argument}"` : argument;
}

let runtimePromise: Promise<RuntimeHandle> | null = null;

function sleep(milliseconds: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
  });
}

function randomSuffix(): string {
  return `${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

function parseDotEnvFile(contents: string): Readonly<Record<string, string>> {
  const entries = contents
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.startsWith('#'))
    .map((line) => {
      const separator = line.indexOf('=');
      return separator === -1 ? [line, ''] : [line.slice(0, separator).trim(), line.slice(separator + 1).trim()];
    });
  return Object.fromEntries(entries);
}

function buildRuntimeDevVarsContents(): string {
  return [
    'ENVIRONMENT=local',
    'APP_ORIGIN=http://localhost:3000',
    'CORS_ALLOWED_ORIGINS=http://localhost:3000',
    'REQUEST_MAX_BYTES=1048576',
    'BILLING_ROUTE_KILL_SWITCH=false',
    'AUTH_ADAPTER_MODE=local-safe-fixture',
    `INTERACTIVE_CSRF_TOKEN=${localRuntimeSecrets.interactiveCsrfToken}`,
    `INTERNAL_QUEUE_SHARED_SECRET=${localRuntimeSecrets.internalQueueSharedSecret}`,
    `STRIPE_SECRET_KEY=${localRuntimeSecrets.stripeSecretKey}`,
    `STRIPE_WEBHOOK_SECRET=${localRuntimeSecrets.stripeWebhookSecret}`,
    `RAZORPAY_KEY_ID=${localRuntimeSecrets.razorpayKeyId}`,
    `RAZORPAY_KEY_SECRET=${localRuntimeSecrets.razorpayKeySecret}`,
    `PAYPAL_CLIENT_ID=${localRuntimeSecrets.paypalClientId}`,
    `PAYPAL_CLIENT_SECRET=${localRuntimeSecrets.paypalClientSecret}`,
    `APPLE_STORE_KEY_REF=${localRuntimeSecrets.appleStoreKeyRef}`,
    `GOOGLE_PLAY_SERVICE_ACCOUNT_REF=${localRuntimeSecrets.googlePlayServiceAccountRef}`,
    `ENTITLEMENT_SIGNING_KEY_REF=${localRuntimeSecrets.entitlementSigningKeyRef}`,
  ].join('\n');
}

function hasErrorCode(error: unknown, code: string): boolean {
  return typeof error === 'object' && error !== null && 'code' in error && (error as { code?: unknown }).code === code;
}

function readRuntimeDevVarsOrNull(): Readonly<Record<string, string>> | null {
  try {
    return parseDotEnvFile(readFileSync(runtimeDevVarsPath, 'utf8'));
  } catch (error) {
    if (hasErrorCode(error, 'ENOENT')) {
      return null;
    }
    throw error;
  }
}

function getStripeWebhookSecretFromRuntimeDevVars(parsed: Readonly<Record<string, string>>): string {
  const stripeWebhookSecret = parsed.STRIPE_WEBHOOK_SECRET;
  if (!stripeWebhookSecret) {
    throw new Error('existing infra/cloudflare/.dev.vars is missing STRIPE_WEBHOOK_SECRET');
  }
  return stripeWebhookSecret;
}

function writeRuntimeDevVarsIfMissing(contents: string): boolean {
  try {
    writeFileSync(runtimeDevVarsPath, contents, {
      encoding: 'utf8',
      flag: 'wx',
    });
    return true;
  } catch (error) {
    if (hasErrorCode(error, 'EEXIST')) {
      return false;
    }
    throw error;
  }
}

function ensureRuntimeDevVars(): {
  created: boolean;
  stripeWebhookSecret: string;
} {
  const existing = readRuntimeDevVarsOrNull();
  if (existing != null) {
    return {
      created: false,
      stripeWebhookSecret: getStripeWebhookSecretFromRuntimeDevVars(existing),
    };
  }

  if (writeRuntimeDevVarsIfMissing(buildRuntimeDevVarsContents())) {
    return {
      created: true,
      stripeWebhookSecret: localRuntimeSecrets.stripeWebhookSecret,
    };
  }

  const racedExisting = readRuntimeDevVarsOrNull();
  if (racedExisting == null) {
    throw new Error('infra/cloudflare/.dev.vars could not be read after concurrent creation');
  }
  return {
    created: false,
    stripeWebhookSecret: getStripeWebhookSecretFromRuntimeDevVars(racedExisting),
  };
}

function currentRuntimeDevVar(name: string, fallback: string): string {
  const parsed = readRuntimeDevVarsOrNull();
  if (parsed == null) {
    return fallback;
  }
  return parsed[name] ?? fallback;
}

function currentRuntimeStripeWebhookSecret(): string {
  return currentRuntimeDevVar('STRIPE_WEBHOOK_SECRET', localRuntimeSecrets.stripeWebhookSecret);
}

async function createHmacSignature(payload: string, secret: string): Promise<string> {
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

async function createStripeSignature(payload: string, secret: string): Promise<string> {
  const timestamp = 1_710_000_000;
  return `t=${timestamp},v1=${await createHmacSignature(`${timestamp}.${payload}`, secret)}`;
}

function getFreePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    const netServer = createServer();
    netServer.on('error', reject);
    netServer.listen(0, '127.0.0.1', () => {
      const address = netServer.address();
      if (!address || typeof address === 'string') {
        netServer.close();
        reject(new Error('failed to allocate a test port'));
        return;
      }
      const port = address.port;
      netServer.close((error) => {
        if (error) {
          reject(error);
          return;
        }
        resolve(port);
      });
    });
  });
}

async function stopRuntimeProcess(child: ChildProcess, persistPath: string, createdDevVars: boolean): Promise<void> {
  if (child.pid && child.exitCode === null && child.signalCode === null) {
    if (process.platform === 'win32') {
      spawnSync('taskkill', ['/pid', String(child.pid), '/t', '/f'], {
        stdio: 'ignore',
      });
    } else {
      child.kill('SIGTERM');
    }
  }

  await sleep(300);
  rmSync(persistPath, {
    force: true,
    recursive: true,
  });
  if (createdDevVars) {
    rmSync(runtimeDevVarsPath, {
      force: true,
    });
  }
}

async function waitForHealthyRuntime(
  baseUrl: string,
  child: ChildProcess,
  logs: { stdout: string[]; stderr: string[] }
): Promise<void> {
  const deadline = Date.now() + 20_000;
  let lastError: string | null = null;

  while (Date.now() < deadline) {
    if (child.exitCode !== null) {
      throw new Error(
        [
          `wrangler dev exited before health check completed: ${child.exitCode}`,
          `stdout: ${logs.stdout.join('')}`,
          `stderr: ${logs.stderr.join('')}`,
        ].join('\n')
      );
    }

    try {
      const response = await fetch(`${baseUrl}/health`);
      if (response.status === 200) {
        return;
      }
      lastError = `unexpected status ${response.status}: ${await response.text()}`;
    } catch (error) {
      lastError = error instanceof Error ? error.message : String(error);
    }

    await sleep(500);
  }

  throw new Error(
    [
      'wrangler dev did not become healthy before timeout',
      `last error: ${lastError ?? 'unknown'}`,
      `stdout: ${logs.stdout.join('')}`,
      `stderr: ${logs.stderr.join('')}`,
    ].join('\n')
  );
}

async function startRuntime(): Promise<RuntimeHandle> {
  const port = await getFreePort();
  const persistPath = path.join(os.tmpdir(), `ocentra-cloudflare-${randomSuffix()}`);
  const logs = {
    stdout: [] as string[],
    stderr: [] as string[],
  };
  const wranglerArgs = [
    'wrangler',
    'dev',
    '--local',
    '--var',
    'ENVIRONMENT:local',
    '--port',
    String(port),
    '--ip',
    '127.0.0.1',
    '--compatibility-date',
    '2025-07-12',
    '--persist-to',
    persistPath,
    '--show-interactive-dev-session=false',
    '--log-level',
    'warn',
  ];
  const runtimeLease = await acquireLocalWranglerRuntimeLease();
  let runtimeDevVars: ReturnType<typeof ensureRuntimeDevVars>;
  try {
    runtimeDevVars = ensureRuntimeDevVars();
  } catch (error) {
    runtimeLease.release();
    throw error;
  }
  let child: ChildProcess;
  try {
    child =
      process.platform === 'win32'
        ? spawn('cmd.exe', ['/d', '/s', '/c', [wranglerCommand, ...wranglerArgs].map(quoteWindowsArgument).join(' ')], {
            cwd: cloudflareDir,
            stdio: ['ignore', 'pipe', 'pipe'],
          })
        : spawn(wranglerCommand, wranglerArgs, {
            cwd: cloudflareDir,
            stdio: ['ignore', 'pipe', 'pipe'],
          });
  } catch (error) {
    if (runtimeDevVars.created) {
      rmSync(runtimeDevVarsPath, { force: true });
    }
    runtimeLease.release();
    throw error;
  }

  child.stdout?.setEncoding('utf8');
  child.stderr?.setEncoding('utf8');
  child.stdout?.on('data', (chunk: string) => {
    logs.stdout.push(chunk);
  });
  child.stderr?.on('data', (chunk: string) => {
    logs.stderr.push(chunk);
  });

  const baseUrl = `http://127.0.0.1:${port}`;
  try {
    await waitForHealthyRuntime(baseUrl, child, logs);
  } catch (error) {
    await stopRuntimeProcess(child, persistPath, runtimeDevVars.created);
    runtimeLease.release();
    throw error;
  }

  return {
    baseUrl,
    stop: async (): Promise<void> => {
      try {
        await stopRuntimeProcess(child, persistPath, runtimeDevVars.created);
      } finally {
        runtimeLease.release();
      }
    },
  };
}

async function getRuntime(): Promise<RuntimeHandle> {
  runtimePromise ??= startRuntime();
  return runtimePromise;
}

after(async () => {
  if (runtimePromise) {
    try {
      const runtime = await runtimePromise;
      await runtime.stop();
    } catch {
      return;
    }
  }
});

describe('wrangler local runtime', () => {
  it('serves health with seeded real local bindings', async () => {
    const runtime = await getRuntime();
    const response = await fetch(`${runtime.baseUrl}/health`);
    const body = (await response.json()) as HealthResponse;

    assert.equal(response.status, 200);
    assert.equal(body.status, 'ok');
    assert.equal(body.service, 'cloudflare-control-plane');
    assert.equal(body.bindingStatus, 'ready');
    assert.ok(Object.values(body.seedSummary.persistence).every((count) => count > 0));
    assert.equal(body.missingBindingCount, 0);
    assert.equal(body.seedSummary.pricingPlanCount, 3);
    assert.equal(body.seedSummary.adminAccountCount, 4);
    assert.equal(body.seedSummary.referralFixtureCount, 2);
  });

  it('serves pricing and billing status through the real worker HTTP surface', async () => {
    const runtime = await getRuntime();

    const pricingResponse = await fetch(`${runtime.baseUrl}/public/pricing`);
    const pricingBody = (await pricingResponse.json()) as PricingResponse;
    assert.equal(pricingResponse.status, 200);
    assert.equal(pricingBody.status, 'ok');
    assert.ok(pricingBody.plans.some((plan) => plan.planId === 'family-core'));

    const statusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-grace',
      },
    });
    const statusBody = (await statusResponse.json()) as BillingStatusResponse;
    assert.equal(statusResponse.status, 200);
    assert.equal(statusBody.status, 'ok');
    assert.equal(statusBody.parentAccountRef, 'parent-account:demo-grace');
    assert.equal(statusBody.portalVisibleState, 'degraded');
    assert.equal(statusBody.providerMode, 'stripe-hosted');
    assert.ok(!statusBody.warnings.includes('account-backend-not-wired'));
  });

  it('serves audit reads through the real worker runtime', async () => {
    const runtime = await getRuntime();
    const response = await fetch(`${runtime.baseUrl}/admin/billing/audit`, {
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const body = (await response.json()) as BillingAuditResponse;

    assert.equal(response.status, 200);
    assert.equal(body.status, 'ok');
    assert.equal(body.actorRole, 'admin');
    assert.ok(body.resultCount >= 1);
  });

  it('executes real hosted checkout and portal writes with durable audit persistence', async () => {
    const runtime = await getRuntime();

    const checkoutRequestInit = {
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        origin: 'http://localhost:3000',
        'content-type': 'application/json',
        'x-ocentra-csrf': 'interactive-parent-session',
      },
      body: JSON.stringify({
        requestId: 'runtime-checkout-session-proof',
        planId: 'family-core',
        successPath: '/family/billing/checkout/success',
        cancelPath: '/family/billing/checkout/cancel',
        abuseGateState: 'passed-turnstile',
      }),
    };
    const firstCheckoutResponse = await fetch(`${runtime.baseUrl}/auth/billing/checkout`, checkoutRequestInit);
    const firstCheckoutBody = (await firstCheckoutResponse.json()) as HostedSessionResponse;
    assert.equal(firstCheckoutResponse.status, 200);
    assert.equal(firstCheckoutBody.status, 'accepted');
    assert.equal(firstCheckoutBody.kind, 'checkout-session-create');
    assert.equal(firstCheckoutBody.ownerSubject, 'parent:demo-maxed');
    assert.equal(firstCheckoutBody.pendingEntitlementConfirmation, true);
    assert.ok(firstCheckoutBody.hostedUrl?.startsWith('https://checkout.stripe.com/'));

    const replayCheckoutResponse = await fetch(`${runtime.baseUrl}/auth/billing/checkout`, checkoutRequestInit);
    const replayCheckoutBody = (await replayCheckoutResponse.json()) as HostedSessionResponse;
    assert.equal(replayCheckoutResponse.status, 200);
    assert.deepEqual(replayCheckoutBody, firstCheckoutBody);

    const portalResponse = await fetch(`${runtime.baseUrl}/auth/billing/portal`, {
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        origin: 'http://localhost:3000',
        'content-type': 'application/json',
        'x-ocentra-csrf': 'interactive-parent-session',
      },
      body: JSON.stringify({
        requestId: 'runtime-portal-session-proof',
        returnPath: '/family/billing/manage',
        abuseGateState: 'trusted-authenticated-session',
      }),
    });
    const portalBody = (await portalResponse.json()) as HostedSessionResponse;
    assert.equal(portalResponse.status, 200);
    assert.equal(portalBody.status, 'accepted');
    assert.equal(portalBody.kind, 'billing-portal-session-create');
    assert.equal(portalBody.ownerSubject, 'parent:demo-maxed');
    assert.ok(portalBody.hostedUrl?.startsWith('https://billing.stripe.com/'));

    const auditResponse = await fetch(`${runtime.baseUrl}/admin/billing/audit?q=runtime-`, {
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const auditBody = (await auditResponse.json()) as BillingAuditResponse;
    const checkoutEvents =
      auditBody.results?.filter((row) => row.eventId === 'billing-checkout-session:runtime-checkout-session-proof') ??
      [];
    const portalEvents =
      auditBody.results?.filter((row) => row.eventId === 'billing-portal-session:runtime-portal-session-proof') ?? [];
    assert.equal(auditResponse.status, 200);
    assert.equal(checkoutEvents.length, 1);
    assert.equal(checkoutEvents[0]?.eventType, 'billing.checkout-session.created');
    assert.equal(portalEvents.length, 1);
    assert.equal(portalEvents[0]?.eventType, 'billing.portal-session.created');
  });

  it('executes a real payment change-plan write with durable-object idempotency', async () => {
    const runtime = await getRuntime();
    const requestBody = {
      requestId: 'runtime-change-plan-proof',
      planId: 'family-max',
      abuseGateState: 'passed-turnstile',
    };
    const requestInit = {
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-active',
        origin: 'http://localhost:3000',
        'content-type': 'application/json',
        'x-ocentra-csrf': 'interactive-parent-session',
      },
      body: JSON.stringify(requestBody),
    };

    const firstResponse = await fetch(`${runtime.baseUrl}/auth/billing/change-plan`, requestInit);
    const firstBody = (await firstResponse.json()) as BillingPlanChangeResponse;
    assert.equal(firstResponse.status, 200);
    assert.equal(firstBody.status, 'accepted');
    assert.equal(firstBody.changeKind, 'upgrade');
    assert.equal(firstBody.targetPlanId, 'family-max');

    const secondResponse = await fetch(`${runtime.baseUrl}/auth/billing/change-plan`, requestInit);
    const secondBody = (await secondResponse.json()) as BillingPlanChangeResponse;
    assert.equal(secondResponse.status, 200);
    assert.deepEqual(secondBody, firstBody);

    const statusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const statusBody = (await statusResponse.json()) as BillingStatusResponse;
    assert.equal(statusResponse.status, 200);
    assert.equal(statusBody.plan.planId, 'family-max');
    assert.equal(statusBody.deviceUsage.limit, 10);
    assert.ok(statusBody.warnings.includes('plan-change-pending-provider-sync'));

    const snapshotResponse = await fetch(`${runtime.baseUrl}/auth/billing/entitlement-snapshot`, {
      headers: {
        authorization: 'Bearer parent:demo-active',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const snapshotBody = (await snapshotResponse.json()) as BillingEntitlementSnapshotResponse;
    assert.equal(snapshotResponse.status, 200);
    assert.equal(snapshotBody.snapshot.planId, 'family-max');
    assert.equal(snapshotBody.snapshot.deviceLimit, 10);

    const cancelResponse = await fetch(`${runtime.baseUrl}/auth/billing/cancel`, {
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-active',
        origin: 'http://localhost:3000',
        'content-type': 'application/json',
        'x-ocentra-csrf': 'interactive-parent-session',
      },
      body: JSON.stringify({
        requestId: 'runtime-cancel-proof',
        abuseGateState: 'trusted-authenticated-session',
      }),
    });
    assert.equal(cancelResponse.status, 200);

    const cancelledStatusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const cancelledStatusBody = (await cancelledStatusResponse.json()) as BillingStatusResponse;
    assert.equal(cancelledStatusResponse.status, 200);
    assert.ok(cancelledStatusBody.warnings.includes('cancellation-scheduled-period-end'));
  });

  it('processes a real signed Stripe webhook and promotes billing state through the worker runtime', async () => {
    const runtime = await getRuntime();
    const payload = JSON.stringify({
      id: 'evt_runtime_invoice_paid',
      type: 'invoice.paid',
      subject: 'parent:demo-grace',
    });
    const signature = await createStripeSignature(payload, currentRuntimeStripeWebhookSecret());

    const firstResponse = await fetch(`${runtime.baseUrl}/webhooks/stripe`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        'stripe-signature': signature,
      },
      body: payload,
    });
    const firstBody = (await firstResponse.json()) as {
      status: string;
      provider: string;
      eventId: string;
    };
    assert.equal(firstResponse.status, 202);
    assert.equal(firstBody.status, 'accepted');
    assert.equal(firstBody.provider, 'stripe');
    assert.equal(LOCAL_WEBHOOK_FIXTURE_INVENTORY.includes(firstBody.provider), true);
    assert.equal(firstBody.eventId, 'evt_runtime_invoice_paid');

    const replayResponse = await fetch(`${runtime.baseUrl}/webhooks/stripe`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        'stripe-signature': signature,
      },
      body: payload,
    });
    assert.equal(replayResponse.status, 202);

    const statusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-grace',
      },
    });
    const statusBody = (await statusResponse.json()) as BillingStatusResponse;
    assert.equal(statusResponse.status, 200);
    assert.equal(statusBody.accountStatus, 'active');
    assert.equal(statusBody.subscriptionStatus, 'active');
    assert.equal(statusBody.parentVisibleState, 'available');
    assert.ok(statusBody.warnings.includes('provider-webhook-synced'));

    const invoicesResponse = await fetch(`${runtime.baseUrl}/auth/billing/invoices`, {
      headers: {
        authorization: 'Bearer parent:demo-grace',
      },
    });
    const invoicesBody = (await invoicesResponse.json()) as {
      invoices: Array<{
        paymentState: string;
      }>;
    };
    assert.equal(invoicesResponse.status, 200);
    assert.ok(invoicesBody.invoices.every((invoice) => invoice.paymentState === 'paid'));

    const snapshotResponse = await fetch(`${runtime.baseUrl}/auth/billing/entitlement-snapshot`, {
      headers: {
        authorization: 'Bearer parent:demo-grace',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const snapshotBody = (await snapshotResponse.json()) as BillingEntitlementSnapshotResponse;
    assert.equal(snapshotResponse.status, 200);
    assert.equal(snapshotBody.snapshot.planId, 'family-max');
    assert.equal(snapshotBody.snapshot.subscriptionStatus, 'active');
    assert.equal(snapshotBody.snapshot.signatureState, 'signed');
  });

  it('processes real signed Stripe failure and dispute lifecycle events through the worker runtime', async () => {
    const runtime = await getRuntime();

    const paymentFailedPayload = JSON.stringify({
      id: 'evt_runtime_payment_failed',
      type: 'payment_failed',
      subject: 'parent:demo-maxed',
    });
    const paymentFailedSignature = await createStripeSignature(
      paymentFailedPayload,
      currentRuntimeStripeWebhookSecret()
    );
    const paymentFailedResponse = await fetch(`${runtime.baseUrl}/webhooks/stripe`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        'stripe-signature': paymentFailedSignature,
      },
      body: paymentFailedPayload,
    });
    assert.equal(paymentFailedResponse.status, 202);

    const graceStatusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-maxed',
      },
    });
    const graceStatusBody = (await graceStatusResponse.json()) as BillingStatusResponse;
    assert.equal(graceStatusResponse.status, 200);
    assert.equal(graceStatusBody.accountStatus, 'grace');
    assert.equal(graceStatusBody.subscriptionStatus, 'grace');
    assert.equal(graceStatusBody.parentVisibleState, 'grace');
    assert.ok(graceStatusBody.warnings.includes('provider-webhook-payment-required'));

    const graceSnapshotResponse = await fetch(`${runtime.baseUrl}/auth/billing/entitlement-snapshot`, {
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const graceSnapshotBody = (await graceSnapshotResponse.json()) as BillingEntitlementSnapshotResponse;
    assert.equal(graceSnapshotResponse.status, 200);
    assert.equal(graceSnapshotBody.snapshot.subscriptionStatus, 'grace');
    assert.equal(graceSnapshotBody.snapshot.parentVisibleState, 'grace');
    assert.equal(graceSnapshotBody.snapshot.signatureState, 'signed');

    const disputeOpenPayload = JSON.stringify({
      id: 'evt_runtime_dispute_open',
      type: 'dispute_open',
      subject: 'parent:demo-maxed',
      disputeId: 'dp_runtime_maxed',
      invoiceId: 'parent-demo-maxed-invoice-current',
    });
    const disputeOpenSignature = await createStripeSignature(disputeOpenPayload, currentRuntimeStripeWebhookSecret());
    const disputeOpenResponse = await fetch(`${runtime.baseUrl}/webhooks/stripe`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        'stripe-signature': disputeOpenSignature,
      },
      body: disputeOpenPayload,
    });
    assert.equal(disputeOpenResponse.status, 202);

    const openedStatusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-maxed',
      },
    });
    const openedStatusBody = (await openedStatusResponse.json()) as BillingStatusResponse;
    assert.equal(openedStatusResponse.status, 200);
    assert.equal(openedStatusBody.accountStatus, 'manual-review');
    assert.equal(openedStatusBody.subscriptionStatus, 'past-due');
    assert.equal(openedStatusBody.parentVisibleState, 'manual-review');

    const openedDisputeResponse = await fetch(`${runtime.baseUrl}/admin/billing/disputes?q=dp_runtime_maxed`, {
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const openedDisputeBody = (await openedDisputeResponse.json()) as BillingAuditResponse;
    assert.equal(openedDisputeResponse.status, 200);
    assert.equal(openedDisputeBody.results?.[0]?.disputeState, 'dispute-opened');

    const disputeWonPayload = JSON.stringify({
      id: 'evt_runtime_dispute_won',
      type: 'dispute_won',
      subject: 'parent:demo-maxed',
      disputeId: 'dp_runtime_maxed',
      invoiceId: 'parent-demo-maxed-invoice-current',
    });
    const disputeWonSignature = await createStripeSignature(disputeWonPayload, currentRuntimeStripeWebhookSecret());
    const disputeWonResponse = await fetch(`${runtime.baseUrl}/webhooks/stripe`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        'stripe-signature': disputeWonSignature,
      },
      body: disputeWonPayload,
    });
    assert.equal(disputeWonResponse.status, 202);

    const recoveredStatusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-maxed',
      },
    });
    const recoveredStatusBody = (await recoveredStatusResponse.json()) as BillingStatusResponse;
    assert.equal(recoveredStatusResponse.status, 200);
    assert.equal(recoveredStatusBody.accountStatus, 'active');
    assert.equal(recoveredStatusBody.subscriptionStatus, 'active');
    assert.equal(recoveredStatusBody.parentVisibleState, 'available');

    const recoveredDisputeResponse = await fetch(`${runtime.baseUrl}/admin/billing/disputes?q=dp_runtime_maxed`, {
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const recoveredDisputeBody = (await recoveredDisputeResponse.json()) as BillingAuditResponse;
    assert.equal(recoveredDisputeResponse.status, 200);
    assert.equal(recoveredDisputeBody.results?.[0]?.disputeState, 'dispute-won');
    assert.equal(recoveredDisputeBody.results?.[0]?.manualRequired, false);
  });

  it('processes real support and admin payment operations through the worker runtime', async () => {
    const runtime = await getRuntime();

    const manualInvoiceResponse = await fetch(`${runtime.baseUrl}/auth/billing/manual-invoice`, {
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:support-agent',
        'content-type': 'application/json',
        'x-ocentra-role': 'support',
      },
      body: JSON.stringify({
        requestId: 'runtime-manual-invoice-proof',
        region: 'pakistan',
      }),
    });
    const manualInvoiceBody = (await manualInvoiceResponse.json()) as ManualInvoiceRuntimeResponse;
    assert.equal(manualInvoiceResponse.status, 202);
    assert.equal(manualInvoiceBody.status, 'accepted');
    assert.equal(manualInvoiceBody.invoiceState, 'manual-support-required');
    assert.equal(manualInvoiceBody.queued, true);
    assert.equal(manualInvoiceBody.region, 'pakistan');

    const supportStatusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:support-agent',
        'x-ocentra-role': 'support',
      },
    });
    const supportStatusBody = (await supportStatusResponse.json()) as BillingStatusResponse;
    assert.equal(supportStatusResponse.status, 200);
    assert.equal(supportStatusBody.accountStatus, 'manual-review');
    assert.equal(supportStatusBody.subscriptionStatus, 'past-due');
    assert.equal(supportStatusBody.parentVisibleState, 'manual-review');
    assert.equal(supportStatusBody.providerMode, 'manual-invoice');

    const adminInvoiceSearchResponse = await fetch(
      `${runtime.baseUrl}/admin/billing/invoices?q=runtime-manual-invoice-proof`,
      {
        headers: {
          authorization: 'Bearer parent:admin-agent',
          'x-ocentra-role': 'admin',
        },
      }
    );
    const adminInvoiceSearchBody = (await adminInvoiceSearchResponse.json()) as AdminInvoicesResponse;
    const runtimeManualInvoice = adminInvoiceSearchBody.results.find((row) =>
      row.invoiceId.includes('runtime-manual-invoice-proof')
    );
    assert.equal(adminInvoiceSearchResponse.status, 200);
    assert.equal(adminInvoiceSearchBody.actorRole, 'admin');
    assert.ok(adminInvoiceSearchBody.resultCount >= 1);
    assert.equal(runtimeManualInvoice?.manualRequired, true);
    assert.equal(runtimeManualInvoice?.paymentState, 'unpaid');

    const refundResponse = await fetch(`${runtime.baseUrl}/admin/billing/refunds`, {
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'content-type': 'application/json',
        'x-ocentra-role': 'admin',
      },
      body: JSON.stringify({
        requestId: 'runtime-refund-proof',
        invoiceId: 'parent-demo-active-invoice-current',
      }),
    });
    const refundBody = (await refundResponse.json()) as RefundRuntimeResponse;
    assert.equal(refundResponse.status, 200);
    assert.equal(refundBody.status, 'accepted');
    assert.equal(refundBody.refundState, 'refund-settled');
    assert.equal(refundBody.invoiceId, 'parent-demo-active-invoice-current');

    const refundedStatusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const refundedStatusBody = (await refundedStatusResponse.json()) as BillingStatusResponse;
    assert.equal(refundedStatusResponse.status, 200);
    assert.equal(refundedStatusBody.accountStatus, 'manual-review');
    assert.equal(refundedStatusBody.subscriptionStatus, 'past-due');
    assert.equal(refundedStatusBody.parentVisibleState, 'manual-review');

    const refundedInvoicesResponse = await fetch(`${runtime.baseUrl}/auth/billing/invoices`, {
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const refundedInvoicesBody = (await refundedInvoicesResponse.json()) as BillingInvoicesResponse;
    const refundedInvoice = refundedInvoicesBody.invoices.find(
      (row) => row.invoiceId === 'parent-demo-active-invoice-current'
    );
    assert.equal(refundedInvoicesResponse.status, 200);
    assert.equal(refundedInvoice?.paymentState, 'refunded');

    const reconciliationResponse = await fetch(`${runtime.baseUrl}/admin/billing/reconciliation`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        'x-ocentra-internal-call': 'true',
        'x-ocentra-internal-secret': currentRuntimeDevVar(
          'INTERNAL_QUEUE_SHARED_SECRET',
          localRuntimeSecrets.internalQueueSharedSecret
        ),
      },
      body: JSON.stringify({
        requestId: 'runtime-reconciliation-proof',
      }),
    });
    const reconciliationBody = (await reconciliationResponse.json()) as ReconciliationRuntimeResponse;
    assert.equal(reconciliationResponse.status, 202);
    assert.equal(reconciliationBody.status, 'accepted');
    assert.equal(reconciliationBody.queued, true);
    assert.equal(reconciliationBody.driftFamiliesVisible, 2);
    assert.equal(reconciliationBody.retryBacklogVisible, 1);
    assert.equal(reconciliationBody.deadLetterVisible, 0);

    const adminAccountsResponse = await fetch(`${runtime.baseUrl}/admin/billing/accounts?q=manual-review`, {
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const adminAccountsBody = (await adminAccountsResponse.json()) as AdminAccountsResponse;
    assert.equal(adminAccountsResponse.status, 200);
    assert.equal(adminAccountsBody.actorRole, 'admin');
    assert.ok(adminAccountsBody.resultCount >= 1);
    assert.ok(adminAccountsBody.manualActionsPending >= 1);

    const auditResponse = await fetch(`${runtime.baseUrl}/admin/billing/audit?q=runtime-`, {
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const auditBody = (await auditResponse.json()) as BillingAuditResponse;
    const manualInvoiceEvents =
      auditBody.results?.filter((row) => row.eventId === 'billing-manual-invoice:runtime-manual-invoice-proof') ?? [];
    const refundEvents =
      auditBody.results?.filter((row) => row.eventId === 'billing-refund:runtime-refund-proof') ?? [];
    const reconciliationEvents =
      auditBody.results?.filter((row) => row.eventId === 'billing-reconciliation:runtime-reconciliation-proof') ?? [];
    assert.equal(auditResponse.status, 200);
    assert.equal(manualInvoiceEvents.length, 1);
    assert.equal(manualInvoiceEvents[0]?.eventType, 'billing.manual-invoice.created');
    assert.equal(refundEvents.length, 1);
    assert.equal(refundEvents[0]?.eventType, 'billing.refund.refund-settled');
    assert.equal(reconciliationEvents.length, 1);
    assert.equal(reconciliationEvents[0]?.eventType, 'billing.reconciliation.accepted');
  });

  it('processes real signed non-stripe provider lifecycle transitions through the worker runtime', async () => {
    const runtime = await getRuntime();

    const razorpayPayload = JSON.stringify({
      id: 'rzp_evt_runtime_active',
      type: 'subscription.charged',
      subject: 'parent:demo-review',
    });
    const razorpaySignature = await createHmacSignature(
      razorpayPayload,
      currentRuntimeDevVar('RAZORPAY_KEY_SECRET', localRuntimeSecrets.razorpayKeySecret)
    );
    const razorpayResponse = await fetch(`${runtime.baseUrl}/webhooks/razorpay`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        'x-razorpay-signature': razorpaySignature,
      },
      body: razorpayPayload,
    });
    const razorpayBody = (await razorpayResponse.json()) as ProviderWebhookRuntimeResponse;
    assert.equal(razorpayResponse.status, 202);
    assert.equal(razorpayBody.status, 'accepted');
    assert.equal(razorpayBody.provider, 'razorpay');

    const activatedStatusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-review',
      },
    });
    const activatedStatusBody = (await activatedStatusResponse.json()) as BillingStatusResponse;
    assert.equal(activatedStatusResponse.status, 200);
    assert.equal(activatedStatusBody.accountStatus, 'active');
    assert.equal(activatedStatusBody.subscriptionStatus, 'active');
    assert.equal(activatedStatusBody.parentVisibleState, 'available');

    const paypalPayload = JSON.stringify({
      id: 'paypal_evt_runtime_failed',
      type: 'payment_failed',
      subject: 'parent:demo-review',
    });
    const paypalTransmissionId = 'paypal-runtime-failed';
    const paypalSignature = await createHmacSignature(
      `${paypalTransmissionId}.${paypalPayload}`,
      currentRuntimeDevVar('PAYPAL_CLIENT_SECRET', localRuntimeSecrets.paypalClientSecret)
    );
    const paypalResponse = await fetch(`${runtime.baseUrl}/webhooks/paypal`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        'paypal-transmission-id': paypalTransmissionId,
        'paypal-transmission-sig': paypalSignature,
      },
      body: paypalPayload,
    });
    const paypalBody = (await paypalResponse.json()) as ProviderWebhookRuntimeResponse;
    assert.equal(paypalResponse.status, 202);
    assert.equal(paypalBody.status, 'accepted');
    assert.equal(paypalBody.provider, 'paypal');

    const graceStatusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-review',
      },
    });
    const graceStatusBody = (await graceStatusResponse.json()) as BillingStatusResponse;
    assert.equal(graceStatusResponse.status, 200);
    assert.equal(graceStatusBody.accountStatus, 'grace');
    assert.equal(graceStatusBody.subscriptionStatus, 'grace');
    assert.equal(graceStatusBody.parentVisibleState, 'grace');
    assert.ok(graceStatusBody.warnings.includes('provider-webhook-payment-required'));

    const googlePayload = JSON.stringify({
      id: 'google_evt_runtime_renewed',
      type: 'SUBSCRIPTION_RENEWED',
      subject: 'parent:demo-review',
    });
    const googleSignature = await createHmacSignature(
      googlePayload,
      currentRuntimeDevVar('GOOGLE_PLAY_SERVICE_ACCOUNT_REF', localRuntimeSecrets.googlePlayServiceAccountRef)
    );
    const googleResponse = await fetch(`${runtime.baseUrl}/webhooks/google`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        'x-goog-signature': googleSignature,
      },
      body: googlePayload,
    });
    const googleBody = (await googleResponse.json()) as ProviderWebhookRuntimeResponse;
    assert.equal(googleResponse.status, 202);
    assert.equal(googleBody.status, 'accepted');
    assert.equal(googleBody.provider, 'google');

    const applePayload = JSON.stringify({
      id: 'apple_evt_runtime_renewed',
      type: 'DID_RENEW',
      subject: 'parent:demo-review',
    });
    const appleResponse = await fetch(`${runtime.baseUrl}/webhooks/apple`, {
      method: 'POST',
      headers: {
        authorization: `Bearer ${currentRuntimeDevVar('APPLE_STORE_KEY_REF', localRuntimeSecrets.appleStoreKeyRef)}`,
      },
      body: applePayload,
    });
    const appleBody = (await appleResponse.json()) as ProviderWebhookRuntimeResponse;
    assert.equal(appleResponse.status, 202);
    assert.equal(appleBody.status, 'accepted');
    assert.equal(appleBody.provider, 'apple');
    const executedProviders = [razorpayBody.provider, paypalBody.provider, googleBody.provider, appleBody.provider];
    assert.deepEqual(
      executedProviders.sort(),
      LOCAL_WEBHOOK_FIXTURE_INVENTORY.filter((provider) => provider !== 'stripe').sort()
    );

    const recoveredStatusResponse = await fetch(`${runtime.baseUrl}/auth/billing/status`, {
      headers: {
        authorization: 'Bearer parent:demo-review',
      },
    });
    const recoveredStatusBody = (await recoveredStatusResponse.json()) as BillingStatusResponse;
    assert.equal(recoveredStatusResponse.status, 200);
    assert.equal(recoveredStatusBody.accountStatus, 'active');
    assert.equal(recoveredStatusBody.subscriptionStatus, 'active');
    assert.equal(recoveredStatusBody.parentVisibleState, 'available');

    const auditResponse = await fetch(`${runtime.baseUrl}/admin/billing/audit?q=runtime_`, {
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const auditBody = (await auditResponse.json()) as BillingAuditResponse;
    const razorpayEvents =
      auditBody.results?.filter((row) => row.eventId === 'billing-webhook:razorpay:rzp_evt_runtime_active') ?? [];
    const paypalEvents =
      auditBody.results?.filter((row) => row.eventId === 'billing-webhook:paypal:paypal_evt_runtime_failed') ?? [];
    const googleEvents =
      auditBody.results?.filter((row) => row.eventId === 'billing-webhook:google:google_evt_runtime_renewed') ?? [];
    const appleEvents =
      auditBody.results?.filter((row) => row.eventId === 'billing-webhook:apple:apple_evt_runtime_renewed') ?? [];
    assert.equal(auditResponse.status, 200);
    assert.equal(razorpayEvents.length, 1);
    assert.equal(paypalEvents.length, 1);
    assert.equal(googleEvents.length, 1);
    assert.equal(appleEvents.length, 1);
  });

  it('processes real referral invite writes and direct license decisions through the worker runtime', async () => {
    const runtime = await getRuntime();

    const referralsBeforeResponse = await fetch(`${runtime.baseUrl}/auth/billing/referrals`, {
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const referralsBeforeBody = (await referralsBeforeResponse.json()) as BillingReferralSummaryResponse;
    assert.equal(referralsBeforeResponse.status, 200);

    const adminReferralsBeforeResponse = await fetch(`${runtime.baseUrl}/admin/billing/referrals`, {
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const adminReferralsBeforeBody = (await adminReferralsBeforeResponse.json()) as AdminReferralsResponse;
    const adminReferralBefore = adminReferralsBeforeBody.results.find(
      (row) => row.referralCode === referralsBeforeBody.referralCode
    );
    assert.equal(adminReferralsBeforeResponse.status, 200);

    const inviteResponse = await fetch(`${runtime.baseUrl}/auth/billing/referral-invite`, {
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-active',
        origin: 'http://localhost:3000',
        'content-type': 'application/json',
        'x-ocentra-csrf': 'interactive-parent-session',
      },
      body: JSON.stringify({
        requestId: 'runtime-referral-proof',
        invitee: 'runtime-family@example.com',
        abuseGateState: 'passed-turnstile',
      }),
    });
    const inviteBody = (await inviteResponse.json()) as ReferralInviteRuntimeResponse;
    assert.equal(inviteResponse.status, 200);
    assert.equal(inviteBody.status, 'accepted');
    assert.equal(inviteBody.inviteState, 'invite-created');
    assert.equal(inviteBody.referralCode, referralsBeforeBody.referralCode);

    const referralsAfterResponse = await fetch(`${runtime.baseUrl}/auth/billing/referrals`, {
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const referralsAfterBody = (await referralsAfterResponse.json()) as BillingReferralSummaryResponse;
    assert.equal(referralsAfterResponse.status, 200);
    assert.equal(referralsAfterBody.pendingInvites, referralsBeforeBody.pendingInvites + 1);
    assert.ok(
      referralsAfterBody.invites.some(
        (row) => row.invitedIdentifier === 'runtime-family@example.com' && row.inviteState === 'invite-created'
      )
    );

    const adminReferralsAfterResponse = await fetch(
      `${runtime.baseUrl}/admin/billing/referrals?q=${referralsAfterBody.referralCode}`,
      {
        headers: {
          authorization: 'Bearer parent:admin-agent',
          'x-ocentra-role': 'admin',
        },
      }
    );
    const adminReferralsAfterBody = (await adminReferralsAfterResponse.json()) as AdminReferralsResponse;
    const adminReferralAfter = adminReferralsAfterBody.results.find(
      (row) => row.referralCode === referralsAfterBody.referralCode
    );
    assert.equal(adminReferralsAfterResponse.status, 200);
    assert.equal(adminReferralsAfterBody.actorRole, 'admin');
    assert.equal(adminReferralAfter?.invitedFamilies, (adminReferralBefore?.invitedFamilies ?? 0) + 1);
    assert.equal(adminReferralAfter?.creditedFamilies, adminReferralBefore?.creditedFamilies);

    const deniedLicenseResponse = await fetch(`${runtime.baseUrl}/auth/billing/license-check`, {
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        'content-type': 'application/json',
        'x-ocentra-trusted-device': 'true',
      },
      body: JSON.stringify({
        requestId: 'runtime-license-maxed',
        deviceId: 'device-runtime-6',
        requestedNewDevice: true,
      }),
    });
    const deniedLicenseBody = (await deniedLicenseResponse.json()) as LicenseCheckRuntimeResponse;
    assert.equal(deniedLicenseResponse.status, 200);
    assert.equal(deniedLicenseBody.decision, 'denied');
    assert.equal(deniedLicenseBody.reasonCode, 'limit-exceeded');
    assert.equal(deniedLicenseBody.currentActiveDevices, 5);
    assert.equal(deniedLicenseBody.limit, 5);

    const allowedLicenseResponse = await fetch(`${runtime.baseUrl}/auth/billing/license-check`, {
      method: 'POST',
      headers: {
        authorization: 'Bearer parent:demo-maxed',
        'content-type': 'application/json',
        'x-ocentra-trusted-device': 'true',
      },
      body: JSON.stringify({
        requestId: 'runtime-license-existing',
        deviceId: 'device-runtime-2',
        requestedNewDevice: false,
      }),
    });
    const allowedLicenseBody = (await allowedLicenseResponse.json()) as LicenseCheckRuntimeResponse;
    assert.equal(allowedLicenseResponse.status, 200);
    assert.equal(allowedLicenseBody.decision, 'allowed');
    assert.equal(allowedLicenseBody.reasonCode, 'within-plan');
    assert.equal(allowedLicenseBody.currentActiveDevices, 5);
    assert.equal(allowedLicenseBody.limit, 5);
  });
});

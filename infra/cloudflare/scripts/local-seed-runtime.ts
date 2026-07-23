import { randomUUID } from 'node:crypto';
import { spawn, spawnSync, type ChildProcess } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { createServer } from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { buildLocalSeedSnapshot } from '../src/billing-binding-read-model.js';
import type { Env } from '../src/env.js';

export const LOCAL_WEBHOOK_FIXTURE_INVENTORY = ['stripe', 'razorpay', 'paypal', 'google', 'apple'] as const;
export const LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY = ['accepted-replay', 'dead-letter-replay'] as const;
export const LOCAL_SEED_RUNTIME_PID_FILE = '.ocentra-cloudflare-local-seed-runtime.pid';

export interface LocalSeedPersistenceEvidence {
  d1StatusRows: number;
  d1AdminAccountRows: number;
  d1ReferralRows: number;
  kvPricingPlanRows: number;
  r2AuditEventRows: number;
}

export interface LocalSeedMutationReceipt {
  runId: string;
  requestedFamily: string;
  persistenceTarget: 'explicit' | 'wrangler-default';
  runtimeBootStatus: 'proven';
  runtimeBootEvidence: string;
  fullBindingSeedApplied: true;
  persistence: LocalSeedPersistenceEvidence;
  webhookFixtureCount: number;
  queueReplayFixtureCount: number;
}

interface LocalSeedHealthResponse {
  status: 'ok';
  service: string;
  bindingStatus: string;
  missingBindingCount: number;
  seedSummary: {
    persistence: LocalSeedPersistenceEvidence;
  };
}

interface RuntimeHandle {
  child: ChildProcess;
  baseUrl: string;
  logs: { stdout: string[]; stderr: string[] };
}

export interface LocalWranglerRuntimeLease {
  release: () => void;
}

export interface LocalWranglerRuntimeLeaseOptions {
  lockPath?: string;
  acquireTimeoutMs?: number;
  heartbeatIntervalMs?: number;
  pollIntervalMs?: number;
  invalidRecordStaleAfterMs?: number;
}

interface RuntimeLeaseRecord {
  schema: 1;
  token: string;
  ownerPid: number;
  createdAt: string;
  heartbeatAt: string;
}

interface SeedRuntimeMilestone {
  runId: string;
  correlationId: string;
  owner: string;
  boundary: string;
  result: 'accepted' | 'proven' | 'rejected' | 'released';
  noClaimReason: string;
  redactionState: 'redacted-safe-fields-only';
  requestedFamily: string;
  failureKind?: string;
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const cloudflareDir = path.resolve(scriptDir, '..');
const loggerModuleUrl = new URL('../../../packages/logging-domain/src/core/logger.ts', import.meta.url).href;
const stackTraceModuleUrl = new URL('../../../packages/logging-domain/src/core/stackTrace.ts', import.meta.url).href;
const [{ Logger }, { getStackTrace }] = await Promise.all([import(loggerModuleUrl), import(stackTraceModuleUrl)]);
const defaultPersistPath = path.join(cloudflareDir, '.wrangler', 'state', 'v3');
const runtimeLeasePath = path.join(os.tmpdir(), 'ocentra-cloudflare-wrangler-runtime.lock');
const seedRuntimeOwner = 'infra/cloudflare/scripts/local-seed-runtime.ts';
const seedRuntimeNoClaimReason = 'local-seed-only;production-deployment-not-owned';
const log = Logger.instance;
function expectedPersistenceEvidence(): LocalSeedPersistenceEvidence {
  const seed = buildLocalSeedSnapshot({ ENVIRONMENT: 'development' } as Env);
  return {
    d1StatusRows: Object.keys(seed.statusBySubject ?? {}).length,
    d1AdminAccountRows: seed.adminAccounts?.length ?? 0,
    d1ReferralRows: seed.adminReferrals?.length ?? 0,
    kvPricingPlanRows: seed.pricingPlans?.length ?? 0,
    r2AuditEventRows: seed.auditEvents?.length ?? 0,
  };
}

function sleep(milliseconds: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
  });
}

function hasErrorCode(error: unknown, code: string): boolean {
  return typeof error === 'object' && error !== null && 'code' in error && (error as { code?: unknown }).code === code;
}

function buildLeaseRecord(token: string, createdAt: string = new Date().toISOString()): RuntimeLeaseRecord {
  return {
    schema: 1,
    token,
    ownerPid: process.pid,
    createdAt,
    heartbeatAt: new Date().toISOString(),
  };
}

function parseLeaseRecord(raw: string): RuntimeLeaseRecord | null {
  try {
    const parsed = JSON.parse(raw) as Partial<RuntimeLeaseRecord>;
    if (
      parsed.schema === 1 &&
      typeof parsed.token === 'string' &&
      Number.isInteger(parsed.ownerPid) &&
      typeof parsed.createdAt === 'string' &&
      typeof parsed.heartbeatAt === 'string'
    ) {
      return parsed as RuntimeLeaseRecord;
    }
  } catch {
    const legacyOwnerPid = Number(raw.split(':', 1)[0]);
    if (Number.isInteger(legacyOwnerPid) && legacyOwnerPid > 0) {
      const now = new Date().toISOString();
      return {
        schema: 1,
        token: raw,
        ownerPid: legacyOwnerPid,
        createdAt: now,
        heartbeatAt: now,
      };
    }
  }
  return null;
}

function isProcessAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return hasErrorCode(error, 'EPERM');
  }
}

function writeLeaseRecord(lockPath: string, record: RuntimeLeaseRecord, flag?: 'wx'): void {
  writeFileSync(lockPath, JSON.stringify(record), flag ? { encoding: 'utf8', flag } : { encoding: 'utf8' });
}

function removeStaleLeaseIfTokenMatches(lockPath: string, observedToken: string): boolean {
  const reclaimPath = `${lockPath}.${observedToken.replaceAll(':', '-')}.reclaim`;
  try {
    writeFileSync(reclaimPath, observedToken, { encoding: 'utf8', flag: 'wx' });
  } catch (error) {
    if (hasErrorCode(error, 'EEXIST') || hasErrorCode(error, 'ENOENT')) {
      return false;
    }
    throw error;
  }
  try {
    const current = parseLeaseRecord(readFileSync(lockPath, 'utf8'));
    if (current?.token !== observedToken) {
      return false;
    }
    rmSync(lockPath);
    return true;
  } finally {
    rmSync(reclaimPath, { force: true });
  }
}

export async function acquireLocalWranglerRuntimeLease(
  options: LocalWranglerRuntimeLeaseOptions = {}
): Promise<LocalWranglerRuntimeLease> {
  const lockPath = options.lockPath ?? runtimeLeasePath;
  const acquireTimeoutMs = options.acquireTimeoutMs ?? 120_000;
  const heartbeatIntervalMs = options.heartbeatIntervalMs ?? 10_000;
  const pollIntervalMs = options.pollIntervalMs ?? 150;
  const invalidRecordStaleAfterMs = options.invalidRecordStaleAfterMs ?? 180_000;
  const token = `${process.pid}:${randomUUID()}`;
  const deadline = Date.now() + acquireTimeoutMs;

  while (Date.now() < deadline) {
    try {
      const record = buildLeaseRecord(token);
      writeLeaseRecord(lockPath, record, 'wx');
      const heartbeat = setInterval(() => {
        try {
          const current = parseLeaseRecord(readFileSync(lockPath, 'utf8'));
          if (current?.token === token) {
            writeLeaseRecord(lockPath, {
              ...current,
              heartbeatAt: new Date().toISOString(),
            });
          }
        } catch (error) {
          if (!hasErrorCode(error, 'ENOENT')) {
            clearInterval(heartbeat);
          }
        }
      }, heartbeatIntervalMs);
      heartbeat.unref();
      return {
        release: (): void => {
          clearInterval(heartbeat);
          try {
            if (parseLeaseRecord(readFileSync(lockPath, 'utf8'))?.token === token) {
              rmSync(lockPath, { force: true });
            }
          } catch (error) {
            if (!hasErrorCode(error, 'ENOENT')) {
              throw error;
            }
          }
        },
      };
    } catch (error) {
      if (!hasErrorCode(error, 'EEXIST')) {
        throw error;
      }
      try {
        const current = parseLeaseRecord(readFileSync(lockPath, 'utf8'));
        const invalidRecordIsStale =
          current === null && Date.now() - statSync(lockPath).mtimeMs > invalidRecordStaleAfterMs;
        if (
          (current !== null &&
            !isProcessAlive(current.ownerPid) &&
            removeStaleLeaseIfTokenMatches(lockPath, current.token)) ||
          invalidRecordIsStale
        ) {
          if (invalidRecordIsStale) {
            rmSync(lockPath, { force: true });
          }
          continue;
        }
      } catch (statError) {
        if (!hasErrorCode(statError, 'ENOENT')) {
          throw statError;
        }
      }
      await sleep(pollIntervalMs);
    }
  }

  throw new Error(`timed out waiting for local Wrangler runtime lease at ${lockPath}`);
}

function getFreePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    const server = createServer();
    server.once('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const address = server.address();
      if (address == null || typeof address === 'string') {
        server.close();
        reject(new Error('failed to allocate a local Wrangler seed port'));
        return;
      }
      server.close((error) => {
        if (error != null) {
          reject(error);
          return;
        }
        resolve(address.port);
      });
    });
  });
}

function formatRuntimeFailure(prefix: string, handle: RuntimeHandle, lastError?: string): Error {
  return new Error(
    [
      prefix,
      lastError ? `last error: ${lastError}` : null,
      `stdout: ${handle.logs.stdout.join('').trim() || '<empty>'}`,
      `stderr: ${handle.logs.stderr.join('').trim() || '<empty>'}`,
    ]
      .filter((line): line is string => line != null)
      .join('\n')
  );
}

async function waitForRuntimeExit(child: ChildProcess, timeoutMs: number): Promise<void> {
  if (child.exitCode !== null) {
    return;
  }
  await new Promise<void>((resolve, reject) => {
    const timeout = setTimeout(
      () => reject(new Error(`Wrangler runtime did not exit within ${timeoutMs}ms`)),
      timeoutMs
    );
    child.once('exit', () => {
      clearTimeout(timeout);
      resolve();
    });
    child.once('error', (error) => {
      clearTimeout(timeout);
      reject(error);
    });
  });
}

async function stopRuntime(handle: RuntimeHandle): Promise<void> {
  const exitWait = handle.child.exitCode === null ? waitForRuntimeExit(handle.child, 10_000) : Promise.resolve();
  if (handle.child.pid != null && handle.child.exitCode === null) {
    if (process.platform === 'win32') {
      spawnSync('taskkill', ['/pid', String(handle.child.pid), '/t', '/f'], {
        stdio: 'ignore',
        windowsHide: true,
      });
    } else {
      process.kill(-handle.child.pid, 'SIGTERM');
    }
  }
  try {
    await exitWait;
  } catch (error) {
    if (handle.child.pid == null || process.platform === 'win32') {
      throw error;
    }
    process.kill(-handle.child.pid, 'SIGKILL');
    await waitForRuntimeExit(handle.child, 5_000);
  }
}

async function startRuntime(persistTo: string): Promise<RuntimeHandle> {
  const npmExecPath = process.env.npm_execpath?.trim();
  if (!npmExecPath) {
    throw new Error('local seed commands must run through npm so Wrangler can be resolved without a shell');
  }

  const port = await getFreePort();
  const logs = { stdout: [] as string[], stderr: [] as string[] };
  const child = spawn(
    process.execPath,
    [
      npmExecPath,
      'exec',
      '--',
      'wrangler',
      'dev',
      '--local',
      '--port',
      String(port),
      '--ip',
      '127.0.0.1',
      '--persist-to',
      persistTo,
      '--show-interactive-dev-session=false',
      '--log-level',
      'warn',
    ],
    {
      cwd: cloudflareDir,
      env: {
        ...process.env,
        ENVIRONMENT: process.env.ENVIRONMENT?.trim() || 'development',
        INTERACTIVE_CSRF_TOKEN: process.env.INTERACTIVE_CSRF_TOKEN?.trim() || 'local-seed-csrf-token',
      },
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
      detached: process.platform !== 'win32',
    }
  );
  child.stdout?.setEncoding('utf8');
  child.stderr?.setEncoding('utf8');
  child.stdout?.on('data', (chunk: string) => logs.stdout.push(chunk));
  child.stderr?.on('data', (chunk: string) => logs.stderr.push(chunk));

  return {
    child,
    baseUrl: `http://127.0.0.1:${port}`,
    logs,
  };
}

async function waitForSeededHealth(handle: RuntimeHandle): Promise<LocalSeedHealthResponse> {
  const deadline = Date.now() + 30_000;
  let lastError = 'runtime did not answer';

  while (Date.now() < deadline) {
    if (handle.child.exitCode !== null) {
      throw formatRuntimeFailure(`Wrangler seed runtime exited with code ${handle.child.exitCode}`, handle);
    }
    try {
      const requestTimeout = AbortSignal.timeout(2_000);
      const response = await fetch(`${handle.baseUrl}/health`, { signal: requestTimeout });
      if (response.status === 200) {
        return (await response.json()) as LocalSeedHealthResponse;
      }
      lastError = `health returned ${response.status}: ${await response.text()}`;
    } catch (error) {
      lastError = error instanceof Error ? error.message : String(error);
    }
    await sleep(400);
  }

  throw formatRuntimeFailure('Wrangler seed runtime did not become healthy before timeout', handle, lastError);
}

function assertPersistedSeed(health: LocalSeedHealthResponse): LocalSeedPersistenceEvidence {
  const persistence = health.seedSummary?.persistence;
  const persistenceKeys = persistence == null ? [] : Object.keys(persistence).sort();
  const expected = expectedPersistenceEvidence();
  const expectedKeys = Object.keys(expected).sort();
  if (
    health.status !== 'ok' ||
    health.bindingStatus !== 'ready' ||
    health.missingBindingCount !== 0 ||
    persistence == null ||
    persistenceKeys.length !== expectedKeys.length ||
    persistenceKeys.some((key, index) => key !== expectedKeys[index]) ||
    expectedKeys.some(
      (key) =>
        persistence[key as keyof LocalSeedPersistenceEvidence] <
        expected[key as keyof LocalSeedPersistenceEvidence]
    )
  ) {
    throw new Error(`local seed health did not prove D1/KV/R2 persistence: ${JSON.stringify(health)}`);
  }
  return persistence;
}

export async function runLocalSeedMutation(requestedFamily: string): Promise<LocalSeedMutationReceipt> {
  const runId = process.env.OCENTRA_CLOUDFLARE_SEED_RUN_ID?.trim() || `cloudflare-local-seed-${randomUUID()}`;
  const correlationId = `${runId}:seed-runtime`;
  const explicitPersistPath = process.env.OCENTRA_CLOUDFLARE_LOCAL_PERSIST_PATH?.trim();
  const persistTo = path.resolve(explicitPersistPath || defaultPersistPath);
  mkdirSync(persistTo, { recursive: true });

  log.configure({
    bridgeEndpoint: process.env.OCENTRA_CLOUDFLARE_SEED_LOG_BRIDGE_ENDPOINT?.trim() || '',
    runId,
    correlationId,
    testName: 'local-seed-runtime.ts',
    environment: 'local-seed',
  });
  log.register(import.meta.url);

  const emitMilestone = (boundary: string, result: SeedRuntimeMilestone['result'], failureKind?: string): void => {
    const milestone: SeedRuntimeMilestone = {
      runId,
      correlationId,
      owner: seedRuntimeOwner,
      boundary,
      result,
      noClaimReason: seedRuntimeNoClaimReason,
      redactionState: 'redacted-safe-fields-only',
      requestedFamily,
      ...(failureKind ? { failureKind } : {}),
    };
    if (result === 'rejected') {
      log.logError('cloudflare local seed runtime failed', getStackTrace(), milestone);
      return;
    }
    log.logInfo('cloudflare local seed runtime milestone', getStackTrace(), milestone, true);
  };

  let boundary = 'startup';
  let lease: LocalWranglerRuntimeLease | null = null;
  let handle: RuntimeHandle | null = null;
  const runtimePidPath = path.join(persistTo, LOCAL_SEED_RUNTIME_PID_FILE);
  let primaryError: unknown = null;
  emitMilestone(boundary, 'accepted');
  try {
    boundary = 'lease-acquire';
    lease = await acquireLocalWranglerRuntimeLease();
    boundary = 'runtime-start';
    handle = await startRuntime(persistTo);
    if (handle.child.pid == null) {
      throw new Error('local seed runtime did not provide a Wrangler process id');
    }
    writeFileSync(runtimePidPath, String(handle.child.pid), 'utf8');
    emitMilestone(boundary, 'accepted');
    boundary = 'seed-validation';
    const health = await waitForSeededHealth(handle);
    const persistence = assertPersistedSeed(health);
    emitMilestone(boundary, 'proven');
    return {
      runId,
      requestedFamily,
      persistenceTarget: explicitPersistPath ? 'explicit' : 'wrangler-default',
      runtimeBootStatus: 'proven',
      runtimeBootEvidence: 'bounded Wrangler local runtime returned /health after direct D1, KV, and R2 readback',
      fullBindingSeedApplied: true,
      persistence,
      webhookFixtureCount: LOCAL_WEBHOOK_FIXTURE_INVENTORY.length,
      queueReplayFixtureCount: LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY.length,
    };
  } catch (error) {
    primaryError = error;
    emitMilestone(boundary, 'rejected', error instanceof Error ? error.name : 'UnknownFailure');
    throw error;
  } finally {
    let teardownError: unknown = null;
    try {
      if (handle != null) {
        await stopRuntime(handle);
      }
    } catch (error) {
      teardownError = error;
      emitMilestone('teardown-runtime', 'rejected', error instanceof Error ? error.name : 'UnknownFailure');
    }
    rmSync(runtimePidPath, { force: true });
    try {
      lease?.release();
    } catch (error) {
      teardownError ??= error;
      emitMilestone('teardown-lease', 'rejected', error instanceof Error ? error.name : 'UnknownFailure');
    }
    if (teardownError === null) {
      emitMilestone('teardown', 'released');
    }
    await log.flush();
    if (primaryError === null && teardownError !== null) {
      throw teardownError;
    }
  }
}

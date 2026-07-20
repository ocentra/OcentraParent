import { randomUUID } from 'node:crypto';
import { spawn, spawnSync, type ChildProcess } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { createServer } from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const LOCAL_WEBHOOK_FIXTURE_INVENTORY = ['stripe', 'razorpay', 'paypal', 'google', 'apple'] as const;
export const LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY = ['accepted-replay', 'dead-letter-replay'] as const;

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

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const cloudflareDir = path.resolve(scriptDir, '..');
const defaultPersistPath = path.join(cloudflareDir, '.wrangler', 'state', 'v3');
const runtimeLeasePath = path.join(os.tmpdir(), 'ocentra-cloudflare-wrangler-runtime.lock');

function sleep(milliseconds: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
  });
}

function hasErrorCode(error: unknown, code: string): boolean {
  return typeof error === 'object' && error !== null && 'code' in error && (error as { code?: unknown }).code === code;
}

export async function acquireLocalWranglerRuntimeLease(): Promise<LocalWranglerRuntimeLease> {
  const token = `${process.pid}:${randomUUID()}`;
  const deadline = Date.now() + 120_000;

  while (Date.now() < deadline) {
    try {
      writeFileSync(runtimeLeasePath, token, { encoding: 'utf8', flag: 'wx' });
      return {
        release: (): void => {
          try {
            if (readFileSync(runtimeLeasePath, 'utf8') === token) {
              rmSync(runtimeLeasePath, { force: true });
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
        if (Date.now() - statSync(runtimeLeasePath).mtimeMs > 180_000) {
          rmSync(runtimeLeasePath, { force: true });
          continue;
        }
      } catch (statError) {
        if (!hasErrorCode(statError, 'ENOENT')) {
          throw statError;
        }
      }
      await sleep(150);
    }
  }

  throw new Error(`timed out waiting for local Wrangler runtime lease at ${runtimeLeasePath}`);
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

async function stopRuntime(handle: RuntimeHandle): Promise<void> {
  if (handle.child.pid != null && handle.child.exitCode === null) {
    if (process.platform === 'win32') {
      spawnSync('taskkill', ['/pid', String(handle.child.pid), '/t', '/f'], {
        stdio: 'ignore',
        windowsHide: true,
      });
    } else {
      handle.child.kill('SIGTERM');
    }
  }
  await sleep(250);
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
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
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
      const response = await fetch(`${handle.baseUrl}/health`);
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
  if (
    health.status !== 'ok' ||
    health.bindingStatus !== 'ready' ||
    health.missingBindingCount !== 0 ||
    persistence == null ||
    Object.values(persistence).some((count) => !Number.isInteger(count) || count <= 0)
  ) {
    throw new Error(`local seed health did not prove D1/KV/R2 persistence: ${JSON.stringify(health)}`);
  }
  return persistence;
}

export async function runLocalSeedMutation(requestedFamily: string): Promise<LocalSeedMutationReceipt> {
  const explicitPersistPath = process.env.OCENTRA_CLOUDFLARE_LOCAL_PERSIST_PATH?.trim();
  const persistTo = path.resolve(explicitPersistPath || defaultPersistPath);
  mkdirSync(persistTo, { recursive: true });

  const lease = await acquireLocalWranglerRuntimeLease();
  let handle: RuntimeHandle | null = null;
  try {
    handle = await startRuntime(persistTo);
    const health = await waitForSeededHealth(handle);
    const persistence = assertPersistedSeed(health);
    return {
      runId: process.env.OCENTRA_CLOUDFLARE_SEED_RUN_ID?.trim() || `cloudflare-local-seed-${randomUUID()}`,
      requestedFamily,
      persistenceTarget: explicitPersistPath ? 'explicit' : 'wrangler-default',
      runtimeBootStatus: 'proven',
      runtimeBootEvidence: 'bounded Wrangler local runtime returned /health after direct D1, KV, and R2 readback',
      fullBindingSeedApplied: true,
      persistence,
      webhookFixtureCount: LOCAL_WEBHOOK_FIXTURE_INVENTORY.length,
      queueReplayFixtureCount: LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY.length,
    };
  } finally {
    if (handle != null) {
      await stopRuntime(handle);
    }
    lease.release();
  }
}

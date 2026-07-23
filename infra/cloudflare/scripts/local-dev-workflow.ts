#!/usr/bin/env node

import { randomUUID } from 'node:crypto';
import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import {
  LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY,
  LOCAL_SEED_RUNTIME_PID_FILE,
  LOCAL_WEBHOOK_FIXTURE_INVENTORY,
} from './local-seed-runtime.js';

export interface RuntimeDependencyBlocker {
  kind: 'missing-runtime-dependency' | 'runtime-import-check' | 'seed-command-timeout';
  path?: string;
  details: string;
}

export interface LocalStartPath {
  rootCommand: string;
  moduleCommand: string;
  wranglerCommand: string;
  origin: string;
  authAdapterMode: string;
  preflightStatus: 'blocked' | 'ready';
  importCheckStatus: 'blocked' | 'passed';
  runtimeBootStatus: 'proven' | 'unproven';
  runtimeBootEvidence: string;
  blockers: ReadonlyArray<RuntimeDependencyBlocker>;
}

export interface FixtureFamilyReport {
  family: string;
  source: string;
  populationState: 'blocked' | 'populated' | 'placeholder' | 'test-fixture-backed';
  itemCount: number | null;
  notes: string;
  blocker?: RuntimeDependencyBlocker;
}

export interface LocalSeedPath {
  aggregateCommand: string;
  commands: ReadonlyArray<string>;
  status: 'blocked' | 'runnable';
  fixtureFamilies: ReadonlyArray<FixtureFamilyReport>;
}

export interface LocalTeardownPath {
  status: 'explicit';
  steps: ReadonlyArray<string>;
  notes: ReadonlyArray<string>;
}

export interface LocalDevWorkflowReport {
  generatedAt: string;
  start: LocalStartPath;
  seed: LocalSeedPath;
  teardown: LocalTeardownPath;
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const cloudflareDir = path.resolve(scriptDir, '..');
const repoRoot = path.resolve(cloudflareDir, '..', '..');
const rootPackageJsonPath = path.join(repoRoot, 'package.json');
const knownRuntimeDependencyPaths = ['src/generated/billing-contracts.ts'] as const;
const allowedCloudflareScriptCommands = new Set([
  'seed:local',
  'seed:products:local',
  'seed:referrals:local',
  'seed:test-accounts:local',
]);
const seedCommandTimeoutMs = 120_000;

function resolveCloudflareModulePath(cloudflareRoot: string, relativePath: string): string {
  return path.resolve(cloudflareRoot, relativePath);
}

function formatCloudflareModulePath(relativePath: string): string {
  return path.posix.join('infra', 'cloudflare', relativePath.replace(/\\/g, '/'));
}

export function collectMissingRuntimeDependencyBlockers(
  cloudflareRoot: string = cloudflareDir,
  dependencyPaths: ReadonlyArray<string> = knownRuntimeDependencyPaths
): ReadonlyArray<RuntimeDependencyBlocker> {
  const blockers: RuntimeDependencyBlocker[] = [];

  for (const relativePath of dependencyPaths) {
    const absolutePath = resolveCloudflareModulePath(cloudflareRoot, relativePath);
    if (!existsSync(absolutePath)) {
      blockers.push({
        kind: 'missing-runtime-dependency',
        path: formatCloudflareModulePath(relativePath),
        details: `required generated billing-contract sidecar missing at ${formatCloudflareModulePath(relativePath)}`,
      });
    }
  }

  return blockers;
}

interface CommandProbeResult {
  command: string;
  status: 'blocked' | 'runnable';
  stdout: string;
  stderr: string;
  blocker?: RuntimeDependencyBlocker;
}

interface MutationReceiptProbe {
  runId?: unknown;
  requestedFamily?: unknown;
  runtimeBootStatus?: unknown;
  fullBindingSeedApplied?: unknown;
  persistence?: unknown;
}

function readWorkspaceScripts(): Record<string, string> {
  return JSON.parse(readFileSync(rootPackageJsonPath, 'utf8')).scripts as Record<string, string>;
}

function terminateTimedOutSeedProcess(pid: number | undefined, persistenceRoot: string): void {
  const runtimePid = Number.parseInt(
    existsSync(path.join(persistenceRoot, LOCAL_SEED_RUNTIME_PID_FILE))
      ? readFileSync(path.join(persistenceRoot, LOCAL_SEED_RUNTIME_PID_FILE), 'utf8')
      : '',
    10
  );
  if (process.platform === 'win32') {
    for (const processId of [pid, runtimePid]) {
      if (processId != null && Number.isInteger(processId) && processId > 0) {
        spawnSync('taskkill', ['/pid', String(processId), '/t', '/f'], { stdio: 'ignore', windowsHide: true });
      }
    }
    return;
  }
  for (const processId of [pid, runtimePid]) {
    if (processId == null || !Number.isInteger(processId) || processId <= 0) {
      continue;
    }
    try {
      process.kill(processId === runtimePid ? -processId : processId, 'SIGTERM');
    } catch (error) {
      if (!(error instanceof Error && 'code' in error && error.code === 'ESRCH')) {
        throw error;
      }
    }
  }
}

function runCloudflareScript(command: string, persistenceRoot: string, seedRunId: string): CommandProbeResult {
  if (!allowedCloudflareScriptCommands.has(command)) {
    return {
      command: `npm --prefix infra/cloudflare run ${command}`,
      status: 'blocked',
      stdout: '',
      stderr: '',
      blocker: {
        kind: 'missing-runtime-dependency',
        details: `unsupported Cloudflare script command: ${command}`,
      },
    };
  }

  const result =
    process.platform === 'win32'
      ? spawnSync('cmd.exe', ['/d', '/s', '/c', `npm run ${command}`], {
          cwd: cloudflareDir,
          encoding: 'utf8',
          env: {
            ...process.env,
            OCENTRA_CLOUDFLARE_LOCAL_PERSIST_PATH: persistenceRoot,
            OCENTRA_CLOUDFLARE_SEED_RUN_ID: seedRunId,
          },
          timeout: seedCommandTimeoutMs,
        })
      : spawnSync('npm', ['run', command], {
          cwd: cloudflareDir,
          encoding: 'utf8',
          env: {
            ...process.env,
            OCENTRA_CLOUDFLARE_LOCAL_PERSIST_PATH: persistenceRoot,
            OCENTRA_CLOUDFLARE_SEED_RUN_ID: seedRunId,
          },
          timeout: seedCommandTimeoutMs,
        });

  if (result.error instanceof Error && 'code' in result.error && result.error.code === 'ETIMEDOUT') {
    terminateTimedOutSeedProcess(result.pid, persistenceRoot);
    return {
      command: `npm --prefix infra/cloudflare run ${command}`,
      status: 'blocked',
      stdout: result.stdout,
      stderr: result.stderr,
      blocker: {
        kind: 'seed-command-timeout',
        details: `${command} exceeded the ${seedCommandTimeoutMs}ms subprocess limit`,
      },
    };
  }

  if (result.status === 0) {
    return {
      command: `npm --prefix infra/cloudflare run ${command}`,
      status: 'runnable',
      stdout: result.stdout,
      stderr: result.stderr,
    };
  }

  return {
    command: `npm --prefix infra/cloudflare run ${command}`,
    status: 'blocked',
    stdout: result.stdout,
    stderr: result.stderr,
    blocker: {
      kind: 'missing-runtime-dependency',
      details: (
        result.stderr ||
        result.stdout ||
        result.error?.message ||
        `${command} failed without diagnostics`
      ).trim(),
    },
  };
}

function inspectLocalStartPath(): LocalStartPath {
  const workspaceScripts = readWorkspaceScripts();
  const blockers: RuntimeDependencyBlocker[] = [...collectMissingRuntimeDependencyBlockers()];
  let importCheckStatus: 'blocked' | 'passed' = 'blocked';
  const runtimeBootStatus: 'proven' | 'unproven' = 'unproven';

  if (workspaceScripts['dev:cloudflare'] !== 'npm --prefix infra/cloudflare run dev') {
    blockers.push({
      kind: 'runtime-import-check',
      details: 'root package.json no longer exposes dev:cloudflare -> npm --prefix infra/cloudflare run dev',
    });
  }

  if (blockers.length === 0) {
    const result = spawnSync(
      process.execPath,
      [
        '--import',
        'tsx',
        '--eval',
        'import("./src/index.ts").then(() => console.log("runtime-import-ok")).catch((error) => { console.error(error instanceof Error ? `${error.name}: ${error.message}` : String(error)); process.exit(1); });',
      ],
      {
        cwd: cloudflareDir,
        encoding: 'utf8',
      }
    );

    if (result.status !== 0) {
      blockers.push({
        kind: 'runtime-import-check',
        details: (
          result.stderr ||
          result.stdout ||
          'Cloudflare worker runtime import failed without diagnostics'
        ).trim(),
      });
    } else {
      importCheckStatus = 'passed';
    }
  }

  return {
    rootCommand: 'npm run dev:cloudflare',
    moduleCommand: 'npm --prefix infra/cloudflare run dev',
    wranglerCommand: 'wrangler dev --local',
    origin: 'http://localhost:3000',
    authAdapterMode: 'account-auth-adapter-manual-required',
    preflightStatus: blockers.length === 0 ? 'ready' : 'blocked',
    importCheckStatus,
    runtimeBootStatus,
    runtimeBootEvidence:
      'unproven: this probe imports the Worker entrypoint but does not start Wrangler or perform a bounded health request',
    blockers,
  };
}

function buildFixtureFamilies(): ReadonlyArray<FixtureFamilyReport> {
  const persistenceRoot = mkdtempSync(path.join(os.tmpdir(), 'ocentra-cloudflare-seed-probe-'));
  const seedRunId = `cloudflare-local-seed-probe-${randomUUID()}`;

  const parseJsonPayload = (probe: CommandProbeResult): Record<string, unknown> | null => {
    if (probe.status !== 'runnable') {
      return null;
    }

    const jsonStart = probe.stdout.indexOf('{');
    if (jsonStart < 0) {
      return null;
    }

    return JSON.parse(probe.stdout.slice(jsonStart)) as Record<string, unknown>;
  };

  const parseCount = (probe: CommandProbeResult, key: string): number | null => {
    const body = parseJsonPayload(probe);
    if (body === null) {
      return null;
    }
    const value = body[key];
    if (Array.isArray(value)) {
      return value.length;
    }
    if (value && typeof value === 'object') {
      return Object.keys(value).length;
    }
    return null;
  };

  const hasPersistedMutation = (probe: CommandProbeResult, requestedFamily: string): boolean => {
    const body = parseJsonPayload(probe);
    const receipt = body?.mutationReceipt as MutationReceiptProbe | undefined;
    if (
      receipt?.runId !== seedRunId ||
      receipt.requestedFamily !== requestedFamily ||
      receipt.runtimeBootStatus !== 'proven' ||
      receipt.fullBindingSeedApplied !== true ||
      receipt.persistence == null ||
      typeof receipt.persistence !== 'object'
    ) {
      return false;
    }
    return Object.values(receipt.persistence).every(
      (count) => typeof count === 'number' && Number.isInteger(count) && count > 0
    );
  };

  try {
    const localSeed = runCloudflareScript('seed:local', persistenceRoot, seedRunId);
    const productsSeed = runCloudflareScript('seed:products:local', persistenceRoot, seedRunId);
    const referralsSeed = runCloudflareScript('seed:referrals:local', persistenceRoot, seedRunId);
    const accountsSeed = runCloudflareScript('seed:test-accounts:local', persistenceRoot, seedRunId);
    const localSeedPersisted = hasPersistedMutation(localSeed, 'composite-local-seed');
    const productsSeedPersisted = hasPersistedMutation(productsSeed, 'pricing-catalog');
    const referralsSeedPersisted = hasPersistedMutation(referralsSeed, 'referral-test-graph');
    const accountsSeedPersisted = hasPersistedMutation(accountsSeed, 'support-admin-test-accounts');

    return [
      {
        family: 'pricing-catalog',
        source: productsSeed.command,
        populationState:
          productsSeed.status === 'blocked'
            ? 'blocked'
            : productsSeedPersisted && (parseCount(productsSeed, 'pricingPlans') ?? 0) > 0
              ? 'populated'
              : 'placeholder',
        itemCount: parseCount(productsSeed, 'pricingPlans'),
        notes: 'Pricing plans are accepted only after Wrangler proves direct D1/KV/R2 persistence.',
        blocker: productsSeed.blocker,
      },
      {
        family: 'parent-test-accounts',
        source: localSeed.command,
        populationState:
          localSeed.status === 'blocked'
            ? 'blocked'
            : localSeedPersisted && (parseCount(localSeed, 'statusBySubject') ?? 0) > 0
              ? 'populated'
              : 'placeholder',
        itemCount: parseCount(localSeed, 'statusBySubject'),
        notes: 'Composite parent seed is accepted only after direct local binding readback.',
        blocker: localSeed.blocker,
      },
      {
        family: 'support-admin-test-accounts',
        source: accountsSeed.command,
        populationState:
          accountsSeed.status === 'blocked'
            ? 'blocked'
            : accountsSeedPersisted && (parseCount(accountsSeed, 'accounts') ?? 0) > 0
              ? 'populated'
              : 'placeholder',
        itemCount: parseCount(accountsSeed, 'accounts'),
        notes: 'Support/admin fixtures are accepted only after direct local binding readback.',
        blocker: accountsSeed.blocker,
      },
      {
        family: 'referral-test-graph',
        source: referralsSeed.command,
        populationState:
          referralsSeed.status === 'blocked'
            ? 'blocked'
            : referralsSeedPersisted && (parseCount(referralsSeed, 'referrals') ?? 0) > 0
              ? 'populated'
              : 'placeholder',
        itemCount: parseCount(referralsSeed, 'referrals'),
        notes: 'Referral fixtures are accepted only after direct local binding readback.',
        blocker: referralsSeed.blocker,
      },
      {
        family: 'webhook-payload-fixtures',
        source: 'infra/cloudflare/tests/integration/worker-runtime-real.test.ts',
        populationState: 'test-fixture-backed',
        itemCount: LOCAL_WEBHOOK_FIXTURE_INVENTORY.length,
        notes: 'Count comes from the shared provider inventory executed by the real Worker runtime test.',
      },
      {
        family: 'queue-replay-fixtures',
        source: 'infra/cloudflare/tests/property/billing-idempotency.property.test.ts',
        populationState: 'test-fixture-backed',
        itemCount: LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY.length,
        notes: 'Count comes from the shared accepted/dead-letter replay inventory executed by property tests.',
      },
    ];
  } finally {
    rmSync(persistenceRoot, { recursive: true, force: true });
  }
}

function inspectLocalSeedPath(): LocalSeedPath {
  const fixtureFamilies = buildFixtureFamilies();
  const allFixtureFamiliesPopulated = fixtureFamilies.every(
    (family) =>
      (family.populationState === 'populated' || family.populationState === 'test-fixture-backed') &&
      (family.itemCount ?? 0) > 0
  );

  return {
    aggregateCommand: 'npm --prefix infra/cloudflare run seed:local',
    commands: [
      'npm --prefix infra/cloudflare run seed:local',
      'npm --prefix infra/cloudflare run seed:products:local',
      'npm --prefix infra/cloudflare run seed:referrals:local',
      'npm --prefix infra/cloudflare run seed:test-accounts:local',
    ],
    status: allFixtureFamiliesPopulated ? 'runnable' : 'blocked',
    fixtureFamilies,
  };
}

function inspectLocalTeardownPath(): LocalTeardownPath {
  return {
    status: 'explicit',
    steps: [
      'Stop the local worker process started by wrangler dev --local.',
      'If a harness run created a temporary --persist-to directory, delete that directory after the worker stops.',
      'Delete infra/cloudflare/.dev.vars only when the local harness created it for the session; do not remove a pre-existing developer file.',
    ],
    notes: [
      'The harness-backed teardown path is exercised in infra/cloudflare/tests/integration/worker-runtime-real.test.ts.',
      'The default npm --prefix infra/cloudflare run dev command does not currently declare --persist-to, so teardown proof stays scoped to the explicit harness path rather than inventing extra cleanup guarantees.',
    ],
  };
}

export function inspectLocalDevWorkflow(): LocalDevWorkflowReport {
  return {
    generatedAt: new Date().toISOString(),
    start: inspectLocalStartPath(),
    seed: inspectLocalSeedPath(),
    teardown: inspectLocalTeardownPath(),
  };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  process.stdout.write(`${JSON.stringify(inspectLocalDevWorkflow(), null, 2)}\n`);
}

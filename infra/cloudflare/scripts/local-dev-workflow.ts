#!/usr/bin/env node

import { readFileSync } from 'node:fs';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

export interface RuntimeDependencyBlocker {
  kind: 'missing-runtime-dependency' | 'runtime-import-check';
  path?: string;
  details: string;
}

export interface LocalStartPath {
  rootCommand: string;
  moduleCommand: string;
  wranglerCommand: string;
  origin: string;
  authAdapterMode: string;
  status: 'blocked' | 'runnable';
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

interface CommandProbeResult {
  command: string;
  status: 'blocked' | 'runnable';
  stdout: string;
  stderr: string;
  blocker?: RuntimeDependencyBlocker;
}

function readWorkspaceScripts(): Record<string, string> {
  return JSON.parse(readFileSync(rootPackageJsonPath, 'utf8')).scripts as Record<string, string>;
}

function runCloudflareScript(command: string): CommandProbeResult {
  const result = spawnSync('cmd.exe', ['/d', '/s', '/c', `npm run ${command}`], {
    cwd: cloudflareDir,
    encoding: 'utf8',
  });

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
      details: (result.stderr || result.stdout || `${command} failed without diagnostics`).trim(),
    },
  };
}

function inspectLocalStartPath(): LocalStartPath {
  const workspaceScripts = readWorkspaceScripts();
  const blockers: RuntimeDependencyBlocker[] = [];

  if (workspaceScripts['dev:cloudflare'] !== 'npm --prefix infra/cloudflare run dev') {
    blockers.push({
      kind: 'runtime-import-check',
      details: 'root package.json no longer exposes dev:cloudflare -> npm --prefix infra/cloudflare run dev',
    });
  }

  for (const relativePath of knownRuntimeDependencyPaths) {
    // Generated billing contracts are a Rust-schema output owned by this
    // Worker module. `repoRoot` is only for workspace-script discovery; using
    // it here probes a non-existent top-level `src/` directory and turns a
    // valid generated edge artifact into a false boot blocker.
    if (!existsSync(path.join(cloudflareDir, relativePath))) {
      blockers.push({
        kind: 'missing-runtime-dependency',
        path: relativePath,
        details: 'required by the Cloudflare worker runtime before wrangler local start can import src/index.ts',
      });
    }
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
    }
  }

  return {
    rootCommand: 'npm run dev:cloudflare',
    moduleCommand: 'npm --prefix infra/cloudflare run dev',
    wranglerCommand: 'wrangler dev --local',
    origin: 'http://localhost:3000',
    authAdapterMode: 'account-auth-adapter-manual-required',
    status: blockers.length === 0 ? 'runnable' : 'blocked',
    blockers,
  };
}

function buildFixtureFamilies(): ReadonlyArray<FixtureFamilyReport> {
  const localSeed = runCloudflareScript('seed:local');
  const productsSeed = runCloudflareScript('seed:products:local');
  const referralsSeed = runCloudflareScript('seed:referrals:local');
  const accountsSeed = runCloudflareScript('seed:test-accounts:local');

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

  return [
    {
      family: 'pricing-catalog',
      source: productsSeed.command,
      populationState:
        productsSeed.status === 'blocked'
          ? 'blocked'
          : (parseCount(productsSeed, 'pricingPlans') ?? 0) > 0
            ? 'populated'
            : 'placeholder',
      itemCount: parseCount(productsSeed, 'pricingPlans'),
      notes: 'Local pricing plans should come from seed-products-local.ts, not a doc placeholder.',
      blocker: productsSeed.blocker,
    },
    {
      family: 'parent-test-accounts',
      source: localSeed.command,
      populationState:
        localSeed.status === 'blocked'
          ? 'blocked'
          : (parseCount(localSeed, 'statusBySubject') ?? 0) > 0
            ? 'populated'
            : 'placeholder',
      itemCount: parseCount(localSeed, 'statusBySubject'),
      notes:
        'Composite seed snapshot should include per-subject status, invoices, referrals, and entitlement snapshots.',
      blocker: localSeed.blocker,
    },
    {
      family: 'support-admin-test-accounts',
      source: accountsSeed.command,
      populationState:
        accountsSeed.status === 'blocked'
          ? 'blocked'
          : (parseCount(accountsSeed, 'accounts') ?? 0) > 0
            ? 'populated'
            : 'placeholder',
      itemCount: parseCount(accountsSeed, 'accounts'),
      notes:
        'Support/admin test accounts must come from the real seed script output, not a manual count written into docs.',
      blocker: accountsSeed.blocker,
    },
    {
      family: 'referral-test-graph',
      source: referralsSeed.command,
      populationState:
        referralsSeed.status === 'blocked'
          ? 'blocked'
          : (parseCount(referralsSeed, 'referrals') ?? 0) > 0
            ? 'populated'
            : 'placeholder',
      itemCount: parseCount(referralsSeed, 'referrals'),
      notes: 'Referral fixtures should back both per-subject referral summaries and admin referral views.',
      blocker: referralsSeed.blocker,
    },
    {
      family: 'webhook-payload-fixtures',
      source:
        'infra/cloudflare/tests/fuzz/provider-webhook-payload.fuzz.test.ts and infra/cloudflare/tests/integration/worker-runtime-real.test.ts',
      populationState: 'test-fixture-backed',
      itemCount: 5,
      notes:
        'Stripe, PayPal, Razorpay, Google, and Apple webhook payload families are explicit test fixtures, not seed placeholders.',
    },
    {
      family: 'queue-replay-fixtures',
      source: 'infra/cloudflare/tests/property/billing-idempotency.property.test.ts',
      populationState: 'test-fixture-backed',
      itemCount: 2,
      notes: 'Queue fixtures explicitly cover accepted reconciliation flow and dead-letter replay stability.',
    },
  ];
}

function inspectLocalSeedPath(): LocalSeedPath {
  const fixtureFamilies = buildFixtureFamilies();
  return {
    aggregateCommand: 'npm --prefix infra/cloudflare run seed:local',
    commands: [
      'npm --prefix infra/cloudflare run seed:local',
      'npm --prefix infra/cloudflare run seed:products:local',
      'npm --prefix infra/cloudflare run seed:referrals:local',
      'npm --prefix infra/cloudflare run seed:test-accounts:local',
    ],
    status: fixtureFamilies.some((family) => family.populationState === 'blocked') ? 'blocked' : 'runnable',
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
  console.log(JSON.stringify(inspectLocalDevWorkflow(), null, 2));
}

#!/usr/bin/env node

import { readFileSync } from 'node:fs';
import { existsSync } from 'node:fs';
import { createHash, randomUUID } from 'node:crypto';
import path from 'node:path';
import { env } from 'node:process';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { appendTestLogEntries } from '@ocentra-parent/logging-domain/test-log/ndjsonWriter';
import { RunType, TestLogOrigin, TestLogScope } from '@ocentra-parent/logging-domain/test-log/types';
import { redactPayload, redactStringValue } from '../src/security/redaction.js';

export interface RuntimeDependencyBlocker {
  kind: 'missing-runtime-dependency' | 'runtime-import-check' | 'population-failure';
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
  noClaimReason: 'local-worker-not-launched-or-response-verified' | 'start-probe-blocked-before-local-worker-launch';
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
  ownershipConditions: ReadonlyArray<string>;
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

type ProofLogStatus = 'started' | 'observed' | 'blocked' | 'completed';

interface SeedProofFixtureFamily {
  readonly family: string;
  readonly source: string;
  readonly populationState: FixtureFamilyReport['populationState'];
  readonly itemCount: number | null;
  readonly blocker?: {
    readonly kind: RuntimeDependencyBlocker['kind'];
    readonly path: string | null;
    readonly details: string;
  };
  readonly noClaimReason?: 'seed-command-blocked' | 'seed-fixture-population-not-proven';
}

interface SeedProofMilestoneDetails {
  readonly status: LocalSeedPath['status'];
  readonly noClaimReason:
    | 'seed-command-blocked'
    | 'seed-fixture-population-not-proven'
    | 'retained-workpack-proof-absent';
  readonly fixtureFamilies: ReadonlyArray<SeedProofFixtureFamily>;
}

interface ProofLogEvent {
  event: string;
  status: ProofLogStatus;
  details: object;
}

export interface LocalDevProofSummary {
  readonly runId: string;
  readonly proofLogLocation: string;
  readonly startStatus: LocalStartPath['status'];
  readonly seedStatus: LocalSeedPath['status'];
  readonly teardownStatus: LocalTeardownPath['status'];
  readonly noClaim: string;
}

const fallbackProofRunId = `cloudflare-local-${randomUUID()}`;
const maxProofRunIdLength = 96;

export function sanitizeProofRunIdSegment(value: string): string | null {
  const trimmed = value.trim();
  if (trimmed.length === 0 || redactStringValue(trimmed) !== trimmed) {
    return null;
  }

  const sanitized = trimmed.replaceAll(/[^A-Za-z0-9_-]+/gu, '-').replaceAll(/^-+|-+$/gu, '');
  if (sanitized.length === 0) {
    return null;
  }
  if (sanitized.length <= maxProofRunIdLength) {
    return sanitized;
  }

  const suffix = createHash('sha256').update(trimmed).digest('hex').slice(0, 16);
  return `${sanitized.slice(0, maxProofRunIdLength - suffix.length - 1)}-${suffix}`;
}

export function resolveWorkflowProofRunId(providedRunId = env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID): string {
  return sanitizeProofRunIdSegment(providedRunId ?? '') ?? fallbackProofRunId;
}

function writeProofLog(event: ProofLogEvent): void {
  const logRoot = (env.OCENTRA_PARENT_LOG_ROOT ?? '').trim();
  if (logRoot.length === 0) {
    return;
  }

  const redactedEvent = redactPayload({
    schemaVersion: 1,
    eventType: 'cloudflare-local-dev-workflow',
    owner: 'cloudflare-control-plane',
    boundaryResult: event.status,
    redactionState: 'applied',
    runId: resolveWorkflowProofRunId(),
    generatedAt: new Date().toISOString(),
    ...event,
  });
  if (redactedEvent === null || typeof redactedEvent !== 'object' || Array.isArray(redactedEvent)) {
    throw new Error('Cloudflare proof log redaction returned a non-record payload');
  }

  const runId = resolveWorkflowProofRunId();
  appendTestLogEntries(
    [
      {
        schemaVersion: 1,
        type: 'log',
        scope: TestLogScope.ParentCloudflare,
        runId,
        runType: RunType.Single,
        suiteType: 'integration',
        testName: 'cloudflare-local-dev-workflow',
        timestamp: Date.now(),
        level: event.status === 'blocked' ? 'warn' : 'info',
        source: 'infra/cloudflare/scripts/local-dev-workflow.ts',
        context: `cloudflare.local-dev.${event.event}`,
        message: `Cloudflare local-dev workflow ${event.event}`,
        data: JSON.stringify(redactedEvent),
        file: 'local-dev-workflow.ts',
        filePath: 'infra/cloudflare/scripts/local-dev-workflow.ts',
        line: null,
        column: null,
        correlationId: runId,
        tags: ['cloudflare', 'local-dev', 'proof-milestone'],
        stack: null,
        origin: TestLogOrigin.Test,
        environment: 'local',
      },
    ],
    logRoot
  );
}

export function writeLocalDevProofSummary(summary: LocalDevProofSummary): void {
  writeProofLog({
    event: 'proof_summary_observed',
    status: 'observed',
    details: summary,
  });
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

export function redactRuntimeBlockerDetails(details: string): string {
  const windowsAndFileUriRedacted = details.replace(
    /(?:[A-Za-z]:[\\/](?:[^\\/\r\n\t:"'`]+[\\/])*[^\\/\r\n\t:"'`]+|file:\/\/\/(?:[A-Za-z]:\/)?(?:[^/\r\n\t:"'`]+\/)*[^/\r\n\t:"'`]+)/gu,
    '[redacted-path]'
  );
  return windowsAndFileUriRedacted.replace(
    /(^|\b(?:at|from|in|path|file|module)\s+['"]?)(\/(?!\/)(?:(?!\s+(?:at|from|in|path|file|module)\s)(?:\/|[^/\r\n\t:"'`]))+)/gu,
    '$1[redacted-path]'
  );
}

export function writeLocalDevInspectionFailure(error: unknown): void {
  const details = error instanceof Error ? `${error.name}: ${error.message}` : String(error);
  writeProofLog({
    event: 'inspection_failed',
    status: 'blocked',
    details: {
      noClaimReason: 'local-workflow-inspection-failed',
      details: redactRuntimeBlockerDetails(details),
    },
  });
}

export function buildStartProofMilestoneDetails(start: LocalStartPath): object {
  return {
    status: start.status,
    noClaimReason: start.noClaimReason,
    blockerCount: start.blockers.length,
    blockerKinds: start.blockers.map((blocker) => blocker.kind),
    blockers: start.blockers.map(({ kind, path: blockerPath, details }) => ({
      kind,
      path: blockerPath ?? null,
      details: redactRuntimeBlockerDetails(details),
    })),
  };
}

function seedNoClaimReason(
  seed: LocalSeedPath
): 'seed-command-blocked' | 'seed-fixture-population-not-proven' | 'retained-workpack-proof-absent' {
  if (seed.status !== 'blocked') {
    return 'retained-workpack-proof-absent';
  }

  return seed.fixtureFamilies.some((fixtureFamily) => fixtureFamily.blocker?.kind === 'population-failure')
    ? 'seed-fixture-population-not-proven'
    : 'seed-command-blocked';
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
        path: path.relative(repoRoot, path.join(cloudflareDir, relativePath)).replaceAll('\\', '/'),
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

  if (blockers.length === 0) {
    const wranglerEntry = [
      path.join(cloudflareDir, 'node_modules', 'wrangler', 'bin', 'wrangler.js'),
      path.join(repoRoot, 'node_modules', 'wrangler', 'bin', 'wrangler.js'),
    ].find((candidate) => existsSync(candidate));
    const wranglerProbe = wranglerEntry
      ? spawnSync(process.execPath, [wranglerEntry, '--version'], { cwd: cloudflareDir, encoding: 'utf8' })
      : null;
    if (!wranglerProbe || wranglerProbe.status !== 0) {
      blockers.push({
        kind: 'missing-runtime-dependency',
        path: wranglerEntry
          ? path.relative(repoRoot, wranglerEntry).replaceAll('\\', '/')
          : 'infra/cloudflare/node_modules/wrangler',
        details: (
          wranglerProbe?.stderr ||
          wranglerProbe?.stdout ||
          'Wrangler is not available for the Cloudflare module'
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
    noClaimReason:
      blockers.length === 0
        ? 'local-worker-not-launched-or-response-verified'
        : 'start-probe-blocked-before-local-worker-launch',
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

export function failClosedRequiredFixtureFamilies(
  fixtureFamilies: ReadonlyArray<FixtureFamilyReport>
): ReadonlyArray<FixtureFamilyReport> {
  return fixtureFamilies.map((fixtureFamily) => {
    if (fixtureFamily.populationState !== 'placeholder') {
      return fixtureFamily;
    }

    return {
      ...fixtureFamily,
      populationState: 'blocked',
      blocker: {
        kind: 'population-failure',
        details: `Required fixture family ${fixtureFamily.family} returned no populated items from ${fixtureFamily.source}.`,
      },
    };
  });
}

export function seedStatusFromFixtureFamilies(
  fixtureFamilies: ReadonlyArray<FixtureFamilyReport>
): LocalSeedPath['status'] {
  return fixtureFamilies.some((fixtureFamily) => fixtureFamily.populationState === 'blocked') ? 'blocked' : 'runnable';
}

function inspectLocalSeedPath(): LocalSeedPath {
  const fixtureFamilies = failClosedRequiredFixtureFamilies(buildFixtureFamilies());
  return {
    aggregateCommand: 'npm --prefix infra/cloudflare run seed:local',
    commands: [
      'npm --prefix infra/cloudflare run seed:local',
      'npm --prefix infra/cloudflare run seed:products:local',
      'npm --prefix infra/cloudflare run seed:referrals:local',
      'npm --prefix infra/cloudflare run seed:test-accounts:local',
    ],
    status: seedStatusFromFixtureFamilies(fixtureFamilies),
    fixtureFamilies,
  };
}

export function buildSeedProofMilestoneDetails(seed: LocalSeedPath): SeedProofMilestoneDetails {
  return {
    status: seed.status,
    noClaimReason: seedNoClaimReason(seed),
    fixtureFamilies: seed.fixtureFamilies.map(({ family, source, populationState, itemCount, blocker }) => ({
      family,
      source,
      populationState,
      itemCount,
      ...(blocker === undefined
        ? {}
        : {
            blocker: {
              kind: blocker.kind,
              path: blocker.path ?? null,
              details: redactRuntimeBlockerDetails(blocker.details),
            },
            noClaimReason:
              blocker.kind === 'population-failure' ? 'seed-fixture-population-not-proven' : 'seed-command-blocked',
          }),
    })),
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
    ownershipConditions: [
      'Stop only the wrangler dev --local process started by this workflow or its harness.',
      'Remove a --persist-to directory only when this workflow or its harness created it.',
      'Remove infra/cloudflare/.dev.vars only when this workflow or its harness created it.',
    ],
  };
}

export function inspectLocalDevWorkflow(): LocalDevWorkflowReport {
  writeProofLog({
    event: 'workflow_started',
    status: 'started',
    details: {
      rootCommand: 'npm run dev:cloudflare',
      moduleCommand: 'npm --prefix infra/cloudflare run dev',
      proofLogging: 'enabled',
    },
  });

  const start = inspectLocalStartPath();
  writeProofLog({
    event: 'start_path_observed',
    status: start.status === 'blocked' ? 'blocked' : 'observed',
    details: buildStartProofMilestoneDetails(start),
  });

  if (start.blockers.length > 0) {
    writeProofLog({
      event: 'start_blocker_observed',
      status: 'blocked',
      details: buildStartProofMilestoneDetails(start),
    });
  }

  const seed = inspectLocalSeedPath();
  writeProofLog({
    event: 'seed_path_observed',
    status: seed.status === 'blocked' ? 'blocked' : 'observed',
    details: buildSeedProofMilestoneDetails(seed),
  });

  const teardown = inspectLocalTeardownPath();
  const report = {
    generatedAt: new Date().toISOString(),
    start,
    seed,
    teardown,
  };

  writeProofLog({
    event: 'teardown_path_observed',
    status: 'observed',
    details: {
      status: teardown.status,
      steps: teardown.steps,
      notes: teardown.notes,
      ownershipConditions: teardown.ownershipConditions,
    },
  });

  writeProofLog({
    event: 'workflow_completed',
    status: 'completed',
    details: {
      startStatus: report.start.status,
      seedStatus: report.seed.status,
      teardownStatus: report.teardown.status,
    },
  });

  return report;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  console.log(JSON.stringify(inspectLocalDevWorkflow(), null, 2));
}

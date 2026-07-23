import assert from 'node:assert/strict';
import { randomUUID } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { existsSync, mkdtempSync, mkdirSync, readFileSync, rmSync, utimesSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, it } from 'node:test';
import { collectMissingRuntimeDependencyBlockers, inspectLocalDevWorkflow } from '../../scripts/local-dev-workflow.js';
import {
  LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY,
  LOCAL_WEBHOOK_FIXTURE_INVENTORY,
  acquireLocalWranglerRuntimeLease,
  runLocalSeedMutation,
  type LocalSeedMutationReceipt,
} from '../../scripts/local-seed-runtime.js';
import { redactPayload } from '../../src/security/redaction.js';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const localDevWorkflowModuleUrl = new URL('../../scripts/local-dev-workflow.ts', import.meta.url).href;
const loggerModuleUrl = new URL('../../../../packages/logging-domain/src/core/logger.ts', import.meta.url).href;
const stackTraceModuleUrl = new URL('../../../../packages/logging-domain/src/core/stackTrace.ts', import.meta.url).href;
const bridgeServerModuleUrl = new URL(
  '../../../../packages/logging-domain/src/transport/bridgeServer.ts',
  import.meta.url
).href;
const loggingTypesModuleUrl = new URL('../../../../packages/logging-domain/src/test-log/types.ts', import.meta.url)
  .href;
const ndjsonPathsModuleUrl = new URL('../../../../packages/logging-domain/src/test-log/ndjsonPaths.ts', import.meta.url)
  .href;
const ndjsonWriterModuleUrl = new URL(
  '../../../../packages/logging-domain/src/test-log/ndjsonWriter.ts',
  import.meta.url
).href;

const proofOwner = 'infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts';
const proofNoClaimReason = 'wrangler-runtime-boot-unproven;production-deployment-not-owned';
const proofRedactionState = 'redacted-safe-fields-only';

interface PersistedProofMilestone {
  runId: string;
  correlationId: string;
  owner: string;
  boundary: string;
  result: string;
  noClaimReason: string;
  redactionState: string;
  redactionProbe?: Readonly<Record<string, unknown>>;
}

interface SeedCommandOutput extends Record<string, unknown> {
  mutationReceipt: LocalSeedMutationReceipt;
}

function runSeedCommand(command: string, persistenceRoot: string, runId: string): SeedCommandOutput {
  const child =
    process.platform === 'win32'
      ? spawnSync('cmd.exe', ['/d', '/s', '/c', `npm run ${command}`], {
          cwd: path.join(repoRoot, 'infra', 'cloudflare'),
          encoding: 'utf8',
          env: {
            ...process.env,
            OCENTRA_CLOUDFLARE_LOCAL_PERSIST_PATH: persistenceRoot,
            OCENTRA_CLOUDFLARE_SEED_RUN_ID: runId,
          },
        })
      : spawnSync('npm', ['run', command], {
          cwd: path.join(repoRoot, 'infra', 'cloudflare'),
          encoding: 'utf8',
          env: {
            ...process.env,
            OCENTRA_CLOUDFLARE_LOCAL_PERSIST_PATH: persistenceRoot,
            OCENTRA_CLOUDFLARE_SEED_RUN_ID: runId,
          },
        });
  assert.equal(child.status, 0, child.stderr || child.stdout);
  const jsonStart = child.stdout.indexOf('{');
  assert.notEqual(jsonStart, -1, child.stdout);
  return JSON.parse(child.stdout.slice(jsonStart)) as SeedCommandOutput;
}

function resolveNpmExecPath(): string {
  const configured = process.env.npm_execpath?.trim();
  if (configured) {
    return configured;
  }
  const bundled = path.join(path.dirname(process.execPath), 'node_modules', 'npm', 'bin', 'npm-cli.js');
  assert.equal(existsSync(bundled), true, 'npm CLI is required for local Wrangler persistence assertions');
  return bundled;
}

function runLocalD1Command(persistenceRoot: string, sql: string): ReadonlyArray<Record<string, unknown>> {
  const child = spawnSync(
    process.execPath,
    [
      resolveNpmExecPath(),
      'exec',
      '--',
      'wrangler',
      'd1',
      'execute',
      'BILLING_D1',
      '--config',
      'wrangler.toml',
      '--local',
      '--persist-to',
      persistenceRoot,
      '--command',
      sql,
      '--json',
    ],
    {
      cwd: path.join(repoRoot, 'infra', 'cloudflare'),
      encoding: 'utf8',
      windowsHide: true,
    }
  );
  assert.equal(child.status, 0, child.stderr || child.stdout);
  const jsonStart = child.stdout.indexOf('[');
  assert.notEqual(jsonStart, -1, child.stdout);
  const operations = JSON.parse(child.stdout.slice(jsonStart)) as Array<{ results?: Array<Record<string, unknown>> }>;
  return operations.flatMap((operation) => operation.results ?? []);
}

function readStatusRowCount(persistenceRoot: string): number {
  const rows = runLocalD1Command(persistenceRoot, 'SELECT COUNT(*) AS row_count FROM billing_status');
  return Number(rows[0]?.row_count ?? 0);
}

function readSeedTableRowCount(
  persistenceRoot: string,
  table: 'billing_admin_accounts' | 'billing_admin_referrals'
): number {
  const rows = runLocalD1Command(persistenceRoot, `SELECT COUNT(*) AS row_count FROM ${table}`);
  return Number(rows[0]?.row_count ?? 0);
}

describe('local dev seeding workflow', () => {
  it('resolves the generated billing-contract sidecar from the module default even when cwd changes', () => {
    const tempCwd = mkdtempSync(path.join(os.tmpdir(), 'cloudflare-local-dev-cwd-'));

    try {
      const child = spawnSync(
        process.execPath,
        [
          '--import',
          'tsx',
          '--eval',
          `
            (async () => {
              try {
                process.chdir(${JSON.stringify(tempCwd)});
                const { collectMissingRuntimeDependencyBlockers } = await import(${JSON.stringify(localDevWorkflowModuleUrl)});
                const blockers = collectMissingRuntimeDependencyBlockers();
                process.stdout.write(JSON.stringify(blockers));
              } catch (error) {
                process.stderr.write(error instanceof Error ? error.name + ': ' + error.message : String(error));
                process.exit(1);
              }
            })().catch((error) => {
              process.stderr.write(error instanceof Error ? error.name + ': ' + error.message : String(error));
              process.exit(1);
            });
          `,
        ],
        {
          cwd: repoRoot,
          encoding: 'utf8',
        }
      );

      assert.equal(child.status, 0, child.stderr || child.stdout);
      assert.deepEqual(JSON.parse(child.stdout.trim() || '[]'), []);

      const generatedRoot = mkdtempSync(path.join(os.tmpdir(), 'cloudflare-local-dev-generated-'));

      try {
        const generatedPath = path.join(generatedRoot, 'src/generated/billing-contracts.ts');
        mkdirSync(path.dirname(generatedPath), { recursive: true });
        writeFileSync(generatedPath, '// generated sidecar');

        assert.deepEqual(collectMissingRuntimeDependencyBlockers(generatedRoot), []);

        const missingBlockers = collectMissingRuntimeDependencyBlockers(generatedRoot, [
          'src/generated/missing-billing-contracts.ts',
        ]);

        assert.deepEqual(missingBlockers, [
          {
            kind: 'missing-runtime-dependency',
            path: 'infra/cloudflare/src/generated/missing-billing-contracts.ts',
            details:
              'required generated billing-contract sidecar missing at infra/cloudflare/src/generated/missing-billing-contracts.ts',
          },
        ]);
      } finally {
        rmSync(generatedRoot, { recursive: true, force: true });
      }
    } finally {
      rmSync(tempCwd, { recursive: true, force: true });
    }
  });

  it(
    'persists the real Wrangler seed idempotently and isolates explicit local stores',
    { timeout: 120_000 },
    async () => {
      const persistenceA = mkdtempSync(path.join(os.tmpdir(), 'cloudflare-local-seed-a-'));
      const persistenceB = mkdtempSync(path.join(os.tmpdir(), 'cloudflare-local-seed-b-'));
      const runA = `cloudflare-wp07-seed-a-${randomUUID()}`;
      const runB = `cloudflare-wp07-seed-b-${randomUUID()}`;

      try {
        const firstA = runSeedCommand('seed:local', persistenceA, runA).mutationReceipt;
        const firstACount = readStatusRowCount(persistenceA);
        const secondA = runSeedCommand('seed:local', persistenceA, runA).mutationReceipt;
        const secondACount = readStatusRowCount(persistenceA);
        const firstB = runSeedCommand('seed:local', persistenceB, runB).mutationReceipt;
        const firstBCount = readStatusRowCount(persistenceB);

        assert.equal(firstA.runId, runA);
        assert.equal(secondA.runId, runA);
        assert.equal(firstB.runId, runB);
        assert.notEqual(runA, runB);
        assert.equal(firstA.persistenceTarget, 'explicit');
        assert.equal(firstA.runtimeBootStatus, 'proven');
        assert.equal(firstA.fullBindingSeedApplied, true);
        assert.ok(Object.values(firstA.persistence).every((count) => count > 0));
        assert.deepEqual(secondA.persistence, firstA.persistence);
        assert.equal(secondACount, firstACount);
        assert.equal(firstBCount, firstACount);

        runLocalD1Command(
          persistenceA,
          [
            'DELETE FROM billing_invoices',
            'DELETE FROM billing_referrals',
            'DELETE FROM billing_snapshots',
            'DELETE FROM billing_admin_accounts',
            'DELETE FROM billing_admin_invoices',
            'DELETE FROM billing_admin_disputes',
            'DELETE FROM billing_admin_referrals',
          ].join('; ')
        );
        assert.equal(readStatusRowCount(persistenceA), firstACount);
        assert.equal(readSeedTableRowCount(persistenceA, 'billing_admin_accounts'), 0);
        assert.equal(readSeedTableRowCount(persistenceA, 'billing_admin_referrals'), 0);

        const repairedA = runSeedCommand('seed:local', persistenceA, runA).mutationReceipt;
        assert.equal(readStatusRowCount(persistenceA), firstACount);
        assert.ok(readSeedTableRowCount(persistenceA, 'billing_admin_accounts') > 0);
        assert.ok(readSeedTableRowCount(persistenceA, 'billing_admin_referrals') > 0);
        assert.deepEqual(repairedA.persistence, firstA.persistence);

        runLocalD1Command(persistenceA, "DELETE FROM billing_status WHERE subject = 'parent:demo-review'");
        assert.equal(readStatusRowCount(persistenceA), firstACount - 1);
        assert.equal(readStatusRowCount(persistenceB), firstBCount);

        const activeLeasePath = path.join(persistenceA, 'active-runtime.lock');
        const activeLease = await acquireLocalWranglerRuntimeLease({
          lockPath: activeLeasePath,
          heartbeatIntervalMs: 20,
          pollIntervalMs: 10,
        });
        const firstHeartbeat = JSON.parse(readFileSync(activeLeasePath, 'utf8')) as { heartbeatAt: string };
        await new Promise((resolve) => setTimeout(resolve, 60));
        const secondHeartbeat = JSON.parse(readFileSync(activeLeasePath, 'utf8')) as { heartbeatAt: string };
        assert.notEqual(secondHeartbeat.heartbeatAt, firstHeartbeat.heartbeatAt);

        const oldTimestamp = new Date(0);
        utimesSync(activeLeasePath, oldTimestamp, oldTimestamp);
        let contenderAcquired = false;
        const contenderPromise = acquireLocalWranglerRuntimeLease({
          lockPath: activeLeasePath,
          acquireTimeoutMs: 1_000,
          heartbeatIntervalMs: 20,
          pollIntervalMs: 10,
          invalidRecordStaleAfterMs: 1,
        }).then((lease) => {
          contenderAcquired = true;
          return lease;
        });
        await new Promise((resolve) => setTimeout(resolve, 80));
        assert.equal(contenderAcquired, false, 'lease age must not evict a live owner');
        activeLease.release();
        const contenderLease = await contenderPromise;
        assert.equal(contenderAcquired, true);
        contenderLease.release();

        const crashedLeasePath = path.join(persistenceA, 'crashed-runtime.lock');
        writeFileSync(
          crashedLeasePath,
          JSON.stringify({
            schema: 1,
            token: 'crashed-owner',
            ownerPid: 2_147_483_647,
            createdAt: new Date().toISOString(),
            heartbeatAt: new Date().toISOString(),
          })
        );
        const recoveredLease = await acquireLocalWranglerRuntimeLease({
          lockPath: crashedLeasePath,
          acquireTimeoutMs: 1_000,
          pollIntervalMs: 10,
        });
        assert.notEqual(
          (JSON.parse(readFileSync(crashedLeasePath, 'utf8')) as { token: string }).token,
          'crashed-owner'
        );
        recoveredLease.release();
      } finally {
        rmSync(persistenceA, { recursive: true, force: true });
        rmSync(persistenceB, { recursive: true, force: true });
      }
    }
  );

  it('persists a correlated redacted proof chain for ready preflight, populated seeds, and teardown', async () => {
    const proofStoreRoot = mkdtempSync(path.join(os.tmpdir(), 'cloudflare-local-dev-proof-'));
    const proofRunId =
      process.env.OCENTRA_PARENT_PROOF_RUN_ID?.trim() || `cloudflare-wp07-local-dev-proof-${randomUUID()}`;
    const proofCorrelationId = `${proofRunId}:local-dev-seeding`;
    const forcedProofEnvironment: Readonly<Record<string, string>> = {
      NODE_ENV: 'test',
      OCENTRA_PARENT_TEST_MODE: 'true',
      OCENTRA_PARENT_LOG_ENABLED: 'true',
      OCENTRA_PARENT_LOG_STORE: 'true',
      OCENTRA_PARENT_LOG_LEVEL: 'debug',
      OCENTRA_PARENT_LOG_CONSOLE: 'false',
      OCENTRA_CLOUDFLARE_SEED_LOG_BRIDGE_ENDPOINT: '',
    };
    const originalProofEnvironment = new Map(
      Object.keys(forcedProofEnvironment).map((key) => [key, process.env[key]] as const)
    );
    const [{ Logger }, { getStackTrace }, { createBridgeServer }, loggingTypes, ndjsonPaths, ndjsonWriter] =
      await Promise.all([
        import(loggerModuleUrl),
        import(stackTraceModuleUrl),
        import(bridgeServerModuleUrl),
        import(loggingTypesModuleUrl),
        import(ndjsonPathsModuleUrl),
        import(ndjsonWriterModuleUrl),
      ]);
    const server = createBridgeServer({ rootDir: proofStoreRoot });
    for (const [key, value] of Object.entries(forcedProofEnvironment)) {
      process.env[key] = value;
    }

    try {
      const address = await new Promise<{ port: number }>((resolve, reject) => {
        server.once('error', reject);
        server.listen(0, '127.0.0.1', () => {
          const boundAddress = server.address();
          if (boundAddress == null || typeof boundAddress === 'string') {
            reject(new Error('Cloudflare WP07 proof bridge did not bind a TCP port'));
            return;
          }
          resolve({ port: boundAddress.port });
        });
      });
      Logger.instance.reset();
      Logger.instance.configure({
        bridgeEndpoint: `http://127.0.0.1:${address.port}`,
        runId: proofRunId,
        testName: 'local-dev-seeding-workflow.test.ts',
        scope: loggingTypes.TestLogScope.ParentCloudflare,
        runType: loggingTypes.RunType.Single,
        suiteType: loggingTypes.TestSuiteType.Integration,
        origin: loggingTypes.TestLogOrigin.Test,
        environment: 'local-test',
        correlationId: proofCorrelationId,
        skipHealthCheck: true,
      });
      Logger.instance.register(import.meta.url);

      const emitProofMilestone = (
        boundary: string,
        result: 'accepted' | 'consumed' | 'stored',
        details: Readonly<Record<string, unknown>>
      ): void => {
        Logger.instance.logInfo(
          'cloudflare WP07 local workflow proof milestone',
          getStackTrace(),
          redactPayload({
            runId: proofRunId,
            correlationId: proofCorrelationId,
            owner: proofOwner,
            boundary,
            result,
            noClaimReason: proofNoClaimReason,
            redactionState: proofRedactionState,
            ...details,
          }),
          true
        );
      };

      emitProofMilestone('workflow-command', 'accepted', {
        command: 'node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts',
        subject: 'infra/cloudflare/scripts/local-dev-workflow.ts',
        redactionProbe: {
          authorization: 'Bearer private-token',
          stripeWebhookSecret: 'whsec_private',
          childProfile: 'child-name-private',
        },
      });

      const workflow = inspectLocalDevWorkflow();

      assert.equal(workflow.start.rootCommand, 'npm run dev:cloudflare');
      assert.equal(workflow.start.moduleCommand, 'npm --prefix infra/cloudflare run dev');
      assert.equal(workflow.start.wranglerCommand, 'wrangler dev --local');
      assert.equal(workflow.start.preflightStatus, 'ready');
      assert.equal(workflow.start.importCheckStatus, 'passed');
      assert.equal(workflow.start.runtimeBootStatus, 'unproven');
      assert.match(workflow.start.runtimeBootEvidence, /does not start Wrangler/);
      assert.deepEqual(workflow.start.blockers, []);
      emitProofMilestone('import-preflight', 'accepted', {
        preflightStatus: workflow.start.preflightStatus,
        importCheckStatus: workflow.start.importCheckStatus,
        runtimeBootStatus: workflow.start.runtimeBootStatus,
        authAdapterMode: workflow.start.authAdapterMode,
      });

      assert.equal(workflow.seed.aggregateCommand, 'npm --prefix infra/cloudflare run seed:local');
      assert.equal(workflow.seed.status, 'runnable');
      assert.deepEqual(
        workflow.seed.fixtureFamilies.map(({ family, populationState, itemCount }) => ({
          family,
          populationState,
          itemCount,
        })),
        [
          { family: 'pricing-catalog', populationState: 'populated', itemCount: 3 },
          { family: 'parent-test-accounts', populationState: 'populated', itemCount: 4 },
          { family: 'support-admin-test-accounts', populationState: 'populated', itemCount: 4 },
          { family: 'referral-test-graph', populationState: 'populated', itemCount: 2 },
          {
            family: 'webhook-payload-fixtures',
            populationState: 'test-fixture-backed',
            itemCount: LOCAL_WEBHOOK_FIXTURE_INVENTORY.length,
          },
          {
            family: 'queue-replay-fixtures',
            populationState: 'test-fixture-backed',
            itemCount: LOCAL_QUEUE_REPLAY_FIXTURE_INVENTORY.length,
          },
        ]
      );
      emitProofMilestone('seed-fixtures', 'consumed', {
        seedStatus: workflow.seed.status,
        fixtureFamilyCount: workflow.seed.fixtureFamilies.length,
        populatedItemCount: workflow.seed.fixtureFamilies.reduce((total, family) => total + (family.itemCount ?? 0), 0),
      });

      assert.equal(workflow.teardown.status, 'explicit');
      assert.equal(workflow.teardown.steps.length, 3);
      emitProofMilestone('teardown-contract', 'accepted', {
        teardownStatus: workflow.teardown.status,
        teardownStepCount: workflow.teardown.steps.length,
      });
      emitProofMilestone('proof-chain', 'stored', { milestoneCount: 5 });
      await Logger.instance.flush();

      const proofFiles = ndjsonPaths.listNdjsonFiles(
        ndjsonPaths.getTestLogScopeDir(loggingTypes.TestLogScope.ParentCloudflare, proofStoreRoot)
      );
      assert.equal(proofFiles.length, 1);
      const persistedRows = ndjsonWriter.readTestLogEntriesFromFile(proofFiles[0]);
      assert.equal(persistedRows.length, 5);
      assert.ok(persistedRows.every((row: { runId: string }) => row.runId === proofRunId));
      assert.ok(
        persistedRows.every((row: { correlationId: string | null }) => row.correlationId === proofCorrelationId)
      );

      const persistedMilestones: PersistedProofMilestone[] = persistedRows.map((row: { data: string | null }) => {
        assert.notEqual(row.data, null);
        return JSON.parse(row.data ?? '{}') as PersistedProofMilestone;
      });
      assert.deepEqual(
        persistedMilestones.map(({ boundary, result }) => ({ boundary, result })),
        [
          { boundary: 'workflow-command', result: 'accepted' },
          { boundary: 'import-preflight', result: 'accepted' },
          { boundary: 'seed-fixtures', result: 'consumed' },
          { boundary: 'teardown-contract', result: 'accepted' },
          { boundary: 'proof-chain', result: 'stored' },
        ]
      );
      assert.ok(
        persistedMilestones.every(
          (milestone) =>
            milestone.runId === proofRunId &&
            milestone.correlationId === proofCorrelationId &&
            milestone.owner === proofOwner &&
            milestone.noClaimReason === proofNoClaimReason &&
            milestone.redactionState === proofRedactionState
        )
      );

      const persistedProofText = JSON.stringify(persistedRows);
      assert.deepEqual(persistedMilestones[0]?.redactionProbe, {
        authorization: '[redacted]',
        stripeWebhookSecret: '[redacted]',
        childProfile: '[redacted]',
      });
      assert.equal(persistedProofText.includes('[redacted]'), true);
      for (const forbiddenValue of ['Bearer private-token', 'whsec_private', 'child-name-private']) {
        assert.equal(persistedProofText.includes(forbiddenValue), false);
      }

      const originalNpmExecPath = process.env.npm_execpath;
      const originalSeedRunId = process.env.OCENTRA_CLOUDFLARE_SEED_RUN_ID;
      process.env.npm_execpath = path.join(proofStoreRoot, 'missing-npm-cli.js');
      process.env.OCENTRA_CLOUDFLARE_SEED_RUN_ID = proofRunId;
      process.env.OCENTRA_CLOUDFLARE_SEED_LOG_BRIDGE_ENDPOINT = `http://127.0.0.1:${address.port}`;
      try {
        await assert.rejects(runLocalSeedMutation('failure-path-proof'), /Wrangler seed runtime exited/);
      } finally {
        if (originalNpmExecPath == null) {
          delete process.env.npm_execpath;
        } else {
          process.env.npm_execpath = originalNpmExecPath;
        }
        if (originalSeedRunId == null) {
          delete process.env.OCENTRA_CLOUDFLARE_SEED_RUN_ID;
        } else {
          process.env.OCENTRA_CLOUDFLARE_SEED_RUN_ID = originalSeedRunId;
        }
      }

      const rowsAfterFailure = ndjsonWriter.readTestLogEntriesFromFile(proofFiles[0]);
      const runtimeMilestones: PersistedProofMilestone[] = rowsAfterFailure
        .slice(persistedRows.length)
        .map((row: { data: string | null }) => JSON.parse(row.data ?? '{}') as PersistedProofMilestone);
      assert.deepEqual(
        runtimeMilestones.map(({ boundary, result }) => ({ boundary, result })),
        [
          { boundary: 'startup', result: 'accepted' },
          { boundary: 'runtime-start', result: 'accepted' },
          { boundary: 'seed-validation', result: 'rejected' },
          { boundary: 'teardown', result: 'released' },
        ]
      );
      assert.ok(
        runtimeMilestones.every(
          (milestone) =>
            milestone.runId === proofRunId &&
            milestone.correlationId === `${proofRunId}:seed-runtime` &&
            milestone.redactionState === proofRedactionState
        )
      );
    } finally {
      Logger.instance.reset();
      for (const [key, value] of originalProofEnvironment) {
        if (value == null) {
          delete process.env[key];
        } else {
          process.env[key] = value;
        }
      }
      server.closeAllConnections();
      await new Promise<void>((resolve, reject) => {
        server.close((error: Error | undefined) => {
          if (error != null) {
            reject(error);
            return;
          }
          resolve();
        });
      });
      rmSync(proofStoreRoot, { recursive: true, force: true });
    }
  });

  it('reports blocked seed truth when promoted dependencies are unavailable', () => {
    const child = spawnSync(
      process.execPath,
      [
        '--import',
        'tsx',
        '--eval',
        `
            (async () => {
              try {
                const { inspectLocalDevWorkflow } = await import(${JSON.stringify(localDevWorkflowModuleUrl)});
                const workflow = inspectLocalDevWorkflow();
                process.stdout.write(JSON.stringify(workflow));
              } catch (error) {
                process.stderr.write(error instanceof Error ? error.name + ': ' + error.message : String(error));
                process.exit(1);
              }
            })().catch((error) => {
              process.stderr.write(error instanceof Error ? error.name + ': ' + error.message : String(error));
              process.exit(1);
            });
          `,
      ],
      {
        cwd: repoRoot,
        encoding: 'utf8',
        env: {
          ...process.env,
          PATH: '',
        },
      }
    );

    assert.equal(child.status, 0, child.stderr || child.stdout);

    const workflow = JSON.parse(child.stdout.trim());
    assert.equal(workflow.start.preflightStatus, 'ready');
    assert.equal(workflow.start.importCheckStatus, 'passed');
    assert.equal(workflow.start.runtimeBootStatus, 'unproven');
    assert.equal(workflow.seed.status, 'blocked');
    const blockedSeedFamilies = workflow.seed.fixtureFamilies.filter(
      (family: { populationState: string }) => family.populationState === 'blocked'
    );
    assert.equal(blockedSeedFamilies.length, 4);
    assert.ok(
      blockedSeedFamilies.every(
        (family: { populationState: string; blocker?: { kind?: string; details?: string } }) =>
          family.populationState === 'blocked' &&
          family.blocker?.kind === 'missing-runtime-dependency' &&
          (family.blocker.details?.length ?? 0) > 0
      )
    );
  });
});

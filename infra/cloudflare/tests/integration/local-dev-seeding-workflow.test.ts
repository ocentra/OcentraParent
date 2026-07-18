import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, it } from 'node:test';
import { collectMissingRuntimeDependencyBlockers, inspectLocalDevWorkflow } from '../../scripts/local-dev-workflow.js';
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

const proofRunId = 'cloudflare-wp07-local-dev-proof';
const proofCorrelationId = 'cloudflare-wp07-local-dev-correlation';
const proofOwner = 'infra/cloudflare/scripts/local-dev-workflow.ts';
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
                const { collectMissingRuntimeDependencyBlockers } = await import(${JSON.stringify(localDevWorkflowModuleUrl)});
                process.chdir(${JSON.stringify(tempCwd)});
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

  it('persists a correlated redacted proof chain for ready preflight, populated seeds, and teardown', async () => {
    const proofStoreRoot = mkdtempSync(path.join(os.tmpdir(), 'cloudflare-local-dev-proof-'));
    const originalLogLevel = process.env.OCENTRA_PARENT_LOG_LEVEL;
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

      process.env.OCENTRA_PARENT_LOG_LEVEL = 'debug';
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
          { family: 'webhook-payload-fixtures', populationState: 'test-fixture-backed', itemCount: 5 },
          { family: 'queue-replay-fixtures', populationState: 'test-fixture-backed', itemCount: 2 },
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
    } finally {
      Logger.instance.reset();
      if (originalLogLevel == null) {
        delete process.env.OCENTRA_PARENT_LOG_LEVEL;
      } else {
        process.env.OCENTRA_PARENT_LOG_LEVEL = originalLogLevel;
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

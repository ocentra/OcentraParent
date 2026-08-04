import fs from 'node:fs';
import assert from 'node:assert/strict';
import os from 'node:os';
import path from 'node:path';
import { env } from 'node:process';
import { describe, it } from 'node:test';
import { readTestLogEntriesFromFile } from '@ocentra-parent/logging-domain/test-log/ndjsonWriter';
import { getRunNdjsonFilePath } from '@ocentra-parent/logging-domain/test-log/ndjsonPaths';
import { RunType, TestLogScope } from '@ocentra-parent/logging-domain/test-log/types';
import { inspectLocalDevWorkflow } from '../../scripts/local-dev-workflow.js';

describe('local dev seeding workflow', () => {
  it('keeps start, seed, teardown, and blocker truth explicit', () => {
    const workflow = inspectLocalDevWorkflow();

    assert.equal(workflow.start.rootCommand, 'npm run dev:cloudflare');
    assert.equal(workflow.start.moduleCommand, 'npm --prefix infra/cloudflare run dev');
    assert.equal(workflow.start.wranglerCommand, 'wrangler dev --local');
    assert.equal(workflow.seed.aggregateCommand, 'npm --prefix infra/cloudflare run seed:local');
    assert.equal(workflow.teardown.status, 'explicit');

    const fixtureFamilies = workflow.seed.fixtureFamilies.map((family) => family.family);
    assert.deepEqual(fixtureFamilies, [
      'pricing-catalog',
      'parent-test-accounts',
      'support-admin-test-accounts',
      'referral-test-graph',
      'webhook-payload-fixtures',
      'queue-replay-fixtures',
    ]);

    const seededFamilies = workflow.seed.fixtureFamilies.filter(
      (family) => family.populationState !== 'test-fixture-backed'
    );
    for (const family of seededFamilies) {
      if (family.populationState === 'blocked') {
        assert.ok(family.blocker);
        assert.ok(family.blocker?.details.length > 0);
      } else {
        assert.equal(family.populationState, 'populated');
        assert.ok((family.itemCount ?? 0) > 0);
      }
    }

    if (workflow.start.status === 'blocked') {
      assert.ok(workflow.start.blockers.length > 0);
      assert.ok(workflow.start.blockers.some((blocker) => blocker.details.length > 0));
    } else {
      assert.deepEqual(workflow.start.blockers, []);
    }

    assert.equal(
      workflow.start.blockers.some(
        (blocker) =>
          blocker.kind === 'missing-runtime-dependency' && blocker.path === 'src/generated/billing-contracts.ts'
      ),
      false,
      'the Rust-generated billing contract sidecar is inside infra/cloudflare and must not be probed from the repo root'
    );

    if (workflow.seed.status === 'blocked') {
      assert.ok(
        workflow.seed.fixtureFamilies.some(
          (family) =>
            family.populationState === 'blocked' && family.blocker?.details.includes('billing-account-runtime-boundary')
        )
      );
    }
  });

  it('emits gated, correlated, redacted proof milestones when logging is enabled', () => {
    const logRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'cloudflare-local-proof-'));
    const previousLogRoot = env.OCENTRA_PARENT_LOG_ROOT;
    const previousRunId = env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID;

    env.OCENTRA_PARENT_LOG_ROOT = logRoot;
    env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID = 'cloudflare-local-test-run';

    try {
      const workflow = inspectLocalDevWorkflow();
      const logFile = getRunNdjsonFilePath(
        TestLogScope.ParentCloudflare,
        RunType.Single,
        'cloudflare-local-test-run',
        'integration',
        logRoot
      );
      const logEntries = readTestLogEntriesFromFile(logFile);
      assert.equal(logEntries.length, workflow.start.blockers.length > 0 ? 5 : 4);

      const logText = fs.readFileSync(logFile, 'utf8');
      const events = logEntries.map((entry) => entry.context?.replace('cloudflare.local-dev.', ''));
      assert.deepEqual(
        events,
        workflow.start.blockers.length > 0
          ? [
              'workflow_started',
              'start_path_observed',
              'start_blocker_observed',
              'seed_path_observed',
              'workflow_completed',
            ]
          : ['workflow_started', 'start_path_observed', 'seed_path_observed', 'workflow_completed']
      );
      for (const entry of logEntries) {
        assert.equal(entry.type, 'log');
        assert.equal(entry.scope, TestLogScope.ParentCloudflare);
        assert.equal(entry.runId, 'cloudflare-local-test-run');
        assert.equal(entry.correlationId, 'cloudflare-local-test-run');
        assert.equal(entry.testName, 'cloudflare-local-dev-workflow');
        assert.ok(entry.timestamp > 0);
        assert.match(entry.message, /^Cloudflare local-dev workflow /);
        assert.ok(entry.data?.includes('"owner":"cloudflare-control-plane"'));
        assert.match(entry.data ?? '', /"boundaryResult":"(started|observed|blocked|completed)"/);
        assert.ok(entry.data?.includes('"redactionState":"applied"'));
      }

      assert.ok(logText.includes('"runId":"cloudflare-local-test-run"'));
      assert.doesNotMatch(logText, /sk_test_/);
      assert.doesNotMatch(logText, /[A-Z]:\\\\/);
    } finally {
      if (previousLogRoot === undefined) {
        delete env.OCENTRA_PARENT_LOG_ROOT;
      } else {
        env.OCENTRA_PARENT_LOG_ROOT = previousLogRoot;
      }
      if (previousRunId === undefined) {
        delete env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID;
      } else {
        env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID = previousRunId;
      }
      fs.rmSync(logRoot, { recursive: true, force: true });
    }
  });
});

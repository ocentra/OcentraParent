import fs from 'node:fs';
import assert from 'node:assert/strict';
import os from 'node:os';
import path from 'node:path';
import { env } from 'node:process';
import { describe, it } from 'node:test';
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
      const streamRoot = path.join(logRoot, 'parent-codex', 'ndjson', 'cloudflare-local-dev-workflow');
      const logFiles = fs.readdirSync(streamRoot).filter((fileName) => fileName.endsWith('.ndjson'));
      assert.equal(logFiles.length, 1);

      const logFile = logFiles[0];
      assert.ok(logFile);
      const logText = fs.readFileSync(path.join(streamRoot, logFile), 'utf8');

      assert.match(logText, /"event":"workflow_started"/);
      assert.match(logText, /"event":"start_path_observed"/);
      if (workflow.start.blockers.length > 0) {
        assert.match(logText, /"event":"start_blocker_observed"/);
      }
      assert.match(logText, /"event":"seed_path_observed"/);
      assert.match(logText, /"event":"workflow_completed"/);
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

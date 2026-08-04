import fs from 'node:fs';
import assert from 'node:assert/strict';
import os from 'node:os';
import path from 'node:path';
import { env } from 'node:process';
import { describe, it } from 'node:test';
import { fileURLToPath } from 'node:url';
import { readTestLogEntriesFromFile } from '@ocentra-parent/logging-domain/test-log/ndjsonWriter';
import { getRunNdjsonFilePath } from '@ocentra-parent/logging-domain/test-log/ndjsonPaths';
import { RunType, TestLogScope } from '@ocentra-parent/logging-domain/test-log/types';
import { redactPayload } from '../../src/security/redaction.js';
import {
  buildStartProofMilestoneDetails,
  buildSeedProofMilestoneDetails,
  failClosedRequiredFixtureFamilies,
  inspectLocalDevWorkflow,
  seedStatusFromFixtureFamilies,
  writeLocalDevProofSummary,
} from '../../scripts/local-dev-workflow.js';
import { summarizeProofLogLocation } from '../../scripts/local-dev-proof.js';

const cloudflareRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

describe('local dev seeding workflow', () => {
  it('keeps start, seed, teardown, and blocker truth explicit', () => {
    const workflow = inspectLocalDevWorkflow();

    assert.equal(workflow.start.rootCommand, 'npm run dev:cloudflare');
    assert.equal(workflow.start.moduleCommand, 'npm --prefix infra/cloudflare run dev');
    assert.equal(workflow.start.wranglerCommand, 'wrangler dev --local');
    assert.equal(workflow.seed.aggregateCommand, 'npm --prefix infra/cloudflare run seed:local');
    assert.equal(workflow.teardown.status, 'explicit');
    assert.ok(workflow.start.noClaimReason.length > 0);
    assert.deepEqual(workflow.teardown.ownershipConditions, [
      'Stop only the wrangler dev --local process started by this workflow or its harness.',
      'Remove a --persist-to directory only when this workflow or its harness created it.',
      'Remove infra/cloudflare/.dev.vars only when this workflow or its harness created it.',
    ]);

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
      const blockedFamilies = workflow.seed.fixtureFamilies.filter((family) => family.populationState === 'blocked');
      assert.ok(blockedFamilies.length > 0);
      for (const family of blockedFamilies) {
        assert.ok(family.blocker);
        assert.ok(family.blocker?.details.length > 0);
      }

      const seedProof = buildSeedProofMilestoneDetails(workflow.seed);
      assert.equal(seedProof.noClaimReason, 'seed-fixture-population-not-proven');
      assert.ok(
        seedProof.fixtureFamilies
          .filter((family) => family.populationState === 'blocked')
          .every((family) => family.noClaimReason === 'seed-command-blocked')
      );

      if (process.platform === 'win32') {
        assert.ok(
          blockedFamilies.some((family) => family.blocker?.details.includes('billing-account-runtime-boundary'))
        );
      }
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
      assert.equal(logEntries.length, workflow.start.blockers.length > 0 ? 6 : 5);

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
              'teardown_path_observed',
              'workflow_completed',
            ]
          : [
              'workflow_started',
              'start_path_observed',
              'seed_path_observed',
              'teardown_path_observed',
              'workflow_completed',
            ]
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

      const seedEntry = logEntries.find((entry) => entry.context === 'cloudflare.local-dev.seed_path_observed');
      assert.ok(seedEntry);
      const seedData = seedEntry.data ?? '';
      const blockedFixture = workflow.seed.fixtureFamilies.find((family) => family.populationState === 'blocked');
      if (blockedFixture !== undefined) {
        assert.match(seedData, /"noClaimReason":"seed-fixture-population-not-proven"/);
        assert.match(seedData, /"blocker":\{"kind":"(?:missing-runtime-dependency|runtime-import-check)"/);
        assert.match(seedData, /"noClaimReason":"seed-command-blocked"/);
      } else {
        assert.match(seedData, /"noClaimReason":"retained-workpack-proof-absent"/);
      }
      for (const fixtureFamily of workflow.seed.fixtureFamilies) {
        assert.ok(seedData.includes(`"source":"${fixtureFamily.source}"`));
      }

      const startEntry = logEntries.find((entry) => entry.context === 'cloudflare.local-dev.start_path_observed');
      assert.ok(startEntry);
      assert.match(
        startEntry.data ?? '',
        /"noClaimReason":"(?:local-worker-not-launched-or-response-verified|start-probe-blocked-before-local-worker-launch)"/
      );

      const teardownEntry = logEntries.find((entry) => entry.context === 'cloudflare.local-dev.teardown_path_observed');
      assert.ok(teardownEntry);
      assert.match(teardownEntry.data ?? '', /"ownershipConditions":\[/);
      assert.match(
        teardownEntry.data ?? '',
        /Remove a --persist-to directory only when this workflow or its harness created it\./
      );
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

  it('keeps fabricated seed blocker diagnostics structured and redacted', () => {
    const details = buildSeedProofMilestoneDetails({
      aggregateCommand: 'npm --prefix infra/cloudflare run seed:local',
      commands: [],
      status: 'blocked',
      fixtureFamilies: [
        {
          family: 'pricing-catalog',
          source: 'seed-products-local',
          populationState: 'blocked',
          itemCount: null,
          notes: 'blocked fixture',
          blocker: {
            kind: 'missing-runtime-dependency',
            path: 'C:\\private\\seed-state',
            details: 'sk_test_12345 C:\\private\\seed-state',
          },
        },
      ],
    });
    const redacted = JSON.stringify(redactPayload(details));
    assert.ok(redacted.includes('"noClaimReason":"seed-fixture-population-not-proven"'));
    assert.ok(redacted.includes('"noClaimReason":"seed-command-blocked"'));
    assert.ok(redacted.includes('"kind":"missing-runtime-dependency"'));
    assert.ok(redacted.includes('"source":"seed-products-local"'));
    assert.doesNotMatch(redacted, /sk_test_/);
    assert.doesNotMatch(redacted, /[A-Z]:\\\\/);
  });

  it('fails closed when a required seed command returns an empty fixture collection', () => {
    const fixtureFamilies = failClosedRequiredFixtureFamilies([
      {
        family: 'pricing-catalog',
        source: 'npm --prefix infra/cloudflare run seed:products:local',
        populationState: 'placeholder',
        itemCount: 0,
        notes: 'empty required seed output',
      },
    ]);

    assert.equal(fixtureFamilies[0]?.populationState, 'blocked');
    assert.equal(fixtureFamilies[0]?.blocker?.kind, 'population-failure');
    assert.equal(
      fixtureFamilies[0]?.blocker?.details,
      'Required fixture family pricing-catalog returned no populated items from npm --prefix infra/cloudflare run seed:products:local.'
    );
    assert.equal(seedStatusFromFixtureFamilies(fixtureFamilies), 'blocked');
  });

  it('redacts only absolute start blocker path segments while retaining safe diagnostics', () => {
    const details = buildStartProofMilestoneDetails({
      rootCommand: 'npm run dev:cloudflare',
      moduleCommand: 'npm --prefix infra/cloudflare run dev',
      wranglerCommand: 'wrangler dev --local',
      origin: 'http://localhost:3000',
      authAdapterMode: 'account-auth-adapter-manual-required',
      status: 'blocked',
      noClaimReason: 'start-probe-blocked-before-local-worker-launch',
      blockers: [
        {
          kind: 'runtime-import-check',
          details: 'Error: failed to import C:\\private\\worker\\entry.ts: billing module was unavailable',
        },
      ],
    });
    const redacted = JSON.stringify(redactPayload(details));

    assert.ok(redacted.includes('Error: failed to import [redacted-path]: billing module was unavailable'));
    assert.doesNotMatch(redacted, /[A-Z]:\\\\/);
    assert.doesNotMatch(redacted, /private\\\\worker/);
  });

  it('writes proof summaries through the canonical redacted logger without exposing an absolute root', () => {
    const logRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'cloudflare-proof-summary-'));
    const previousLogRoot = env.OCENTRA_PARENT_LOG_ROOT;
    const previousRunId = env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID;
    const runId = 'cloudflare-proof-summary-test';
    try {
      env.OCENTRA_PARENT_LOG_ROOT = logRoot;
      env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID = runId;
      const repoRelativeRoot = path.join(
        cloudflareRoot,
        '..',
        '..',
        'output',
        'cloudflare-control-plane-plan-proof',
        'summary-test'
      );
      assert.equal(
        summarizeProofLogLocation(repoRelativeRoot),
        'output/cloudflare-control-plane-plan-proof/summary-test'
      );
      assert.equal(
        summarizeProofLogLocation(path.resolve(cloudflareRoot, '..', '..', '..', 'external-proof-root')),
        'external-proof-root-redacted'
      );

      writeLocalDevProofSummary({
        runId,
        proofLogLocation: summarizeProofLogLocation(repoRelativeRoot),
        startStatus: 'runnable',
        seedStatus: 'runnable',
        teardownStatus: 'explicit',
        noClaim: 'local validation logs are not a tracked WP07 proof bundle or local Worker response proof',
      });
      const logFile = getRunNdjsonFilePath(
        TestLogScope.ParentCloudflare,
        RunType.Single,
        runId,
        'integration',
        logRoot
      );
      const summaryEntry = readTestLogEntriesFromFile(logFile).find(
        (entry) => entry.context === 'cloudflare.local-dev.proof_summary_observed'
      );
      assert.ok(summaryEntry);
      assert.match(
        summaryEntry.data ?? '',
        /"proofLogLocation":"output\/cloudflare-control-plane-plan-proof\/summary-test"/
      );
      assert.doesNotMatch(summaryEntry.data ?? '', /[A-Z]:\\\\/);
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

  it('prepares the canonical logger before default, focused, and integration test paths', () => {
    const packageJson = fs.readFileSync(path.join(cloudflareRoot, 'package.json'), 'utf8');
    assert.match(
      packageJson,
      /"proof:logger-ready": "npm --prefix \.\.\/\.\. --workspace @ocentra-parent\/logging-domain run build"/
    );
    assert.match(packageJson, /"test": "npm run test:logger-ready && tsx scripts\/test-runner\.ts"/);
    assert.match(
      packageJson,
      /"test:local-dev-workflow": "npm run test:logger-ready && node --import tsx --test tests\/integration\/local-dev-seeding-workflow\.test\.ts"/
    );
    assert.match(
      packageJson,
      /"test:integration": "npm run test:logger-ready && tsx scripts\/test-runner\.ts --type=integration"/
    );
  });
});

import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { describe, it } from 'node:test';
import {
  collectMissingRuntimeDependencyBlockers,
  inspectLocalDevWorkflow,
} from '../../scripts/local-dev-workflow.js';

describe('local dev seeding workflow', () => {
  it('resolves the generated billing-contract sidecar relative to infra/cloudflare from any cwd', () => {
    const tempRoot = mkdtempSync(path.join(os.tmpdir(), 'cloudflare-local-dev-'));

    try {
      const generatedPath = path.join(tempRoot, 'src/generated/billing-contracts.ts');
      mkdirSync(path.dirname(generatedPath), { recursive: true });
      writeFileSync(generatedPath, '// generated sidecar');

      assert.deepEqual(collectMissingRuntimeDependencyBlockers(tempRoot), []);

      const missingBlockers = collectMissingRuntimeDependencyBlockers(tempRoot, [
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
      rmSync(tempRoot, { recursive: true, force: true });
    }
  });

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

    if (workflow.seed.status === 'blocked') {
      assert.ok(
        workflow.seed.fixtureFamilies.some(
          (family) =>
            family.populationState === 'blocked' &&
            family.blocker?.details.includes('generated billing-contract sidecar')
        )
      );
    }
  });
});

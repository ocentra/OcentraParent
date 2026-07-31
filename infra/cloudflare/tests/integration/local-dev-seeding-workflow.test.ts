import assert from 'node:assert/strict';
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
        (blocker) => blocker.kind === 'missing-runtime-dependency' && blocker.path === 'src/generated/billing-contracts.ts'
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
});

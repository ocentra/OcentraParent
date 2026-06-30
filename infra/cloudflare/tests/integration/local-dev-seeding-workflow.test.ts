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

    const seededFamilies = workflow.seed.fixtureFamilies.filter((family) => family.populationState !== 'test-fixture-backed');
    for (const family of seededFamilies) {
      if (family.populationState === 'blocked') {
        assert.ok(family.blocker);
        assert.ok(family.blocker?.details.includes('billing-domain') || family.blocker?.details.includes('ERR_MODULE_NOT_FOUND'));
      } else {
        assert.equal(family.populationState, 'populated');
        assert.ok((family.itemCount ?? 0) > 0);
      }
    }

    if (workflow.start.status === 'blocked') {
      assert.ok(workflow.start.blockers.length > 0);
      assert.ok(
        workflow.start.blockers.some(
          (blocker) =>
            blocker.path === 'packages/billing-domain/src/billing-checkout-portal-boundary.js' ||
            blocker.details.includes('billing-checkout-portal-boundary')
        )
      );
    } else {
      assert.deepEqual(workflow.start.blockers, []);
    }

    if (workflow.seed.status === 'blocked') {
      assert.ok(
        workflow.seed.fixtureFamilies.some(
          (family) =>
            family.populationState === 'blocked' &&
            family.blocker?.details.includes('billing-account-runtime-boundary')
        )
      );
    }
  });
});

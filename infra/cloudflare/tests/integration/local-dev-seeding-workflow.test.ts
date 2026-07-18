import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, it } from 'node:test';
import { collectMissingRuntimeDependencyBlockers, inspectLocalDevWorkflow } from '../../scripts/local-dev-workflow.js';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const localDevWorkflowModuleUrl = new URL('../../scripts/local-dev-workflow.ts', import.meta.url).href;

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
              const { collectMissingRuntimeDependencyBlockers } = await import(${JSON.stringify(localDevWorkflowModuleUrl)});
              process.chdir(${JSON.stringify(tempCwd)});
              const blockers = collectMissingRuntimeDependencyBlockers();
              process.stdout.write(JSON.stringify(blockers));
            })().catch((error) => {
              console.error(error instanceof Error ? error.name + ': ' + error.message : String(error));
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

  it('keeps start, seed, teardown, and blocker truth explicit on the current source surface', () => {
    const workflow = inspectLocalDevWorkflow();

    assert.equal(workflow.start.rootCommand, 'npm run dev:cloudflare');
    assert.equal(workflow.start.moduleCommand, 'npm --prefix infra/cloudflare run dev');
    assert.equal(workflow.start.wranglerCommand, 'wrangler dev --local');
    assert.equal(workflow.seed.aggregateCommand, 'npm --prefix infra/cloudflare run seed:local');
    assert.equal(workflow.teardown.status, 'explicit');
    assert.equal(workflow.start.status, 'runnable');
    assert.deepEqual(workflow.start.blockers, []);
    assert.equal(workflow.seed.status, 'runnable');
    assert.ok(workflow.seed.fixtureFamilies.every((family) => family.populationState !== 'blocked'));

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
            const { inspectLocalDevWorkflow } = await import(${JSON.stringify(localDevWorkflowModuleUrl)});
            const workflow = inspectLocalDevWorkflow();
            process.stdout.write(JSON.stringify(workflow));
          })().catch((error) => {
            console.error(error instanceof Error ? error.name + ': ' + error.message : String(error));
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
    assert.equal(workflow.start.status, 'runnable');
    assert.equal(workflow.seed.status, 'blocked');
    assert.ok(
      workflow.seed.fixtureFamilies.some(
        (family: { populationState: string; blocker?: { kind?: string; details?: string } }) =>
          family.populationState === 'blocked' &&
          family.blocker?.kind === 'missing-runtime-dependency' &&
          (family.blocker.details?.length ?? 0) > 0
      )
    );
  });
});

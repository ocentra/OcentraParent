import assert from 'node:assert/strict';
import { access, readFile } from 'node:fs/promises';
import path from 'node:path';
import test from 'node:test';

const repoRoot = path.resolve(import.meta.dirname, '../..', '..');

async function readRepoFile(relativePath) {
  return readFile(path.join(repoRoot, relativePath), 'utf8');
}

async function assertRepoFile(relativePath) {
  await access(path.join(repoRoot, relativePath));
}

test('WP08 routes catalog references without turning them into runtime proof', async () => {
  const workpack = await readRepoFile('docs/plans/network-plan/workpacks/08-control-catalog-reference-routing.md');
  const index = await readRepoFile('docs/plans/network-plan/WORKPACK_INDEX.md');
  const proofExpectations = await readRepoFile('docs/plans/network-plan/TEST_PROOF_EXPECTATIONS.md');

  assert.match(workpack, /WP08 owns reference routing and token control only\./);
  assert.match(workpack, /Network control catalog docs are source\/reference material, not implementation proof\./);
  assert.match(workpack, /Settings inventory rows are never copied wholesale into context/);
  assert.match(workpack, /network\.control-catalog\.route-note/);
  assert.match(workpack, /network\.control-catalog\.no-default-read/);
  assert.match(workpack, /network\.control-catalog\.claim-boundary/);
  assert.match(
    index,
    /\[08 Control Catalog Reference Routing\]\(workpacks\/08-control-catalog-reference-routing\.md\)/
  );
  assert.match(proofExpectations, /output\/network-plan-proof\/<workpack-file-stem>\//);

  await Promise.all(
    [
      'docs/plans/network-plan/workpacks/network-control-capability-guide.md',
      'docs/plans/network-plan/workpacks/network-control-schema-proposal.md',
      'docs/plans/network-plan/workpacks/network-control-settings-inventory.md',
    ].map(assertRepoFile)
  );
});

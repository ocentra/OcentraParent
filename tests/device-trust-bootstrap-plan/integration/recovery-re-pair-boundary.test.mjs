import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const planStatePath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/PLAN_STATE.md');
const workpackPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/workpacks/06-recovery-reset-re-pair.md');

test('recovery and re-pair remain separate from normal trust bootstrap', () => {
  const planState = readFileSync(planStatePath, 'utf8');
  const workpack = readFileSync(workpackPath, 'utf8');

  assert.match(planState, /Recovery must use an encrypted bundle/i);
  assert.match(workpack, /encrypted recovery bundle/i);
  assert.match(workpack, /wrong-household/i);
  assert.match(workpack, /wrong-key restores fail/i);
});

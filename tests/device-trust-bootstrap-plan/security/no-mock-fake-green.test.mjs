import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const expectationsPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/TEST_PROOF_EXPECTATIONS.md');
const planStatePath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/PLAN_STATE.md');

test('security proof keeps device trust away from surrogate-green evidence', () => {
  const expectations = readFileSync(expectationsPath, 'utf8');
  const planState = readFileSync(planStatePath, 'utf8');

  assert.match(expectations, /surrogate proof not product proof/i);
  assert.match(expectations, /wrong household\/device blocked/i);
  assert.match(planState, /No execution-grade device-trust state machine exists yet in repo code\./i);
  assert.match(planState, /No execution-grade local key sealing implementation exists yet in repo code\./i);
});

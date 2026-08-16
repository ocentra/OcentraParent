import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const planStatePath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/PLAN_STATE.md');
const workpackIndexPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/WORKPACK_INDEX.md');
const testExpectationsPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/TEST_PROOF_EXPECTATIONS.md');

test('device trust route gate keeps proof paths and test taxonomy aligned', () => {
  const planState = readFileSync(planStatePath, 'utf8');
  const workpackIndex = readFileSync(workpackIndexPath, 'utf8');
  const testExpectations = readFileSync(testExpectationsPath, 'utf8');

  assert.match(planState, /tests\/device-trust-bootstrap-plan\/<major-category>\//i);
  assert.match(workpackIndex, /output\/device-trust-bootstrap-plan-proof\/04-phone-qr-approval-bridge\//i);
  assert.match(workpackIndex, /output\/device-trust-bootstrap-plan-proof\/05-entitlement-device-license\//i);
  assert.match(testExpectations, /Current device-trust coverage starts in:/i);
  assert.match(testExpectations, /major categories/i);
});

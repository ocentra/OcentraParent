import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const modelPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/RECOVERY_RESET_MODEL.md');

test('recovery reset re-pair stays encrypted, household-bound, and revocation-preserving', () => {
  const model = readFileSync(modelPath, 'utf8');

  assert.match(model, /Recovery is not account login/i);
  assert.match(model, /encrypted/i);
  assert.match(model, /household-bound/i);
  assert.match(model, /parent-authorized/i);
  assert.match(model, /stale trust cannot be resurrected/i);
  assert.match(model, /wrong household/i);
});

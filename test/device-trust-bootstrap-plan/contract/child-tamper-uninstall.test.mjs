import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const modelPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/CHILD_TAMPER_UNINSTALL_MODEL.md');

test('child tamper uninstall stays parent-controlled and fails closed on revocation', () => {
  const model = readFileSync(modelPath, 'utf8');

  assert.match(model, /child device cannot be the authority/i);
  assert.match(model, /parent-controlled/i);
  assert.match(model, /fail closed/i);
  assert.match(model, /revoked device cannot keep using cached trust indefinitely/i);
  assert.match(model, /child device cannot revoke its own parent relationship/i);
});

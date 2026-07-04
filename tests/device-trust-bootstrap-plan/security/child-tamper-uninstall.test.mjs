import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const modelPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/CHILD_TAMPER_UNINSTALL_MODEL.md');

test('child tamper and uninstall stay parent-controlled and fail closed on revocation', () => {
  const model = readFileSync(modelPath, 'utf8');

  assert.match(model, /The child device cannot be the authority/i);
  assert.match(model, /fail closed or degrade to a safe state/i);
  assert.match(model, /No promise of perfect anti-root/i);
  assert.match(model, /A tampered package cannot silently become trusted/i);
  assert.match(model, /A revoked device cannot keep using cached trust indefinitely/i);
});

import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const modelPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/PARENT_STEP_UP_AUTH_MODEL.md');

test('parent step-up auth stays action-bound and rejects cached-login bypass', () => {
  const model = readFileSync(modelPath, 'utf8');

  assert.match(model, /high-risk action/i);
  assert.match(model, /passkey/i);
  assert.match(model, /OS-native biometric/i);
  assert.match(model, /A cached session cannot bypass step-up/i);
  assert.match(model, /A child device cannot satisfy parent step-up on its own/i);
});

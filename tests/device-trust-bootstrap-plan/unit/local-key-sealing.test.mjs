import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const modelPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/LOCAL_KEY_SEALING_MODEL.md');
const custodyPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/PLATFORM_KEY_CUSTODY_MATRIX.md');

test('local key sealing keeps trust material inside platform-backed custody', () => {
  const model = readFileSync(modelPath, 'utf8');
  const custody = readFileSync(custodyPath, 'utf8');

  assert.match(model, /platform-backed key stores first/i);
  assert.match(model, /app-managed plaintext secret/i);
  assert.match(model, /Wrong-user, wrong-device, or revoked-device attempts must fail\./i);
  assert.match(custody, /platform store is the security boundary/i);
  assert.match(custody, /encrypted recovery and re-pair/i);
});

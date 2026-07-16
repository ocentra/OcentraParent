import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const modelPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/PHONE_QR_APPROVAL_MODEL.md');

test('phone QR approval stays tied to the specific action and rejects replay', () => {
  const model = readFileSync(modelPath, 'utf8');

  assert.match(model, /QR challenge/i);
  assert.match(model, /single action/i);
  assert.match(model, /Short-lived/i);
  assert.match(model, /Replay-rejected/i);
  assert.match(model, /wrong household/i);
  assert.match(model, /target session/i);
});

import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const modelPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/ENTITLEMENT_DEVICE_LICENSE_MODEL.md');

test('entitlement device license remains signed, device-bound, and stale-cache resistant', () => {
  const model = readFileSync(modelPath, 'utf8');

  assert.match(model, /signed entitlement snapshot/i);
  assert.match(model, /trusted device/i);
  assert.match(model, /copied binary or copied config cannot replace a signed entitlement snapshot/i);
  assert.match(model, /revocation must override local cache/i);
  assert.match(model, /wrong device must not unlock the entitlement/i);
});

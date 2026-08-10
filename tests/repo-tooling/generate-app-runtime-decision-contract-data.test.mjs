import assert from 'node:assert/strict';
import test from 'node:test';

import { normalizeLineEndings } from '../../scripts/generate-app-runtime-decision-contract-data.mjs';

test('generated contract comparison ignores checkout line endings', () => {
  const generated = 'first line\nsecond line\n';
  const windowsCheckout = generated.replaceAll('\n', '\r\n');

  assert.equal(normalizeLineEndings(windowsCheckout), generated);
});

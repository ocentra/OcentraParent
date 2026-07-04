import test from 'node:test';
import assert from 'node:assert/strict';
import { readdirSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const base = resolve(repoRoot, 'tests', 'device-trust-bootstrap-plan');

test('device trust test taxonomy exposes the major category roots', () => {
  const categories = readdirSync(base, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort();

  assert.deepEqual(categories, ['contract', 'e2e', 'integration', 'security', 'unit']);
});

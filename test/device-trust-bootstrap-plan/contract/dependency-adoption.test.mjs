import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const repoRoot = resolve(import.meta.dirname, '..', '..', '..');
const matrixPath = resolve(repoRoot, 'docs/plans/device-trust-bootstrap-plan/DEPENDENCY_RESEARCH_AND_ADOPTION.md');
const workpackPath = resolve(
  repoRoot,
  'docs/plans/device-trust-bootstrap-plan/workpacks/08-open-source-dependency-adoption.md'
);

test('dependency adoption keeps trust roots explicit and candidate decisions visible', () => {
  const matrix = readFileSync(matrixPath, 'utf8');
  const workpack = readFileSync(workpackPath, 'utf8');

  assert.match(matrix, /webauthn-rs/i);
  assert.match(matrix, /passkey-rs/i);
  assert.match(matrix, /keyring-rs/i);
  assert.match(matrix, /rage/i);
  assert.match(matrix, /RustDesk/i);
  assert.match(matrix, /Adopt candidate/i);
  assert.match(matrix, /Research-only until a client boundary is chosen/i);
  assert.match(matrix, /Research-only reference/i);
  assert.match(workpack, /Every candidate is marked adopt, research-only, or reject\./i);
  assert.match(workpack, /No dependency is treated as a hidden trust root\./i);
});

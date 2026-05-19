import assert from 'node:assert/strict';
import { test } from 'node:test';
import { collectTestDoubleFindings, inspectTestDoubleText } from '../check-no-test-doubles.mjs';

test('test-double guard rejects common bypass APIs', () => {
  const source = [
    "vi.mock('@scope/module')",
    'const replacement = vi.fn()',
    'const sandbox = sinon.createSandbox()',
  ].join('\n');

  const findings = inspectTestDoubleText('packages/example/tests/bad.test.ts', source);

  assert.equal(findings.length >= 3, true);
});

test('test-double guard accepts real parser and service assertions', () => {
  const source = [
    "import { describe, expect, it } from 'vitest';",
    "import { ContractSchema } from '../src/contracts';",
    "it('decodes a real contract payload', () => {",
    '  expect(ContractSchema.safeParse({ schemaVersion: 1 }).success).toBe(true);',
    '});',
  ].join('\n');

  const findings = inspectTestDoubleText('packages/example/tests/contract.test.ts', source);

  assert.deepEqual(findings, []);
});

test('test-double guard scans the current source tree cleanly', () => {
  const { findings } = collectTestDoubleFindings();

  assert.deepEqual(findings, []);
});

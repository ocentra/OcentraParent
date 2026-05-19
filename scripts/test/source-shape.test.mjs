import assert from 'node:assert/strict';
import { test } from 'node:test';
import { inspectRustSource, inspectTypeScriptSource } from '../check-source-shape.mjs';

test('source shape guard rejects oversized TypeScript files', () => {
  const source = Array.from({ length: 241 }, () => 'export const value = 1;').join('\n');
  const { findings } = inspectTypeScriptSource('apps/portal/src/oversized.ts', source);

  assert.equal(
    findings.some((finding) => finding.reason.includes('file has 241 lines')),
    true
  );
});

test('source shape guard rejects oversized TypeScript functions', () => {
  const body = Array.from({ length: 81 }, () => '  const value = 1;').join('\n');
  const { findings } = inspectTypeScriptSource('apps/portal/src/function.ts', `export function bad() {\n${body}\n}`);

  assert.equal(
    findings.some((finding) => finding.reason.includes('function has')),
    true
  );
});

test('source shape guard rejects oversized Rust files', () => {
  const source = Array.from({ length: 221 }, () => 'pub fn tiny() {}').join('\n');
  const { findings } = inspectRustSource('crates/example/src/lib.rs', source);

  assert.equal(
    findings.some((finding) => finding.reason.includes('file has 221 lines')),
    true
  );
});

test('source shape guard warns before hard failure', () => {
  const source = Array.from({ length: 192 }, (_value, index) => `const value${index} = 1;`).join('\n');
  const { findings, warnings } = inspectTypeScriptSource('apps/portal/src/near-limit.ts', source);

  assert.equal(findings.length, 0);
  assert.equal(
    warnings.some((warning) => warning.reason.includes('warning starts at 192 of 240')),
    true
  );
});

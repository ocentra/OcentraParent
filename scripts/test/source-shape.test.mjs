import assert from 'node:assert/strict';
import { test } from 'node:test';
import { inspectRustSource, inspectTypeScriptSource } from '../check-source-shape.mjs';

test('source shape guard rejects oversized TypeScript files', () => {
  const source = Array.from({ length: 1001 }, () => 'const value = 1;').join('\n');
  const { findings } = inspectTypeScriptSource('apps/portal/src/oversized.ts', source);

  assert.equal(
    findings.some((finding) => finding.reason.includes('file has 1001 lines')),
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
  const source = Array.from({ length: 1001 }, () => 'const VALUE: u8 = 1;').join('\n');
  const { findings } = inspectRustSource('crates/example/src/lib.rs', source);

  assert.equal(
    findings.some((finding) => finding.reason.includes('file has 1001 lines')),
    true
  );
});

test('source shape guard warns on 250-line file bands before hard failure', () => {
  const source = Array.from({ length: 251 }, (_value, index) => `const value${index} = 1;`).join('\n');
  const { findings, warnings } = inspectTypeScriptSource('apps/portal/src/near-limit.ts', source);

  assert.equal(findings.length, 0);
  assert.equal(
    warnings.some((warning) => warning.reason.includes('crossed 250-line advisory band')),
    true
  );
});

test('source shape guard moves file warnings to the next 250-line band', () => {
  const source = Array.from({ length: 501 }, (_value, index) => `const value${index} = 1;`).join('\n');
  const { findings, warnings } = inspectTypeScriptSource('apps/portal/src/next-band.ts', source);

  assert.equal(findings.length, 0);
  assert.equal(
    warnings.some((warning) => warning.reason.includes('crossed 500-line advisory band')),
    true
  );
});

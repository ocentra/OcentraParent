import assert from 'node:assert/strict';
import test from 'node:test';

import { parseDiagnostics } from '../../scripts/dev/lib/agent-diagnostic-parsers.mjs';

function parse(command, stderr) {
  return parseDiagnostics({
    runId: 'run-parser-fixtures',
    commandId: 'cmd-parser-fixtures',
    command,
    stdout: '',
    stderr,
    stdoutArtifactPath: 'stdout.log',
    stderrArtifactPath: 'stderr.log',
    exitCode: 1,
  });
}

test('agent diagnostic parsers preserve structured evidence for each supported toolchain class', () => {
  const cases = [
    [['tsc', '--noEmit'], 'src/example.ts(4,8): error TS2322: incompatible value', 'typescript', 'TS2322'],
    [['cargo', 'check'], 'error[E0308]: mismatched types\n --> src/lib.rs:9:3', 'rustc', 'E0308'],
    [['cargo', 'clippy'], 'warning: needless borrow\n --> src/lib.rs:3:1', 'clippy', 'rust-needless borrow'],
    [
      ['cargo', 'test'],
      'test unit::rejects_invalid_input ... FAILED',
      'cargo-test',
      'cargo-test:unit::rejects_invalid_input',
    ],
    [
      ['eslint', 'src/example.ts'],
      'src/example.ts\n  4:8  error  unexpected any  @typescript-eslint/no-explicit-any',
      'eslint',
      '@typescript-eslint/no-explicit-any',
    ],
    [['npm', 'run', 'validate'], 'npm error code 1', 'npm-script', 'npm:code 1'],
    [
      ['node', 'scripts/check-architecture-policy.mjs'],
      'guard failed for src/example.ts',
      'architecture-policy',
      'architecture-policy',
    ],
    [
      ['node', 'scripts/check-no-reexports.mjs'],
      'no-reexport violation for src/example.ts',
      'no-reexport-policy',
      'no-reexport-policy',
    ],
  ];

  for (const [command, stderr, kind, signature] of cases) {
    const [diagnostic] = parse(command, stderr);
    assert.equal(diagnostic.kind, kind);
    assert.equal(diagnostic.signature, signature);
    assert.equal(diagnostic.severity, kind === 'clippy' ? 'warning' : 'error');
    assert.equal(diagnostic.runId, 'run-parser-fixtures');
    assert.equal(diagnostic.commandId, 'cmd-parser-fixtures');
    assert.equal(diagnostic.rawArtifact, 'stderr.log');
    assert.equal(diagnostic.rawStartLine >= 1, true);
    assert.equal(diagnostic.rawEndLine >= diagnostic.rawStartLine, true);
    assert.equal(diagnostic.hitCount, 1);
  }
});

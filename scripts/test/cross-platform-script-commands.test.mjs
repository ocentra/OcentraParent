import assert from 'node:assert/strict';
import { test } from 'node:test';
import { inspectCrossPlatformScriptCommands } from '../check-cross-platform-script-commands.mjs';

test('cross-platform script command guard rejects unguarded Windows npm shell invocations', () => {
  const source = "runCommand('cmd', ['/c', 'npm', 'run', 'build']);";

  const findings = inspectCrossPlatformScriptCommands('scripts/test/bad-proof.mjs', source);

  assert.equal(findings.length, 1);
  assert.match(findings[0].reason, /explicit process\.platform guard/u);
});

test('cross-platform script command guard accepts explicit Windows-only branches', () => {
  const source = [
    "if (process.platform === 'win32') {",
    "  runCommand('cmd', ['/c', 'npm', 'run', 'build']);",
    '  process.exit(0);',
    '}',
    "run('npm', ['run', 'build']);",
  ].join('\n');

  const findings = inspectCrossPlatformScriptCommands('scripts/test/windows-proof.mjs', source);

  assert.deepEqual(findings, []);
});

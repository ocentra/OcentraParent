import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import {
  buildCrateRustValidationCommands,
  buildWorkspaceRustValidationCommands,
} from '../../scripts/ci/rust-validation-commands.mjs';

const repoRoot = new URL('../../', import.meta.url);

function commandText(commands) {
  return commands.map(([command, args]) => [command, ...args].join(' ')).join('\n');
}

test('changed-crate Rust validation uses the native parallel test harness', () => {
  const commands = buildCrateRustValidationCommands('crates/agent-service');

  assert.deepEqual(commands, [
    ['cargo', ['check', '--manifest-path', 'crates/agent-service/Cargo.toml']],
    ['cargo', ['test', '--manifest-path', 'crates/agent-service/Cargo.toml']],
  ]);
  assert.doesNotMatch(commandText(commands), /test-threads/u);
});

test('workspace Rust validation keeps compilation and tests parallel by default', () => {
  const commands = buildWorkspaceRustValidationCommands();

  assert.deepEqual(commands, [
    ['npm', ['run', 'format:rust']],
    ['npm', ['run', 'lint:rust']],
    ['cargo', ['check', '--workspace']],
    ['cargo', ['test', '--workspace']],
  ]);
  assert.doesNotMatch(commandText(commands), /test-threads/u);

  const runner = readFileSync(new URL('scripts/ci/run-rust-validation.mjs', repoRoot), 'utf8');
  assert.doesNotMatch(runner, /CARGO_BUILD_JOBS|test-threads/u);
});

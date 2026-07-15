#!/usr/bin/env node

import { spawnSync } from 'node:child_process';

const environment =
  process.platform === 'win32'
    ? { ...process.env, CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS ?? '1' }
    : process.env;
const commands = [
  ['npm', ['run', 'format:rust']],
  ['npm', ['run', 'lint:rust']],
  ['cargo', ['check', '--workspace']],
  ['cargo', ['test', '--workspace', '--', '--test-threads=1']],
];

for (const [command, arguments_] of commands) {
  const usesWindowsNpm = process.platform === 'win32' && command === 'npm';
  const result = spawnSync(
    usesWindowsNpm ? (process.env.ComSpec ?? 'cmd.exe') : command,
    usesWindowsNpm ? ['/d', '/s', '/c', 'npm.cmd', ...arguments_] : arguments_,
    {
      cwd: process.cwd(),
      env: environment,
      stdio: 'inherit',
      shell: false,
    }
  );

  if (result.error) {
    throw result.error;
  }
  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}

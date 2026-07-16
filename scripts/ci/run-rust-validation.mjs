#!/usr/bin/env node

import { spawnSync } from 'node:child_process';

import { buildWorkspaceRustValidationCommands } from './rust-validation-commands.mjs';

const commands = buildWorkspaceRustValidationCommands();

for (const [command, arguments_] of commands) {
  const usesWindowsNpm = process.platform === 'win32' && command === 'npm';
  const result = spawnSync(
    usesWindowsNpm ? (process.env.ComSpec ?? 'cmd.exe') : command,
    usesWindowsNpm ? ['/d', '/s', '/c', 'npm.cmd', ...arguments_] : arguments_,
    {
      cwd: process.cwd(),
      env: process.env,
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

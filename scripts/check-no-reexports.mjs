#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { pathToFileURL } from 'node:url';

export function main(rawArgs = process.argv.slice(2)) {
  const result = spawnSync(
    process.execPath,
    ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'reexports', ...rawArgs],
    {
      cwd: process.cwd(),
      env: process.env,
      stdio: 'inherit',
      windowsHide: true,
    }
  );
  if (result.error) {
    throw result.error;
  }
  process.exit(result.status ?? 1);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}

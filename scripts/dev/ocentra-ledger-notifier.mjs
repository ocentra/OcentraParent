#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { join } from 'node:path';

const repoRoot = process.cwd();
const ledgerWrapper = process.env.OCENTRA_LEDGER_WRAPPER ?? join(repoRoot, 'scripts', 'dev', 'ocentra-ledger.mjs');

const result = spawnSync(process.execPath, [ledgerWrapper, 'notify', ...process.argv.slice(2)], {
  cwd: repoRoot,
  encoding: 'utf8',
  env: process.env,
  stdio: 'inherit',
  windowsHide: true,
});

process.exit(result.status ?? 1);

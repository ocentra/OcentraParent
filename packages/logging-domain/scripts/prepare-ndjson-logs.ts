#!/usr/bin/env node

import { clearDirectory, getTestLogScopeDir } from '../src/test-log/ndjsonPaths';
import { parseTestLogScopeOrDefault } from '../src/test-log/types';

const args = process.argv.slice(2);
const scopeArg = args.find((value) => value.startsWith('--scope='))?.slice('--scope='.length) ?? null;
const scope = parseTestLogScopeOrDefault(scopeArg);
const rootDir = process.env.OCENTRA_PARENT_LOG_DIR;
const targetDir = getTestLogScopeDir(scope, rootDir);

if (args.includes('--wipe')) {
  clearDirectory(targetDir);
}

process.stdout.write(`${getTestLogScopeDir(scope, rootDir)}\n`);

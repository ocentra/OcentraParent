#!/usr/bin/env node

import fs from 'node:fs';
import { getTestLogScopeDir, listNdjsonFiles } from '../src/test-log/ndjsonPaths';
import { parseTestLogScopeOrDefault } from '../src/test-log/types';

const scope = parseTestLogScopeOrDefault(
  process.argv
    .slice(2)
    .find((value) => value.startsWith('--scope='))
    ?.slice('--scope='.length) ?? null
);
const rootDir = process.env.OCENTRA_PARENT_LOG_DIR;
const files = listNdjsonFiles(getTestLogScopeDir(scope, rootDir));

if (files.length === 0) {
  process.stdout.write('[]\n');
  process.exit(0);
}

const filePath = files[0];
process.stdout.write(fs.readFileSync(filePath, 'utf8'));

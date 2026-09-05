#!/usr/bin/env node

import { readLocalArtifactText } from '../src/local-artifact-file';
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
const content = readLocalArtifactText(filePath, rootDir);
if (content == null) {
  throw new Error('selected NDJSON file disappeared before its owned snapshot was read');
}
process.stdout.write(content);

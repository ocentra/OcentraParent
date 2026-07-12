#!/usr/bin/env node

import { parseRunTypeOrDefault, parseSuiteTypeOrNull, parseTestLogScopeOrDefault } from '../src/test-log/types';
import { wipeNdjsonScope } from '../src/test-log/wipeNdjsonScope';

function getFlag(name: string): string | undefined {
  const prefix = `--${name}=`;
  return process.argv
    .slice(2)
    .find((arg) => arg.startsWith(prefix))
    ?.slice(prefix.length);
}

const result = wipeNdjsonScope({
  scope: parseTestLogScopeOrDefault(getFlag('scope')),
  runType: getFlag('run-type') != null ? parseRunTypeOrDefault(getFlag('run-type')) : null,
  suiteType: parseSuiteTypeOrNull(getFlag('suite')),
  runId: getFlag('run-id') ?? null,
  filePath: getFlag('file') ?? null,
  rootDir: getFlag('root-dir'),
});

process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);

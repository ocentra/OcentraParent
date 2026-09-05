#!/usr/bin/env node

import { pruneAppLogSessions } from '../src/app-log/appNdjsonWriter';
import { pruneTestLogRuns } from '../src/test-log/testLogRetention';
import { parseTestLogScopeOrDefault } from '../src/test-log/types';

function getFlag(name: string): string | undefined {
  const prefix = `--${name}=`;
  return process.argv
    .slice(2)
    .find((arg) => arg.startsWith(prefix))
    ?.slice(prefix.length);
}

function parseKeepNewest(value: string | undefined): number {
  const normalized = value?.trim() ?? '10';
  if (!/^\d+$/u.test(normalized)) {
    throw new Error('keep must be a non-negative integer');
  }
  const keepNewest = Number(normalized);
  if (!Number.isSafeInteger(keepNewest)) {
    throw new Error('keep must be a non-negative integer');
  }
  return keepNewest;
}

const scope = parseTestLogScopeOrDefault(getFlag('scope'));
const keepNewest = parseKeepNewest(getFlag('keep'));
const rootDir = getFlag('root-dir');

process.stdout.write(
  `${JSON.stringify(
    {
      scope,
      keepNewest,
      testRunsDeleted: pruneTestLogRuns(scope, keepNewest, rootDir),
      appSessionsDeleted: pruneAppLogSessions(scope, keepNewest, rootDir),
    },
    null,
    2
  )}\n`
);

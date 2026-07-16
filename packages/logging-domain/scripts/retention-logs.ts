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

const scope = parseTestLogScopeOrDefault(getFlag('scope'));
const keepNewest = Number.parseInt(getFlag('keep') ?? '10', 10);
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

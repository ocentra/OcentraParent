#!/usr/bin/env node

import { TestLogDuckDb } from '../src/test-log/testLogDuckDb';
import { parseTestLogScopeOrDefault } from '../src/test-log/types';

function parseFlag(name: string): string | null {
  const prefix = `--${name}=`;
  const match = process.argv.slice(2).find((value) => value.startsWith(prefix));
  return match == null ? null : match.slice(prefix.length);
}

async function main(): Promise<void> {
  const command = process.argv[2] ?? 'stats';
  const scope = parseTestLogScopeOrDefault(parseFlag('scope'));
  const rootDir = parseFlag('root-dir') ?? process.env.OCENTRA_PARENT_LOG_DIR;
  const db = await TestLogDuckDb.create(scope, rootDir);

  try {
    await db.ingestFromScope(scope, rootDir, false);

    if (command === 'stats') {
      process.stdout.write(`${JSON.stringify(await db.getStats(scope), null, 2)}\n`);
      return;
    }

    if (command === 'search') {
      const query = process.argv[3] ?? '';
      process.stdout.write(`${JSON.stringify(await db.search(scope, query), null, 2)}\n`);
      return;
    }

    if (command === 'failures' || command === 'latest-failures') {
      process.stdout.write(`${JSON.stringify(await db.latestFailures(scope), null, 2)}\n`);
      return;
    }

    process.stderr.write(`Unknown command: ${command}\n`);
    process.exitCode = 1;
  } finally {
    await db.close();
  }
}

void main();

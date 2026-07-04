#!/usr/bin/env node

import { TestLogDuckDb } from '../src/test-log/testLogDuckDb';
import { parseTestLogScopeOrDefault } from '../src/test-log/types';

async function main(): Promise<void> {
  const scope = parseTestLogScopeOrDefault(process.argv[2] ?? null);
  const db = await TestLogDuckDb.create(scope, process.env.OCENTRA_PARENT_LOG_DIR);
  try {
    process.stdout.write(`${db.dbFilePath()}\n`);
  } finally {
    await db.close();
  }
}

void main();

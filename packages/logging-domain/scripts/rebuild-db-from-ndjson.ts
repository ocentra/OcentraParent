#!/usr/bin/env node

import { TestLogDuckDb } from '../src/test-log/testLogDuckDb';
import { parseTestLogScopeOrDefault } from '../src/test-log/types';

async function main(): Promise<void> {
  const args = process.argv.slice(2);
  const rebuild = !args.includes('--no-delete');
  const scopeArg = args.find((value) => value.startsWith('--scope='))?.slice('--scope='.length) ?? null;
  const scope = parseTestLogScopeOrDefault(scopeArg);
  const db = await TestLogDuckDb.create(scope, process.env.OCENTRA_PARENT_LOG_DIR);

  try {
    const result = await db.ingestFromScope(scope, process.env.OCENTRA_PARENT_LOG_DIR, rebuild);
    process.stdout.write(`${JSON.stringify(result)}\n`);
  } finally {
    await db.close();
  }
}

void main();

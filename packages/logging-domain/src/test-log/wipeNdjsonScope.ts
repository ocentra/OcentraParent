import fs from 'node:fs';
import type { RunType, TestLogScope, TestSuiteType } from './types';
import { getTestLogScopeDir, listNdjsonFiles } from './ndjsonPaths';
import { readTestLogEntriesFromFile } from './ndjsonWriter';
import { matchesGeneratedWipeEntry } from '../local-test-log';

export interface WipeNdjsonScopeOptions {
  readonly scope: TestLogScope;
  readonly runType?: RunType | null;
  readonly suiteType?: TestSuiteType | null;
  readonly runId?: string | null;
  readonly filePath?: string | null;
  readonly rootDir?: string;
}

export interface WipeNdjsonScopeResult {
  readonly deletedEntries: number;
  readonly deletedFiles: string[];
  readonly rewrittenFiles: string[];
}

export function wipeNdjsonScope(options: WipeNdjsonScopeOptions): WipeNdjsonScopeResult {
  const scopeDir = getTestLogScopeDir(options.scope, options.rootDir);
  if (!fs.existsSync(scopeDir)) {
    return {
      deletedEntries: 0,
      deletedFiles: [],
      rewrittenFiles: [],
    };
  }

  const deletedFiles: string[] = [];
  const rewrittenFiles: string[] = [];
  let deletedEntries = 0;

  for (const filePath of listNdjsonFiles(scopeDir)) {
    const entries = readTestLogEntriesFromFile(filePath);
    const keptEntries = entries.filter((entry) => {
      const remove = matchesGeneratedWipeEntry(entry, options);
      if (remove) {
        deletedEntries += 1;
      }
      return !remove;
    });

    if (keptEntries.length === entries.length) {
      continue;
    }

    if (keptEntries.length === 0) {
      fs.rmSync(filePath, { force: true });
      deletedFiles.push(filePath);
      continue;
    }

    fs.writeFileSync(filePath, `${keptEntries.map((entry) => JSON.stringify(entry)).join('\n')}\n`, 'utf8');
    rewrittenFiles.push(filePath);
  }

  return {
    deletedEntries,
    deletedFiles: deletedFiles.sort((left, right) => left.localeCompare(right)),
    rewrittenFiles: rewrittenFiles.sort((left, right) => left.localeCompare(right)),
  };
}

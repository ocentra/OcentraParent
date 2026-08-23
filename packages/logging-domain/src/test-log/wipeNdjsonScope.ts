import type { RunType, TestLogScope, TestSuiteType } from './types';
import { getTestLogScopeDir, listNdjsonFiles } from './ndjsonPaths';
import { readTestLogEntriesFromFile } from './ndjsonWriter';
import { matchesGeneratedWipeEntry } from '../local-test-log';
import { durableRemoveLocalArtifact, durableReplaceLocalArtifact } from '../local-artifact-file';
import { invalidateTestLogDerivedArtifacts } from './testLogLifecycle';

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

interface PlannedWipeMutation {
  readonly filePath: string;
  readonly deletedEntries: number;
  readonly replacement: string | null;
}

export function wipeNdjsonScope(options: WipeNdjsonScopeOptions): WipeNdjsonScopeResult {
  const scopeDir = getTestLogScopeDir(options.scope, options.rootDir);
  const mutations: PlannedWipeMutation[] = [];

  for (const filePath of listNdjsonFiles(scopeDir)) {
    const entries = readTestLogEntriesFromFile(filePath);
    const keptEntries = entries.filter((entry) => !matchesGeneratedWipeEntry(entry, options));

    if (keptEntries.length === entries.length) {
      continue;
    }
    mutations.push({
      filePath,
      deletedEntries: entries.length - keptEntries.length,
      replacement:
        keptEntries.length === 0 ? null : `${keptEntries.map((entry) => JSON.stringify(entry)).join('\n')}\n`,
    });
  }

  invalidateTestLogDerivedArtifacts(options.scope, options.rootDir);
  if (mutations.length === 0) {
    return { deletedEntries: 0, deletedFiles: [], rewrittenFiles: [] };
  }
  const deletedFiles: string[] = [];
  const rewrittenFiles: string[] = [];
  for (const mutation of mutations) {
    if (mutation.replacement == null) {
      durableRemoveLocalArtifact(mutation.filePath);
      deletedFiles.push(mutation.filePath);
    } else {
      durableReplaceLocalArtifact(mutation.filePath, mutation.replacement);
      rewrittenFiles.push(mutation.filePath);
    }
  }

  return {
    deletedEntries: mutations.reduce((sum, mutation) => sum + mutation.deletedEntries, 0),
    deletedFiles: deletedFiles.sort((left, right) => left.localeCompare(right)),
    rewrittenFiles: rewrittenFiles.sort((left, right) => left.localeCompare(right)),
  };
}

/* generated from crates/logging-core/src/local_ndjson_log.rs */

import path from 'node:path';

import type { StoredTestLogLine } from './test-log/types';
import type { GeneratedPrunableFile, GeneratedWipeNdjsonScopeOptions } from './local-test-log';

function matchesGeneratedFile(entry: StoredTestLogLine, filePath: string): boolean {
  if (entry.filePath === filePath) {
    return true;
  }
  return entry.file === path.basename(filePath);
}

export function matchesGeneratedWipeEntry(entry: StoredTestLogLine, options: GeneratedWipeNdjsonScopeOptions): boolean {
  if (entry.scope !== options.scope) {
    return false;
  }
  if (options.runType != null && entry.runType !== options.runType) {
    return false;
  }
  if (options.suiteType != null && entry.suiteType !== options.suiteType) {
    return false;
  }
  if (options.runId != null && entry.runId !== options.runId) {
    return false;
  }
  if (options.filePath != null && !matchesGeneratedFile(entry, options.filePath)) {
    return false;
  }
  return true;
}

export function selectGeneratedPruneCandidates(files: readonly GeneratedPrunableFile[], keepNewest: number): string[] {
  const keepCount = Math.max(keepNewest, 0);
  return [...files]
    .sort((left, right) => {
      if (right.modifiedMs !== left.modifiedMs) {
        return right.modifiedMs - left.modifiedMs;
      }
      return left.filePath.localeCompare(right.filePath);
    })
    .slice(keepCount)
    .map((file) => file.filePath);
}

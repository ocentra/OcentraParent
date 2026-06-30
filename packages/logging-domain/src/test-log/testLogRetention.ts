import fs from 'node:fs';
import { getTestLogScopeDir, listNdjsonFiles } from './ndjsonPaths';
import type { TestLogScope } from '@ocentra-parent/schema-domain/test-log/types';
import { selectGeneratedPruneCandidates } from '../generated/local-test-log';

export function pruneTestLogRuns(scope: TestLogScope, keepNewest: number, rootDir?: string): number {
  const files = listNdjsonFiles(getTestLogScopeDir(scope, rootDir))
    .map((filePath) => ({
      filePath,
      modifiedMs: fs.statSync(filePath).mtimeMs,
    }))
    .sort((left, right) => right.modifiedMs - left.modifiedMs);

  const filesToDelete = selectGeneratedPruneCandidates(files, keepNewest);
  for (const file of filesToDelete) {
    fs.rmSync(file, { force: true });
  }

  return filesToDelete.length;
}

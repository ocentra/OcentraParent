import fs from 'node:fs';
import { getTestLogScopeDir, listNdjsonFiles } from './ndjsonPaths';
import type { TestLogScope } from './types';

export function pruneTestLogRuns(
  scope: TestLogScope,
  keepNewest: number,
  rootDir?: string
): number {
  const files = listNdjsonFiles(getTestLogScopeDir(scope, rootDir))
    .map((filePath) => ({
      filePath,
      modifiedMs: fs.statSync(filePath).mtimeMs,
    }))
    .sort((left, right) => right.modifiedMs - left.modifiedMs);

  const filesToDelete = files.slice(Math.max(keepNewest, 0));
  for (const file of filesToDelete) {
    fs.rmSync(file.filePath, { force: true });
  }

  return filesToDelete.length;
}

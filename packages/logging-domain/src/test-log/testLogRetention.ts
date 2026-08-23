import fs from 'node:fs';
import { getTestLogScopeDir, listNdjsonFiles } from './ndjsonPaths';
import type { TestLogScope } from './types';
import { selectGeneratedPruneCandidates } from '../local-test-log';
import { assertReadableLocalArtifactFile, durableRemoveLocalArtifact } from '../local-artifact-file';
import { invalidateTestLogDerivedArtifacts } from './testLogLifecycle';

export function pruneTestLogRuns(scope: TestLogScope, keepNewest: number, rootDir?: string): number {
  const files = listNdjsonFiles(getTestLogScopeDir(scope, rootDir))
    .map((filePath) => {
      assertReadableLocalArtifactFile(filePath);
      return {
        filePath,
        modifiedMs: fs.statSync(filePath).mtimeMs,
      };
    })
    .sort((left, right) => right.modifiedMs - left.modifiedMs);

  const filesToDelete = selectGeneratedPruneCandidates(files, keepNewest);
  if (filesToDelete.length > 0) {
    invalidateTestLogDerivedArtifacts(scope, rootDir);
  }
  for (const file of filesToDelete) {
    durableRemoveLocalArtifact(file);
  }

  return filesToDelete.length;
}

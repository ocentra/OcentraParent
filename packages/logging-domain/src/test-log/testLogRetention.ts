import { getDefaultLogRoot, getTestLogScopeDir, listNdjsonFiles } from './ndjsonPaths';
import type { TestLogScope } from './types';
import { selectGeneratedPruneCandidates } from '../local-test-log';
import { statLocalArtifact } from '../local-artifact-file';
import { withLocalArtifactLock } from '../local-artifact-lock';
import type { LocalArtifactMutation } from '../local-artifact-transaction';
import { readTestLogEntriesFromFile } from './ndjsonWriter';
import { applyTestLogMutation, replacementMapEntry } from './testLogMutation';

export function pruneTestLogRuns(scope: TestLogScope, keepNewest: number, rootDir?: string): number {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  return withLocalArtifactLock(resolvedRoot, () => {
    const files = listNdjsonFiles(getTestLogScopeDir(scope, resolvedRoot))
      .map((filePath) => {
        const stat = statLocalArtifact(filePath, resolvedRoot);
        if (stat == null) {
          throw new Error('test log disappeared during retention planning');
        }
        return { filePath, modifiedMs: stat.modifiedMs };
      })
      .sort((left, right) => right.modifiedMs - left.modifiedMs);
    const filesToDelete = selectGeneratedPruneCandidates(files, keepNewest);
    if (filesToDelete.length === 0) {
      return 0;
    }
    const affectedRunIds = new Set<string>();
    for (const filePath of filesToDelete) {
      for (const entry of readTestLogEntriesFromFile(filePath, resolvedRoot)) {
        affectedRunIds.add(entry.runId);
      }
    }
    const mutations: LocalArtifactMutation[] = filesToDelete.map((filePath) => ({ kind: 'remove', filePath }));
    const replacements = new Map(filesToDelete.map((filePath) => replacementMapEntry(filePath, null)));
    applyTestLogMutation(scope, resolvedRoot, mutations, replacements, affectedRunIds);
    return filesToDelete.length;
  });
}

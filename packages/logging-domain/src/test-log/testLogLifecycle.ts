import type { TestLogScope } from './types';
import { getDefaultLogRoot } from './ndjsonPaths';
import { statLocalArtifact } from '../local-artifact-file';
import { withLocalArtifactLock } from '../local-artifact-lock';
import { applyLocalArtifactTransaction } from '../local-artifact-transaction';
import { testLogDerivedArtifactMutations, testLogDerivedArtifactPaths } from './testLogMutation';

export interface TestLogDerivedArtifactInvalidation {
  readonly removedManifest: boolean;
  readonly removedDatabase: boolean;
  readonly removedDatabaseWal: boolean;
}

export function invalidateTestLogDerivedArtifacts(
  scope: TestLogScope,
  rootDir?: string
): TestLogDerivedArtifactInvalidation {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  return withLocalArtifactLock(resolvedRoot, () => {
    const artifacts = testLogDerivedArtifactPaths(scope, resolvedRoot);
    const result = {
      removedManifest: statLocalArtifact(artifacts.manifest, resolvedRoot) != null,
      removedDatabase: statLocalArtifact(artifacts.database, resolvedRoot) != null,
      removedDatabaseWal: statLocalArtifact(artifacts.databaseWal, resolvedRoot) != null,
    };
    applyLocalArtifactTransaction(resolvedRoot, testLogDerivedArtifactMutations(scope, resolvedRoot));
    return result;
  });
}

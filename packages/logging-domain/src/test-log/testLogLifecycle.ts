import path from 'node:path';
import type { TestLogScope } from './types';
import { getDefaultLogRoot } from './ndjsonPaths';
import { getGeneratedDefaultDuckDbFileName } from '../duckdb-log-query';
import { getGeneratedDbDir, getGeneratedManifestPath } from '../local-test-log';
import { durableRemoveLocalArtifact } from '../local-artifact-file';

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
  const databasePath = path.join(getGeneratedDbDir(resolvedRoot), getGeneratedDefaultDuckDbFileName(scope));
  const removedDatabaseWal = durableRemoveLocalArtifact(`${databasePath}.wal`);
  const removedDatabase = durableRemoveLocalArtifact(databasePath);
  const removedManifest = durableRemoveLocalArtifact(getGeneratedManifestPath(scope, resolvedRoot));
  return { removedManifest, removedDatabase, removedDatabaseWal };
}

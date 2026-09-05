import path from 'node:path';
import { getGeneratedDefaultDuckDbFileName } from '../duckdb-log-query';
import { getGeneratedDbDir, getGeneratedManifestPath } from '../local-test-log';
import { statLocalArtifact } from '../local-artifact-file';
import { applyLocalArtifactTransaction, type LocalArtifactMutation } from '../local-artifact-transaction';
import { bridgeLifecycleReconciliationMutation } from '../transport/bridgeLifecycleState';
import { getDefaultLogRoot, listNdjsonFiles } from './ndjsonPaths';
import { readTestLogEntriesFromFile } from './ndjsonWriter';
import type { StoredTestLogLine, TestLogScope } from './types';

export interface TestLogDerivedArtifactPaths {
  readonly manifest: string;
  readonly database: string;
  readonly databaseWal: string;
}

function normalizedFileKey(filePath: string): string {
  const resolved = path.resolve(filePath);
  return process.platform === 'win32' ? resolved.toLowerCase() : resolved;
}

export function testLogDerivedArtifactPaths(scope: TestLogScope, rootDir?: string): TestLogDerivedArtifactPaths {
  const resolvedRoot = rootDir ?? getDefaultLogRoot();
  const database = path.join(getGeneratedDbDir(resolvedRoot), getGeneratedDefaultDuckDbFileName(scope));
  return {
    manifest: getGeneratedManifestPath(scope, resolvedRoot),
    database,
    databaseWal: `${database}.wal`,
  };
}

export function testLogDerivedArtifactMutations(scope: TestLogScope, rootDir: string): LocalArtifactMutation[] {
  const artifacts = testLogDerivedArtifactPaths(scope, rootDir);
  return [artifacts.databaseWal, artifacts.database, artifacts.manifest]
    .filter((filePath) => statLocalArtifact(filePath, rootDir) != null)
    .map((filePath) => ({ kind: 'remove', filePath }));
}

function remainingRunCounts(
  rootDir: string,
  overrides: ReadonlyMap<string, readonly StoredTestLogLine[] | null>
): Map<string, number> {
  const counts = new Map<string, number>();
  const observed = new Set<string>();
  const logsRoot = path.join(rootDir, 'test-logs');
  for (const filePath of listNdjsonFiles(logsRoot)) {
    const key = normalizedFileKey(filePath);
    observed.add(key);
    const entries = overrides.has(key) ? overrides.get(key) : readTestLogEntriesFromFile(filePath);
    for (const entry of entries ?? []) {
      counts.set(entry.runId, (counts.get(entry.runId) ?? 0) + 1);
    }
  }
  for (const [key, entries] of overrides) {
    if (observed.has(key)) {
      continue;
    }
    for (const entry of entries ?? []) {
      counts.set(entry.runId, (counts.get(entry.runId) ?? 0) + 1);
    }
  }
  return counts;
}

export function countTestLogEntriesForRun(rootDir: string, runId: string): number {
  return remainingRunCounts(rootDir, new Map()).get(runId) ?? 0;
}

export function applyTestLogMutation(
  scope: TestLogScope,
  rootDir: string,
  logMutations: readonly LocalArtifactMutation[],
  replacements: ReadonlyMap<string, readonly StoredTestLogLine[] | null>,
  affectedRunIds: ReadonlySet<string>
): void {
  const counts = remainingRunCounts(rootDir, replacements);
  const lifecycle = bridgeLifecycleReconciliationMutation(rootDir, counts, affectedRunIds);
  applyLocalArtifactTransaction(rootDir, [
    ...testLogDerivedArtifactMutations(scope, rootDir),
    ...logMutations,
    ...(lifecycle == null ? [] : [lifecycle]),
  ]);
}

export function replacementMapEntry(
  filePath: string,
  entries: readonly StoredTestLogLine[] | null
): readonly [string, readonly StoredTestLogLine[] | null] {
  return [normalizedFileKey(filePath), entries];
}

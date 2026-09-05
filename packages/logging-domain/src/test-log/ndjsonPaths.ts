import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { TestLogScopeSchema, type RunType, type TestLogScope, type TestSuiteType } from './types';
import {
  GeneratedLocalLogRootEnv,
  getGeneratedAppLogScopeDir,
  getGeneratedAppSessionFilePath,
  getGeneratedDbDir,
  getGeneratedManifestPath,
  getGeneratedManifestDir,
  getGeneratedRunNdjsonFilePath,
  getGeneratedTestLogScopeDir,
  resolveGeneratedLocalLogRoot,
  sanitizeGeneratedTestNameForNdjson,
} from '../local-test-log';
import { recoverLocalArtifactAppends } from '../local-artifact-append';
import { applyLocalArtifactTransaction } from '../local-artifact-transaction';
import { ensureOwnedDirectory, relativeLocalArtifactPath, resolveLocalArtifactPath } from '../local-artifact-path';
import { providerList, providerStat } from '../local-artifact-mutation-provider';
import { inferLocalArtifactRoot } from '../local-artifact-root';
import { bridgeLifecycleClearCountersMutation } from '../transport/bridgeLifecycleState';
import { withLocalArtifactLock } from '../local-artifact-lock';
import { getGeneratedDefaultDuckDbFileName } from '../duckdb-log-query';

function clearedTestLogScope(rootDir: string, targetDir: string): TestLogScope | null {
  const segments = path.relative(rootDir, path.resolve(targetDir)).split(path.sep);
  const rootEntry = process.platform === 'win32' ? segments[0]?.toLowerCase() : segments[0];
  if (rootEntry !== 'test-logs') {
    return null;
  }
  if (segments.length < 2) {
    throw new Error('test log root clearing requires an exact owned scope');
  }
  const scope = process.platform === 'win32' ? segments[1]?.toLowerCase() : segments[1];
  return TestLogScopeSchema.parse(scope);
}

function clearedScopeDerivedMutations(rootDir: string, scope: TestLogScope) {
  const database = path.join(getGeneratedDbDir(rootDir), getGeneratedDefaultDuckDbFileName(scope));
  return [
    { kind: 'remove' as const, filePath: `${database}.wal` },
    { kind: 'remove' as const, filePath: database },
    { kind: 'remove' as const, filePath: getGeneratedManifestPath(scope, rootDir) },
  ];
}

function getPackageRoot(): string {
  const filename = fileURLToPath(import.meta.url);
  return path.resolve(path.dirname(filename), '..', '..');
}

function getWorkspaceRoot(): string {
  return path.resolve(getPackageRoot(), '..', '..');
}

export function sanitizeTestNameForNdjson(testName: string): string {
  return sanitizeGeneratedTestNameForNdjson(testName);
}

export function ensureDirectory(dirPath: string): string {
  return ensureOwnedDirectory(dirPath);
}

export function getDefaultLogRoot(): string {
  return resolveGeneratedLocalLogRoot(process.env[GeneratedLocalLogRootEnv], getWorkspaceRoot());
}

export function getTestLogScopeDir(scope: TestLogScope, rootDir?: string): string {
  return ensureDirectory(getGeneratedTestLogScopeDir(scope, rootDir ?? getDefaultLogRoot()));
}

export function getRunNdjsonFilePath(
  scope: TestLogScope,
  runType: RunType,
  runId: string,
  suiteType: TestSuiteType | null,
  rootDir?: string
): string {
  return getGeneratedRunNdjsonFilePath(scope, runType, runId, suiteType, rootDir ?? getDefaultLogRoot());
}

export function getAppLogScopeDir(scope: TestLogScope, rootDir?: string): string {
  return ensureDirectory(getGeneratedAppLogScopeDir(scope, rootDir ?? getDefaultLogRoot()));
}

export function getAppSessionFilePath(scope: TestLogScope, sessionId: string, rootDir?: string): string {
  return getGeneratedAppSessionFilePath(scope, sessionId, rootDir ?? getDefaultLogRoot());
}

export function getDbDir(rootDir?: string): string {
  return ensureDirectory(getGeneratedDbDir(rootDir ?? getDefaultLogRoot()));
}

export function getManifestDir(rootDir?: string): string {
  return ensureDirectory(getGeneratedManifestDir(rootDir ?? getDefaultLogRoot()));
}

export function listNdjsonFiles(rootPath: string): string[] {
  const resolvedRoot = resolveLocalArtifactPath(rootPath);
  const artifactRoot = inferLocalArtifactRoot(resolvedRoot);
  return withLocalArtifactLock(artifactRoot, () => {
    recoverLocalArtifactAppends(artifactRoot);
    const relativeRoot = relativeLocalArtifactPath(artifactRoot, resolvedRoot).split(path.sep).join('/');
    const stat = providerStat(artifactRoot, relativeRoot);
    if (stat == null) {
      return [];
    }
    if (!stat.is_directory) {
      throw new Error('NDJSON listing root is not an owned directory');
    }
    return listOwnedNdjsonFiles(artifactRoot, relativeRoot, resolvedRoot).sort((left, right) =>
      left.localeCompare(right)
    );
  });
}

function listOwnedNdjsonFiles(artifactRoot: string, relativeRoot: string, rootPath: string): string[] {
  const entries = providerList(artifactRoot, relativeRoot);
  const files: string[] = [];

  for (const entry of entries) {
    const fullPath = path.join(rootPath, entry.name);
    if (entry.is_directory) {
      const childRelative = path.posix.join(relativeRoot, entry.name);
      files.push(...listOwnedNdjsonFiles(artifactRoot, childRelative, fullPath));
      continue;
    }
    if (entry.name.endsWith('.ndjson')) {
      files.push(fullPath);
    }
  }
  return files;
}

export function clearDirectory(targetDir: string): void {
  const rootDir = inferLocalArtifactRoot(targetDir);
  withLocalArtifactLock(rootDir, () => {
    recoverLocalArtifactAppends(rootDir);
    const clearedScope = clearedTestLogScope(rootDir, targetDir);
    applyLocalArtifactTransaction(rootDir, [
      { kind: 'removeTree', filePath: targetDir },
      ...(clearedScope == null ? [] : clearedScopeDerivedMutations(rootDir, clearedScope)),
      ...(clearedScope == null ? [] : [bridgeLifecycleClearCountersMutation(rootDir)]),
    ]);
  });
}

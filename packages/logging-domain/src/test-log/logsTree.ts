import path from 'node:path';
import type { RunType, TestLogScope, TestSuiteType } from './types';
import { getDefaultLogRoot, getRunNdjsonFilePath } from './ndjsonPaths';
import { GeneratedLocalLogDirs, buildGeneratedLogsTreeKey, getGeneratedRunDirPath } from '../local-test-log';
import { sanitizeGeneratedPathSegment } from '../local-test-log-paths';
import {
  assertNotFileSystemRoot,
  ensureLocalArtifactRoot,
  relativeLocalArtifactPath,
  resolveLocalArtifactPath,
} from '../local-artifact-path';
import { withLocalArtifactLock } from '../local-artifact-lock';
import { recoverLocalArtifactAppends } from '../local-artifact-append';
import { providerList, providerStat } from '../local-artifact-mutation-provider';
import { scanLogsTree } from './logsTreeScan';

export type LogsTree = Map<string, string> & { readonly __brand: 'LogsTree' };

export interface LogsTreeScope {
  readonly scope: TestLogScope | string;
  readonly runType: RunType | string;
  readonly suiteType: TestSuiteType | string | null;
}

function normalizeRoot(rootDir?: string): string {
  const resolved = resolveLocalArtifactPath(rootDir ?? getDefaultLogRoot());
  assertNotFileSystemRoot(resolved);
  return ensureLocalArtifactRoot(resolved);
}

function suiteSegment(value: LogsTreeScope['suiteType']): string {
  return sanitizeGeneratedPathSegment(value ?? 'unspecified');
}

function validatedScope(scope: LogsTreeScope): LogsTreeScope {
  return {
    scope: sanitizeGeneratedPathSegment(String(scope.scope)),
    runType: sanitizeGeneratedPathSegment(String(scope.runType)),
    suiteType: scope.suiteType == null ? null : sanitizeGeneratedPathSegment(String(scope.suiteType)),
  };
}

function testLogRoot(rootDir?: string): string {
  return path.join(normalizeRoot(rootDir), GeneratedLocalLogDirs.TestLogs);
}

function relativePath(rootDir: string, targetPath: string): string {
  return relativeLocalArtifactPath(rootDir, targetPath).split(path.sep).join('/');
}

export function buildLogsTree(rootDir?: string): LogsTree {
  const normalizedRoot = normalizeRoot(rootDir);
  return withLocalArtifactLock(normalizedRoot, () => {
    recoverLocalArtifactAppends(normalizedRoot);
    const tree = scanLogsTree(normalizedRoot, testLogRoot(normalizedRoot));
    return tree as LogsTree;
  });
}

export function getLogsTree(rootDir?: string): LogsTree {
  return buildLogsTree(rootDir);
}

export function refreshLogsTree(rootDir?: string): void {
  buildLogsTree(rootDir);
}

export function getRunFilePath(scope: LogsTreeScope, fileKey: string, rootDir?: string): string {
  const validated = validatedScope(scope);
  return getRunNdjsonFilePath(
    validated.scope as TestLogScope,
    validated.runType as RunType,
    sanitizeGeneratedPathSegment(fileKey),
    validated.suiteType as TestSuiteType | null,
    rootDir
  );
}

export function getDirPath(scope: LogsTreeScope, fileKey: string, rootDir?: string): string {
  return getGeneratedRunDirPath(validatedScope(scope), sanitizeGeneratedPathSegment(fileKey), normalizeRoot(rootDir));
}

export function listFileKeysInScope(scope: LogsTreeScope, rootDir?: string): string[] {
  const validated = validatedScope(scope);
  const normalizedRoot = normalizeRoot(rootDir);
  const scopePath = path.join(
    testLogRoot(normalizedRoot),
    String(validated.scope),
    String(validated.runType),
    suiteSegment(validated.suiteType)
  );
  return withLocalArtifactLock(normalizedRoot, () => {
    recoverLocalArtifactAppends(normalizedRoot);
    const stat = providerStat(normalizedRoot, relativePath(normalizedRoot, scopePath));
    if (stat == null) return [];
    if (!stat.is_directory) throw new Error('test log scope is not an owned directory');
    return providerList(normalizedRoot, relativePath(normalizedRoot, scopePath))
      .filter((entry) => !entry.is_directory && entry.name.endsWith('.ndjson'))
      .map((entry) => sanitizeGeneratedPathSegment(entry.name.slice(0, -'.ndjson'.length)))
      .sort((left, right) => left.localeCompare(right));
  });
}

export function tryGet(tree: LogsTree, scope: LogsTreeScope, fileKey: string): string | undefined {
  const validated = validatedScope(scope);
  return tree.get(
    buildGeneratedLogsTreeKey(
      String(validated.scope),
      String(validated.runType),
      suiteSegment(validated.suiteType),
      sanitizeGeneratedPathSegment(fileKey)
    )
  );
}

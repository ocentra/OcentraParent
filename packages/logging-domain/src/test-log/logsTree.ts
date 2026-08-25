import fs from 'node:fs';
import path from 'node:path';
import type { RunType, TestLogScope, TestSuiteType } from './types';
import { getDefaultLogRoot, getRunNdjsonFilePath } from './ndjsonPaths';
import { GeneratedLocalLogDirs, buildGeneratedLogsTreeKey, getGeneratedRunDirPath } from '../local-test-log';
import { sanitizeGeneratedPathSegment } from '../local-test-log-paths';
import { assertExistingOwnedPath, assertNotFileSystemRoot, resolveLocalArtifactPath } from '../local-artifact-path';
import { withLocalArtifactLock } from '../local-artifact-lock';
import { recoverLocalArtifactAppends } from '../local-artifact-append';

export type LogsTree = Map<string, string> & { readonly __brand: 'LogsTree' };

export interface LogsTreeScope {
  readonly scope: TestLogScope | string;
  readonly runType: RunType | string;
  readonly suiteType: TestSuiteType | string | null;
}

function normalizeRoot(rootDir?: string): string {
  const resolved = resolveLocalArtifactPath(rootDir ?? getDefaultLogRoot());
  assertNotFileSystemRoot(resolved);
  if (fs.existsSync(resolved)) {
    assertExistingOwnedPath(resolved, 'directory');
  }
  return resolved;
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

function addSuiteFiles(
  tree: Map<string, string>,
  scopeName: string,
  runTypeName: string,
  suiteName: string,
  suitePath: string
): void {
  for (const fileEntry of fs.readdirSync(suitePath, { withFileTypes: true })) {
    if (!fileEntry.isFile() || !fileEntry.name.endsWith('.ndjson')) {
      continue;
    }
    const fileKey = fileEntry.name.slice(0, -'.ndjson'.length);
    sanitizeGeneratedPathSegment(fileKey);
    const filePath = path.join(suitePath, fileEntry.name);
    assertExistingOwnedPath(filePath, 'file');
    tree.set(buildGeneratedLogsTreeKey(scopeName, runTypeName, suiteName, fileKey), filePath);
  }
}

function addRunTypeEntries(
  tree: Map<string, string>,
  scopeName: string,
  runTypeName: string,
  runTypePath: string
): void {
  for (const suiteEntry of fs.readdirSync(runTypePath, { withFileTypes: true })) {
    if (!suiteEntry.isDirectory()) {
      continue;
    }
    const suiteName = sanitizeGeneratedPathSegment(suiteEntry.name);
    const suitePath = path.join(runTypePath, suiteName);
    assertExistingOwnedPath(suitePath, 'directory');
    addSuiteFiles(tree, scopeName, runTypeName, suiteName, suitePath);
  }
}

function addScopeEntries(tree: Map<string, string>, scopeName: string, scopePath: string): void {
  for (const runTypeEntry of fs.readdirSync(scopePath, { withFileTypes: true })) {
    if (!runTypeEntry.isDirectory()) {
      continue;
    }
    const runTypeName = sanitizeGeneratedPathSegment(runTypeEntry.name);
    const runTypePath = path.join(scopePath, runTypeName);
    assertExistingOwnedPath(runTypePath, 'directory');
    addRunTypeEntries(tree, scopeName, runTypeName, runTypePath);
  }
}

function walk(rootPath: string, tree: Map<string, string>): void {
  if (!fs.existsSync(rootPath)) {
    return;
  }
  assertExistingOwnedPath(rootPath, 'directory');

  for (const scopeEntry of fs.readdirSync(rootPath, { withFileTypes: true })) {
    if (!scopeEntry.isDirectory()) {
      continue;
    }
    const scopeName = sanitizeGeneratedPathSegment(scopeEntry.name);
    const scopePath = path.join(rootPath, scopeName);
    assertExistingOwnedPath(scopePath, 'directory');
    addScopeEntries(tree, scopeName, scopePath);
  }
}

export function buildLogsTree(rootDir?: string): LogsTree {
  const normalizedRoot = normalizeRoot(rootDir);
  return withLocalArtifactLock(normalizedRoot, () => {
    recoverLocalArtifactAppends(normalizedRoot);
    const tree = new Map<string, string>();
    walk(testLogRoot(normalizedRoot), tree);
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
    if (!fs.existsSync(scopePath)) {
      return [];
    }
    assertExistingOwnedPath(scopePath, 'directory');
    return fs
      .readdirSync(scopePath, { withFileTypes: true })
      .filter((entry) => entry.isFile() && entry.name.endsWith('.ndjson'))
      .map((entry) => {
        assertExistingOwnedPath(path.join(scopePath, entry.name), 'file');
        return sanitizeGeneratedPathSegment(entry.name.slice(0, -'.ndjson'.length));
      })
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

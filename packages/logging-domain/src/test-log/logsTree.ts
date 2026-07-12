import fs from 'node:fs';
import path from 'node:path';
import type { RunType, TestLogScope, TestSuiteType } from './types';
import { getDefaultLogRoot, getRunNdjsonFilePath } from './ndjsonPaths';
import { GeneratedLocalLogDirs, buildGeneratedLogsTreeKey, getGeneratedRunDirPath } from '../local-test-log';

export type LogsTree = Map<string, string> & { readonly __brand: 'LogsTree' };

export interface LogsTreeScope {
  readonly scope: TestLogScope | string;
  readonly runType: RunType | string;
  readonly suiteType: TestSuiteType | string | null;
}

let cachedRoot: string | null = null;
let cachedTree: LogsTree | null = null;

function normalizeRoot(rootDir?: string): string {
  return path.resolve(rootDir ?? getDefaultLogRoot());
}

function suiteSegment(value: LogsTreeScope['suiteType']): string {
  return value ?? 'unspecified';
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
    tree.set(
      buildGeneratedLogsTreeKey(scopeName, runTypeName, suiteName, fileKey),
      path.join(suitePath, fileEntry.name)
    );
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
    addSuiteFiles(tree, scopeName, runTypeName, suiteEntry.name, path.join(runTypePath, suiteEntry.name));
  }
}

function addScopeEntries(tree: Map<string, string>, scopeName: string, scopePath: string): void {
  for (const runTypeEntry of fs.readdirSync(scopePath, { withFileTypes: true })) {
    if (!runTypeEntry.isDirectory()) {
      continue;
    }
    addRunTypeEntries(tree, scopeName, runTypeEntry.name, path.join(scopePath, runTypeEntry.name));
  }
}

function walk(rootPath: string, tree: Map<string, string>): void {
  if (!fs.existsSync(rootPath)) {
    return;
  }

  for (const scopeEntry of fs.readdirSync(rootPath, { withFileTypes: true })) {
    if (!scopeEntry.isDirectory()) {
      continue;
    }
    addScopeEntries(tree, scopeEntry.name, path.join(rootPath, scopeEntry.name));
  }
}

export function buildLogsTree(rootDir?: string): LogsTree {
  const tree = new Map<string, string>();
  walk(testLogRoot(rootDir), tree);
  return tree as LogsTree;
}

export function getLogsTree(rootDir?: string): LogsTree {
  const normalizedRoot = normalizeRoot(rootDir);
  if (cachedTree == null || cachedRoot !== normalizedRoot) {
    cachedRoot = normalizedRoot;
    cachedTree = buildLogsTree(normalizedRoot);
  }
  return cachedTree;
}

export function refreshLogsTree(rootDir?: string): void {
  cachedRoot = normalizeRoot(rootDir);
  cachedTree = buildLogsTree(cachedRoot);
}

export function getRunFilePath(scope: LogsTreeScope, fileKey: string, rootDir?: string): string {
  return getRunNdjsonFilePath(
    scope.scope as TestLogScope,
    scope.runType as RunType,
    fileKey,
    scope.suiteType as TestSuiteType | null,
    rootDir
  );
}

export function getDirPath(scope: LogsTreeScope, fileKey: string, rootDir?: string): string {
  return getGeneratedRunDirPath(scope, fileKey, rootDir ?? getDefaultLogRoot());
}

export function listFileKeysInScope(scope: LogsTreeScope, rootDir?: string): string[] {
  const scopePath = path.join(
    testLogRoot(rootDir),
    String(scope.scope),
    String(scope.runType),
    suiteSegment(scope.suiteType)
  );
  if (!fs.existsSync(scopePath)) {
    return [];
  }

  return fs
    .readdirSync(scopePath, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith('.ndjson'))
    .map((entry) => entry.name.slice(0, -'.ndjson'.length))
    .sort((left, right) => left.localeCompare(right));
}

export function tryGet(tree: LogsTree, scope: LogsTreeScope, fileKey: string): string | undefined {
  return tree.get(
    buildGeneratedLogsTreeKey(String(scope.scope), String(scope.runType), suiteSegment(scope.suiteType), fileKey)
  );
}

export function asLogsTree(tree: Map<string, string>): LogsTree {
  return tree as LogsTree;
}

import fs from 'node:fs';
import path from 'node:path';
import { getDefaultLogRoot, getRunNdjsonFilePath } from './ndjsonPaths';
import type { RunType, TestLogScope, TestSuiteType } from './types';

const DELIMITER = '\0';
const TEST_LOG_DIR = 'test-logs';

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

function buildCompositeKey(scope: string, runType: string, suiteType: string, fileKey: string): string {
  return [scope, runType, suiteType, fileKey].join(DELIMITER);
}

function testLogRoot(rootDir?: string): string {
  return path.join(normalizeRoot(rootDir), TEST_LOG_DIR);
}

function walk(rootPath: string, tree: Map<string, string>): void {
  if (!fs.existsSync(rootPath)) {
    return;
  }

  for (const scopeEntry of fs.readdirSync(rootPath, { withFileTypes: true })) {
    if (!scopeEntry.isDirectory()) {
      continue;
    }
    const scopePath = path.join(rootPath, scopeEntry.name);
    for (const runTypeEntry of fs.readdirSync(scopePath, { withFileTypes: true })) {
      if (!runTypeEntry.isDirectory()) {
        continue;
      }
      const runTypePath = path.join(scopePath, runTypeEntry.name);
      for (const suiteEntry of fs.readdirSync(runTypePath, { withFileTypes: true })) {
        if (!suiteEntry.isDirectory()) {
          continue;
        }
        const suitePath = path.join(runTypePath, suiteEntry.name);
        for (const fileEntry of fs.readdirSync(suitePath, { withFileTypes: true })) {
          if (!fileEntry.isFile() || !fileEntry.name.endsWith('.ndjson')) {
            continue;
          }
          const fileKey = fileEntry.name.slice(0, -'.ndjson'.length);
          tree.set(
            buildCompositeKey(scopeEntry.name, runTypeEntry.name, suiteEntry.name, fileKey),
            path.join(suitePath, fileEntry.name)
          );
        }
      }
    }
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
  return path.dirname(getRunFilePath(scope, fileKey, rootDir));
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
    buildCompositeKey(
      String(scope.scope),
      String(scope.runType),
      suiteSegment(scope.suiteType),
      fileKey
    )
  );
}

export function asLogsTree(tree: Map<string, string>): LogsTree {
  return tree as LogsTree;
}

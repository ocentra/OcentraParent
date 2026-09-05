import path from 'node:path';
import { buildGeneratedLogsTreeKey } from '../local-test-log';
import { sanitizeGeneratedPathSegment } from '../local-test-log-paths';
import { providerList, providerStat } from '../local-artifact-mutation-provider';
import { relativeLocalArtifactPath } from '../local-artifact-path';

function relativePath(rootDir: string, targetPath: string): string {
  return relativeLocalArtifactPath(rootDir, targetPath).split(path.sep).join('/');
}

function addSuiteFiles(
  rootDir: string,
  tree: Map<string, string>,
  scopeName: string,
  runTypeName: string,
  suiteName: string,
  suitePath: string
): void {
  for (const fileEntry of providerList(rootDir, relativePath(rootDir, suitePath))) {
    if (fileEntry.is_directory || !fileEntry.name.endsWith('.ndjson')) continue;
    const fileKey = fileEntry.name.slice(0, -'.ndjson'.length);
    sanitizeGeneratedPathSegment(fileKey);
    tree.set(
      buildGeneratedLogsTreeKey(scopeName, runTypeName, suiteName, fileKey),
      path.join(suitePath, fileEntry.name)
    );
  }
}

function addRunTypeEntries(
  rootDir: string,
  tree: Map<string, string>,
  scopeName: string,
  runTypeName: string,
  runTypePath: string
): void {
  for (const suiteEntry of providerList(rootDir, relativePath(rootDir, runTypePath))) {
    if (!suiteEntry.is_directory) continue;
    const suiteName = sanitizeGeneratedPathSegment(suiteEntry.name);
    addSuiteFiles(rootDir, tree, scopeName, runTypeName, suiteName, path.join(runTypePath, suiteName));
  }
}

function addScopeEntries(rootDir: string, tree: Map<string, string>, scopeName: string, scopePath: string): void {
  for (const runTypeEntry of providerList(rootDir, relativePath(rootDir, scopePath))) {
    if (!runTypeEntry.is_directory) continue;
    const runTypeName = sanitizeGeneratedPathSegment(runTypeEntry.name);
    addRunTypeEntries(rootDir, tree, scopeName, runTypeName, path.join(scopePath, runTypeName));
  }
}

export function scanLogsTree(rootDir: string, rootPath: string): Map<string, string> {
  const stat = providerStat(rootDir, relativePath(rootDir, rootPath));
  if (stat == null) return new Map();
  if (!stat.is_directory) throw new Error('test log root is not an owned directory');
  const tree = new Map<string, string>();
  for (const scopeEntry of providerList(rootDir, relativePath(rootDir, rootPath))) {
    if (!scopeEntry.is_directory) continue;
    const scopeName = sanitizeGeneratedPathSegment(scopeEntry.name);
    addScopeEntries(rootDir, tree, scopeName, path.join(rootPath, scopeName));
  }
  return tree;
}

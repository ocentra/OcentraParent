/* generated from crates/logging-core/src/local_ndjson_log.rs */

import path from 'node:path';
import type { AppLogEntry, AppLogQuery } from '@ocentra-parent/schema-domain/app-log/types';
import type { RunType, StoredTestLogLine, TestLogScope, TestSuiteType } from '@ocentra-parent/schema-domain/test-log/types';

export const GeneratedLocalLogRootEnv = 'OCENTRA_PARENT_LOG_DIR';

export const GeneratedLocalLogDirs = {
  TestLogs: 'test-logs',
  AppLogs: 'app-logs',
  Db: 'db',
  Manifests: 'manifests',
} as const;

export interface GeneratedLogsTreeScope {
  readonly scope: TestLogScope | string;
  readonly runType: RunType | string;
  readonly suiteType: TestSuiteType | string | null;
}

export interface GeneratedWipeNdjsonScopeOptions {
  readonly scope: TestLogScope;
  readonly runType?: RunType | null;
  readonly suiteType?: TestSuiteType | null;
  readonly runId?: string | null;
  readonly filePath?: string | null;
}

export interface GeneratedPrunableFile {
  readonly filePath: string;
  readonly modifiedMs: number;
}

export interface GeneratedManifestEntry {
  readonly size: number;
  readonly modifiedMs: number;
  readonly sha256: string;
}

export interface GeneratedIngestManifest {
  readonly scope: TestLogScope;
  readonly updatedAt: number;
  readonly files: Record<string, GeneratedManifestEntry>;
}

export interface GeneratedObservedFileState {
  readonly resolvedPath: string;
  readonly size: number;
  readonly modifiedMs: number;
  readonly sha256: string;
}

function generatedIsAsciiAlphaNumeric(character: string): boolean {
  return /^[0-9A-Za-z]$/.test(character);
}

function generatedIsAsciiLowercaseAlphaNumeric(character: string): boolean {
  return /^[0-9a-z]$/.test(character);
}

function generatedTrimEdgeDashes(value: string): string {
  return value.replace(/^-+|-+$/g, '');
}

function generatedSanitizeWithCollapsedDashes(
  value: string,
  isAllowed: (character: string) => boolean
): string {
  let sanitized = '';
  for (const character of value) {
    if (isAllowed(character)) {
      sanitized += character;
      continue;
    }
    if (sanitized.length === 0 || sanitized.endsWith('-')) {
      continue;
    }
    sanitized += '-';
  }
  return generatedTrimEdgeDashes(sanitized);
}

function generatedSuiteSegment(value: GeneratedLogsTreeScope['suiteType'] | TestSuiteType | null | undefined): string {
  return value ?? 'unspecified';
}

export function sanitizeGeneratedPathSegment(value: string): string {
  const sanitized = generatedSanitizeWithCollapsedDashes(
    value,
    (character) => generatedIsAsciiAlphaNumeric(character) || character === '.' || character === '_' || character === '-'
  );
  return sanitized.length > 0 ? sanitized : 'default';
}

export function sanitizeGeneratedTestNameForNdjson(testName: string): string {
  const sanitized = generatedSanitizeWithCollapsedDashes(
    testName.toLowerCase(),
    generatedIsAsciiLowercaseAlphaNumeric
  ).slice(0, 100);
  return sanitized || 'unnamed-test';
}

export function resolveGeneratedLocalLogRoot(fromEnv: string | null | undefined, workspaceRoot: string): string {
  if (fromEnv != null && fromEnv.trim().length > 0) {
    return path.resolve(fromEnv);
  }
  return path.join(workspaceRoot, 'output', 'logging-domain');
}

export function getGeneratedTestLogScopeDir(scope: TestLogScope, rootDir: string): string {
  return path.join(rootDir, GeneratedLocalLogDirs.TestLogs, scope);
}

export function getGeneratedRunNdjsonFilePath(
  scope: TestLogScope,
  runType: RunType,
  runId: string,
  suiteType: TestSuiteType | null,
  rootDir: string
): string {
  return path.join(
    getGeneratedTestLogScopeDir(scope, rootDir),
    runType,
    generatedSuiteSegment(suiteType),
    `${sanitizeGeneratedPathSegment(runId)}.ndjson`
  );
}

export function getGeneratedAppLogScopeDir(scope: TestLogScope, rootDir: string): string {
  return path.join(rootDir, GeneratedLocalLogDirs.AppLogs, scope);
}

export function getGeneratedAppSessionFilePath(scope: TestLogScope, sessionId: string, rootDir: string): string {
  return path.join(getGeneratedAppLogScopeDir(scope, rootDir), `${sanitizeGeneratedPathSegment(sessionId)}.ndjson`);
}

export function getGeneratedDbDir(rootDir: string): string {
  return path.join(rootDir, GeneratedLocalLogDirs.Db);
}

export function getGeneratedManifestDir(rootDir: string): string {
  return path.join(rootDir, GeneratedLocalLogDirs.Manifests);
}

export function getGeneratedManifestPath(scope: TestLogScope, rootDir: string): string {
  return path.join(getGeneratedManifestDir(rootDir), `${scope}-ingest-manifest.json`);
}

export function buildGeneratedLogsTreeKey(
  scope: string,
  runType: string,
  suiteType: string | null | undefined,
  fileKey: string
): string {
  return [scope, runType, generatedSuiteSegment(suiteType), fileKey].join('\0');
}

export function getGeneratedRunDirPath(scope: GeneratedLogsTreeScope, fileKey: string, rootDir: string): string {
  return path.dirname(
    getGeneratedRunNdjsonFilePath(
      scope.scope as TestLogScope,
      scope.runType as RunType,
      fileKey,
      scope.suiteType as TestSuiteType | null,
      rootDir
    )
  );
}

export function groupGeneratedTestLogEntriesByFilePath(
  entries: readonly StoredTestLogLine[],
  rootDir: string
): Map<string, StoredTestLogLine[]> {
  const grouped = new Map<string, StoredTestLogLine[]>();
  for (const entry of entries) {
    const filePath = getGeneratedRunNdjsonFilePath(entry.scope, entry.runType, entry.runId, entry.suiteType, rootDir);
    const existing = grouped.get(filePath);
    if (existing == null) {
      grouped.set(filePath, [entry]);
      continue;
    }
    existing.push(entry);
  }
  return grouped;
}

export function splitGeneratedNdjsonContent(content: string): string[] {
  const trimmed = content.trim();
  if (trimmed.length === 0) {
    return [];
  }
  return trimmed.split(/\r?\n/).map((line) => line.trim()).filter((line) => line.length > 0);
}

function matchesGeneratedFile(entry: StoredTestLogLine, filePath: string): boolean {
  if (entry.filePath === filePath) {
    return true;
  }
  return entry.file === path.basename(filePath);
}

export function matchesGeneratedWipeEntry(
  entry: StoredTestLogLine,
  options: GeneratedWipeNdjsonScopeOptions
): boolean {
  if (entry.scope !== options.scope) {
    return false;
  }
  if (options.runType != null && entry.runType !== options.runType) {
    return false;
  }
  if (options.suiteType != null && entry.suiteType !== options.suiteType) {
    return false;
  }
  if (options.runId != null && entry.runId !== options.runId) {
    return false;
  }
  if (options.filePath != null && !matchesGeneratedFile(entry, options.filePath)) {
    return false;
  }
  return true;
}

export function selectGeneratedPruneCandidates(
  files: readonly GeneratedPrunableFile[],
  keepNewest: number
): string[] {
  const keepCount = Math.max(keepNewest, 0);
  return [...files]
    .sort((left, right) => {
      if (right.modifiedMs !== left.modifiedMs) {
        return right.modifiedMs - left.modifiedMs;
      }
      return left.filePath.localeCompare(right.filePath);
    })
    .slice(keepCount)
    .map((file) => file.filePath);
}

export function classifyGeneratedManifestChanges(
  manifest: GeneratedIngestManifest,
  observedFiles: readonly GeneratedObservedFileState[]
): { readonly newFiles: string[]; readonly changedFiles: string[] } {
  const newFiles: string[] = [];
  const changedFiles: string[] = [];

  for (const observed of observedFiles) {
    const existing = manifest.files[observed.resolvedPath];
    if (existing == null) {
      newFiles.push(observed.resolvedPath);
      continue;
    }
    if (
      existing.size !== observed.size ||
      existing.modifiedMs !== observed.modifiedMs ||
      existing.sha256 !== observed.sha256
    ) {
      changedFiles.push(observed.resolvedPath);
    }
  }

  return { newFiles, changedFiles };
}

export function buildGeneratedManifest(
  scope: TestLogScope,
  updatedAt: number,
  observedFiles: readonly GeneratedObservedFileState[]
): GeneratedIngestManifest {
  const files = Object.fromEntries(
    observedFiles.map((observed) => [
      observed.resolvedPath,
      {
        size: observed.size,
        modifiedMs: observed.modifiedMs,
        sha256: observed.sha256,
      } satisfies GeneratedManifestEntry,
    ])
  );

  return {
    scope,
    updatedAt,
    files,
  };
}

export function matchesGeneratedAppLogQuery(entry: AppLogEntry, query?: AppLogQuery): boolean {
  if (query?.level != null && entry.level !== query.level) {
    return false;
  }

  if (query?.search != null && query.search.trim().length > 0) {
    const search = query.search.toLowerCase();
    const haystack = `${entry.message} ${entry.context ?? ''} ${entry.data ?? ''}`.toLowerCase();
    if (!haystack.includes(search)) {
      return false;
    }
  }

  return true;
}

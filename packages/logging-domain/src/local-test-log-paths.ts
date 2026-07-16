/* generated from crates/logging-core/src/local_ndjson_log.rs */

import path from 'node:path';

import type { RunType, TestLogScope, TestSuiteType } from './test-log/types';

function generatedIsAsciiAlphaNumeric(character: string): boolean {
  return /^[0-9A-Za-z]$/.test(character);
}

function generatedIsAsciiLowercaseAlphaNumeric(character: string): boolean {
  return /^[0-9a-z]$/.test(character);
}

function generatedTrimEdgeDashes(value: string): string {
  let start = 0;
  let end = value.length;
  while (start < end && value.charCodeAt(start) === 45) {
    start += 1;
  }
  while (end > start && value.charCodeAt(end - 1) === 45) {
    end -= 1;
  }
  return value.slice(start, end);
}

function generatedSanitizeWithCollapsedDashes(value: string, isAllowed: (character: string) => boolean): string {
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

function generatedSuiteSegment(value: TestSuiteType | string | null | undefined): string {
  return value ?? 'unspecified';
}

export function sanitizeGeneratedPathSegment(value: string): string {
  const sanitized = generatedSanitizeWithCollapsedDashes(
    value,
    (character) =>
      generatedIsAsciiAlphaNumeric(character) || character === '.' || character === '_' || character === '-'
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
  return path.join(rootDir, 'test-logs', scope);
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
  return path.join(rootDir, 'app-logs', scope);
}

export function getGeneratedAppSessionFilePath(scope: TestLogScope, sessionId: string, rootDir: string): string {
  return path.join(getGeneratedAppLogScopeDir(scope, rootDir), `${sanitizeGeneratedPathSegment(sessionId)}.ndjson`);
}

export function getGeneratedDbDir(rootDir: string): string {
  return path.join(rootDir, 'db');
}

export function getGeneratedManifestDir(rootDir: string): string {
  return path.join(rootDir, 'manifests');
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

export function getGeneratedRunDirPath(
  scope: {
    readonly scope: TestLogScope | string;
    readonly runType: RunType | string;
    readonly suiteType: TestSuiteType | string | null;
  },
  fileKey: string,
  rootDir: string
): string {
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

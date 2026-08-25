/* generated from crates/logging-core/src/local_ndjson_log.rs */

import path from 'node:path';

import type { RunType, TestLogScope, TestSuiteType } from './test-log/types';

const MaximumGeneratedPathSegmentBytes = 256;
const GeneratedPathSegmentPattern = /^[0-9A-Za-z._-]+$/u;
const WindowsReservedSegmentPattern = /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/iu;

function assertGeneratedPathSegment(value: string, label: string): string {
  const tooLong = value.length > MaximumGeneratedPathSegmentBytes;
  const bytes = tooLong ? MaximumGeneratedPathSegmentBytes + 1 : new TextEncoder().encode(value).byteLength;
  if (
    value.length === 0 ||
    value.trim() !== value ||
    bytes > MaximumGeneratedPathSegmentBytes ||
    !GeneratedPathSegmentPattern.test(value) ||
    value === '.' ||
    value === '..' ||
    value.endsWith('.') ||
    WindowsReservedSegmentPattern.test(value)
  ) {
    throw new Error(`${label} must be a bounded literal path segment`);
  }
  return value;
}

function assertGeneratedTestName(value: string): string {
  if (
    value.length === 0 ||
    value.length > 4_096 ||
    value.trim() !== value ||
    /[\\/]/u.test(value) ||
    hasControlCharacter(value) ||
    path.isAbsolute(value) ||
    /^[A-Za-z]:/u.test(value)
  ) {
    throw new Error('test name must not contain path syntax');
  }
  return value;
}

function hasControlCharacter(value: string): boolean {
  return [...value].some((character) => character.charCodeAt(0) <= 0x1f);
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
  return assertGeneratedPathSegment(value ?? 'unspecified', 'suite type');
}

export function sanitizeGeneratedPathSegment(value: string): string {
  return assertGeneratedPathSegment(value, 'generated path value');
}

export function sanitizeGeneratedTestNameForNdjson(testName: string): string {
  const sanitized = generatedSanitizeWithCollapsedDashes(
    assertGeneratedTestName(testName).toLowerCase(),
    generatedIsAsciiLowercaseAlphaNumeric
  ).slice(0, 100);
  return assertGeneratedPathSegment(sanitized, 'generated test name');
}

export function resolveGeneratedLocalLogRoot(fromEnv: string | null | undefined, workspaceRoot: string): string {
  if (fromEnv != null && fromEnv.trim().length > 0) {
    return path.resolve(fromEnv);
  }
  return path.join(workspaceRoot, 'output', 'logging-domain');
}

export function getGeneratedTestLogScopeDir(scope: TestLogScope, rootDir: string): string {
  return path.join(rootDir, 'test-logs', assertGeneratedPathSegment(scope, 'test log scope'));
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
    assertGeneratedPathSegment(runType, 'run type'),
    generatedSuiteSegment(suiteType),
    `${sanitizeGeneratedPathSegment(runId)}.ndjson`
  );
}

export function getGeneratedAppLogScopeDir(scope: TestLogScope, rootDir: string): string {
  return path.join(rootDir, 'app-logs', assertGeneratedPathSegment(scope, 'app log scope'));
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
  return path.join(
    getGeneratedManifestDir(rootDir),
    `${assertGeneratedPathSegment(scope, 'manifest scope')}-ingest-manifest.json`
  );
}

export function buildGeneratedLogsTreeKey(
  scope: string,
  runType: string,
  suiteType: string | null | undefined,
  fileKey: string
): string {
  return [
    assertGeneratedPathSegment(scope, 'logs tree scope'),
    assertGeneratedPathSegment(runType, 'logs tree run type'),
    generatedSuiteSegment(suiteType),
    assertGeneratedPathSegment(fileKey, 'logs tree file key'),
  ].join('\0');
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

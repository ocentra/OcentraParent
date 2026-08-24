import type { FileKey, NdjsonSummaryContent, TestName } from './ndjsonBrands';
import type { LogsTreeScope } from './logsTree';
import { appendTestLogEntries } from './ndjsonWriter';
import { StoredTestLogLineSchema, type StoredTestLogLine } from './types';
import { splitGeneratedNdjsonContent } from '../local-test-log';
import { sanitizeGeneratedPathSegment, sanitizeGeneratedTestNameForNdjson } from '../local-test-log-paths';
import { assertBoundedText } from '../core/logTextCustody';

const MaximumLegacyNdjsonAppendBytes = 1024 * 1024;

function parseCustodiedEntries(content: string): StoredTestLogLine[] {
  assertBoundedText(content, 'legacy NDJSON append', MaximumLegacyNdjsonAppendBytes);
  const lines = splitGeneratedNdjsonContent(content);
  if (lines.length === 0) {
    throw new Error('legacy NDJSON append requires at least one complete log record');
  }
  return lines.map((line) => StoredTestLogLineSchema.parse(JSON.parse(line) as unknown));
}

function assertLegacyScope(entries: readonly StoredTestLogLine[], scope: LogsTreeScope, fileKey: FileKey): void {
  const expectedFileKey = sanitizeGeneratedPathSegment(fileKey);
  if (
    entries.some(
      (entry) =>
        entry.scope !== scope.scope ||
        entry.runType !== scope.runType ||
        entry.suiteType !== scope.suiteType ||
        entry.runId !== expectedFileKey
    )
  ) {
    throw new Error('legacy NDJSON append metadata does not match its declared scope');
  }
}

export function writeSummary(scope: LogsTreeScope, fileKey: FileKey, content: NdjsonSummaryContent): void {
  const entries = parseCustodiedEntries(content);
  assertLegacyScope(entries, scope, fileKey);
  appendTestLogEntries(entries);
}

export function writeLogEntry(scope: LogsTreeScope, fileKey: FileKey, testName: TestName, lines: string): void {
  const entries = parseCustodiedEntries(lines);
  assertLegacyScope(entries, scope, fileKey);
  sanitizeGeneratedTestNameForNdjson(testName);
  if (entries.some((entry) => entry.testName !== testName)) {
    throw new Error('legacy NDJSON append test name does not match its declared target');
  }
  appendTestLogEntries(entries);
}

import { BridgeEntrySchema, type BridgeEntry, type BridgeLogPayload } from '../transport/bridgeLogPayload';
import {
  assertBoundedText,
  MaximumMessageBytes,
  MaximumStackBytes,
  MaximumStructuredDataBytes,
  sanitizeLogIdentity,
  sanitizeLogPath,
  sanitizeLogTags,
  sanitizeLogText,
  sanitizeNullableLogText,
  utf8Bytes,
} from './logTextCustody';
import { serializeStructuredLogDataForCustody } from './structuredLogCustody';
import { AppLogEntrySchema, type AppLogEntry } from '../app-log/types';
import { StoredTestLogLineSchema, type StoredTestLogLine } from '../test-log/types';
import { ownedLogArray, ownedLogRecord } from './logRecordShapeCustody';

export const MaximumQueuedBridgeEntries = 512;
export const MaximumQueuedBridgeBytes = 4 * 1024 * 1024;
export const MaximumBridgeBatchEntries = 128;
export const MaximumBridgeBatchBytes = 768 * 1024;

const MaximumBridgeEntryBytes = 192 * 1024;
function parseBridgeEntryFromOwnedData(input: unknown): BridgeEntry {
  const entry = ownedLogRecord(input, 'bridge log entry');
  const log = ownedLogRecord(entry['log'], 'bridge log payload');
  const tags = ownedLogArray(log['tags'] ?? [], 32, 'bridge log tags');
  return BridgeEntrySchema.parse({ ...entry, log: { ...log, tags } });
}

function sanitizeSerializedData(value: string | null): string | null {
  if (value == null) {
    return null;
  }
  assertBoundedText(value, 'serialized log data', MaximumStructuredDataBytes);
  let parsed: unknown;
  try {
    parsed = JSON.parse(value) as unknown;
  } catch {
    throw new Error('serialized log data must be valid JSON');
  }
  return serializeStructuredLogDataForCustody(parsed);
}

function sanitizePayload(log: BridgeLogPayload): BridgeLogPayload {
  return {
    ...log,
    source: sanitizeNullableLogText(log.source, 'log source'),
    context: sanitizeNullableLogText(log.context, 'log context'),
    message: sanitizeLogText(log.message, 'log message', MaximumMessageBytes),
    data: sanitizeSerializedData(log.data),
    file: sanitizeLogPath(log.file, 'log file'),
    file_path: sanitizeLogPath(log.file_path, 'log file path'),
    correlation_id: log.correlation_id == null ? null : sanitizeLogIdentity(log.correlation_id, 'log correlation id'),
    tags: sanitizeLogTags(log.tags),
    stack: sanitizeNullableLogText(log.stack, 'log stack', MaximumStackBytes),
    environment: sanitizeNullableLogText(log.environment, 'log environment'),
  };
}

export function sanitizeStoredTestLogLineForCustody(input: unknown): StoredTestLogLine {
  const owned = ownedLogRecord(input, 'stored test log entry');
  const entry = StoredTestLogLineSchema.parse({ ...owned, tags: ownedLogArray(owned['tags'] ?? [], 32, 'log tags') });
  const payload = sanitizePayload({
    log_timestamp: entry.timestamp,
    level: entry.level,
    source: entry.source,
    context: entry.context,
    message: entry.message,
    data: entry.data,
    file: entry.file,
    file_path: entry.filePath,
    line: entry.line,
    column: entry.column,
    correlation_id: entry.correlationId,
    tags: entry.tags,
    stack: entry.stack,
    suite_type: entry.suiteType,
    origin: entry.origin,
    environment: entry.environment,
  });
  return StoredTestLogLineSchema.parse({
    ...entry,
    runId: sanitizeLogIdentity(entry.runId, 'stored log run id'),
    testName: sanitizeLogIdentity(entry.testName, 'stored log test name'),
    timestamp: payload.log_timestamp,
    level: payload.level,
    source: payload.source,
    context: payload.context,
    message: payload.message,
    data: payload.data,
    file: payload.file,
    filePath: payload.file_path,
    line: payload.line,
    column: payload.column,
    correlationId: payload.correlation_id,
    tags: payload.tags,
    stack: payload.stack,
    suiteType: payload.suite_type,
    origin: payload.origin,
    environment: payload.environment,
  });
}

export function sanitizeStoredTestLogBatchForCustody(input: unknown): readonly StoredTestLogLine[] {
  return ownedLogArray(input, MaximumBridgeBatchEntries, 'test log append batch').map(
    sanitizeStoredTestLogLineForCustody
  );
}

export function sanitizeAppLogEntryForCustody(input: unknown): AppLogEntry {
  const entry = AppLogEntrySchema.parse(ownedLogRecord(input, 'app log entry'));
  return AppLogEntrySchema.parse({
    ...entry,
    sessionId: sanitizeLogIdentity(entry.sessionId, 'app log session id'),
    source: sanitizeNullableLogText(entry.source, 'app log source'),
    context: sanitizeNullableLogText(entry.context, 'app log context'),
    message: sanitizeLogText(entry.message, 'app log message', MaximumMessageBytes),
    data: sanitizeSerializedData(entry.data),
    file: sanitizeLogPath(entry.file, 'app log file'),
    filePath: sanitizeLogPath(entry.filePath, 'app log file path'),
    correlationId:
      entry.correlationId == null ? null : sanitizeLogIdentity(entry.correlationId, 'app log correlation id'),
    environment: sanitizeNullableLogText(entry.environment, 'app log environment'),
  });
}

export function sanitizeAppLogBatchForCustody(input: unknown): readonly AppLogEntry[] {
  return ownedLogArray(input, MaximumBridgeBatchEntries, 'app log append batch').map(sanitizeAppLogEntryForCustody);
}

export function sanitizeBridgeEntryForCustody(input: unknown): BridgeEntry {
  const entry = parseBridgeEntryFromOwnedData(input);
  const sanitized = BridgeEntrySchema.parse({
    ...entry,
    testName: sanitizeLogIdentity(entry.testName, 'bridge test name'),
    runId: sanitizeLogIdentity(entry.runId, 'bridge run id'),
    log: sanitizePayload(entry.log),
  });
  if (bridgeEntryCustodyBytes(sanitized) > MaximumBridgeEntryBytes) {
    throw new Error('bridge log entry exceeds its custody limit');
  }
  return sanitized;
}

export function sanitizeBridgeBatchForCustody(input: unknown): readonly BridgeEntry[] {
  const entries = ownedLogArray(input, MaximumBridgeBatchEntries, 'bridge log batch').map(
    sanitizeBridgeEntryForCustody
  );
  if (utf8Bytes(JSON.stringify(entries)) > MaximumBridgeBatchBytes) {
    throw new Error('bridge log batch exceeds its custody limit');
  }
  return entries;
}

export function bridgeEntryCustodyBytes(entry: BridgeEntry): number {
  return utf8Bytes(JSON.stringify(entry));
}

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

export const MaximumQueuedBridgeEntries = 512;
export const MaximumQueuedBridgeBytes = 4 * 1024 * 1024;
export const MaximumBridgeBatchEntries = 128;
export const MaximumBridgeBatchBytes = 768 * 1024;

const MaximumBridgeEntryBytes = 192 * 1024;

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

export function sanitizeBridgeEntryForCustody(input: unknown): BridgeEntry {
  const entry = BridgeEntrySchema.parse(input);
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
  if (!Array.isArray(input) || input.length > MaximumBridgeBatchEntries) {
    throw new Error('bridge log batch exceeds its custody limit');
  }
  const entries = input.map(sanitizeBridgeEntryForCustody);
  if (utf8Bytes(JSON.stringify(entries)) > MaximumBridgeBatchBytes) {
    throw new Error('bridge log batch exceeds its custody limit');
  }
  return entries;
}

export function bridgeEntryCustodyBytes(entry: BridgeEntry): number {
  return utf8Bytes(JSON.stringify(entry));
}

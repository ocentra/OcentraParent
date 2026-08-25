import {
  bridgeEntryCustodyBytes,
  MaximumBridgeBatchEntries,
  MaximumQueuedBridgeBytes,
  MaximumQueuedBridgeEntries,
  sanitizeBridgeEntryForCustody,
} from './logCustody';
import type { PersistedBridgeQueue } from './bridgeLogQueueState';

const MaximumPersistedQueueBytes = MaximumQueuedBridgeBytes + 64 * 1024;

export function parsePersistedBridgeQueue(raw: string): PersistedBridgeQueue {
  if (new TextEncoder().encode(raw).byteLength > MaximumPersistedQueueBytes) {
    throw new Error('persisted log bridge queue exceeds its custody limit');
  }
  const input = persistedQueueRecord(JSON.parse(raw) as unknown);
  assertPersistedQueueHeader(input);
  const entries = (input['entries'] as unknown[]).map(sanitizeBridgeEntryForCustody);
  const ambiguousBatchSize = input['ambiguousBatchSize'] as number;
  assertPersistedQueueContents(input['status'], ambiguousBatchSize, entries);
  return {
    schemaVersion: 1,
    status: input['status'] as PersistedBridgeQueue['status'],
    ambiguousBatchSize,
    entries,
  };
}

function persistedQueueRecord(value: unknown): Record<string, unknown> {
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid persisted log bridge queue');
  }
  return value as Record<string, unknown>;
}

function assertPersistedQueueHeader(input: Record<string, unknown>): void {
  if (
    input['schemaVersion'] !== 1 ||
    (input['status'] !== 'ready' && input['status'] !== 'ambiguous') ||
    !Number.isSafeInteger(input['ambiguousBatchSize']) ||
    (input['ambiguousBatchSize'] as number) > MaximumBridgeBatchEntries ||
    !Array.isArray(input['entries']) ||
    input['entries'].length > MaximumQueuedBridgeEntries
  ) {
    throw new Error('invalid persisted log bridge queue');
  }
}

function assertPersistedQueueContents(
  status: unknown,
  ambiguousBatchSize: number,
  entries: PersistedBridgeQueue['entries']
): void {
  const validAmbiguity =
    status === 'ambiguous' ? ambiguousBatchSize > 0 && ambiguousBatchSize <= entries.length : ambiguousBatchSize === 0;
  const queuedBytes = entries.reduce((sum, entry) => sum + bridgeEntryCustodyBytes(entry), 0);
  if (!validAmbiguity || queuedBytes > MaximumQueuedBridgeBytes) {
    throw new Error('invalid persisted log bridge queue');
  }
}

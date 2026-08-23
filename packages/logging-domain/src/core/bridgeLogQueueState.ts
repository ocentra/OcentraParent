import type { BridgeEntry } from '../transport/bridgeLogPayload';
import {
  bridgeEntryCustodyBytes,
  MaximumBridgeBatchBytes,
  MaximumBridgeBatchEntries,
  MaximumQueuedBridgeBytes,
  MaximumQueuedBridgeEntries,
  sanitizeBridgeEntryForCustody,
} from './logCustody';

export interface PersistedBridgeQueue {
  readonly schemaVersion: 1;
  readonly status: 'ready' | 'ambiguous';
  readonly ambiguousBatchSize: number;
  readonly entries: readonly BridgeEntry[];
}

export type AmbiguousBridgeDeliveryResolution = 'assume-stored' | 'retry-accepting-duplicates';

const MaximumPersistedQueueBytes = MaximumQueuedBridgeBytes + 64 * 1024;

export function parsePersistedBridgeQueue(raw: string): PersistedBridgeQueue {
  if (new TextEncoder().encode(raw).byteLength > MaximumPersistedQueueBytes) {
    throw new Error('persisted log bridge queue exceeds its custody limit');
  }
  const value = JSON.parse(raw) as unknown;
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid persisted log bridge queue');
  }
  const input = value as Record<string, unknown>;
  if (
    input['schemaVersion'] !== 1 ||
    (input['status'] !== 'ready' && input['status'] !== 'ambiguous') ||
    !Number.isSafeInteger(input['ambiguousBatchSize']) ||
    !Array.isArray(input['entries']) ||
    input['entries'].length > MaximumQueuedBridgeEntries
  ) {
    throw new Error('invalid persisted log bridge queue');
  }
  const entries = input['entries'].map(sanitizeBridgeEntryForCustody);
  const ambiguousBatchSize = input['ambiguousBatchSize'] as number;
  const validAmbiguity =
    input['status'] === 'ambiguous'
      ? ambiguousBatchSize > 0 && ambiguousBatchSize <= entries.length
      : ambiguousBatchSize === 0;
  if (
    !validAmbiguity ||
    entries.reduce((sum, entry) => sum + bridgeEntryCustodyBytes(entry), 0) > MaximumQueuedBridgeBytes
  ) {
    throw new Error('invalid persisted log bridge queue');
  }
  return {
    schemaVersion: 1,
    status: input['status'] as PersistedBridgeQueue['status'],
    ambiguousBatchSize,
    entries,
  };
}

export class BridgeLogQueueState {
  private entries: BridgeEntry[] = [];
  private status: PersistedBridgeQueue['status'] = 'ready';
  private ambiguousBatchSize = 0;

  enqueuedState(rawEntry: BridgeEntry): PersistedBridgeQueue {
    const entry = sanitizeBridgeEntryForCustody(rawEntry);
    if (
      this.entries.length >= MaximumQueuedBridgeEntries ||
      this.queuedBytes() + bridgeEntryCustodyBytes(entry) > MaximumQueuedBridgeBytes
    ) {
      throw new Error('log bridge queue capacity exceeded; entry was rejected');
    }
    return {
      schemaVersion: 1,
      status: this.status,
      ambiguousBatchSize: this.ambiguousBatchSize,
      entries: [...this.entries, entry],
    };
  }

  persisted(): PersistedBridgeQueue {
    return {
      schemaVersion: 1,
      status: this.status,
      ambiguousBatchSize: this.ambiguousBatchSize,
      entries: [...this.entries],
    };
  }

  apply(state: PersistedBridgeQueue): void {
    this.entries = [...state.entries];
    this.status = state.status;
    this.ambiguousBatchSize = state.ambiguousBatchSize;
  }

  resetState(): PersistedBridgeQueue {
    return { schemaVersion: 1, status: 'ready', ambiguousBatchSize: 0, entries: [] };
  }

  ambiguousState(batchSize: number): PersistedBridgeQueue {
    return { schemaVersion: 1, status: 'ambiguous', ambiguousBatchSize: batchSize, entries: [...this.entries] };
  }

  successfulState(batchSize: number): PersistedBridgeQueue {
    return { schemaVersion: 1, status: 'ready', ambiguousBatchSize: 0, entries: this.entries.slice(batchSize) };
  }

  resolvedState(resolution: AmbiguousBridgeDeliveryResolution): PersistedBridgeQueue {
    if (this.status !== 'ambiguous' || this.ambiguousBatchSize <= 0) {
      throw new Error('log bridge delivery is not ambiguous');
    }
    if (resolution !== 'assume-stored' && resolution !== 'retry-accepting-duplicates') {
      throw new Error('invalid ambiguous log bridge delivery resolution');
    }
    const entries = resolution === 'assume-stored' ? this.entries.slice(this.ambiguousBatchSize) : [...this.entries];
    return { schemaVersion: 1, status: 'ready', ambiguousBatchSize: 0, entries };
  }

  deliveryBatch(): BridgeEntry[] {
    const batch: BridgeEntry[] = [];
    let bytes = 2;
    for (const entry of this.entries) {
      const nextBytes = bytes + bridgeEntryCustodyBytes(entry) + (batch.length === 0 ? 0 : 1);
      if (batch.length >= MaximumBridgeBatchEntries || nextBytes > MaximumBridgeBatchBytes) {
        break;
      }
      batch.push(entry);
      bytes = nextBytes;
    }
    if (batch.length === 0) {
      throw new Error('queued bridge entry cannot fit a delivery batch');
    }
    return batch;
  }

  queuedBytes(): number {
    return this.entries.reduce((sum, entry) => sum + bridgeEntryCustodyBytes(entry), 0);
  }

  queuedEntries(): number {
    return this.entries.length;
  }

  deliveryStatus(): PersistedBridgeQueue['status'] {
    return this.status;
  }

  ambiguousEntries(): number {
    return this.ambiguousBatchSize;
  }
}

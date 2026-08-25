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
    if (
      !Number.isSafeInteger(batchSize) ||
      batchSize <= 0 ||
      batchSize > MaximumBridgeBatchEntries ||
      batchSize > this.entries.length
    ) {
      throw new Error('invalid ambiguous log bridge delivery batch');
    }
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

import type { BridgeEntry } from '../transport/bridgeLogPayload';
import { sendToBridge } from '../transport/bridgeTransport';
import { BridgeLogQueuePersistence, type BridgeQueueStorage } from './bridgeLogQueuePersistence';
import {
  BridgeLogQueueState,
  type AmbiguousBridgeDeliveryResolution as QueueStateResolution,
  type PersistedBridgeQueue,
} from './bridgeLogQueueState';

interface BridgeLogQueueRuntime {
  readonly endpoint: string | null;
  readonly skipHealthCheck: boolean;
}

export interface BridgeQueueDeliveryState {
  readonly status: 'unconfigured' | 'ready' | 'ambiguous' | 'persistence-unavailable';
  readonly queuedEntries: number;
  readonly queuedBytes: number;
  readonly ambiguousBatchSize: number;
}

export type AmbiguousBridgeDeliveryResolution = QueueStateResolution;

export class BridgeLogQueue {
  private readonly state = new BridgeLogQueueState();
  private readonly persistence: BridgeLogQueuePersistence;
  private flushInFlight: Promise<void> | null = null;

  constructor(
    private readonly resolveRuntime: () => BridgeLogQueueRuntime,
    storage: BridgeQueueStorage
  ) {
    this.persistence = new BridgeLogQueuePersistence(storage);
    const persisted = this.persistence.restore();
    if (persisted != null) {
      this.state.apply(persisted);
    }
  }

  storageIs(storage: BridgeQueueStorage): boolean {
    return this.persistence.storageIs(storage);
  }

  enqueue(entry: BridgeEntry): void {
    this.commitState(this.state.enqueuedState(entry));
  }

  reset(): void {
    if (this.flushInFlight != null || this.state.deliveryStatus() === 'ambiguous' || this.state.queuedEntries() > 0) {
      throw new Error('log bridge queue cannot be reset while delivery ownership is pending');
    }
    this.persistence.clear();
    this.state.apply(this.state.resetState());
  }

  deliveryState(): BridgeQueueDeliveryState {
    return {
      status: this.visibleDeliveryStatus(),
      queuedEntries: this.state.queuedEntries(),
      queuedBytes: this.state.queuedBytes(),
      ambiguousBatchSize: this.state.ambiguousEntries(),
    };
  }

  resolveAmbiguousDelivery(resolution: AmbiguousBridgeDeliveryResolution): void {
    if (this.flushInFlight != null) {
      throw new Error('log bridge delivery is still in flight');
    }
    this.commitState(this.state.resolvedState(resolution));
  }

  async flush(): Promise<void> {
    if (this.flushInFlight != null) {
      return this.flushInFlight;
    }
    const flush = this.drain();
    this.flushInFlight = flush;
    try {
      await flush;
    } finally {
      if (this.flushInFlight === flush) {
        this.flushInFlight = null;
      }
    }
  }

  private async drain(): Promise<void> {
    this.assertReadyToDeliver();
    while (this.state.queuedEntries() > 0) {
      const runtime = this.resolveRuntime();
      if (runtime.endpoint == null || runtime.endpoint.length === 0) {
        throw new Error('log bridge endpoint is unavailable; queued entries were retained');
      }
      const entries = this.state.deliveryBatch();
      await sendToBridge(entries, runtime.endpoint, {
        skipHealthCheck: runtime.skipHealthCheck,
        onDeliveryAttempt: () => this.commitState(this.state.ambiguousState(entries.length)),
      });
      this.commitState(this.state.successfulState(entries.length));
    }
  }

  private assertReadyToDeliver(): void {
    if (this.state.deliveryStatus() === 'ambiguous') {
      throw new Error('log bridge delivery is ambiguous and requires owner resolution');
    }
    if (this.persistence.isBlocked()) {
      throw new Error('log bridge queue persistence is unavailable');
    }
    if (this.state.queuedEntries() > 0) {
      this.persistence.save(this.state.persisted());
    }
  }

  private commitState(next: PersistedBridgeQueue): void {
    this.persistence.save(next);
    this.state.apply(next);
  }

  private visibleDeliveryStatus(): BridgeQueueDeliveryState['status'] {
    if (this.state.deliveryStatus() === 'ambiguous') {
      return 'ambiguous';
    }
    return this.persistence.available() ? 'ready' : 'persistence-unavailable';
  }
}

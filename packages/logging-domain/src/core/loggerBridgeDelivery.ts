import type { BridgeEntry } from '../transport/bridgeLogPayload';
import { bridgeEndpoint as normalizeBridgeEndpoint } from '../transport/bridgeTransportHttp';
import {
  BridgeLogQueue,
  type AmbiguousBridgeDeliveryResolution,
  type BridgeQueueDeliveryState,
} from './bridgeLogQueue';
import type { BridgeQueueStorage } from './bridgeLogQueuePersistence';

interface LoggerBridgeRuntime {
  readonly endpoint: string | null;
  readonly skipHealthCheck: boolean;
}

export class LoggerBridgeDelivery {
  private queue: BridgeLogQueue | null = null;
  private endpoint: string | null = null;

  constructor(private readonly resolveRuntime: () => LoggerBridgeRuntime) {}

  configure(endpoint: string | null | undefined, storage: BridgeQueueStorage | null | undefined): void {
    if (endpoint == null) {
      this.disable();
      return;
    }
    const normalizedEndpoint = normalizeBridgeEndpoint(endpoint);
    const durableStorage = requireDurableStorage(storage);
    const state = this.queue?.deliveryState();
    if (deliveryOwnershipPending(state) && this.deliveryOwnerChanges(normalizedEndpoint, durableStorage)) {
      throw new Error('log bridge delivery ownership cannot change while entries are pending');
    }
    if (this.queue?.storageIs(durableStorage) === true) {
      this.endpoint = normalizedEndpoint;
      return;
    }
    const replacement = new BridgeLogQueue(this.resolveRuntime, durableStorage);
    this.queue?.reset();
    this.queue = replacement;
    this.endpoint = normalizedEndpoint;
  }

  private disable(): void {
    const state = this.queue?.deliveryState();
    if (deliveryOwnershipPending(state)) {
      throw new Error('log bridge delivery cannot be disabled while delivery ownership is pending');
    }
    this.queue?.reset();
    this.queue = null;
    this.endpoint = null;
  }

  private deliveryOwnerChanges(endpoint: string, storage: BridgeQueueStorage): boolean {
    return this.endpoint !== endpoint || this.queue?.storageIs(storage) !== true;
  }

  configured(): boolean {
    return this.queue != null;
  }

  enqueue(entry: BridgeEntry): void {
    if (this.queue == null) {
      throw new Error('log bridge delivery is not configured');
    }
    this.queue.enqueue(entry);
  }

  async flush(): Promise<void> {
    await this.queue?.flush();
  }

  reset(): void {
    this.queue?.reset();
    this.queue = null;
    this.endpoint = null;
  }

  deliveryState(): BridgeQueueDeliveryState {
    return (
      this.queue?.deliveryState() ?? {
        status: 'unconfigured',
        queuedEntries: 0,
        queuedBytes: 0,
        ambiguousBatchSize: 0,
      }
    );
  }

  resolveAmbiguousDelivery(resolution: AmbiguousBridgeDeliveryResolution): void {
    if (this.queue == null) {
      throw new Error('log bridge delivery is not configured');
    }
    this.queue.resolveAmbiguousDelivery(resolution);
  }
}

function requireDurableStorage(storage: BridgeQueueStorage | null | undefined): BridgeQueueStorage {
  if (storage == null || storage.durability !== 'persistent') {
    throw new Error('configured log bridge delivery requires environment-owned durable queue storage');
  }
  return storage;
}

function deliveryOwnershipPending(state: BridgeQueueDeliveryState | undefined): boolean {
  return state != null && (state.queuedEntries > 0 || state.status === 'ambiguous');
}

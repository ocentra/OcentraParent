import { MaximumQueuedBridgeBytes } from './logCustody';
import { parsePersistedBridgeQueue, type PersistedBridgeQueue } from './bridgeLogQueueState';
import { utf8Bytes } from './logTextCustody';

export interface BridgeQueueStorage {
  readonly durability: 'persistent';
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

const StorageKey = 'ocentra.logging.bridge-queue.v1';
const MaximumPersistedQueueBytes = MaximumQueuedBridgeBytes + 64 * 1024;

export class BridgeLogQueuePersistence {
  private blocked = false;

  constructor(private readonly storage: BridgeQueueStorage) {}

  restore(): PersistedBridgeQueue | null {
    try {
      const raw = this.storage.getItem(StorageKey);
      if (raw != null && (raw.length > MaximumPersistedQueueBytes || utf8Bytes(raw) > MaximumPersistedQueueBytes)) {
        throw new Error('persisted log bridge queue exceeds its custody limit');
      }
      return raw == null ? null : parsePersistedBridgeQueue(raw);
    } catch {
      this.blocked = true;
      throw new Error('durable log bridge queue state could not be restored');
    }
  }

  save(state: PersistedBridgeQueue): void {
    if (this.blocked) {
      throw new Error('log bridge queue persistence is unavailable');
    }
    const serialized = JSON.stringify(state);
    if (utf8Bytes(serialized) > MaximumPersistedQueueBytes) {
      throw new Error('persisted log bridge queue exceeds its custody limit');
    }
    try {
      this.storage.setItem(StorageKey, serialized);
      if (this.storage.getItem(StorageKey) !== serialized) {
        throw new Error('durable log bridge queue write could not be verified');
      }
    } catch (error) {
      this.blocked = true;
      throw error;
    }
  }

  clear(): void {
    try {
      this.storage.removeItem(StorageKey);
      if (this.storage.getItem(StorageKey) != null) {
        throw new Error('durable log bridge queue removal could not be verified');
      }
      this.blocked = false;
    } catch (error) {
      this.blocked = true;
      throw error;
    }
  }

  available(): boolean {
    return !this.blocked;
  }

  isBlocked(): boolean {
    return this.blocked;
  }

  storageIs(storage: BridgeQueueStorage): boolean {
    return this.storage === storage;
  }
}

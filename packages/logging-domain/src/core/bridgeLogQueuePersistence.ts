import { MaximumQueuedBridgeBytes } from './logCustody';
import { parsePersistedBridgeQueue, type PersistedBridgeQueue } from './bridgeLogQueueState';
import { utf8Bytes } from './logTextCustody';

interface QueueStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

const StorageKey = 'ocentra.logging.bridge-queue.v1';
const MaximumPersistedQueueBytes = MaximumQueuedBridgeBytes + 64 * 1024;

function queueStorage(): QueueStorage | null {
  try {
    const storage = (globalThis as { readonly localStorage?: QueueStorage }).localStorage;
    return storage == null ? null : storage;
  } catch {
    return null;
  }
}

export class BridgeLogQueuePersistence {
  private blocked = false;

  restore(): PersistedBridgeQueue | null {
    const storage = queueStorage();
    if (storage == null) {
      return null;
    }
    try {
      const raw = storage.getItem(StorageKey);
      return raw == null ? null : parsePersistedBridgeQueue(raw);
    } catch {
      this.blocked = true;
      return null;
    }
  }

  save(state: PersistedBridgeQueue): void {
    if (this.blocked) {
      throw new Error('log bridge queue persistence is unavailable');
    }
    const storage = queueStorage();
    if (storage == null) {
      throw new Error('durable log bridge queue storage is unavailable');
    }
    const serialized = JSON.stringify(state);
    if (utf8Bytes(serialized) > MaximumPersistedQueueBytes) {
      throw new Error('persisted log bridge queue exceeds its custody limit');
    }
    try {
      storage.setItem(StorageKey, serialized);
    } catch (error) {
      this.blocked = true;
      throw error;
    }
  }

  clear(): void {
    const storage = queueStorage();
    if (storage == null && this.blocked) {
      throw new Error('log bridge queue persistence cannot be cleared');
    }
    storage?.removeItem(StorageKey);
    this.blocked = false;
  }

  available(): boolean {
    return !this.blocked && queueStorage() != null;
  }

  isBlocked(): boolean {
    return this.blocked;
  }
}

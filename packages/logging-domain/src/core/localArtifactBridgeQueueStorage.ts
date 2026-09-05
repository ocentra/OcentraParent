import crypto from 'node:crypto';
import path from 'node:path';
import { durableRemoveLocalArtifact, durableReplaceLocalArtifact, readLocalArtifactText } from '../local-artifact-file';
import { ensureLocalArtifactRoot } from '../local-artifact-path';
import { MaximumQueuedBridgeBytes } from './logCustody';
import type { BridgeQueueStorage } from './bridgeLogQueuePersistence';

const MaximumStorageKeyBytes = 4 * 1024;
const MaximumPersistedQueueBytes = MaximumQueuedBridgeBytes + 64 * 1024;

function storagePath(rootDir: string, key: string): string {
  if (Buffer.byteLength(key, 'utf8') > MaximumStorageKeyBytes) {
    throw new Error('log bridge queue storage key exceeds its custody limit');
  }
  const keySha256 = crypto.createHash('sha256').update(key, 'utf8').digest('hex');
  return path.join(rootDir, '.bridge', 'queue-storage', `${keySha256}.json`);
}

function assertBoundedValue(value: string): void {
  if (Buffer.byteLength(value, 'utf8') > MaximumPersistedQueueBytes) {
    throw new Error('persisted log bridge queue exceeds its custody limit');
  }
}

export function createLocalArtifactBridgeQueueStorage(rootDir: string): BridgeQueueStorage {
  const root = ensureLocalArtifactRoot(rootDir);
  return {
    durability: 'persistent',
    getItem: (key) => readLocalArtifactText(storagePath(root, key), root, MaximumPersistedQueueBytes),
    setItem: (key, value) => {
      assertBoundedValue(value);
      durableReplaceLocalArtifact(storagePath(root, key), value, root);
    },
    removeItem: (key) => {
      durableRemoveLocalArtifact(storagePath(root, key), root);
    },
  };
}

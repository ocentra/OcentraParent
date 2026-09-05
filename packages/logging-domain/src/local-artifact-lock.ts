import path from 'node:path';
import { beginLocalArtifactProviderLease, endLocalArtifactProviderLease } from './local-artifact-mutation-provider';
import {
  currentLocalArtifactLease,
  localArtifactLeaseKey,
  runWithLocalArtifactLease,
} from './local-artifact-provider-lease-context';
import { ensureLocalArtifactRoot } from './local-artifact-path';

const processLeases = new Map<string, string>();

function isContextOwner(rootDir: string): boolean {
  const rootKey = localArtifactLeaseKey(rootDir);
  const leaseId = currentLocalArtifactLease(rootDir);
  return leaseId != null && processLeases.get(rootKey) === leaseId;
}

function acquireLease(rootDir: string): string {
  const rootKey = localArtifactLeaseKey(rootDir);
  if (processLeases.has(rootKey)) {
    throw new Error('local artifact root is busy in this process');
  }
  const leaseId = beginLocalArtifactProviderLease(rootDir);
  processLeases.set(rootKey, leaseId);
  return leaseId;
}

function releaseLease(rootDir: string, leaseId: string): void {
  const rootKey = localArtifactLeaseKey(rootDir);
  if (processLeases.get(rootKey) !== leaseId) {
    throw new Error('local artifact process lease ownership changed');
  }
  try {
    endLocalArtifactProviderLease(rootDir, leaseId);
  } finally {
    processLeases.delete(rootKey);
  }
}

export function withLocalArtifactLock<T>(rootDir: string, operation: () => T): T {
  const resolvedRoot = path.resolve(ensureLocalArtifactRoot(rootDir));
  if (isContextOwner(resolvedRoot)) return operation();
  const leaseId = acquireLease(resolvedRoot);
  try {
    return runWithLocalArtifactLease(resolvedRoot, leaseId, operation);
  } finally {
    releaseLease(resolvedRoot, leaseId);
  }
}

export async function withLocalArtifactLockAsync<T>(rootDir: string, operation: () => Promise<T>): Promise<T> {
  const resolvedRoot = path.resolve(ensureLocalArtifactRoot(rootDir));
  if (isContextOwner(resolvedRoot)) return operation();
  const leaseId = acquireLease(resolvedRoot);
  try {
    return await runWithLocalArtifactLease(resolvedRoot, leaseId, operation);
  } finally {
    releaseLease(resolvedRoot, leaseId);
  }
}

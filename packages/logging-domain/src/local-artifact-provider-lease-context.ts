import { AsyncLocalStorage } from 'node:async_hooks';
import path from 'node:path';

const leaseContext = new AsyncLocalStorage<ReadonlyMap<string, string>>();

export function localArtifactLeaseKey(rootDir: string): string {
  const resolvedRoot = path.resolve(rootDir);
  return process.platform === 'win32' ? resolvedRoot.toLowerCase() : resolvedRoot;
}

export function currentLocalArtifactLease(rootDir: string): string | null {
  return leaseContext.getStore()?.get(localArtifactLeaseKey(rootDir)) ?? null;
}

export function runWithLocalArtifactLease<T>(rootDir: string, leaseId: string, operation: () => T): T {
  const next = new Map(leaseContext.getStore() ?? []);
  next.set(localArtifactLeaseKey(rootDir), leaseId);
  return leaseContext.run(next, operation);
}

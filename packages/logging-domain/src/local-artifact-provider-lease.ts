import path from 'node:path';
import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { parseBooleanResult } from './local-artifact-provider-protocol-results';
import {
  currentProviderSession,
  discardProviderSession,
  providerSessionRoots,
  startProviderSession,
} from './local-artifact-provider-registry';
import { requestProvider } from './local-artifact-provider-request';

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value != null && !Array.isArray(value);
}

function hasOnlyKeys(value: Record<string, unknown>, keys: readonly string[]): boolean {
  const allowed = new Set(keys);
  return Object.keys(value).every((key) => allowed.has(key));
}

export function beginProviderLease(rootDir: string): string {
  const root = path.resolve(rootDir);
  const session = startProviderSession(root);
  if (session.leaseId != null) return session.leaseId;
  const result = requestProvider(root, { kind: 'beginLease' });
  if (!isRecord(result) || !hasOnlyKeys(result, ['lease_id'])) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider lease result is invalid');
  }
  const leaseId = result['lease_id'];
  if (typeof leaseId !== 'string' || !/^[0-9a-f]{32,128}$/u.test(leaseId)) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider lease identity is invalid');
  }
  const current = currentProviderSession(root);
  if (current == null || current.closed) {
    throw new LocalArtifactProviderError(
      'provider-unavailable',
      'provider lease session ended before ownership was retained'
    );
  }
  current.leaseId = leaseId;
  return leaseId;
}

export function endProviderLease(rootDir: string, leaseId: string): void {
  const root = path.resolve(rootDir);
  const session = currentProviderSession(root);
  if (session == null || session.leaseId !== leaseId) {
    throw new LocalArtifactProviderError('provider-authority', 'provider lease is not current');
  }
  const released = parseBooleanResult(
    requestProvider(root, { kind: 'endLease', lease_id: leaseId }),
    'released',
    'endLease'
  );
  if (!released) {
    throw new LocalArtifactProviderError('provider-authority', 'provider lease was not released');
  }
  session.leaseId = null;
}

async function closeProviderRoot(root: string): Promise<void> {
  const session = currentProviderSession(root);
  if (session == null) return;
  if (!session.closed) {
    try {
      parseBooleanResult(requestProvider(root, { kind: 'shutdown' }), 'shutdown', 'shutdown');
    } catch {
      // The completion barrier below still forces and awaits teardown.
    }
  }
  await discardProviderSession(root, session);
}

export async function closeProvider(rootDir?: string): Promise<void> {
  const roots = rootDir == null ? providerSessionRoots() : [path.resolve(rootDir)];
  const leasedRoot = roots.find((root) => currentProviderSession(root)?.leaseId != null);
  if (leasedRoot != null) {
    throw new LocalArtifactProviderError(
      'provider-authority',
      'provider cannot close while its mutation lease is active'
    );
  }
  const results = await Promise.allSettled(roots.map(closeProviderRoot));
  const failure = results.find((result): result is PromiseRejectedResult => result.status === 'rejected');
  if (failure != null) throw failure.reason;
}

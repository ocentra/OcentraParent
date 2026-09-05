import path from 'node:path';
import { providerPackageRoot, requireProviderBinary } from './local-artifact-provider-binary';
import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { type LocalArtifactProviderIdentity } from './local-artifact-provider-protocol';
import {
  closeLocalArtifactProviderSession,
  shutdownLocalArtifactProviderSession,
  startLocalArtifactProviderSession,
  type LocalArtifactProviderSession,
} from './local-artifact-provider-session';

const sessions = new Map<string, LocalArtifactProviderSession>();
const expectedRootIdentities = new Map<string, LocalArtifactProviderIdentity>();

export function providerSessionKey(rootDir: string): string {
  const resolvedRoot = path.resolve(rootDir);
  return process.platform === 'win32' ? resolvedRoot.toLowerCase() : resolvedRoot;
}

function sameIdentity(left: LocalArtifactProviderIdentity, right: LocalArtifactProviderIdentity): boolean {
  return left.device === right.device && left.inode === right.inode;
}

function rejectUntilSessionDisposalCompletes(root: string, session: LocalArtifactProviderSession): never {
  void discardProviderSession(root, session).catch(() => undefined);
  throw new LocalArtifactProviderError('provider-unavailable', 'provider session disposal is still in progress');
}

export function startProviderSession(rootDir: string): LocalArtifactProviderSession {
  const root = path.resolve(rootDir);
  const sessionKey = providerSessionKey(root);
  const existing = sessions.get(sessionKey);
  if (existing != null) {
    if (!existing.closed && existing.process.exitCode == null) return existing;
    rejectUntilSessionDisposalCompletes(root, existing);
  }
  const binary = requireProviderBinary();
  const session = startLocalArtifactProviderSession(root, binary.path, providerPackageRoot());
  sessions.set(sessionKey, session);
  if (session.binarySha256 !== binary.sha256) {
    void discardProviderSession(root, session).catch(() => undefined);
    throw new LocalArtifactProviderError(
      'provider-authority',
      'provider process identity does not match the pinned package binary'
    );
  }
  const expectedRootIdentity = expectedRootIdentities.get(sessionKey);
  if (expectedRootIdentity != null && !sameIdentity(expectedRootIdentity, session.rootIdentity)) {
    try {
      shutdownLocalArtifactProviderSession(session);
    } catch {
      // Forced termination below remains fail-closed when graceful shutdown fails.
    }
    void discardProviderSession(root, session).catch(() => undefined);
    throw new LocalArtifactProviderError(
      'root-identity-changed',
      'provider root identity changed across process restart'
    );
  }
  expectedRootIdentities.set(sessionKey, session.rootIdentity);
  return session;
}

export async function discardProviderSession(rootDir: string, session: LocalArtifactProviderSession): Promise<void> {
  await closeLocalArtifactProviderSession(session);
  const sessionKey = providerSessionKey(rootDir);
  if (sessions.get(sessionKey) === session) sessions.delete(sessionKey);
}

export function currentProviderSession(rootDir: string): LocalArtifactProviderSession | undefined {
  return sessions.get(providerSessionKey(rootDir));
}

export function providerSessionRoots(): readonly string[] {
  return [...sessions.keys()];
}

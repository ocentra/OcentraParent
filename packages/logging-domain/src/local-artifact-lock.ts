import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import { AsyncLocalStorage } from 'node:async_hooks';
import {
  lockPaths,
  processIsRunning,
  readOwner,
  removeCandidate,
  removeLockDirectory,
  writeOwner,
} from './local-artifact-lock-owner';
import {
  assertLocalArtifactIdentity,
  ensureLocalArtifactRoot,
  localArtifactIdentity,
  syncOwnedDirectory,
} from './local-artifact-path';

interface ArtifactLockHandle {
  readonly rootDir: string;
  readonly lockDir: string;
  readonly token: string;
}

const lockContext = new AsyncLocalStorage<ReadonlyMap<string, string>>();
const processLocks = new Map<string, string>();

function recoverDeadLock(bridgeDir: string, lockDir: string): boolean {
  if (!fs.existsSync(lockDir)) {
    return true;
  }
  const owner = readOwner(lockDir);
  if (processIsRunning(owner.pid)) {
    return false;
  }
  const staleDir = path.join(bridgeDir, `artifact.lock.stale.${crypto.randomUUID()}`);
  fs.renameSync(lockDir, staleDir);
  removeLockDirectory(staleDir, owner.token);
  return true;
}

function acquireArtifactLock(rootDir: string): ArtifactLockHandle {
  const resolvedRoot = ensureLocalArtifactRoot(rootDir);
  const { bridgeDir, lockDir } = lockPaths(resolvedRoot);
  for (let attempt = 0; attempt < 3; attempt += 1) {
    const token = crypto.randomUUID();
    const candidateDir = path.join(bridgeDir, `artifact.lock.candidate.${token}`);
    writeOwner(candidateDir, { schemaVersion: 1, pid: process.pid, token, createdAt: Date.now() });
    try {
      fs.renameSync(candidateDir, lockDir);
      syncOwnedDirectory(bridgeDir);
      processLocks.set(resolvedRoot, token);
      return { rootDir: resolvedRoot, lockDir, token };
    } catch (error) {
      removeCandidate(candidateDir);
      const code = (error as NodeJS.ErrnoException).code;
      if (!fs.existsSync(lockDir) || (code !== 'EEXIST' && code !== 'EPERM' && code !== 'ENOTEMPTY')) {
        throw error;
      }
      if (!recoverDeadLock(bridgeDir, lockDir)) {
        throw new Error('local artifact root is locked by another process');
      }
    }
  }
  throw new Error('local artifact lock could not be acquired');
}

function releaseArtifactLock(handle: ArtifactLockHandle): void {
  const processToken = processLocks.get(handle.rootDir);
  if (processToken !== handle.token) {
    throw new Error('local artifact process lock ownership changed');
  }
  const lockIdentity = localArtifactIdentity(handle.lockDir, 'directory');
  const releasedDir = path.join(path.dirname(handle.lockDir), `artifact.lock.released.${crypto.randomUUID()}`);
  fs.renameSync(handle.lockDir, releasedDir);
  assertLocalArtifactIdentity(releasedDir, 'directory', lockIdentity);
  syncOwnedDirectory(path.dirname(handle.lockDir));
  processLocks.delete(handle.rootDir);
  removeLockDirectory(releasedDir, handle.token);
}

function contextWith(handle: ArtifactLockHandle): ReadonlyMap<string, string> {
  const next = new Map(lockContext.getStore() ?? []);
  next.set(handle.rootDir, handle.token);
  return next;
}

function isContextOwner(rootDir: string): boolean {
  const token = lockContext.getStore()?.get(rootDir);
  return token != null && processLocks.get(rootDir) === token;
}

export function withLocalArtifactLock<T>(rootDir: string, operation: () => T): T {
  const resolvedRoot = ensureLocalArtifactRoot(rootDir);
  if (isContextOwner(resolvedRoot)) {
    return operation();
  }
  if (processLocks.has(resolvedRoot)) {
    throw new Error('local artifact root is busy in this process');
  }
  const handle = acquireArtifactLock(resolvedRoot);
  try {
    return lockContext.run(contextWith(handle), operation);
  } finally {
    releaseArtifactLock(handle);
  }
}

export async function withLocalArtifactLockAsync<T>(rootDir: string, operation: () => Promise<T>): Promise<T> {
  const resolvedRoot = ensureLocalArtifactRoot(rootDir);
  if (isContextOwner(resolvedRoot)) {
    return operation();
  }
  if (processLocks.has(resolvedRoot)) {
    throw new Error('local artifact root is busy in this process');
  }
  const handle = acquireArtifactLock(resolvedRoot);
  try {
    return await lockContext.run(contextWith(handle), operation);
  } finally {
    releaseArtifactLock(handle);
  }
}

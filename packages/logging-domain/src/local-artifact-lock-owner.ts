import fs from 'node:fs';
import path from 'node:path';
import {
  assertExistingOwnedPath,
  assertLocalArtifactIdentity,
  assertOpenedFileMatchesPath,
  ensureLocalArtifactRoot,
  ensureOwnedDirectory,
  localArtifactIdentity,
  syncOwnedDirectory,
} from './local-artifact-path';

export interface ArtifactLockOwner {
  readonly schemaVersion: 1;
  readonly pid: number;
  readonly token: string;
  readonly createdAt: number;
}

const LockOwnerFile = 'owner.json';
const MaximumLockOwnerBytes = 4_096;

export function lockPaths(rootDir: string): { readonly bridgeDir: string; readonly lockDir: string } {
  const bridgeDir = ensureOwnedDirectory(path.join(ensureLocalArtifactRoot(rootDir), '.bridge'));
  return { bridgeDir, lockDir: path.join(bridgeDir, 'artifact.lock') };
}

export function writeOwner(candidateDir: string, owner: ArtifactLockOwner): void {
  fs.mkdirSync(candidateDir, { mode: 0o700 });
  const ownerPath = path.join(candidateDir, LockOwnerFile);
  const descriptor = fs.openSync(ownerPath, 'wx', 0o600);
  try {
    assertOpenedFileMatchesPath(ownerPath, descriptor);
    fs.writeFileSync(descriptor, `${JSON.stringify(owner)}\n`, 'utf8');
    fs.fsyncSync(descriptor);
    assertOpenedFileMatchesPath(ownerPath, descriptor);
  } finally {
    fs.closeSync(descriptor);
  }
  syncOwnedDirectory(candidateDir);
}

function parseOwner(value: unknown): ArtifactLockOwner {
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid local artifact lock owner');
  }
  const input = value as Record<string, unknown>;
  if (
    input['schemaVersion'] !== 1 ||
    !Number.isSafeInteger(input['pid']) ||
    (input['pid'] as number) <= 0 ||
    typeof input['token'] !== 'string' ||
    input['token'].length === 0 ||
    !Number.isSafeInteger(input['createdAt']) ||
    (input['createdAt'] as number) < 0
  ) {
    throw new Error('invalid local artifact lock owner');
  }
  return input as unknown as ArtifactLockOwner;
}

export function readOwner(lockDir: string): ArtifactLockOwner {
  assertExistingOwnedPath(lockDir, 'directory');
  const entries = fs.readdirSync(lockDir, { withFileTypes: true });
  if (entries.length !== 1 || entries[0]?.name !== LockOwnerFile || !entries[0].isFile()) {
    throw new Error('invalid local artifact lock directory');
  }
  const ownerPath = path.join(lockDir, LockOwnerFile);
  assertExistingOwnedPath(ownerPath, 'file');
  const descriptor = fs.openSync(ownerPath, 'r');
  try {
    assertOpenedFileMatchesPath(ownerPath, descriptor);
    if (fs.fstatSync(descriptor).size > MaximumLockOwnerBytes) {
      throw new Error('invalid local artifact lock owner');
    }
    const owner = parseOwner(JSON.parse(fs.readFileSync(descriptor, 'utf8')) as unknown);
    assertOpenedFileMatchesPath(ownerPath, descriptor);
    return owner;
  } finally {
    fs.closeSync(descriptor);
  }
}

export function processIsRunning(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code !== 'ESRCH';
  }
}

export function removeLockDirectory(lockDir: string, expectedToken: string): void {
  const lockIdentity = localArtifactIdentity(lockDir, 'directory');
  const owner = readOwner(lockDir);
  if (owner.token !== expectedToken) {
    throw new Error('local artifact lock ownership changed');
  }
  const ownerPath = path.join(lockDir, LockOwnerFile);
  const ownerIdentity = localArtifactIdentity(ownerPath, 'file');
  assertLocalArtifactIdentity(ownerPath, 'file', ownerIdentity);
  fs.unlinkSync(ownerPath);
  assertLocalArtifactIdentity(lockDir, 'directory', lockIdentity);
  fs.rmdirSync(lockDir);
  syncOwnedDirectory(path.dirname(lockDir));
}

export function removeCandidate(candidateDir: string): void {
  const ownerPath = path.join(candidateDir, LockOwnerFile);
  if (fs.existsSync(ownerPath)) {
    assertExistingOwnedPath(ownerPath, 'file');
    fs.unlinkSync(ownerPath);
  }
  if (fs.existsSync(candidateDir)) {
    assertExistingOwnedPath(candidateDir, 'directory');
    fs.rmdirSync(candidateDir);
  }
}

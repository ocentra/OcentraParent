import fs from 'node:fs';
import path from 'node:path';
import {
  assertOpenedFileMatchesPath,
  assertOpenedPrivateFile,
  ensureLocalArtifactRoot,
  ensureOwnedDirectory,
  relativeLocalArtifactPath,
  resolveContainedLocalArtifactPath,
  type LocalArtifactIdentity,
} from './local-artifact-path';

export type LocalArtifactMutation =
  | { readonly kind: 'remove'; readonly filePath: string }
  | { readonly kind: 'replace'; readonly filePath: string; readonly payload: string };

export interface PersistedMutation {
  readonly kind: 'remove' | 'replace';
  readonly relativePath: string;
  readonly stageFile: string | null;
  readonly stageIdentity: LocalArtifactIdentity | null;
  readonly backupFile: string;
  readonly targetKind: 'file' | 'directory' | null;
  readonly targetIdentity: LocalArtifactIdentity | null;
}

export interface PersistedTransaction {
  readonly schemaVersion: 1;
  readonly transactionId: string;
  readonly mutations: readonly PersistedMutation[];
}

export interface LoadedTransaction {
  readonly state: PersistedTransaction;
  readonly intentIdentity: LocalArtifactIdentity;
}

export const TransactionIdPattern = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u;
export const IntentFile = 'intent.json';

const TransactionDirectory = 'transactions';

function protectedTarget(relativePath: string): boolean {
  return (
    relativePath === '.bridge' ||
    relativePath === '.bridge/artifact.lock' ||
    relativePath.startsWith('.bridge/artifact.lock/') ||
    relativePath === `.bridge/${TransactionDirectory}` ||
    relativePath.startsWith(`.bridge/${TransactionDirectory}/`) ||
    relativePath === '.bridge/append-intents' ||
    relativePath.startsWith('.bridge/append-intents/')
  );
}

function validateRelativePath(relativePath: string): string {
  const normalized = path.posix.normalize(relativePath);
  if (
    normalized.length === 0 ||
    normalized === '.' ||
    normalized !== relativePath ||
    path.posix.isAbsolute(normalized) ||
    /^[A-Za-z]:\//u.test(normalized) ||
    normalized === '..' ||
    normalized.startsWith('../') ||
    protectedTarget(normalized)
  ) {
    throw new Error('invalid local artifact transaction target');
  }
  return normalized;
}

export function transactionRoot(rootDir: string): string {
  return ensureOwnedDirectory(path.join(ensureLocalArtifactRoot(rootDir), '.bridge', TransactionDirectory));
}

export function transactionRelativePath(rootDir: string, filePath: string): string {
  return validateRelativePath(relativeLocalArtifactPath(rootDir, filePath).split(path.sep).join('/'));
}

export function transactionTargetPath(rootDir: string, relativePath: string): string {
  const normalized = validateRelativePath(relativePath);
  return resolveContainedLocalArtifactPath(rootDir, path.join(rootDir, ...normalized.split('/')));
}

export function transactionTargetKey(relativePath: string): string {
  return process.platform === 'win32' ? relativePath.toLowerCase() : relativePath;
}

export function writePrivateFile(filePath: string, payload: string): LocalArtifactIdentity {
  const descriptor = fs.openSync(filePath, 'wx', 0o600);
  try {
    const identity = assertOpenedPrivateFile(descriptor);
    fs.writeFileSync(descriptor, payload, 'utf8');
    fs.fsyncSync(descriptor);
    assertOpenedFileMatchesPath(filePath, descriptor);
    return identity;
  } finally {
    fs.closeSync(descriptor);
  }
}

import fs from 'node:fs';
import path from 'node:path';
import {
  IntentFile,
  TransactionIdPattern,
  transactionTargetKey,
  transactionTargetPath,
  type LoadedTransaction,
  type PersistedMutation,
  type PersistedTransaction,
} from './local-artifact-transaction-codec';
import {
  assertExistingOwnedPath,
  assertOpenedFileMatchesPath,
  type LocalArtifactIdentity,
} from './local-artifact-path';

const MaximumIntentBytes = 256 * 1024;

function parseIdentity(value: unknown): LocalArtifactIdentity | null {
  if (value === null) {
    return null;
  }
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid local artifact transaction');
  }
  const input = value as Record<string, unknown>;
  if (!Number.isSafeInteger(input['device']) || !Number.isSafeInteger(input['inode'])) {
    throw new Error('invalid local artifact transaction');
  }
  return { device: input['device'] as number, inode: input['inode'] as number };
}

function parsePersistedMutation(value: unknown, index: number): PersistedMutation {
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid local artifact transaction');
  }
  const input = value as Record<string, unknown>;
  const kind = input['kind'];
  const stageFile = input['stageFile'];
  const targetKind = input['targetKind'];
  if (
    (kind !== 'remove' && kind !== 'replace') ||
    typeof input['relativePath'] !== 'string' ||
    typeof input['backupFile'] !== 'string' ||
    (stageFile !== null && typeof stageFile !== 'string') ||
    (kind === 'replace' && stageFile !== `stage-${index}.data`) ||
    (kind === 'remove' && stageFile !== null) ||
    (targetKind !== null && targetKind !== 'file' && targetKind !== 'directory') ||
    input['backupFile'] !== `backup-${index}` ||
    (kind === 'replace' && targetKind === 'directory')
  ) {
    throw new Error('invalid local artifact transaction');
  }
  const stageIdentity = parseIdentity(input['stageIdentity']);
  const targetIdentity = parseIdentity(input['targetIdentity']);
  if (
    (kind === 'replace' && stageIdentity == null) ||
    (kind === 'remove' && stageIdentity != null) ||
    (targetKind == null) !== (targetIdentity == null)
  ) {
    throw new Error('invalid local artifact transaction');
  }
  return {
    kind,
    relativePath: input['relativePath'] as string,
    stageFile,
    stageIdentity,
    backupFile: input['backupFile'] as string,
    targetKind,
    targetIdentity,
  };
}

function parseTransactionValue(rootDir: string, transactionDir: string, value: unknown): PersistedTransaction {
  if (typeof value !== 'object' || value == null || Array.isArray(value)) {
    throw new Error('invalid local artifact transaction');
  }
  const input = value as Record<string, unknown>;
  if (
    input['schemaVersion'] !== 1 ||
    typeof input['transactionId'] !== 'string' ||
    !TransactionIdPattern.test(input['transactionId']) ||
    input['transactionId'] !== path.basename(transactionDir) ||
    !Array.isArray(input['mutations']) ||
    input['mutations'].length === 0
  ) {
    throw new Error('invalid local artifact transaction');
  }
  const mutations = input['mutations'].map(parsePersistedMutation);
  for (const mutation of mutations) {
    transactionTargetPath(rootDir, mutation.relativePath);
  }
  if (new Set(mutations.map((item) => transactionTargetKey(item.relativePath))).size !== mutations.length) {
    throw new Error('invalid local artifact transaction');
  }
  return { schemaVersion: 1, transactionId: input['transactionId'], mutations };
}

export function readTransaction(rootDir: string, transactionDir: string): LoadedTransaction | null {
  const intentPath = path.join(transactionDir, IntentFile);
  if (!fs.existsSync(intentPath)) {
    return null;
  }
  assertExistingOwnedPath(intentPath, 'file');
  const descriptor = fs.openSync(intentPath, 'r');
  try {
    assertOpenedFileMatchesPath(intentPath, descriptor);
    const stat = fs.fstatSync(descriptor);
    if (stat.size > MaximumIntentBytes) {
      throw new Error('invalid local artifact transaction');
    }
    const value = JSON.parse(fs.readFileSync(descriptor, 'utf8')) as unknown;
    assertOpenedFileMatchesPath(intentPath, descriptor);
    const state = parseTransactionValue(rootDir, transactionDir, value);
    return { state, intentIdentity: { device: stat.dev, inode: stat.ino } };
  } finally {
    fs.closeSync(descriptor);
  }
}

import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import {
  IntentFile,
  transactionRelativePath,
  transactionRoot,
  transactionTargetPath,
  writePrivateFile,
  type LocalArtifactMutation,
  type PersistedMutation,
  type PersistedTransaction,
} from './local-artifact-transaction-codec';
import {
  assertOwnedDirectoryTree,
  localArtifactIdentity,
  syncOwnedDirectory,
  type LocalArtifactIdentity,
} from './local-artifact-path';

function captureMutationTarget(
  rootDir: string,
  mutation: LocalArtifactMutation
): {
  readonly relativePath: string;
  readonly targetKind: 'file' | 'directory' | null;
  readonly targetIdentity: LocalArtifactIdentity | null;
} {
  const relativePath = transactionRelativePath(rootDir, mutation.filePath);
  const target = transactionTargetPath(rootDir, relativePath);
  if (!fs.existsSync(target)) {
    return { relativePath, targetKind: null, targetIdentity: null };
  }
  const stat = fs.lstatSync(target);
  const targetKind = stat.isDirectory() && !stat.isSymbolicLink() ? 'directory' : 'file';
  if (mutation.kind === 'replace' && targetKind !== 'file') {
    throw new Error('local artifact replacement target must be an owned file');
  }
  const targetIdentity = localArtifactIdentity(target, targetKind);
  if (targetKind === 'directory') {
    assertOwnedDirectoryTree(target);
  }
  return { relativePath, targetKind, targetIdentity };
}

export function prepareTransaction(rootDir: string, mutations: readonly LocalArtifactMutation[]): string {
  const transactionId = crypto.randomUUID();
  const transactionDir = path.join(transactionRoot(rootDir), transactionId);
  fs.mkdirSync(transactionDir, { mode: 0o700 });
  const persisted = mutations.map((mutation, index): PersistedMutation => {
    const target = captureMutationTarget(rootDir, mutation);
    const stageFile = mutation.kind === 'replace' ? `stage-${index}.data` : null;
    const stageIdentity =
      mutation.kind === 'replace'
        ? writePrivateFile(path.join(transactionDir, stageFile as string), mutation.payload)
        : null;
    return {
      kind: mutation.kind,
      relativePath: target.relativePath,
      stageFile,
      stageIdentity,
      backupFile: `backup-${index}`,
      targetKind: target.targetKind,
      targetIdentity: target.targetIdentity,
    };
  });
  const transaction: PersistedTransaction = { schemaVersion: 1, transactionId, mutations: persisted };
  writePrivateFile(path.join(transactionDir, IntentFile), `${JSON.stringify(transaction)}\n`);
  syncOwnedDirectory(transactionDir);
  syncOwnedDirectory(path.dirname(transactionDir));
  return transactionDir;
}

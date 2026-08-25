import fs from 'node:fs';
import path from 'node:path';
import { withLocalArtifactLock } from './local-artifact-lock';
import {
  TransactionIdPattern,
  transactionRelativePath,
  transactionRoot,
  transactionTargetKey,
  type LocalArtifactMutation as ArtifactMutation,
} from './local-artifact-transaction-codec';
import { readTransaction } from './local-artifact-transaction-parsing';
import { finishTransaction, removeOrphanTransaction } from './local-artifact-transaction-cleanup';
import { prepareTransaction } from './local-artifact-transaction-prepare';
import { applyPersistedMutation } from './local-artifact-transaction-tree';
import { assertExistingOwnedPath, ensureLocalArtifactRoot } from './local-artifact-path';

export type LocalArtifactMutation = ArtifactMutation;

function recoverTransactionsLocked(rootDir: string): void {
  const transactions = transactionRoot(rootDir);
  for (const entry of fs.readdirSync(transactions, { withFileTypes: true })) {
    if (!entry.isDirectory() || !TransactionIdPattern.test(entry.name)) {
      throw new Error('invalid local artifact transaction entry');
    }
    const transactionDir = path.join(transactions, entry.name);
    assertExistingOwnedPath(transactionDir, 'directory');
    const transaction = readTransaction(rootDir, transactionDir);
    if (transaction == null) {
      removeOrphanTransaction(transactionDir);
      continue;
    }
    for (const mutation of transaction.state.mutations) {
      applyPersistedMutation(rootDir, transactionDir, mutation);
    }
    finishTransaction(transactionDir, transaction);
  }
}

export function recoverLocalArtifactTransactions(rootDir: string): void {
  withLocalArtifactLock(rootDir, () => recoverTransactionsLocked(ensureLocalArtifactRoot(rootDir)));
}

export function applyLocalArtifactTransaction(rootDir: string, mutations: readonly LocalArtifactMutation[]): void {
  if (mutations.length === 0) {
    return;
  }
  withLocalArtifactLock(rootDir, () => {
    const resolvedRoot = ensureLocalArtifactRoot(rootDir);
    recoverTransactionsLocked(resolvedRoot);
    const targets = mutations.map((mutation) =>
      transactionTargetKey(transactionRelativePath(resolvedRoot, mutation.filePath))
    );
    if (new Set(targets).size !== targets.length) {
      throw new Error('local artifact transaction contains duplicate targets');
    }
    const transactionDir = prepareTransaction(resolvedRoot, mutations);
    const transaction = readTransaction(resolvedRoot, transactionDir);
    if (transaction == null) {
      throw new Error('local artifact transaction intent was not persisted');
    }
    for (const mutation of transaction.state.mutations) {
      applyPersistedMutation(resolvedRoot, transactionDir, mutation);
    }
    finishTransaction(transactionDir, transaction);
  });
}

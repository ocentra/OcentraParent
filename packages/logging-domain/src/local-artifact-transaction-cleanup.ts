import fs from 'node:fs';
import path from 'node:path';
import { IntentFile, type LoadedTransaction } from './local-artifact-transaction-codec';
import {
  assertExistingOwnedPath,
  assertLocalArtifactIdentity,
  assertOwnedDirectoryTree,
  syncOwnedDirectory,
  type LocalArtifactIdentity,
} from './local-artifact-path';

function removeOwnedTree(targetPath: string): void {
  if (!fs.existsSync(targetPath)) {
    return;
  }
  const stat = fs.lstatSync(targetPath);
  if (stat.isDirectory() && !stat.isSymbolicLink()) {
    assertOwnedDirectoryTree(targetPath);
    for (const entry of fs.readdirSync(targetPath, { withFileTypes: true })) {
      removeOwnedTree(path.join(targetPath, entry.name));
    }
    fs.rmdirSync(targetPath);
    return;
  }
  assertExistingOwnedPath(targetPath, 'file');
  fs.unlinkSync(targetPath);
}

export function finishTransaction(transactionDir: string, transaction: LoadedTransaction): void {
  for (const mutation of transaction.state.mutations) {
    const backupPath = path.join(transactionDir, mutation.backupFile);
    if (fs.existsSync(backupPath)) {
      if (mutation.targetIdentity == null || mutation.targetKind == null) {
        throw new Error('local artifact transaction has an unexpected backup');
      }
      assertLocalArtifactIdentity(backupPath, mutation.targetKind, mutation.targetIdentity);
    }
    removeOwnedTree(backupPath);
    if (mutation.stageFile != null) {
      const stagePath = path.join(transactionDir, mutation.stageFile);
      if (fs.existsSync(stagePath)) {
        assertLocalArtifactIdentity(stagePath, 'file', mutation.stageIdentity as LocalArtifactIdentity);
      }
      removeOwnedTree(stagePath);
    }
  }
  const intentPath = path.join(transactionDir, IntentFile);
  if (fs.existsSync(intentPath)) {
    assertLocalArtifactIdentity(intentPath, 'file', transaction.intentIdentity);
    fs.unlinkSync(intentPath);
  }
  fs.rmdirSync(transactionDir);
  syncOwnedDirectory(path.dirname(transactionDir));
}

export function removeOrphanTransaction(transactionDir: string): void {
  assertExistingOwnedPath(transactionDir, 'directory');
  for (const entry of fs.readdirSync(transactionDir, { withFileTypes: true })) {
    if (!entry.isFile()) {
      throw new Error('invalid orphan local artifact transaction');
    }
    removeOwnedTree(path.join(transactionDir, entry.name));
  }
  fs.rmdirSync(transactionDir);
  syncOwnedDirectory(path.dirname(transactionDir));
}

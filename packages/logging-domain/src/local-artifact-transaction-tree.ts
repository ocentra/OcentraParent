import fs from 'node:fs';
import path from 'node:path';
import { assertMissingMutationTarget } from './local-artifact-transaction-recovery';
import { transactionTargetPath, type PersistedMutation } from './local-artifact-transaction-codec';
import {
  assertLocalArtifactIdentity,
  assertOwnedDirectoryTree,
  ensureOwnedDirectory,
  syncOwnedDirectory,
  type LocalArtifactIdentity,
} from './local-artifact-path';

function moveTargetToBackup(target: string, backup: string, mutation: PersistedMutation): void {
  if (!fs.existsSync(target)) {
    assertMissingMutationTarget(backup, mutation);
    return;
  }
  if (mutation.targetIdentity == null || mutation.targetKind == null) {
    throw new Error('local artifact transaction target appeared after preparation');
  }
  if (fs.existsSync(backup)) {
    throw new Error('local artifact transaction has conflicting target and backup');
  }
  const stat = fs.lstatSync(target);
  const kind = stat.isDirectory() && !stat.isSymbolicLink() ? 'directory' : 'file';
  if (mutation.kind === 'replace' && kind !== 'file') {
    throw new Error('local artifact replacement target must be an owned file');
  }
  assertLocalArtifactIdentity(target, mutation.targetKind, mutation.targetIdentity);
  if (kind === 'directory') {
    assertOwnedDirectoryTree(target);
  }
  fs.renameSync(target, backup);
  assertLocalArtifactIdentity(backup, mutation.targetKind, mutation.targetIdentity);
  syncOwnedDirectory(path.dirname(target));
  syncOwnedDirectory(path.dirname(backup));
}

export function applyPersistedMutation(rootDir: string, transactionDir: string, mutation: PersistedMutation): void {
  const target = transactionTargetPath(rootDir, mutation.relativePath);
  const backup = path.join(transactionDir, mutation.backupFile);
  const stage = mutation.stageFile == null ? null : path.join(transactionDir, mutation.stageFile);
  if (stage != null && fs.existsSync(stage)) {
    assertLocalArtifactIdentity(stage, 'file', mutation.stageIdentity as LocalArtifactIdentity);
    moveTargetToBackup(target, backup, mutation);
    ensureOwnedDirectory(path.dirname(target));
    fs.renameSync(stage, target);
    assertLocalArtifactIdentity(target, 'file', mutation.stageIdentity as LocalArtifactIdentity);
    syncOwnedDirectory(path.dirname(target));
    return;
  }
  if (mutation.kind === 'remove') {
    moveTargetToBackup(target, backup, mutation);
    return;
  }
  if (!fs.existsSync(target)) {
    throw new Error('local artifact replacement lost both stage and target');
  }
  assertLocalArtifactIdentity(target, 'file', mutation.stageIdentity as LocalArtifactIdentity);
}

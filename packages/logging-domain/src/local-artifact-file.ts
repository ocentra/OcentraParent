import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import {
  assertExistingOwnedAncestors,
  assertExistingOwnedPath,
  assertOpenedPrivateFile,
  ensureOwnedDirectory,
  resolveLocalArtifactPath,
  syncOwnedDirectory,
} from './local-artifact-path';

export function assertReadableLocalArtifactFile(filePath: string): boolean {
  const targetPath = resolveLocalArtifactPath(filePath);
  if (!fs.existsSync(targetPath)) {
    return false;
  }
  assertExistingOwnedAncestors(path.dirname(targetPath));
  assertExistingOwnedPath(targetPath, 'file');
  return true;
}

export function durableAppendLocalArtifact(filePath: string, payload: string): void {
  const targetPath = resolveLocalArtifactPath(filePath);
  const parentDir = ensureOwnedDirectory(path.dirname(targetPath));
  if (fs.existsSync(targetPath)) {
    assertExistingOwnedPath(targetPath, 'file');
  }
  const descriptor = fs.openSync(targetPath, 'a', 0o600);
  try {
    assertOpenedPrivateFile(descriptor);
    fs.writeFileSync(descriptor, payload, 'utf8');
    fs.fsyncSync(descriptor);
  } finally {
    fs.closeSync(descriptor);
  }
  syncOwnedDirectory(parentDir);
}

export function durableReplaceLocalArtifact(filePath: string, payload: string): void {
  const targetPath = resolveLocalArtifactPath(filePath);
  const parentDir = ensureOwnedDirectory(path.dirname(targetPath));
  if (fs.existsSync(targetPath)) {
    assertExistingOwnedPath(targetPath, 'file');
  }
  const tempPath = path.join(parentDir, `.${path.basename(targetPath)}.${crypto.randomUUID()}.tmp`);
  let descriptor: number | null = null;
  try {
    descriptor = fs.openSync(tempPath, 'wx', 0o600);
    assertOpenedPrivateFile(descriptor);
    fs.writeFileSync(descriptor, payload, 'utf8');
    fs.fsyncSync(descriptor);
    fs.closeSync(descriptor);
    descriptor = null;
    fs.renameSync(tempPath, targetPath);
    syncOwnedDirectory(parentDir);
  } finally {
    if (descriptor != null) {
      fs.closeSync(descriptor);
    }
    fs.rmSync(tempPath, { force: true });
  }
}

export function durableRemoveLocalArtifact(filePath: string): boolean {
  const targetPath = resolveLocalArtifactPath(filePath);
  if (!fs.existsSync(targetPath)) {
    return false;
  }
  assertExistingOwnedAncestors(path.dirname(targetPath));
  assertExistingOwnedPath(targetPath, 'file');
  fs.unlinkSync(targetPath);
  syncOwnedDirectory(path.dirname(targetPath));
  return true;
}

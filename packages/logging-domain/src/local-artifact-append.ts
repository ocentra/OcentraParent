import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import {
  appendIntentDirectory,
  appendTargetPath,
  isAppendIntentName,
  MaximumAppendBytes,
  readAppendIntent,
  writeAppendIntent,
  type AppendIntent,
} from './local-artifact-append-codec';
import { withLocalArtifactLock } from './local-artifact-lock';
import {
  assertExistingOwnedAncestors,
  assertLocalArtifactIdentity,
  assertOpenedFileMatchesPath,
  ensureLocalArtifactRoot,
  ensureOwnedDirectory,
  relativeLocalArtifactPath,
  resolveContainedLocalArtifactPath,
  syncOwnedDirectory,
  type LocalArtifactIdentity,
} from './local-artifact-path';
import { inferLocalArtifactRoot } from './local-artifact-root';
import { recoverLocalArtifactTransactions } from './local-artifact-transaction';

function openAppendTarget(filePath: string, expectedIdentity: LocalArtifactIdentity | null): number {
  const parentDir = ensureOwnedDirectory(path.dirname(filePath));
  if (expectedIdentity != null) {
    const descriptor = fs.openSync(filePath, 'r+');
    assertOpenedFileMatchesPath(filePath, descriptor);
    return descriptor;
  }
  const descriptor = fs.openSync(filePath, 'wx+', 0o600);
  assertOpenedFileMatchesPath(filePath, descriptor);
  syncOwnedDirectory(parentDir);
  return descriptor;
}

function writeRemainingPayload(descriptor: number, payload: Buffer, offset: number, alreadyWritten: number): void {
  let written = alreadyWritten;
  while (written < payload.byteLength) {
    written += fs.writeSync(descriptor, payload, written, payload.byteLength - written, offset + written);
  }
}

function performAppendIntent(rootDir: string, intent: AppendIntent): void {
  const filePath = appendTargetPath(rootDir, intent.relativePath);
  const payload = Buffer.from(intent.payloadBase64, 'base64');
  const descriptor = openAppendTarget(filePath, intent.targetIdentity);
  try {
    assertOpenedFileMatchesPath(filePath, descriptor);
    const opened = fs.fstatSync(descriptor);
    if (
      intent.targetIdentity != null &&
      (opened.dev !== intent.targetIdentity.device || opened.ino !== intent.targetIdentity.inode)
    ) {
      throw new Error('local artifact append target identity changed');
    }
    if (intent.targetIdentity == null && opened.size !== 0) {
      throw new Error('local artifact append target appeared after intent preparation');
    }
    if (opened.size < intent.offset || opened.size > intent.offset + payload.byteLength) {
      throw new Error('local artifact append target has an unexpected size');
    }
    const existingLength = opened.size - intent.offset;
    if (existingLength > 0) {
      const existing = Buffer.alloc(existingLength);
      fs.readSync(descriptor, existing, 0, existingLength, intent.offset);
      if (!existing.equals(payload.subarray(0, existingLength))) {
        fs.ftruncateSync(descriptor, intent.offset);
        writeRemainingPayload(descriptor, payload, intent.offset, 0);
      } else {
        writeRemainingPayload(descriptor, payload, intent.offset, existingLength);
      }
    } else {
      writeRemainingPayload(descriptor, payload, intent.offset, 0);
    }
    fs.fsyncSync(descriptor);
    assertOpenedFileMatchesPath(filePath, descriptor);
  } finally {
    fs.closeSync(descriptor);
  }
  syncOwnedDirectory(path.dirname(filePath));
}

function recoverAppendsLocked(rootDir: string): void {
  const intentDir = appendIntentDirectory(rootDir);
  for (const entry of fs.readdirSync(intentDir, { withFileTypes: true })) {
    if (!entry.isFile() || !isAppendIntentName(entry.name)) {
      throw new Error('invalid local artifact append intent entry');
    }
    const intentPath = path.join(intentDir, entry.name);
    const { intent, identity } = readAppendIntent(intentPath);
    performAppendIntent(rootDir, intent);
    assertLocalArtifactIdentity(intentPath, 'file', identity);
    fs.unlinkSync(intentPath);
    syncOwnedDirectory(intentDir);
  }
}

function appendTargetSnapshot(targetPath: string): {
  readonly identity: LocalArtifactIdentity | null;
  readonly size: number;
} {
  try {
    const descriptor = fs.openSync(targetPath, 'r');
    try {
      assertOpenedFileMatchesPath(targetPath, descriptor);
      const stat = fs.fstatSync(descriptor);
      return { identity: { device: stat.dev, inode: stat.ino }, size: stat.size };
    } finally {
      fs.closeSync(descriptor);
    }
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== 'ENOENT') {
      throw error;
    }
    return { identity: null, size: 0 };
  }
}

function prepareAppendIntent(rootDir: string, filePath: string, payload: Buffer): string {
  const targetPath = resolveContainedLocalArtifactPath(rootDir, filePath);
  const relativePath = relativeLocalArtifactPath(rootDir, targetPath).split(path.sep).join('/');
  appendTargetPath(rootDir, relativePath);
  assertExistingOwnedAncestors(path.dirname(targetPath));
  const target = appendTargetSnapshot(targetPath);
  return writeAppendIntent(rootDir, {
    schemaVersion: 1,
    relativePath,
    offset: target.size,
    payloadLength: payload.byteLength,
    payloadSha256: crypto.createHash('sha256').update(payload).digest('hex'),
    payloadBase64: payload.toString('base64'),
    targetIdentity: target.identity,
  });
}

export function recoverLocalArtifactAppends(rootDir: string): void {
  withLocalArtifactLock(rootDir, () => {
    const resolvedRoot = ensureLocalArtifactRoot(rootDir);
    recoverLocalArtifactTransactions(resolvedRoot);
    recoverAppendsLocked(resolvedRoot);
  });
}

export function durableAppendLocalArtifact(filePath: string, payload: string, rootDir?: string): void {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  const bytes = Buffer.from(payload, 'utf8');
  if (bytes.byteLength === 0 || bytes.byteLength > MaximumAppendBytes || bytes[bytes.byteLength - 1] !== 0x0a) {
    throw new Error('local artifact append must be a bounded newline-terminated record batch');
  }
  withLocalArtifactLock(resolvedRoot, () => {
    recoverLocalArtifactTransactions(resolvedRoot);
    recoverAppendsLocked(resolvedRoot);
    const intentPath = prepareAppendIntent(resolvedRoot, filePath, bytes);
    const persisted = readAppendIntent(intentPath);
    performAppendIntent(resolvedRoot, persisted.intent);
    assertLocalArtifactIdentity(intentPath, 'file', persisted.identity);
    fs.unlinkSync(intentPath);
    syncOwnedDirectory(path.dirname(intentPath));
  });
}

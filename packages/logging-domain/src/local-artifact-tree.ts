import fs from 'node:fs';
import path from 'node:path';
import { recoverLocalArtifactAppends } from './local-artifact-append';
import { applyLocalArtifactTransaction, type LocalArtifactMutation } from './local-artifact-transaction';
import {
  assertExistingOwnedPath,
  assertNotFileSystemRoot,
  ensureLocalArtifactRoot,
  resolveLocalArtifactPath,
} from './local-artifact-path';
import { withLocalArtifactLock } from './local-artifact-lock';

const LoggingArtifactRootEntries = new Set(['test-logs', 'app-logs', 'db', 'manifests', '.bridge']);

export function assertLoggingArtifactRootLayout(rootDir: string): void {
  const invalidEntry = fs
    .readdirSync(rootDir, { withFileTypes: true })
    .find((entry) => !LoggingArtifactRootEntries.has(entry.name) || !entry.isDirectory());
  if (invalidEntry != null) {
    throw new Error('logging artifact root contains an unowned entry');
  }
}

export function clearLoggingArtifactRoot(
  rootDir: string,
  additionalMutations: readonly LocalArtifactMutation[] = []
): void {
  const targetPath = resolveLocalArtifactPath(rootDir);
  assertNotFileSystemRoot(targetPath);
  ensureLocalArtifactRoot(targetPath);
  withLocalArtifactLock(targetPath, () => {
    recoverLocalArtifactAppends(targetPath);
    assertExistingOwnedPath(targetPath, 'directory');
    assertLoggingArtifactRootLayout(targetPath);
    const removals = fs
      .readdirSync(targetPath, { withFileTypes: true })
      .filter((entry) => entry.name !== '.bridge')
      .map((entry): LocalArtifactMutation => ({ kind: 'remove', filePath: path.join(targetPath, entry.name) }));
    applyLocalArtifactTransaction(targetPath, [...removals, ...additionalMutations]);
  });
}

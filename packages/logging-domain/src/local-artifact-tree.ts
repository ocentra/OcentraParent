import path from 'node:path';
import { withLocalArtifactLock } from './local-artifact-lock';
import { providerList, providerRecover } from './local-artifact-mutation-provider';
import { assertNotFileSystemRoot, ensureLocalArtifactRoot, resolveLocalArtifactPath } from './local-artifact-path';
import { applyLocalArtifactTransaction, type LocalArtifactMutation } from './local-artifact-transaction';

const LoggingArtifactRootEntries = new Set(['test-logs', 'app-logs', 'db', 'manifests', '.bridge']);

export function assertLoggingArtifactRootLayout(rootDir: string): void {
  const resolvedRoot = ensureLocalArtifactRoot(rootDir);
  const invalidEntry = providerList(resolvedRoot, '').find(
    (entry) => !LoggingArtifactRootEntries.has(entry.name) || !entry.is_directory
  );
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
    providerRecover(targetPath);
    const entries = providerList(targetPath, '');
    const invalidEntry = entries.find((entry) => !LoggingArtifactRootEntries.has(entry.name) || !entry.is_directory);
    if (invalidEntry != null) {
      throw new Error('logging artifact root contains an unowned entry');
    }
    const removals = entries
      .filter((entry) => entry.name !== '.bridge')
      .map(
        (entry): LocalArtifactMutation => ({
          kind: 'removeTree',
          filePath: path.join(targetPath, entry.name),
        })
      );
    applyLocalArtifactTransaction(targetPath, [...removals, ...additionalMutations]);
  });
}

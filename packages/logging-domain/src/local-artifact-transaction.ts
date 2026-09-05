import path from 'node:path';
import { withLocalArtifactLock } from './local-artifact-lock';
import {
  providerApplyTransaction,
  providerRecover,
  type LocalArtifactProviderMutation,
} from './local-artifact-mutation-provider';
import { ensureLocalArtifactRoot, relativeLocalArtifactPath } from './local-artifact-path';

export type LocalArtifactMutation =
  | { readonly kind: 'remove'; readonly filePath: string }
  | { readonly kind: 'removeTree'; readonly filePath: string }
  | { readonly kind: 'replace'; readonly filePath: string; readonly payload: string };

function protectedTarget(relativePath: string): boolean {
  const normalized = process.platform === 'win32' ? relativePath.toLowerCase() : relativePath;
  return (
    normalized === '.bridge' ||
    normalized === '.bridge/.mutation-owner' ||
    normalized.startsWith('.bridge/.mutation-owner/')
  );
}

function transactionRelativePath(rootDir: string, filePath: string): string {
  const relativePath = relativeLocalArtifactPath(rootDir, filePath).split(path.sep).join('/');
  const normalized = path.posix.normalize(relativePath);
  if (
    normalized.length === 0 ||
    normalized === '.' ||
    normalized !== relativePath ||
    path.posix.isAbsolute(normalized) ||
    normalized === '..' ||
    normalized.startsWith('../') ||
    protectedTarget(normalized)
  ) {
    throw new Error('invalid local artifact transaction target');
  }
  return normalized;
}

function providerMutations(
  rootDir: string,
  mutations: readonly LocalArtifactMutation[]
): readonly LocalArtifactProviderMutation[] {
  const targets = new Set<string>();
  return mutations.map((mutation) => {
    const relativePath = transactionRelativePath(rootDir, mutation.filePath);
    const targetKey = process.platform === 'win32' ? relativePath.toLowerCase() : relativePath;
    if (targets.has(targetKey)) {
      throw new Error('local artifact transaction contains duplicate targets');
    }
    targets.add(targetKey);
    if (mutation.kind === 'replace') {
      return { kind: 'replace', relativePath, payload: Buffer.from(mutation.payload, 'utf8') };
    }
    return { kind: mutation.kind, relativePath };
  });
}

export function recoverLocalArtifactTransactions(rootDir: string): void {
  const resolvedRoot = ensureLocalArtifactRoot(rootDir);
  withLocalArtifactLock(resolvedRoot, () => {
    providerRecover(resolvedRoot);
  });
}

export function applyLocalArtifactTransaction(rootDir: string, mutations: readonly LocalArtifactMutation[]): void {
  if (mutations.length === 0) return;
  const resolvedRoot = ensureLocalArtifactRoot(rootDir);
  withLocalArtifactLock(resolvedRoot, () => {
    providerRecover(resolvedRoot);
    providerApplyTransaction(resolvedRoot, providerMutations(resolvedRoot, mutations));
  });
}

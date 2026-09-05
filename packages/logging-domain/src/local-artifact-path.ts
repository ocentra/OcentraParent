import path from 'node:path';
import {
  localArtifactMutationCapability as providerMutationCapability,
  providerEnsureDirectory,
  providerSyncDirectory,
} from './local-artifact-mutation-provider';

const ArtifactRootEntries = new Set(['test-logs', 'app-logs', 'db', 'manifests', '.bridge']);

export const LocalArtifactMutationUnsupportedCode = 'LOCAL_ARTIFACT_MUTATION_UNSUPPORTED';

export interface LocalArtifactMutationCapability {
  readonly status: 'supported' | 'unsupported';
  readonly platform: NodeJS.Platform;
  readonly provider: 'rust-windows-owner' | 'unavailable';
  readonly reason?: 'provider-binary-invalid' | 'provider-binary-not-found' | 'unsupported-platform';
}

export class LocalArtifactMutationUnsupportedError extends Error {
  readonly code = LocalArtifactMutationUnsupportedCode;
  readonly capability: LocalArtifactMutationCapability;

  constructor(capability: LocalArtifactMutationCapability = localArtifactMutationCapability()) {
    super(
      `local artifact mutation is unsupported on ${capability.platform}: ` +
        'the pinned Rust Windows mutation owner is unavailable'
    );
    this.name = 'LocalArtifactMutationUnsupportedError';
    this.capability = capability;
  }
}

export type LocalArtifactDirectoryDurability = 'synced' | 'recovery-intent-only' | 'mutation-unsupported';

export function localArtifactMutationCapability(): LocalArtifactMutationCapability {
  return providerMutationCapability();
}

export function assertLocalArtifactMutationSupported(): void {
  const capability = localArtifactMutationCapability();
  if (capability.status !== 'supported') {
    throw new LocalArtifactMutationUnsupportedError(capability);
  }
}

function requireOwnedPath(condition: boolean, message: string): void {
  if (!condition) {
    throw new Error(message);
  }
}

export function resolveLocalArtifactPath(filePath: string): string {
  return path.resolve(filePath);
}

export function resolveContainedLocalArtifactPath(rootDir: string, filePath: string): string {
  const rootPath = resolveLocalArtifactPath(rootDir);
  const targetPath = resolveLocalArtifactPath(filePath);
  const relativePath = path.relative(rootPath, targetPath);
  requireOwnedPath(
    relativePath.length > 0 &&
      !path.isAbsolute(relativePath) &&
      relativePath !== '..' &&
      !relativePath.startsWith(`..${path.sep}`),
    'local artifact target must remain inside its declared root'
  );
  return targetPath;
}

export function relativeLocalArtifactPath(rootDir: string, filePath: string): string {
  return path.relative(resolveLocalArtifactPath(rootDir), resolveContainedLocalArtifactPath(rootDir, filePath));
}

export function assertNotFileSystemRoot(targetPath: string): void {
  requireOwnedPath(targetPath !== path.parse(targetPath).root, 'local artifact root must not be a filesystem root');
}

export function ensureOwnedDirectory(dirPath: string): string {
  const targetPath = resolveLocalArtifactPath(dirPath);
  const owned = providerOwnedPath(targetPath);
  providerEnsureDirectory(owned.rootDir, owned.relativePath);
  return targetPath;
}

export function localArtifactDirectoryDurability(rootDir?: string): LocalArtifactDirectoryDurability {
  if (localArtifactMutationCapability().status !== 'supported') {
    return 'mutation-unsupported';
  }
  if (rootDir == null) {
    return 'synced';
  }
  try {
    return providerSyncDirectory(resolveLocalArtifactPath(rootDir), '') ? 'synced' : 'mutation-unsupported';
  } catch {
    return 'mutation-unsupported';
  }
}

export function ensureLocalArtifactRoot(rootDir: string): string {
  const targetPath = resolveLocalArtifactPath(rootDir);
  assertNotFileSystemRoot(targetPath);
  assertLocalArtifactMutationSupported();
  providerEnsureDirectory(targetPath, '');
  return targetPath;
}

export function syncOwnedDirectory(dirPath: string): boolean {
  const targetPath = resolveLocalArtifactPath(dirPath);
  const owned = providerOwnedPath(targetPath, true);
  return providerSyncDirectory(owned.rootDir, owned.relativePath);
}

function providerOwnedPath(
  targetPath: string,
  allowTargetAsRoot = false
): { readonly rootDir: string; readonly relativePath: string } {
  let current = targetPath;
  while (path.dirname(current) !== current) {
    if (ArtifactRootEntries.has(path.basename(current))) {
      const rootDir = path.dirname(current);
      const relativePath = path.relative(rootDir, targetPath).split(path.sep).join('/');
      return { rootDir, relativePath };
    }
    current = path.dirname(current);
  }
  if (allowTargetAsRoot) {
    assertNotFileSystemRoot(targetPath);
    return { rootDir: targetPath, relativePath: '' };
  }
  throw new Error('local artifact directory is not below an owned artifact root entry');
}

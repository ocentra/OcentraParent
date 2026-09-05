import path from 'node:path';
import { withLocalArtifactLock } from './local-artifact-lock';
import {
  providerEnsureDirectory,
  providerReadSnapshot,
  providerRecover,
  providerRemove,
  providerReplace,
  providerStat,
} from './local-artifact-mutation-provider';
import { MaximumProviderReadBytes } from './local-artifact-provider-protocol';
import { relativeLocalArtifactPath } from './local-artifact-path';
import { inferLocalArtifactRoot } from './local-artifact-root';

export interface LocalArtifactStat {
  readonly size: number;
  readonly modifiedMs: number;
  readonly identity: {
    readonly device: string;
    readonly inode: string;
  };
}

export interface LocalArtifactTextSnapshot {
  readonly content: string;
  readonly stat: LocalArtifactStat;
}

function relativePath(rootDir: string, filePath: string): string {
  return relativeLocalArtifactPath(rootDir, filePath).split('\\').join('/');
}

function decodeUtf8(content: Buffer): string {
  try {
    return new TextDecoder('utf-8', { fatal: true }).decode(content);
  } catch {
    throw new Error('local artifact is not valid UTF-8');
  }
}

function localStat(stat: ReturnType<typeof providerStat>): LocalArtifactStat | null {
  if (stat == null) return null;
  if (stat.is_directory) {
    throw new Error('local artifact target is not a regular file');
  }
  return {
    size: stat.size,
    modifiedMs: stat.modified_ms,
    identity: stat.identity,
  };
}

export function readLocalArtifactTextSnapshot(
  filePath: string,
  rootDir?: string,
  maximumBytes = MaximumProviderReadBytes
): LocalArtifactTextSnapshot | null {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  return withLocalArtifactLock(resolvedRoot, () => {
    providerRecover(resolvedRoot);
    const snapshot = providerReadSnapshot(resolvedRoot, relativePath(resolvedRoot, filePath), maximumBytes);
    if (snapshot == null) return null;
    return {
      content: decodeUtf8(snapshot.content),
      stat: {
        size: snapshot.stat.size,
        modifiedMs: snapshot.stat.modified_ms,
        identity: snapshot.stat.identity,
      },
    };
  });
}

export function readLocalArtifactText(
  filePath: string,
  rootDir?: string,
  maximumBytes = MaximumProviderReadBytes
): string | null {
  return readLocalArtifactTextSnapshot(filePath, rootDir, maximumBytes)?.content ?? null;
}

export function statLocalArtifact(filePath: string, rootDir?: string): LocalArtifactStat | null {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  return withLocalArtifactLock(resolvedRoot, () => {
    providerRecover(resolvedRoot);
    return localStat(providerStat(resolvedRoot, relativePath(resolvedRoot, filePath)));
  });
}

export function assertReadableLocalArtifactFile(filePath: string, rootDir?: string): boolean {
  return statLocalArtifact(filePath, rootDir) != null;
}

export function durableReplaceLocalArtifact(filePath: string, payload: string, rootDir?: string): void {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  withLocalArtifactLock(resolvedRoot, () => {
    providerRecover(resolvedRoot);
    const target = relativePath(resolvedRoot, filePath);
    const parent = path.posix.dirname(target);
    if (parent !== '.' && parent !== '.bridge') providerEnsureDirectory(resolvedRoot, parent);
    providerReplace(resolvedRoot, target, Buffer.from(payload, 'utf8'));
  });
}

export function durableRemoveLocalArtifact(filePath: string, rootDir?: string): boolean {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  return withLocalArtifactLock(resolvedRoot, () => {
    providerRecover(resolvedRoot);
    return providerRemove(resolvedRoot, relativePath(resolvedRoot, filePath));
  });
}

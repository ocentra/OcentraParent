import fs from 'node:fs';
import path from 'node:path';
import { recoverLocalArtifactAppends } from './local-artifact-append';
import { withLocalArtifactLock } from './local-artifact-lock';
import { applyLocalArtifactTransaction } from './local-artifact-transaction';
import {
  assertExistingOwnedAncestors,
  assertExistingOwnedPath,
  assertOpenedFileMatchesPath,
  resolveContainedLocalArtifactPath,
  type LocalArtifactIdentity,
} from './local-artifact-path';
import { inferLocalArtifactRoot } from './local-artifact-root';

export interface LocalArtifactStat {
  readonly size: number;
  readonly modifiedMs: number;
  readonly identity: LocalArtifactIdentity;
}

export interface LocalArtifactTextSnapshot {
  readonly content: string;
  readonly stat: LocalArtifactStat;
}

export function readLocalArtifactTextSnapshot(
  filePath: string,
  rootDir?: string,
  maximumBytes = Number.MAX_SAFE_INTEGER
): LocalArtifactTextSnapshot | null {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  return withLocalArtifactLock(resolvedRoot, () => {
    recoverLocalArtifactAppends(resolvedRoot);
    const targetPath = resolveContainedLocalArtifactPath(resolvedRoot, filePath);
    if (!fs.existsSync(targetPath)) {
      return null;
    }
    assertExistingOwnedAncestors(path.dirname(targetPath));
    assertExistingOwnedPath(targetPath, 'file');
    const descriptor = fs.openSync(targetPath, 'r');
    try {
      assertOpenedFileMatchesPath(targetPath, descriptor);
      const stat = fs.fstatSync(descriptor);
      if (stat.size > maximumBytes) {
        throw new Error('local artifact exceeds its read limit');
      }
      const content = fs.readFileSync(descriptor, 'utf8');
      assertOpenedFileMatchesPath(targetPath, descriptor);
      return {
        content,
        stat: {
          size: stat.size,
          modifiedMs: stat.mtimeMs,
          identity: { device: stat.dev, inode: stat.ino },
        },
      };
    } finally {
      fs.closeSync(descriptor);
    }
  });
}

export function readLocalArtifactText(
  filePath: string,
  rootDir?: string,
  maximumBytes = Number.MAX_SAFE_INTEGER
): string | null {
  return readLocalArtifactTextSnapshot(filePath, rootDir, maximumBytes)?.content ?? null;
}

export function statLocalArtifact(filePath: string, rootDir?: string): LocalArtifactStat | null {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  return withLocalArtifactLock(resolvedRoot, () => {
    recoverLocalArtifactAppends(resolvedRoot);
    const targetPath = resolveContainedLocalArtifactPath(resolvedRoot, filePath);
    if (!fs.existsSync(targetPath)) {
      return null;
    }
    const descriptor = fs.openSync(targetPath, 'r');
    try {
      assertOpenedFileMatchesPath(targetPath, descriptor);
      const stat = fs.fstatSync(descriptor);
      return {
        size: stat.size,
        modifiedMs: stat.mtimeMs,
        identity: { device: stat.dev, inode: stat.ino },
      };
    } finally {
      fs.closeSync(descriptor);
    }
  });
}

export function assertReadableLocalArtifactFile(filePath: string, rootDir?: string): boolean {
  return statLocalArtifact(filePath, rootDir) != null;
}

export function durableReplaceLocalArtifact(filePath: string, payload: string, rootDir?: string): void {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  withLocalArtifactLock(resolvedRoot, () => {
    recoverLocalArtifactAppends(resolvedRoot);
    applyLocalArtifactTransaction(resolvedRoot, [{ kind: 'replace', filePath, payload }]);
  });
}

export function durableRemoveLocalArtifact(filePath: string, rootDir?: string): boolean {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  return withLocalArtifactLock(resolvedRoot, () => {
    recoverLocalArtifactAppends(resolvedRoot);
    const targetPath = resolveContainedLocalArtifactPath(resolvedRoot, filePath);
    const existed = fs.existsSync(targetPath);
    if (existed) {
      applyLocalArtifactTransaction(resolvedRoot, [{ kind: 'remove', filePath: targetPath }]);
    }
    return existed;
  });
}

import path from 'node:path';
import { withLocalArtifactLock } from './local-artifact-lock';
import { providerAppend, providerEnsureDirectory, providerRecover } from './local-artifact-mutation-provider';
import { relativeLocalArtifactPath } from './local-artifact-path';
import { inferLocalArtifactRoot } from './local-artifact-root';

export function recoverLocalArtifactAppends(rootDir: string): void {
  const resolvedRoot = inferLocalArtifactRoot(rootDir, rootDir);
  withLocalArtifactLock(resolvedRoot, () => {
    providerRecover(resolvedRoot);
  });
}

export function durableAppendLocalArtifact(filePath: string, payload: string, rootDir?: string): void {
  const resolvedRoot = inferLocalArtifactRoot(filePath, rootDir);
  const relativePath = relativeLocalArtifactPath(resolvedRoot, filePath).split('\\').join('/');
  const bytes = Buffer.from(payload, 'utf8');
  withLocalArtifactLock(resolvedRoot, () => {
    providerRecover(resolvedRoot);
    const parentPath = path.posix.dirname(relativePath);
    if (parentPath !== '.bridge') providerEnsureDirectory(resolvedRoot, parentPath === '.' ? '' : parentPath);
    providerAppend(resolvedRoot, relativePath, bytes);
  });
}

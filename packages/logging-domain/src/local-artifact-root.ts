import path from 'node:path';
import { ensureLocalArtifactRoot, resolveLocalArtifactPath } from './local-artifact-path';

const ArtifactRootEntries = new Set(['test-logs', 'app-logs', 'db', 'manifests', '.bridge']);

export function inferLocalArtifactRoot(filePath: string, explicitRoot?: string): string {
  if (explicitRoot != null) {
    return ensureLocalArtifactRoot(explicitRoot);
  }
  const targetPath = resolveLocalArtifactPath(filePath);
  let current = targetPath;
  while (path.dirname(current) !== current) {
    if (ArtifactRootEntries.has(path.basename(current))) {
      return ensureLocalArtifactRoot(path.dirname(current));
    }
    current = path.dirname(current);
  }
  throw new Error('local artifact root could not be inferred');
}

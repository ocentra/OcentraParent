import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { providerRelativePath } from './local-artifact-provider-path';
import {
  MaximumProviderReadBytes,
  type LocalArtifactProviderSnapshot,
  type LocalArtifactProviderStat,
} from './local-artifact-provider-protocol';
import { parseProviderSnapshot } from './local-artifact-provider-protocol-collections';
import { parseBooleanResult, parseProviderStat } from './local-artifact-provider-protocol-results';
import { requestProvider } from './local-artifact-provider-request';

export function ensureProviderDirectory(rootDir: string, relativePath: string): void {
  const ready = parseBooleanResult(
    requestProvider(rootDir, {
      kind: 'ensureDirectory',
      relative_path: providerRelativePath(relativePath, true),
    }),
    'ready',
    'ensureDirectory'
  );
  if (!ready) {
    throw new LocalArtifactProviderError('io', 'provider did not establish the directory');
  }
}

export function syncProviderDirectory(rootDir: string, relativePath: string): boolean {
  return parseBooleanResult(
    requestProvider(rootDir, {
      kind: 'syncDirectory',
      relative_path: providerRelativePath(relativePath, true),
    }),
    'synced',
    'syncDirectory'
  );
}

export function statProviderPath(rootDir: string, relativePath: string): LocalArtifactProviderStat | null {
  try {
    return parseProviderStat(
      requestProvider(rootDir, {
        kind: 'stat',
        relative_path: providerRelativePath(relativePath, true),
      })
    );
  } catch (error) {
    if (error instanceof LocalArtifactProviderError && error.code === 'not-found') return null;
    throw error;
  }
}

export function readProviderSnapshot(
  rootDir: string,
  relativePath: string,
  maximumBytes: number
): LocalArtifactProviderSnapshot | null {
  if (!Number.isSafeInteger(maximumBytes) || maximumBytes <= 0 || maximumBytes > MaximumProviderReadBytes) {
    throw new LocalArtifactProviderError('size-limit', 'provider read bound is invalid');
  }
  return parseProviderSnapshot(
    requestProvider(rootDir, {
      kind: 'readSnapshot',
      relative_path: providerRelativePath(relativePath, false),
      maximum_bytes: maximumBytes,
    }),
    maximumBytes
  );
}

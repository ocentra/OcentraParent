import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { providerRelativePath } from './local-artifact-provider-path';
import { MaximumProviderAppendBytes, MaximumProviderReplaceBytes } from './local-artifact-provider-protocol';
import { requestProvider } from './local-artifact-provider-request';
import { parseMutationBoolean, parseMutationCount } from './local-artifact-provider-results';

export function appendProviderArtifact(rootDir: string, relativePath: string, payload: Buffer): number {
  if (
    payload.byteLength === 0 ||
    payload.byteLength > MaximumProviderAppendBytes ||
    payload[payload.byteLength - 1] !== 0x0a
  ) {
    throw new LocalArtifactProviderError('size-limit', 'provider append must be bounded and newline terminated');
  }
  return parseMutationCount(
    requestProvider(rootDir, {
      kind: 'append',
      relative_path: providerRelativePath(relativePath, false),
      payload_base64: payload.toString('base64'),
    }),
    'written',
    payload.byteLength,
    'append'
  );
}

export function replaceProviderArtifact(rootDir: string, relativePath: string, payload: Buffer): number {
  if (payload.byteLength > MaximumProviderReplaceBytes) {
    throw new LocalArtifactProviderError('size-limit', 'provider replacement exceeded its bound');
  }
  return parseMutationCount(
    requestProvider(rootDir, {
      kind: 'replace',
      relative_path: providerRelativePath(relativePath, false),
      payload_base64: payload.toString('base64'),
    }),
    'written',
    payload.byteLength,
    'replace'
  );
}

export function removeProviderArtifact(rootDir: string, relativePath: string): boolean {
  try {
    return parseMutationBoolean(
      requestProvider(rootDir, {
        kind: 'remove',
        relative_path: providerRelativePath(relativePath, false),
      }),
      'removed',
      'remove'
    );
  } catch (error) {
    if (error instanceof LocalArtifactProviderError && error.code === 'not-found') return false;
    throw error;
  }
}

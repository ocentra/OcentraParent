import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { providerRelativePath } from './local-artifact-provider-path';
import {
  MaximumProviderReplaceBytes,
  MaximumProviderTransactionMutations,
  type LocalArtifactProviderTransactionMutation,
} from './local-artifact-provider-protocol';
import { requestProvider } from './local-artifact-provider-request';
import { parseMutationCount } from './local-artifact-provider-results';

export type ProviderTransactionMutation =
  | { readonly kind: 'replace'; readonly relativePath: string; readonly payload: Buffer }
  | { readonly kind: 'remove'; readonly relativePath: string }
  | { readonly kind: 'removeTree'; readonly relativePath: string };

function wireTransactionMutation(mutation: ProviderTransactionMutation): LocalArtifactProviderTransactionMutation {
  if (mutation.kind !== 'replace') {
    return {
      kind: mutation.kind,
      relative_path: providerRelativePath(mutation.relativePath, false),
    };
  }
  if (mutation.payload.byteLength > MaximumProviderReplaceBytes) {
    throw new LocalArtifactProviderError('size-limit', 'provider transaction replacement exceeded its bound');
  }
  return {
    kind: 'replace',
    relative_path: providerRelativePath(mutation.relativePath, false),
    payload_base64: mutation.payload.toString('base64'),
  };
}

export function applyProviderTransaction(rootDir: string, mutations: readonly ProviderTransactionMutation[]): number {
  if (mutations.length === 0) return 0;
  if (mutations.length > MaximumProviderTransactionMutations) {
    throw new LocalArtifactProviderError('protocol-limit', 'provider transaction exceeded its mutation bound');
  }
  const wireMutations = mutations.map(wireTransactionMutation);
  return parseMutationCount(
    requestProvider(rootDir, { kind: 'applyTransaction', mutations: wireMutations }),
    'applied',
    mutations.length,
    'applyTransaction'
  );
}

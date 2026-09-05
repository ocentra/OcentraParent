import { providerRelativePath } from './local-artifact-provider-path';
import {
  type LocalArtifactProviderEntry,
  type LocalArtifactProviderIdentity,
} from './local-artifact-provider-protocol';
import { parseProviderEntries } from './local-artifact-provider-protocol-collections';
import { parseCountResult } from './local-artifact-provider-protocol-results';
import { startProviderSession } from './local-artifact-provider-registry';
import { requestProvider } from './local-artifact-provider-request';
import { parseMutationBoolean } from './local-artifact-provider-results';

export function providerRootIdentityValue(rootDir: string): LocalArtifactProviderIdentity {
  return startProviderSession(rootDir).rootIdentity;
}

export function recoverProviderArtifacts(rootDir: string): number {
  return parseCountResult(requestProvider(rootDir, { kind: 'recover' }), 'recovered', 'recover');
}

export function listProviderArtifacts(rootDir: string, relativePath: string): readonly LocalArtifactProviderEntry[] {
  return parseProviderEntries(
    requestProvider(rootDir, {
      kind: 'list',
      relative_path: providerRelativePath(relativePath, true),
    })
  );
}

export function removeProviderArtifactTree(rootDir: string, relativePath: string): boolean {
  return parseMutationBoolean(
    requestProvider(rootDir, {
      kind: 'removeTree',
      relative_path: providerRelativePath(relativePath, false),
    }),
    'removed',
    'removeTree'
  );
}

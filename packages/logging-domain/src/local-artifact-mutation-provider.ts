import { providerMutationCapability, type LocalArtifactMutationCapability } from './local-artifact-provider-binary';
import { beginProviderLease, closeProvider, endProviderLease } from './local-artifact-provider-lease';
import {
  ensureProviderDirectory,
  readProviderSnapshot,
  statProviderPath,
  syncProviderDirectory,
} from './local-artifact-provider-read-operations';
import {
  applyProviderTransaction,
  type ProviderTransactionMutation,
} from './local-artifact-provider-transaction-operations';
import {
  listProviderArtifacts,
  providerRootIdentityValue,
  recoverProviderArtifacts,
  removeProviderArtifactTree,
} from './local-artifact-provider-tree-operations';
import {
  appendProviderArtifact,
  removeProviderArtifact,
  replaceProviderArtifact,
} from './local-artifact-provider-write-operations';
import {
  type LocalArtifactProviderEntry,
  type LocalArtifactProviderIdentity,
  type LocalArtifactProviderSnapshot,
  type LocalArtifactProviderStat,
} from './local-artifact-provider-protocol';

export type LocalArtifactProviderMutation = ProviderTransactionMutation;

export function localArtifactMutationCapability(): LocalArtifactMutationCapability {
  return providerMutationCapability();
}

export function beginLocalArtifactProviderLease(rootDir: string): string {
  return beginProviderLease(rootDir);
}

export function endLocalArtifactProviderLease(rootDir: string, leaseId: string): void {
  endProviderLease(rootDir, leaseId);
}

export async function closeLocalArtifactMutationProvider(rootDir?: string): Promise<void> {
  await closeProvider(rootDir);
}

export function providerRootIdentity(rootDir: string): LocalArtifactProviderIdentity {
  return providerRootIdentityValue(rootDir);
}

export function providerRecover(rootDir: string): number {
  return recoverProviderArtifacts(rootDir);
}

export function providerEnsureDirectory(rootDir: string, relativePath: string): void {
  ensureProviderDirectory(rootDir, relativePath);
}

export function providerSyncDirectory(rootDir: string, relativePath: string): boolean {
  return syncProviderDirectory(rootDir, relativePath);
}

export function providerStat(rootDir: string, relativePath: string): LocalArtifactProviderStat | null {
  return statProviderPath(rootDir, relativePath);
}

export function providerReadSnapshot(
  rootDir: string,
  relativePath: string,
  maximumBytes: number
): LocalArtifactProviderSnapshot | null {
  return readProviderSnapshot(rootDir, relativePath, maximumBytes);
}

export function providerAppend(rootDir: string, relativePath: string, payload: Buffer): number {
  return appendProviderArtifact(rootDir, relativePath, payload);
}

export function providerReplace(rootDir: string, relativePath: string, payload: Buffer): number {
  return replaceProviderArtifact(rootDir, relativePath, payload);
}

export function providerRemove(rootDir: string, relativePath: string): boolean {
  return removeProviderArtifact(rootDir, relativePath);
}

export function providerList(rootDir: string, relativePath: string): readonly LocalArtifactProviderEntry[] {
  return listProviderArtifacts(rootDir, relativePath);
}

export function providerRemoveTree(rootDir: string, relativePath: string): boolean {
  return removeProviderArtifactTree(rootDir, relativePath);
}

export function providerApplyTransaction(rootDir: string, mutations: readonly LocalArtifactProviderMutation[]): number {
  return applyProviderTransaction(rootDir, mutations);
}

import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { type LocalArtifactProviderStat } from './local-artifact-provider-protocol';
import {
  isProviderRecord,
  parseProviderIdentity,
  providerRecordHasOnlyKeys,
  safeNonNegativeProviderInteger,
} from './local-artifact-provider-protocol-values';

export function parseProviderStat(value: unknown): LocalArtifactProviderStat | null {
  if (value == null) return null;
  if (
    !isProviderRecord(value) ||
    !providerRecordHasOnlyKeys(value, ['size', 'modified_ms', 'is_directory', 'identity'])
  ) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider stat is invalid');
  }
  const size = safeNonNegativeProviderInteger(value['size']);
  const modifiedMs = safeNonNegativeProviderInteger(value['modified_ms']);
  const isDirectory = value['is_directory'];
  if (size == null || modifiedMs == null || typeof isDirectory !== 'boolean') {
    throw new LocalArtifactProviderError('protocol-frame', 'provider stat is invalid');
  }
  return {
    size,
    modified_ms: modifiedMs,
    is_directory: isDirectory,
    identity: parseProviderIdentity(value['identity']),
  };
}

export function parseBooleanResult(value: unknown, key: string, operation: string): boolean {
  if (!isProviderRecord(value) || !providerRecordHasOnlyKeys(value, [key]) || typeof value[key] !== 'boolean') {
    throw new LocalArtifactProviderError('protocol-frame', `provider ${operation} result is invalid`);
  }
  return value[key];
}

export function parseCountResult(value: unknown, key: string, operation: string): number {
  if (!isProviderRecord(value) || !providerRecordHasOnlyKeys(value, [key])) {
    throw new LocalArtifactProviderError('protocol-frame', `provider ${operation} result is invalid`);
  }
  const count = safeNonNegativeProviderInteger(value[key]);
  if (count == null) {
    throw new LocalArtifactProviderError('protocol-frame', `provider ${operation} result is invalid`);
  }
  return count;
}

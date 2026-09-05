import { LocalArtifactProviderError } from './local-artifact-provider-error';

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value != null && !Array.isArray(value);
}

function hasOnlyKeys(value: Record<string, unknown>, keys: readonly string[]): boolean {
  const allowed = new Set(keys);
  return Object.keys(value).every((key) => allowed.has(key));
}

export function parseMutationCount(value: unknown, key: string, expected: number, operation: string): number {
  if (!isRecord(value) || !hasOnlyKeys(value, [key, 'replayed'])) {
    throw new LocalArtifactProviderError('protocol-frame', `provider ${operation} result is invalid`);
  }
  const count = value[key];
  if (
    typeof count !== 'number' ||
    !Number.isSafeInteger(count) ||
    count !== expected ||
    typeof value['replayed'] !== 'boolean'
  ) {
    throw new LocalArtifactProviderError('protocol-frame', `provider ${operation} result is invalid`);
  }
  return count;
}

export function parseMutationBoolean(value: unknown, key: string, operation: string): boolean {
  if (
    !isRecord(value) ||
    !hasOnlyKeys(value, [key, 'replayed']) ||
    typeof value[key] !== 'boolean' ||
    typeof value['replayed'] !== 'boolean'
  ) {
    throw new LocalArtifactProviderError('protocol-frame', `provider ${operation} result is invalid`);
  }
  return value[key];
}

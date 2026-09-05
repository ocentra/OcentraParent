import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { type LocalArtifactProviderIdentity } from './local-artifact-provider-protocol';

export function isProviderRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value != null && !Array.isArray(value);
}

export function providerRecordHasOnlyKeys(value: Record<string, unknown>, keys: readonly string[]): boolean {
  const allowed = new Set(keys);
  return Object.keys(value).every((key) => allowed.has(key));
}

export function safeNonNegativeProviderInteger(value: unknown): number | null {
  return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0 ? value : null;
}

export function boundedProviderString(value: unknown, maximum: number, label: string): string {
  if (typeof value !== 'string' || value.length === 0 || value.length > maximum) {
    throw new LocalArtifactProviderError('protocol-frame', `${label} is invalid`);
  }
  return value;
}

function canonicalUnsignedDecimal(value: unknown, maximum: bigint): string | null {
  if (typeof value !== 'string' || !/^(?:0|[1-9]\d*)$/u.test(value)) return null;
  try {
    return BigInt(value) <= maximum ? value : null;
  } catch {
    return null;
  }
}

export function parseProviderIdentity(value: unknown): LocalArtifactProviderIdentity {
  if (!isProviderRecord(value) || !providerRecordHasOnlyKeys(value, ['device', 'inode'])) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider identity is not exact');
  }
  const device = canonicalUnsignedDecimal(value['device'], (1n << 64n) - 1n);
  const inode = canonicalUnsignedDecimal(value['inode'], (1n << 128n) - 1n);
  if (device == null || inode == null) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider identity is invalid');
  }
  return { device, inode };
}

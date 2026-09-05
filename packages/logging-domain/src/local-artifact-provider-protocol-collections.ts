import { LocalArtifactProviderError } from './local-artifact-provider-error';
import {
  MaximumProviderReadBytes,
  type LocalArtifactProviderEntry,
  type LocalArtifactProviderSnapshot,
} from './local-artifact-provider-protocol';
import { parseProviderStat } from './local-artifact-provider-protocol-results';
import { isProviderRecord, providerRecordHasOnlyKeys } from './local-artifact-provider-protocol-values';

function assertSnapshotBound(maximumBytes: number): void {
  if (!Number.isSafeInteger(maximumBytes) || maximumBytes <= 0 || maximumBytes > MaximumProviderReadBytes) {
    throw new LocalArtifactProviderError('protocol-limit', 'provider snapshot bound is invalid');
  }
}

function parseSnapshotContent(value: unknown, maximumBytes: number): Buffer {
  if (typeof value !== 'string' || value.length > Math.ceil((maximumBytes * 4) / 3) + 4) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider snapshot payload is invalid');
  }
  const content = Buffer.from(value, 'base64');
  if (content.toString('base64') !== value) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider snapshot payload is not canonical base64');
  }
  return content;
}

function parseSnapshotStat(
  value: unknown,
  content: Buffer,
  maximumBytes: number
): LocalArtifactProviderSnapshot['stat'] {
  const stat = parseProviderStat(value);
  if (stat == null || stat.is_directory || stat.size !== content.byteLength || stat.size > maximumBytes) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider snapshot identity does not match its payload');
  }
  return stat;
}

export function parseProviderSnapshot(value: unknown, maximumBytes: number): LocalArtifactProviderSnapshot | null {
  assertSnapshotBound(maximumBytes);
  if (value == null) return null;
  if (!isProviderRecord(value) || !providerRecordHasOnlyKeys(value, ['content_base64', 'stat'])) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider snapshot is invalid');
  }
  const content = parseSnapshotContent(value['content_base64'], maximumBytes);
  const stat = parseSnapshotStat(value['stat'], content, maximumBytes);
  return { content, stat };
}

export function parseProviderEntries(value: unknown): readonly LocalArtifactProviderEntry[] {
  if (!Array.isArray(value) || value.length > 65_536) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider entry list is invalid');
  }
  return value.map((entry) => {
    if (
      !isProviderRecord(entry) ||
      !providerRecordHasOnlyKeys(entry, ['name', 'is_directory']) ||
      typeof entry['name'] !== 'string' ||
      typeof entry['is_directory'] !== 'boolean'
    ) {
      throw new LocalArtifactProviderError('protocol-frame', 'provider entry is invalid');
    }
    const name = entry['name'];
    if (
      name.length === 0 ||
      name.length > 255 ||
      name === '.' ||
      name === '..' ||
      name.includes('/') ||
      name.includes('\\') ||
      name.includes('\0')
    ) {
      throw new LocalArtifactProviderError('protocol-frame', 'provider entry name is invalid');
    }
    return { name, is_directory: entry['is_directory'] };
  });
}

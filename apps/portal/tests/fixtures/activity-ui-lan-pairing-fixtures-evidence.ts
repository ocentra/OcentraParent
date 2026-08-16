import { nonEmptyLanString } from './activity-ui-lan-pairing-fixtures-normalization';

const LanEvidenceSourceValues = new Set([
  'local-service',
  'windows-neighbor-table',
  'dns-cache',
  'netbios',
  'previous-scan-snapshot',
  'trusted-registry',
  'parent-assignment',
  'child-agent-hello',
  'child-agent-heartbeat',
]);
const LanEvidenceSourceAliases = new Map([
  ['network-neighbor', 'windows-neighbor-table'],
  ['gateway', 'windows-neighbor-table'],
  ['mdns', 'dns-cache'],
]);

const LanEvidenceKindValues = new Set([
  'interface',
  'ip-address',
  'mac-address',
  'hostname',
  'vendor',
  'router-classification',
  'historical-identity-hint',
  'child-agent-presence',
  'trusted-registry',
  'parent-decision',
  'route',
]);
const LanEvidenceKindBySource = new Map([
  ['trusted-registry', 'trusted-registry'],
  ['parent-assignment', 'parent-decision'],
  ['child-agent-hello', 'child-agent-presence'],
  ['child-agent-heartbeat', 'child-agent-presence'],
  ['previous-scan-snapshot', 'historical-identity-hint'],
  ['local-service', 'interface'],
]);

const LanEvidenceConfidenceValues = new Set(['confirmed', 'strong', 'weak', 'manual-required', 'rejected']);
const LanEvidenceConfidenceAliases = new Map([
  ['mdns-advertisement', 'weak'],
  ['network-neighbor', 'weak'],
  ['trusted-registry', 'strong'],
  ['agent-confirmed', 'confirmed'],
]);
const LanEvidenceConfidenceBySource = new Map([
  ['local-service', 'confirmed'],
  ['child-agent-hello', 'confirmed'],
  ['child-agent-heartbeat', 'confirmed'],
  ['trusted-registry', 'strong'],
  ['parent-assignment', 'strong'],
]);

function normalizeLanEvidenceSource(value: unknown, fallbackSource: string): string {
  return typeof value === 'string' && LanEvidenceSourceValues.has(value)
    ? value
    : (LanEvidenceSourceAliases.get(String(value)) ?? fallbackSource);
}

function normalizeLanEvidenceKind(value: unknown, source: string): string {
  return typeof value === 'string' && LanEvidenceKindValues.has(value)
    ? value
    : (LanEvidenceKindBySource.get(source) ?? 'ip-address');
}

function normalizeLanEvidenceConfidence(value: unknown, source: string): string {
  return typeof value === 'string' && LanEvidenceConfidenceValues.has(value)
    ? value
    : (LanEvidenceConfidenceAliases.get(String(value)) ?? LanEvidenceConfidenceBySource.get(source) ?? 'weak');
}

function lanFixtureEvidenceRecord(canonicalDeviceId: unknown, source: string) {
  const deviceId =
    typeof canonicalDeviceId === 'string' && canonicalDeviceId.length > 0 ? canonicalDeviceId : 'lan-fixture-device';
  const evidenceKind = normalizeLanEvidenceKind(null, source);
  return {
    schemaVersion: 1,
    evidenceId: `lan-fixture-evidence-${deviceId}`,
    source,
    evidenceKind,
    deviceId,
    value: deviceId,
    normalizedValue: deviceId,
    firstSeenAt: '2026-06-01T15:20:00Z',
    lastSeenAt: '2026-06-01T15:20:00Z',
    expiresAt: null,
    confidence: normalizeLanEvidenceConfidence(null, source),
    mergeKey: `merge-${deviceId}`,
    note: 'normalized portal fixture evidence',
  };
}

function lanFixtureDeviceId(record: Record<string, unknown>, canonicalDeviceId: unknown): string {
  return lanFixtureStringField(record, 'deviceId') ?? nonEmptyLanString(canonicalDeviceId) ?? 'lan-fixture-device';
}

function lanFixtureEvidenceValue(record: Record<string, unknown>, deviceId: string): string {
  return lanFixtureStringField(record, 'value') ?? lanFixtureStringField(record, 'normalizedValue') ?? deviceId;
}

function lanFixtureStringField(record: Record<string, unknown>, field: string): string | null {
  return nonEmptyLanString(record[field]);
}

function normalizeLanEvidenceRecord(
  value: unknown,
  canonicalDeviceId: unknown,
  fallbackSource: string,
  index: number
): Record<string, unknown> {
  const record = typeof value === 'object' && value !== null ? (value as Record<string, unknown>) : {};
  const source = normalizeLanEvidenceSource(record['source'], fallbackSource);
  const evidenceKind = normalizeLanEvidenceKind(record['evidenceKind'], source);
  const deviceId = lanFixtureDeviceId(record, canonicalDeviceId);
  const rawValue = lanFixtureEvidenceValue(record, deviceId);
  const normalizedValue = lanFixtureStringField(record, 'normalizedValue') ?? rawValue.toLowerCase();
  return {
    schemaVersion: 1,
    evidenceId: lanFixtureStringField(record, 'evidenceId') ?? `lan-fixture-evidence-${deviceId}-${index + 1}`,
    source,
    evidenceKind,
    deviceId,
    value: rawValue,
    normalizedValue,
    firstSeenAt: lanFixtureStringField(record, 'firstSeenAt') ?? '2026-06-01T15:20:00Z',
    lastSeenAt: lanFixtureStringField(record, 'lastSeenAt') ?? '2026-06-01T15:20:00Z',
    expiresAt: lanFixtureStringField(record, 'expiresAt') ?? null,
    confidence: normalizeLanEvidenceConfidence(record['confidence'], source),
    mergeKey: lanFixtureStringField(record, 'mergeKey') ?? `merge-${deviceId}-${index + 1}`,
    note: lanFixtureStringField(record, 'note') ?? null,
  };
}

function normalizeLanEvidenceRecords(
  value: unknown,
  canonicalDeviceId: unknown,
  fallbackSource: string
): Array<Record<string, unknown>> {
  if (!Array.isArray(value) || value.length === 0) {
    return [lanFixtureEvidenceRecord(canonicalDeviceId, fallbackSource)];
  }
  const records = value.map((entry, index) =>
    normalizeLanEvidenceRecord(entry, canonicalDeviceId, fallbackSource, index)
  );
  return records.length > 0 ? records : [lanFixtureEvidenceRecord(canonicalDeviceId, fallbackSource)];
}

export {
  lanFixtureEvidenceRecord,
  lanFixtureDeviceId,
  lanFixtureEvidenceValue,
  lanFixtureStringField,
  normalizeLanEvidenceConfidence,
  normalizeLanEvidenceKind,
  normalizeLanEvidenceRecord,
  normalizeLanEvidenceRecords,
  normalizeLanEvidenceSource,
};

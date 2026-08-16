import type {
  GeneratedPortalLanCanonicalHouseholdDeviceSnapshot,
  GeneratedPortalLanCanonicalHouseholdNetworkIdentitySnapshot,
  GeneratedPortalLanDiscoveryEvidenceRecordSnapshot,
} from './generated-portal-contracts';
import {
  isNullableString,
  isRecord,
  isString,
  normalizeRecordArray,
  normalizeStringArray,
  recordHasStringFields,
} from './live-activity-lan-add-device-shared';

export type PortalLanCanonicalHouseholdDevice = GeneratedPortalLanCanonicalHouseholdDeviceSnapshot;
export type PortalLanCanonicalNetworkIdentity = GeneratedPortalLanCanonicalHouseholdNetworkIdentitySnapshot;
export type PortalLanDiscoveryEvidenceRecord = GeneratedPortalLanDiscoveryEvidenceRecordSnapshot;

const PortalLanDiscoveryEvidenceStringFields = [
  'source',
  'evidenceKind',
  'value',
  'firstSeenAt',
  'lastSeenAt',
  'confidence',
] as const;

export function normalizePortalLanCanonicalHouseholdDevices(
  value: unknown
): readonly PortalLanCanonicalHouseholdDevice[] | null {
  if (!Array.isArray(value)) {
    return null;
  }

  const devices = value
    .map((entry) => normalizePortalLanCanonicalHouseholdDevice(entry))
    .filter((entry): entry is PortalLanCanonicalHouseholdDevice => entry !== null);

  return devices.length === value.length ? devices : null;
}

function normalizePortalLanCanonicalHouseholdDevice(value: unknown): PortalLanCanonicalHouseholdDevice | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    displayName?: unknown;
    networkIdentity?: unknown;
    policyTargetSurfaces?: unknown;
    roleBadges?: unknown;
    sourceLabels?: unknown;
  };

  const roleBadges = normalizeStringArray(raw.roleBadges);
  const sourceLabels = normalizeStringArray(raw.sourceLabels);
  const policyTargetSurfaces =
    raw.policyTargetSurfaces === undefined ? undefined : normalizeStringArray(raw.policyTargetSurfaces);
  const networkIdentity = normalizePortalLanCanonicalNetworkIdentity(raw.networkIdentity);
  const { displayName } = raw;

  if (
    !isString(displayName) ||
    roleBadges === null ||
    sourceLabels === null ||
    networkIdentity === null ||
    policyTargetSurfaces === null
  ) {
    return null;
  }

  const {
    displayName: _ignoredDisplayName,
    networkIdentity: _ignoredNetworkIdentity,
    policyTargetSurfaces: _ignoredPolicyTargetSurfaces,
    roleBadges: _ignoredRoleBadges,
    sourceLabels: _ignoredSourceLabels,
    ...rawWithoutNormalizedFields
  } = raw;

  return {
    ...rawWithoutNormalizedFields,
    displayName,
    roleBadges,
    sourceLabels,
    networkIdentity,
    ...(policyTargetSurfaces === undefined ? {} : { policyTargetSurfaces }),
  } as PortalLanCanonicalHouseholdDevice;
}

function normalizePortalLanCanonicalNetworkIdentity(value: unknown): PortalLanCanonicalNetworkIdentity | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    evidenceRecords?: unknown;
    ipAddresses?: unknown;
    reachability?: unknown;
  };

  const ipAddresses = normalizeStringArray(raw.ipAddresses);
  const evidenceRecords =
    raw.evidenceRecords === undefined
      ? undefined
      : normalizeRecordArray(raw.evidenceRecords, normalizePortalLanDiscoveryEvidenceRecord);
  const { reachability } = raw;
  if (ipAddresses === null || !isString(reachability) || evidenceRecords === null) {
    return null;
  }

  const { evidenceRecords: _ignoredEvidenceRecords, ...rawWithoutEvidenceRecords } = raw;
  return {
    ...rawWithoutEvidenceRecords,
    ipAddresses,
    reachability,
    evidenceRecords: evidenceRecords ?? [],
  } as PortalLanCanonicalNetworkIdentity;
}

function normalizePortalLanDiscoveryEvidenceRecord(value: unknown): PortalLanDiscoveryEvidenceRecord | null {
  if (
    !isRecord(value) ||
    !recordHasStringFields(value, PortalLanDiscoveryEvidenceStringFields) ||
    !isNullableString(value['expiresAt']) ||
    !isNullableString(value['note'])
  ) {
    return null;
  }
  return {
    ...value,
    source: value['source'],
    evidenceKind: value['evidenceKind'],
    value: value['value'],
    firstSeenAt: value['firstSeenAt'],
    lastSeenAt: value['lastSeenAt'],
    expiresAt: value['expiresAt'] ?? null,
    confidence: value['confidence'],
    note: value['note'] ?? null,
  } as PortalLanDiscoveryEvidenceRecord;
}

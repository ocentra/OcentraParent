import type {
  GeneratedPortalLanDiscoverySourceMatrixSnapshot,
  GeneratedPortalLanDiscoverySourceMatrixSourceRowSnapshot,
  GeneratedPortalLanDiscoverySourceMatrixWorkpackRowSnapshot,
  GeneratedPortalLanHouseholdDeviceDecisionSnapshot,
  GeneratedPortalLanTrustedDeviceRegistryEntrySnapshot,
} from './generated-portal-contracts';
import {
  isNullableString,
  isNumber,
  isRecord,
  isString,
  normalizeRecordArray,
  normalizeStringArray,
  recordHasNullableStringFields,
  recordHasBooleanFields,
  recordHasStringFields,
} from './live-activity-lan-add-device-shared';

export type PortalLanTrustedDeviceRegistryEntry = GeneratedPortalLanTrustedDeviceRegistryEntrySnapshot;
export type PortalLanHouseholdDeviceDecision = GeneratedPortalLanHouseholdDeviceDecisionSnapshot;
export type PortalLanPairingDeviceRef = GeneratedPortalLanTrustedDeviceRegistryEntrySnapshot['childDevice'];
export type PortalLanDiscoverySourceMatrixWorkpackRow = GeneratedPortalLanDiscoverySourceMatrixWorkpackRowSnapshot;
export type PortalLanDiscoverySourceMatrixSourceRow = GeneratedPortalLanDiscoverySourceMatrixSourceRowSnapshot;
export type PortalLanDiscoverySourceMatrix = GeneratedPortalLanDiscoverySourceMatrixSnapshot;

const PortalLanDiscoverySourceStringFields = [
  'source',
  'workpackId',
  'status',
  'authority',
  'runtimePath',
  'uiSurface',
  'evidenceLabel',
] as const;
const PortalLanDiscoverySourceBooleanFields = [
  'canConfirmChildAgent',
  'canAssignChildProfile',
  'canControlRoute',
  'requiresSelectedInterface',
  'persistsAcrossRestart',
] as const;
const PortalLanTrustedDeviceStringFields = ['pairingId', 'routeId', 'trustState', 'trustedAt', 'expiresAt'] as const;
const PortalLanDecisionStringFields = ['actionKind', 'canonicalDeviceId', 'decidedAt'] as const;
const PortalLanDecisionNullableStringFields = ['childProfileId', 'displayName', 'deviceKind', 'revokedAt'] as const;

export function normalizePortalLanDiscoverySourceMatrix(
  value: unknown
): PortalLanDiscoverySourceMatrix | null | undefined {
  if (value === null || value === undefined) {
    return null;
  }
  if (!isRecord(value)) {
    return undefined;
  }

  const raw = value as Record<string, unknown> & {
    claimsNotProved?: unknown;
    claimsProved?: unknown;
    generatedAt?: unknown;
    schemaVersion?: unknown;
    sourceRows?: unknown;
    workpackRows?: unknown;
  };
  const workpackRows = normalizeRecordArray(raw.workpackRows, normalizePortalLanDiscoveryWorkpackRow);
  const sourceRows = normalizeRecordArray(raw.sourceRows, normalizePortalLanDiscoverySourceRow);
  const claimsProved = normalizeStringArray(raw.claimsProved);
  const claimsNotProved = normalizeStringArray(raw.claimsNotProved);
  if (
    !isNumber(raw.schemaVersion) ||
    !isString(raw.generatedAt) ||
    workpackRows === null ||
    sourceRows === null ||
    claimsProved === null ||
    claimsNotProved === null
  ) {
    return undefined;
  }

  return {
    ...raw,
    schemaVersion: raw.schemaVersion,
    generatedAt: raw.generatedAt,
    workpackRows,
    sourceRows,
    claimsProved,
    claimsNotProved,
  };
}

export function normalizePortalLanTrustedDeviceRegistry(
  value: unknown
): readonly PortalLanTrustedDeviceRegistryEntry[] | null {
  return normalizeRecordArray(value, normalizePortalLanTrustedDeviceRegistryEntry);
}

export function normalizePortalLanHouseholdDeviceDecisions(
  value: unknown
): readonly PortalLanHouseholdDeviceDecision[] | null {
  return normalizeRecordArray(value, normalizePortalLanHouseholdDeviceDecision);
}

function normalizePortalLanDiscoveryWorkpackRow(value: unknown): PortalLanDiscoverySourceMatrixWorkpackRow | null {
  if (
    !isRecord(value) ||
    !recordHasStringFields(value, ['workpackId', 'title', 'discoveryState', 'proofState', 'runtimeOwner', 'status']) ||
    typeof value['readModelVisible'] !== 'boolean'
  ) {
    return null;
  }
  return value as unknown as PortalLanDiscoverySourceMatrixWorkpackRow;
}

function normalizePortalLanDiscoverySourceRow(value: unknown): PortalLanDiscoverySourceMatrixSourceRow | null {
  if (!isRecord(value) || !hasPortalLanDiscoverySourceRowFields(value)) {
    return null;
  }
  return value as unknown as PortalLanDiscoverySourceMatrixSourceRow;
}

function hasPortalLanDiscoverySourceRowFields(raw: Record<string, unknown>): boolean {
  return (
    recordHasStringFields(raw, PortalLanDiscoverySourceStringFields) &&
    recordHasBooleanFields(raw, PortalLanDiscoverySourceBooleanFields)
  );
}

function normalizePortalLanTrustedDeviceRegistryEntry(value: unknown): PortalLanTrustedDeviceRegistryEntry | null {
  if (!isRecord(value)) {
    return null;
  }
  const childDevice = normalizePortalLanDeviceRef(value['childDevice']);
  const parentDevice = normalizePortalLanDeviceRef(value['parentDevice']);
  if (
    childDevice === null ||
    parentDevice === null ||
    !recordHasStringFields(value, PortalLanTrustedDeviceStringFields) ||
    !isNullableString(value['revokedAt'])
  ) {
    return null;
  }
  return {
    ...value,
    pairingId: value['pairingId'],
    childDevice,
    parentDevice,
    routeId: value['routeId'],
    trustState: value['trustState'],
    trustedAt: value['trustedAt'],
    expiresAt: value['expiresAt'],
    revokedAt: value['revokedAt'] ?? null,
  } as PortalLanTrustedDeviceRegistryEntry;
}

function normalizePortalLanDeviceRef(value: unknown): PortalLanPairingDeviceRef | null {
  if (!isRecord(value) || !isString(value['deviceId']) || !isString(value['label']) || !isString(value['platform'])) {
    return null;
  }
  return {
    ...value,
    deviceId: value['deviceId'],
    label: value['label'],
    platform: value['platform'],
  } as PortalLanPairingDeviceRef;
}

function normalizePortalLanHouseholdDeviceDecision(value: unknown): PortalLanHouseholdDeviceDecision | null {
  if (
    !isRecord(value) ||
    !recordHasStringFields(value, PortalLanDecisionStringFields) ||
    !recordHasNullableStringFields(value, PortalLanDecisionNullableStringFields)
  ) {
    return null;
  }
  return {
    ...value,
    actionKind: value['actionKind'],
    canonicalDeviceId: value['canonicalDeviceId'],
    childProfileId: value['childProfileId'] ?? null,
    displayName: value['displayName'] ?? null,
    deviceKind: value['deviceKind'] ?? null,
    decidedAt: value['decidedAt'],
    revokedAt: value['revokedAt'] ?? null,
  } as PortalLanHouseholdDeviceDecision;
}

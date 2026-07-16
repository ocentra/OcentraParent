import type {
  GeneratedPortalLanAddDeviceScanSummarySnapshot,
  GeneratedPortalLanSelectedDeviceReadinessSnapshot,
  GeneratedPortalLanDiscoveryEventHistorySnapshot,
  GeneratedPortalLanDiscoveryEventRowSnapshot,
} from './generated-portal-contracts';
import {
  isNullableString,
  isNumber,
  isRecord,
  isString,
  notNull,
  normalizeStringArray,
  recordHasNullableStringFields,
  recordHasStringFields,
} from './live-activity-lan-add-device-shared';

export type PortalLanScanSummary = GeneratedPortalLanAddDeviceScanSummarySnapshot;
export type PortalLanSelectedDeviceReadiness = GeneratedPortalLanSelectedDeviceReadinessSnapshot;
export type PortalLanDiscoveryEventHistoryRow = GeneratedPortalLanDiscoveryEventRowSnapshot;
export type PortalLanDiscoveryEventHistory = GeneratedPortalLanDiscoveryEventHistorySnapshot;

const PortalLanDiscoveryEventHistoryRowStringFields = ['eventId', 'eventKind', 'occurredAt', 'summary'] as const;
const PortalLanDiscoveryEventHistoryRowNullableStringFields = [
  'previousEventId',
  'scanSessionId',
  'affectedDeviceId',
  'evidenceId',
] as const;

export function normalizePortalLanScanSummary(value: unknown): PortalLanScanSummary | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    agentDeviceCount?: unknown;
    infrastructureDeviceCount?: unknown;
    passiveDeviceCount?: unknown;
    scannedDeviceCount?: unknown;
    schemaVersion?: unknown;
    sourceLabels?: unknown;
    unsupportedDeviceCount?: unknown;
  };
  const {
    agentDeviceCount,
    infrastructureDeviceCount,
    passiveDeviceCount,
    scannedDeviceCount,
    schemaVersion,
    unsupportedDeviceCount,
  } = raw;

  const sourceLabels = normalizeStringArray(raw.sourceLabels);
  if (
    !isNumber(schemaVersion) ||
    !isNumber(scannedDeviceCount) ||
    !isNumber(agentDeviceCount) ||
    !isNumber(passiveDeviceCount) ||
    !isNumber(infrastructureDeviceCount) ||
    !isNumber(unsupportedDeviceCount) ||
    sourceLabels === null
  ) {
    return null;
  }

  return {
    schemaVersion,
    sourceLabels,
    scannedDeviceCount,
    agentDeviceCount,
    passiveDeviceCount,
    infrastructureDeviceCount,
    unsupportedDeviceCount,
  };
}

export function normalizePortalLanSelectedDeviceReadiness(value: unknown): PortalLanSelectedDeviceReadiness | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    offlineAt?: unknown;
    pairingId?: unknown;
    readyForControl?: unknown;
    reachability?: unknown;
    routeId?: unknown;
    schemaVersion?: unknown;
    selectedChildDeviceId?: unknown;
    staleAt?: unknown;
    trustState?: unknown;
  };
  const {
    offlineAt,
    pairingId,
    readyForControl,
    reachability,
    routeId,
    schemaVersion,
    selectedChildDeviceId,
    staleAt,
    trustState,
  } = raw;

  if (
    !isNumber(schemaVersion) ||
    !isNullableString(selectedChildDeviceId) ||
    !isNullableString(routeId) ||
    !isNullableString(pairingId) ||
    !isString(trustState) ||
    !isString(reachability) ||
    typeof readyForControl !== 'boolean' ||
    !isNullableString(staleAt) ||
    !isNullableString(offlineAt)
  ) {
    return null;
  }

  return {
    schemaVersion,
    selectedChildDeviceId,
    routeId,
    pairingId,
    trustState,
    reachability,
    readyForControl,
    staleAt,
    offlineAt,
  };
}

export function normalizePortalLanDiscoveryEventHistory(value: unknown): PortalLanDiscoveryEventHistory | null {
  if (!isRecord(value)) {
    return null;
  }
  const raw = value as Record<string, unknown> & {
    schemaVersion?: unknown;
    generatedAt?: unknown;
    state?: unknown;
    latestEventId?: unknown;
    latestObservedAt?: unknown;
    rows?: unknown;
  };
  const rows = normalizePortalLanDiscoveryEventHistoryRows(raw.rows);
  if (
    !isNumber(raw.schemaVersion) ||
    !isString(raw.generatedAt) ||
    !isString(raw.state) ||
    !isNullableString(raw.latestEventId) ||
    !isNullableString(raw.latestObservedAt) ||
    rows === null
  ) {
    return null;
  }
  return {
    schemaVersion: raw.schemaVersion,
    generatedAt: raw.generatedAt,
    state: raw.state,
    latestEventId: raw.latestEventId ?? null,
    latestObservedAt: raw.latestObservedAt ?? null,
    rows,
  };
}

export function createEmptyPortalLanDiscoveryEventHistory(): PortalLanDiscoveryEventHistory {
  return {
    schemaVersion: 1,
    generatedAt: '',
    state: 'unknown',
    latestEventId: null,
    latestObservedAt: null,
    rows: [],
  };
}

function normalizePortalLanDiscoveryEventHistoryRows(
  value: unknown
): readonly PortalLanDiscoveryEventHistoryRow[] | null {
  if (!Array.isArray(value)) {
    return null;
  }
  const rows = value.map(normalizePortalLanDiscoveryEventHistoryRow).filter(notNull);
  return rows.length === value.length ? rows : null;
}

function normalizePortalLanDiscoveryEventHistoryRow(value: unknown): PortalLanDiscoveryEventHistoryRow | null {
  if (!isRecord(value)) {
    return null;
  }
  const raw = value as Record<string, unknown> & {
    schemaVersion?: unknown;
    eventId?: unknown;
    eventKind?: unknown;
    occurredAt?: unknown;
    previousEventId?: unknown;
    scanSessionId?: unknown;
    affectedDeviceId?: unknown;
    evidenceId?: unknown;
    summary?: unknown;
  };
  const schemaVersion = raw['schemaVersion'];
  if (
    !isNumber(schemaVersion) ||
    !recordHasStringFields(raw, PortalLanDiscoveryEventHistoryRowStringFields) ||
    !recordHasNullableStringFields(raw, PortalLanDiscoveryEventHistoryRowNullableStringFields)
  ) {
    return null;
  }
  return {
    schemaVersion,
    eventId: raw['eventId'],
    eventKind: raw['eventKind'],
    occurredAt: raw['occurredAt'],
    previousEventId: raw['previousEventId'] ?? null,
    scanSessionId: raw['scanSessionId'] ?? null,
    affectedDeviceId: raw['affectedDeviceId'] ?? null,
    evidenceId: raw['evidenceId'] ?? null,
    summary: raw['summary'],
  };
}

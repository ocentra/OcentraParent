import type { AgentProtocolLogFields } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentLanBrowserAddDeviceReadModelSchema } from '@ocentra-parent/schema-domain/agent-lan-add-device';
import {
  AgentLanDiscoverySourceMatrixSchema,
  type AgentLanDiscoverySourceMatrix,
} from '@ocentra-parent/schema-domain/lan-source-matrix';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import type { StackTrace } from '@ocentra-parent/schema-domain/logging-contracts';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';

interface PortalLanScanSummary {
  readonly schemaVersion: number;
  readonly sourceLabels: readonly string[];
  readonly scannedDeviceCount: number;
  readonly agentDeviceCount: number;
  readonly passiveDeviceCount: number;
  readonly infrastructureDeviceCount: number;
  readonly unsupportedDeviceCount: number;
}

interface PortalLanSelectedDeviceReadiness {
  readonly schemaVersion: number;
  readonly selectedChildDeviceId: string | null;
  readonly routeId: string | null;
  readonly pairingId: string | null;
  readonly trustState: string;
  readonly reachability: string;
  readonly readyForControl: boolean;
  readonly staleAt: string | null;
  readonly offlineAt: string | null;
}

interface PortalLanCanonicalNetworkIdentity {
  readonly ipAddresses: readonly string[];
  readonly reachability: string;
  readonly evidenceRecords?: readonly unknown[];
  readonly [key: string]: unknown;
}

interface PortalLanCanonicalHouseholdDevice {
  readonly displayName: string;
  readonly roleBadges: readonly string[];
  readonly sourceLabels: readonly string[];
  readonly networkIdentity: PortalLanCanonicalNetworkIdentity;
  readonly [key: string]: unknown;
}

interface PortalLanSignedProofRow {
  readonly check: string;
  readonly proofState: string;
  readonly [key: string]: unknown;
}

interface PortalLanRouteSafetyRow {
  readonly check: string;
  readonly custodyLabel: string;
  readonly [key: string]: unknown;
}

interface PortalLanRelayCacheRow {
  readonly check: string;
  readonly decisionState: string;
  readonly [key: string]: unknown;
}

interface PortalLanSignedDiscoveryRelaySpine {
  readonly signedProofRows: readonly PortalLanSignedProofRow[];
  readonly routeSafetyRows: readonly PortalLanRouteSafetyRow[];
  readonly relayCacheRows: readonly PortalLanRelayCacheRow[];
  readonly [key: string]: unknown;
}

export interface PortalLanAddDeviceReadModel {
  readonly addDeviceState: string;
  readonly discoverySource: string;
  readonly localServiceDiscoveryState: string;
  readonly physicalHouseholdLanState: string;
  readonly cloudRelayState: string;
  readonly scanSummary: PortalLanScanSummary;
  readonly discoveredDevices: readonly unknown[];
  readonly canonicalHouseholdDevices: readonly PortalLanCanonicalHouseholdDevice[];
  readonly pairingRequests: readonly unknown[];
  readonly trustedDeviceRegistry: readonly unknown[];
  readonly householdDeviceDecisions: readonly unknown[];
  readonly signedDiscoveryRelaySpine: PortalLanSignedDiscoveryRelaySpine | null;
  readonly lanDiscoverySourceMatrix: AgentLanDiscoverySourceMatrix | null;
  readonly trustedDeviceIds: readonly string[];
  readonly revokedDeviceIds: readonly string[];
  readonly selectedDeviceReadiness: PortalLanSelectedDeviceReadiness;
  readonly controllerAuthority: string;
  readonly observerAuthority: string;
  readonly routeRequirementLabels: readonly string[];
  readonly auditCheckLabels: readonly string[];
  readonly honestNonClaims: readonly string[];
  readonly [key: string]: unknown;
}

type PortalLanAddDeviceRaw = Record<string, unknown> & {
  addDeviceState?: unknown;
  auditCheckLabels?: unknown;
  canonicalHouseholdDevices?: unknown;
  cloudRelayState?: unknown;
  controllerAuthority?: unknown;
  discoveredDevices?: unknown;
  discoverySource?: unknown;
  honestNonClaims?: unknown;
  householdDeviceDecisions?: unknown;
  localServiceDiscoveryState?: unknown;
  lanDiscoverySourceMatrix?: unknown;
  observerAuthority?: unknown;
  pairingRequests?: unknown;
  physicalHouseholdLanState?: unknown;
  revokedDeviceIds?: unknown;
  routeRequirementLabels?: unknown;
  scanSummary?: unknown;
  selectedDeviceReadiness?: unknown;
  signedDiscoveryRelaySpine?: unknown;
  trustedDeviceIds?: unknown;
  trustedDeviceRegistry?: unknown;
};

type PortalLanAddDeviceCollections = {
  readonly scanSummary: PortalLanScanSummary;
  readonly selectedDeviceReadiness: PortalLanSelectedDeviceReadiness;
  readonly canonicalHouseholdDevices: readonly PortalLanCanonicalHouseholdDevice[];
  readonly discoveredDevices: readonly unknown[];
  readonly pairingRequests: readonly unknown[];
  readonly trustedDeviceRegistry: readonly unknown[];
  readonly householdDeviceDecisions: readonly unknown[];
  readonly signedDiscoveryRelaySpine: PortalLanSignedDiscoveryRelaySpine | null;
  readonly lanDiscoverySourceMatrix: AgentLanDiscoverySourceMatrix | null;
  readonly trustedDeviceIds: readonly string[];
  readonly revokedDeviceIds: readonly string[];
  readonly routeRequirementLabels: readonly string[];
  readonly auditCheckLabels: readonly string[];
  readonly honestNonClaims: readonly string[];
};
type PortalLanAddDeviceCollectionsCandidate = {
  readonly scanSummary: PortalLanScanSummary | null;
  readonly selectedDeviceReadiness: PortalLanSelectedDeviceReadiness | null;
  readonly canonicalHouseholdDevices: readonly PortalLanCanonicalHouseholdDevice[] | null;
  readonly discoveredDevices: readonly unknown[] | null;
  readonly pairingRequests: readonly unknown[] | null;
  readonly trustedDeviceRegistry: readonly unknown[] | null;
  readonly householdDeviceDecisions: readonly unknown[] | null;
  readonly signedDiscoveryRelaySpine: PortalLanSignedDiscoveryRelaySpine | null;
  readonly lanDiscoverySourceMatrix: AgentLanDiscoverySourceMatrix | null | undefined;
  readonly trustedDeviceIds: readonly string[] | null;
  readonly revokedDeviceIds: readonly string[] | null;
  readonly routeRequirementLabels: readonly string[] | null;
  readonly auditCheckLabels: readonly string[] | null;
  readonly honestNonClaims: readonly string[] | null;
};

type PortalLanAddDeviceScalarFields = {
  readonly addDeviceState: string;
  readonly discoverySource: string;
  readonly localServiceDiscoveryState: string;
  readonly physicalHouseholdLanState: string;
  readonly cloudRelayState: string;
  readonly controllerAuthority: string;
  readonly observerAuthority: string;
};

const log = Logger.instance;
const moduleUrl =
  (import.meta as ImportMeta & { readonly url?: string }).url ??
  'packages/portal-domain/src/live-activity-lan-add-device.ts';
log.register(moduleUrl);

const logWarn = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false): void => {
  log.logWarn(message, stackTrace, data, enabled);
};

export function parsePortalLanAddDeviceReadModel(payload: AgentProtocolLogFields): PortalLanAddDeviceReadModel | null {
  const rawReadModel = payload[AgentProtocolDefaults.Field.LanAddDeviceReadModel];
  const readModel = parseJsonRecord(rawReadModel);
  const parsed = AgentLanBrowserAddDeviceReadModelSchema.safeParse(readModel);

  if (parsed.success) {
    return parsed.data;
  }

  const normalizedReadModel = normalizePortalLanAddDeviceReadModel(readModel);
  if (normalizedReadModel !== null) {
    logWarn(
      'portal-domain.live-activity-state.lan-add-device-read-model.recovered-from-strict-schema-mismatch',
      getStackTrace(),
      {
        issueCount: parsed.error.issues.length,
        fallback: 'portal-domain-structural-normalizer',
      }
    );
    return normalizedReadModel;
  }

  logWarn('portal-domain.live-activity-state.lan-add-device-read-model.unparsed', getStackTrace(), {
    issueCount: parsed.error.issues.length,
    fallback: 'none',
  });
  return null;
}

function parseJsonRecord(value: unknown): unknown {
  if (typeof value !== AgentProtocolDefaults.Primitive.String) {
    return value;
  }

  try {
    return JSON.parse(String(value));
  } catch {
    return null;
  }
}

function normalizePortalLanAddDeviceReadModel(value: unknown): PortalLanAddDeviceReadModel | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as PortalLanAddDeviceRaw;
  const scalarFields = normalizePortalLanAddDeviceScalarFields(raw);
  const collections = normalizePortalLanAddDeviceCollections(raw);
  if (scalarFields === null || collections === null) {
    return null;
  }

  return {
    ...raw,
    ...scalarFields,
    ...collections,
  };
}

function normalizePortalLanAddDeviceScalarFields(raw: PortalLanAddDeviceRaw): PortalLanAddDeviceScalarFields | null {
  const {
    addDeviceState,
    discoverySource,
    localServiceDiscoveryState,
    physicalHouseholdLanState,
    cloudRelayState,
    controllerAuthority,
    observerAuthority,
  } = raw;

  if (
    !isString(addDeviceState) ||
    !isString(discoverySource) ||
    !isString(localServiceDiscoveryState) ||
    !isString(physicalHouseholdLanState) ||
    !isString(cloudRelayState)
  ) {
    return null;
  }

  return {
    addDeviceState,
    discoverySource,
    localServiceDiscoveryState,
    physicalHouseholdLanState,
    cloudRelayState,
    controllerAuthority: stringOrUnknown(controllerAuthority),
    observerAuthority: stringOrUnknown(observerAuthority),
  };
}

function normalizePortalLanAddDeviceCollections(raw: PortalLanAddDeviceRaw): PortalLanAddDeviceCollections | null {
  const collections: PortalLanAddDeviceCollectionsCandidate = {
    scanSummary: normalizePortalLanScanSummary(raw.scanSummary),
    selectedDeviceReadiness: normalizePortalLanSelectedDeviceReadiness(raw.selectedDeviceReadiness),
    canonicalHouseholdDevices: normalizePortalLanCanonicalHouseholdDevices(raw.canonicalHouseholdDevices),
    discoveredDevices: normalizeUnknownArray(raw.discoveredDevices),
    pairingRequests: normalizeUnknownArray(raw.pairingRequests),
    trustedDeviceRegistry: normalizeUnknownArray(raw.trustedDeviceRegistry),
    householdDeviceDecisions: normalizeUnknownArray(raw.householdDeviceDecisions),
    signedDiscoveryRelaySpine: normalizePortalLanSignedDiscoveryRelaySpine(raw.signedDiscoveryRelaySpine),
    lanDiscoverySourceMatrix: normalizePortalLanDiscoverySourceMatrix(raw.lanDiscoverySourceMatrix),
    trustedDeviceIds: normalizeStringArray(raw.trustedDeviceIds),
    revokedDeviceIds: normalizeStringArray(raw.revokedDeviceIds),
    routeRequirementLabels: normalizeStringArray(raw.routeRequirementLabels),
    auditCheckLabels: normalizeStringArray(raw.auditCheckLabels),
    honestNonClaims: normalizeStringArray(raw.honestNonClaims),
  };

  if (!hasPortalLanAddDeviceCollections(collections)) {
    return null;
  }

  return {
    ...collections,
  };
}

function hasPortalLanAddDeviceCollections(
  collections: PortalLanAddDeviceCollectionsCandidate
): collections is PortalLanAddDeviceCollections {
  return (
    collections.lanDiscoverySourceMatrix !== undefined &&
    [
    collections.scanSummary,
    collections.selectedDeviceReadiness,
    collections.canonicalHouseholdDevices,
    collections.discoveredDevices,
    collections.pairingRequests,
    collections.trustedDeviceRegistry,
    collections.householdDeviceDecisions,
    collections.trustedDeviceIds,
    collections.revokedDeviceIds,
    collections.routeRequirementLabels,
    collections.auditCheckLabels,
    collections.honestNonClaims,
    ].every((value) => value !== null)
  );
}

function normalizePortalLanScanSummary(value: unknown): PortalLanScanSummary | null {
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

function normalizePortalLanSelectedDeviceReadiness(value: unknown): PortalLanSelectedDeviceReadiness | null {
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

function normalizePortalLanCanonicalHouseholdDevices(
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
    roleBadges?: unknown;
    sourceLabels?: unknown;
  };

  const roleBadges = normalizeStringArray(raw.roleBadges);
  const sourceLabels = normalizeStringArray(raw.sourceLabels);
  const networkIdentity = normalizePortalLanCanonicalNetworkIdentity(raw.networkIdentity);
  const { displayName } = raw;

  if (!isString(displayName) || roleBadges === null || sourceLabels === null || networkIdentity === null) {
    return null;
  }

  return {
    ...raw,
    displayName,
    roleBadges,
    sourceLabels,
    networkIdentity,
  };
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
  const evidenceRecords = raw.evidenceRecords === undefined ? undefined : normalizeUnknownArray(raw.evidenceRecords);
  const { reachability } = raw;
  if (ipAddresses === null || !isString(reachability) || evidenceRecords === null) {
    return null;
  }

  const { evidenceRecords: _ignoredEvidenceRecords, ...rawWithoutEvidenceRecords } = raw;
  return {
    ...rawWithoutEvidenceRecords,
    ipAddresses,
    reachability,
    ...(evidenceRecords === undefined ? {} : { evidenceRecords }),
  };
}

function normalizePortalLanSignedDiscoveryRelaySpine(value: unknown): PortalLanSignedDiscoveryRelaySpine | null {
  if (value === null || value === undefined) {
    return null;
  }
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    relayCacheRows?: unknown;
    routeSafetyRows?: unknown;
    signedProofRows?: unknown;
  };

  const signedProofRows = normalizePortalLanSignedProofRows(raw.signedProofRows);
  const routeSafetyRows = normalizePortalLanRouteSafetyRows(raw.routeSafetyRows);
  const relayCacheRows = normalizePortalLanRelayCacheRows(raw.relayCacheRows);
  if (signedProofRows === null || routeSafetyRows === null || relayCacheRows === null) {
    return null;
  }

  return {
    ...raw,
    signedProofRows,
    routeSafetyRows,
    relayCacheRows,
  };
}

function normalizePortalLanDiscoverySourceMatrix(
  value: unknown
): AgentLanDiscoverySourceMatrix | null | undefined {
  if (value === null || value === undefined) {
    return null;
  }

  const parsed = AgentLanDiscoverySourceMatrixSchema.safeParse(value);
  return parsed.success ? parsed.data : undefined;
}

function normalizePortalLanSignedProofRows(value: unknown): readonly PortalLanSignedProofRow[] | null {
  if (!Array.isArray(value)) {
    return null;
  }

  const rows = value.map((entry) => normalizePortalLanSignedProofRow(entry)).filter(notNull);
  return rows.length === value.length ? rows : null;
}

function normalizePortalLanSignedProofRow(value: unknown): PortalLanSignedProofRow | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    check?: unknown;
    proofState?: unknown;
  };
  const { check, proofState } = raw;
  if (!isString(check) || !isString(proofState)) {
    return null;
  }

  return {
    ...raw,
    check,
    proofState,
  };
}

function normalizePortalLanRouteSafetyRows(value: unknown): readonly PortalLanRouteSafetyRow[] | null {
  if (!Array.isArray(value)) {
    return null;
  }

  const rows = value.map((entry) => normalizePortalLanRouteSafetyRow(entry)).filter(notNull);
  return rows.length === value.length ? rows : null;
}

function normalizePortalLanRouteSafetyRow(value: unknown): PortalLanRouteSafetyRow | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    check?: unknown;
    custodyLabel?: unknown;
  };
  const { check, custodyLabel } = raw;
  if (!isString(check) || !isString(custodyLabel)) {
    return null;
  }

  return {
    ...raw,
    check,
    custodyLabel,
  };
}

function normalizePortalLanRelayCacheRows(value: unknown): readonly PortalLanRelayCacheRow[] | null {
  if (!Array.isArray(value)) {
    return null;
  }

  const rows = value.map((entry) => normalizePortalLanRelayCacheRow(entry)).filter(notNull);
  return rows.length === value.length ? rows : null;
}

function normalizePortalLanRelayCacheRow(value: unknown): PortalLanRelayCacheRow | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    check?: unknown;
    decisionState?: unknown;
  };
  const { check, decisionState } = raw;
  if (!isString(check) || !isString(decisionState)) {
    return null;
  }

  return {
    ...raw,
    check,
    decisionState,
  };
}

function normalizeUnknownArray(value: unknown): readonly unknown[] | null {
  return Array.isArray(value) ? value : null;
}

function normalizeStringArray(value: unknown): readonly string[] | null {
  if (!Array.isArray(value) || value.some((entry) => !isString(entry))) {
    return null;
  }

  return value as readonly string[];
}

function isNullableString(value: unknown): value is string | null {
  return value === null || isString(value);
}

function isNumber(value: unknown): value is number {
  return typeof value === AgentProtocolDefaults.Primitive.Number;
}

function isString(value: unknown): value is string {
  return typeof value === AgentProtocolDefaults.Primitive.String;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

function notNull<T>(value: T | null): value is T {
  return value !== null;
}

function stringOrUnknown(value: unknown): string {
  return isString(value) ? value : 'unknown';
}

import type { GeneratedPortalLanAddDeviceReadModelSnapshot } from './generated-portal-contracts';
import {
  createEmptyPortalLanDiscoveryEventHistory,
  normalizePortalLanDiscoveryEventHistory,
  normalizePortalLanScanSummary,
  normalizePortalLanSelectedDeviceReadiness,
} from './live-activity-lan-add-device-summary';
import { normalizePortalLanCanonicalHouseholdDevices } from './live-activity-lan-add-device-device';
import {
  normalizePortalLanDiscoverySourceMatrix,
  normalizePortalLanHouseholdDeviceDecisions,
  normalizePortalLanTrustedDeviceRegistry,
} from './live-activity-lan-add-device-registry';
import { normalizePortalLanSignedDiscoveryRelaySpine } from './live-activity-lan-add-device-spine';
import {
  isNumber,
  isRecord,
  isString,
  normalizeGeneratedArray,
  normalizeStringArray,
} from './live-activity-lan-add-device-shared';

export type PortalLanAddDeviceReadModel = GeneratedPortalLanAddDeviceReadModelSnapshot;

type PortalLanAddDeviceCollectionKey =
  | 'scanSummary'
  | 'selectedDeviceReadiness'
  | 'canonicalHouseholdDevices'
  | 'discoveredDevices'
  | 'discoveryEventHistory'
  | 'pairingRequests'
  | 'trustedDeviceRegistry'
  | 'householdDeviceDecisions'
  | 'signedDiscoveryRelaySpine'
  | 'lanDiscoverySourceMatrix'
  | 'trustedDeviceIds'
  | 'revokedDeviceIds'
  | 'routeRequirementLabels'
  | 'auditCheckLabels'
  | 'honestNonClaims';
type PortalLanAddDeviceCollections = Pick<PortalLanAddDeviceReadModel, PortalLanAddDeviceCollectionKey>;
type PortalLanAddDeviceCollectionsCandidate = {
  readonly [K in PortalLanAddDeviceCollectionKey]: PortalLanAddDeviceReadModel[K] | null | undefined;
};
type PortalLanAddDeviceScalarKey =
  | 'addDeviceState'
  | 'discoverySource'
  | 'localServiceDiscoveryState'
  | 'physicalHouseholdLanState'
  | 'cloudRelayState'
  | 'controllerAuthority'
  | 'observerAuthority';
type PortalLanAddDeviceScalarFields = Pick<PortalLanAddDeviceReadModel, PortalLanAddDeviceScalarKey>;
type PortalLanAddDeviceRaw = Record<string, unknown> &
  Partial<Record<PortalLanAddDeviceCollectionKey | PortalLanAddDeviceScalarKey, unknown>>;

export function normalizePortalLanAddDeviceReadModel(value: unknown): PortalLanAddDeviceReadModel | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as PortalLanAddDeviceRaw;
  const { schemaVersion, generatedAt } = raw;
  const scalarFields = normalizePortalLanAddDeviceScalarFields(raw);
  const collections = normalizePortalLanAddDeviceCollections(raw);
  if (!isNumber(schemaVersion) || !isString(generatedAt) || scalarFields === null || collections === null) {
    return null;
  }

  return {
    ...raw,
    schemaVersion,
    generatedAt,
    ...scalarFields,
    ...collections,
  } as PortalLanAddDeviceReadModel;
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
    controllerAuthority: isString(controllerAuthority) ? controllerAuthority : 'unknown',
    observerAuthority: isString(observerAuthority) ? observerAuthority : 'unknown',
  };
}

function normalizePortalLanAddDeviceCollections(raw: PortalLanAddDeviceRaw): PortalLanAddDeviceCollections | null {
  const collections: PortalLanAddDeviceCollectionsCandidate = {
    scanSummary: normalizePortalLanScanSummary(raw.scanSummary),
    selectedDeviceReadiness: normalizePortalLanSelectedDeviceReadiness(raw.selectedDeviceReadiness),
    canonicalHouseholdDevices: normalizePortalLanCanonicalHouseholdDevices(raw.canonicalHouseholdDevices),
    discoveredDevices: normalizeGeneratedArray<PortalLanAddDeviceReadModel['discoveredDevices'][number]>(
      raw.discoveredDevices
    ),
    discoveryEventHistory:
      raw.discoveryEventHistory === undefined
        ? createEmptyPortalLanDiscoveryEventHistory()
        : normalizePortalLanDiscoveryEventHistory(raw.discoveryEventHistory),
    pairingRequests: normalizeGeneratedArray<PortalLanAddDeviceReadModel['pairingRequests'][number]>(
      raw.pairingRequests
    ),
    trustedDeviceRegistry: normalizePortalLanTrustedDeviceRegistry(raw.trustedDeviceRegistry),
    householdDeviceDecisions: normalizePortalLanHouseholdDeviceDecisions(raw.householdDeviceDecisions),
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
    signedDiscoveryRelaySpine: collections.signedDiscoveryRelaySpine ?? null,
    lanDiscoverySourceMatrix: collections.lanDiscoverySourceMatrix ?? null,
  } as PortalLanAddDeviceCollections;
}

function hasPortalLanAddDeviceCollections(collections: PortalLanAddDeviceCollectionsCandidate): boolean {
  return (
    collections.lanDiscoverySourceMatrix !== undefined &&
    [
      collections.scanSummary,
      collections.selectedDeviceReadiness,
      collections.canonicalHouseholdDevices,
      collections.discoveredDevices,
      collections.discoveryEventHistory,
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

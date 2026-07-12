import { normalizeLanEvidenceRecords } from './activity-ui-lan-pairing-fixtures-evidence';
import { inferLanEvidenceSource } from './activity-ui-lan-pairing-fixtures-source';

const LanDiscoveryStatusFallback = 'network-neighbor';
const LanDiscoveryStatusValues = new Set(['planned-unsupported', 'websocket-direct', 'network-neighbor']);
const LanDiscoveryStatusAliases = new Map([
  ['mdns-passive', LanDiscoveryStatusFallback],
  ['trusted-registry', LanDiscoveryStatusFallback],
]);

const LanSourceLabelAliases = new Map([
  ['gateway', 'network-neighbor'],
  ['mdns', 'network-neighbor'],
]);

const LanConfidenceValues = new Set(['agent-confirmed', 'mac-ip-match', 'network-neighbor', 'manual-required']);
const LanConfidenceFallback = 'network-neighbor';
const LanRouteStateValues = new Set(['localhost', 'local-network', 'manual-required', 'unavailable']);
const LanRouteStateFallback = 'unavailable';
const LanRoleBadgeValues = new Set(['parent-controller', 'parent-observer', 'child-agent', 'portal', 'ai-provider']);

function nonEmptyLanString(value: unknown): string | null {
  return typeof value === 'string' && value.length > 0 ? value : null;
}

function normalizeLanDiscoveryStatus(value: unknown): string {
  return typeof value === 'string' && LanDiscoveryStatusValues.has(value)
    ? value
    : (LanDiscoveryStatusAliases.get(String(value)) ?? LanDiscoveryStatusFallback);
}

function normalizeLanSourceLabels(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.flatMap((entry) => {
    if (entry === 'local-service' || entry === 'network-neighbor' || entry === 'trusted-registry') {
      return [entry];
    }
    return LanSourceLabelAliases.has(String(entry))
      ? [LanSourceLabelAliases.get(String(entry)) ?? 'network-neighbor']
      : [];
  });
}

function normalizeLanConfidence(value: unknown): string {
  return typeof value === 'string' && LanConfidenceValues.has(value) ? value : LanConfidenceFallback;
}

function normalizeLanRouteState(value: unknown): string {
  return typeof value === 'string' && LanRouteStateValues.has(value) ? value : LanRouteStateFallback;
}

function normalizeLanRoleBadges(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.filter((entry): entry is string => typeof entry === 'string' && LanRoleBadgeValues.has(entry));
}

function normalizeLanChildAgentInventory(value: unknown) {
  if (typeof value !== 'object' || value === null) {
    return value;
  }
  const inventory = { ...(value as Record<string, unknown>) };
  inventory['routeState'] = normalizeLanRouteState(inventory['routeState']);
  return inventory;
}

function normalizeLanAddDeviceFixture(value: unknown) {
  const fixture = structuredClone(value) as {
    generatedAt?: string;
    discoveryEventHistory?: unknown;
    discoveredDevices?: Array<Record<string, unknown>>;
    canonicalHouseholdDevices?: Array<Record<string, unknown>>;
  };
  if (fixture.discoveryEventHistory === undefined) {
    fixture.discoveryEventHistory = {
      schemaVersion: 1,
      generatedAt: fixture.generatedAt ?? null,
      state: 'ready',
      latestEventId: null,
      latestObservedAt: null,
      rows: [],
    };
  }
  if (Array.isArray(fixture.discoveredDevices)) {
    fixture.discoveredDevices = fixture.discoveredDevices.map((device) => ({
      ...device,
      discoveryStatus: normalizeLanDiscoveryStatus(device['discoveryStatus']),
    }));
  }
  if (Array.isArray(fixture.canonicalHouseholdDevices)) {
    fixture.canonicalHouseholdDevices = fixture.canonicalHouseholdDevices.map((device) => {
      const sourceLabels = normalizeLanSourceLabels(device['sourceLabels']);
      const networkIdentity =
        typeof device['networkIdentity'] === 'object' && device['networkIdentity'] !== null
          ? { ...(device['networkIdentity'] as Record<string, unknown>) }
          : {};
      networkIdentity['evidenceRecords'] = normalizeLanEvidenceRecords(
        networkIdentity['evidenceRecords'],
        device['canonicalDeviceId'],
        inferLanEvidenceSource(sourceLabels)
      );
      networkIdentity['confidence'] = normalizeLanConfidence(networkIdentity['confidence']);
      return {
        ...device,
        roleBadges: normalizeLanRoleBadges(device['roleBadges']),
        sourceLabels,
        routeState: normalizeLanRouteState(device['routeState']),
        childAgentInventory: normalizeLanChildAgentInventory(device['childAgentInventory']),
        networkIdentity,
      };
    });
  }
  return fixture;
}

export {
  inferLanEvidenceSource,
  normalizeLanAddDeviceFixture,
  normalizeLanChildAgentInventory,
  normalizeLanConfidence,
  normalizeLanDiscoveryStatus,
  normalizeLanRoleBadges,
  normalizeLanRouteState,
  normalizeLanSourceLabels,
  nonEmptyLanString,
};

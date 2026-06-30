import { readFileSync } from 'node:fs';
import {
  AgentLanBrowserAddDeviceReadModelSchema,
  type AgentLanBrowserAddDeviceReadModel,
} from '@ocentra-parent/schema-domain/agent-lan-add-device';

const LanPlanFixtureDirectory = new URL('./lan-plan/', import.meta.url);

function readLanAddDeviceFixture(fileName: string): AgentLanBrowserAddDeviceReadModel {
  const fixture = JSON.parse(readFileSync(new URL(fileName, LanPlanFixtureDirectory), 'utf8'));
  return AgentLanBrowserAddDeviceReadModelSchema.parse(
    normalizeLanAddDeviceFixture(fixture)
  );
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

function lanFixtureEvidenceRecord(canonicalDeviceId: unknown, source: string) {
  const deviceId =
    typeof canonicalDeviceId === 'string' && canonicalDeviceId.length > 0
      ? canonicalDeviceId
      : 'lan-fixture-device';
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

function inferLanEvidenceSource(sourceLabels: readonly string[]): string {
  if (sourceLabels.includes('local-service')) {
    return 'local-service';
  }
  if (sourceLabels.includes('trusted-registry')) {
    return 'trusted-registry';
  }
  return 'windows-neighbor-table';
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

function normalizeLanEvidenceSource(value: unknown, fallbackSource: string): string {
  return typeof value === 'string' && LanEvidenceSourceValues.has(value)
    ? value
    : LanEvidenceSourceAliases.get(String(value)) ?? fallbackSource;
}

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

function normalizeLanEvidenceKind(value: unknown, source: string): string {
  return typeof value === 'string' && LanEvidenceKindValues.has(value)
    ? value
    : LanEvidenceKindBySource.get(source) ?? 'ip-address';
}

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

function normalizeLanEvidenceConfidence(value: unknown, source: string): string {
  return typeof value === 'string' && LanEvidenceConfidenceValues.has(value)
    ? value
    : LanEvidenceConfidenceAliases.get(String(value)) ?? LanEvidenceConfidenceBySource.get(source) ?? 'weak';
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

function normalizeLanDiscoveryStatus(value: unknown): string {
  switch (value) {
    case 'planned-unsupported':
    case 'websocket-direct':
    case 'network-neighbor':
      return value;
    case 'mdns-passive':
    case 'trusted-registry':
    default:
      return 'network-neighbor';
  }
}

function normalizeLanSourceLabels(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.flatMap((entry) => {
    switch (entry) {
      case 'local-service':
      case 'network-neighbor':
      case 'trusted-registry':
        return [entry];
      case 'gateway':
      case 'mdns':
        return ['network-neighbor'];
      default:
        return [];
    }
  });
}

function normalizeLanConfidence(value: unknown): string {
  switch (value) {
    case 'agent-confirmed':
    case 'mac-ip-match':
    case 'network-neighbor':
    case 'manual-required':
      return value;
    case 'mdns-advertisement':
    default:
      return 'network-neighbor';
  }
}

function normalizeLanRouteState(value: unknown): string {
  switch (value) {
    case 'localhost':
    case 'local-network':
    case 'manual-required':
    case 'unavailable':
      return value;
    default:
      return 'unavailable';
  }
}

function normalizeLanRoleBadges(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.filter((entry): entry is string =>
    entry === 'parent-controller' ||
    entry === 'parent-observer' ||
    entry === 'child-agent' ||
    entry === 'portal' ||
    entry === 'ai-provider'
  );
}

function normalizeLanChildAgentInventory(value: unknown) {
  if (typeof value !== 'object' || value === null) {
    return value;
  }
  const inventory = { ...(value as Record<string, unknown>) };
  inventory['routeState'] = normalizeLanRouteState(inventory['routeState']);
  return inventory;
}

function nonEmptyLanString(value: unknown): string | null {
  return typeof value === 'string' && value.length > 0 ? value : null;
}

function requiredCanonicalHouseholdDevice(canonicalDeviceId: string) {
  const device = canonicalRuntimeLanAddDeviceReadModel().canonicalHouseholdDevices.find(
    (candidate) => candidate.canonicalDeviceId === canonicalDeviceId
  );
  if (device === undefined) {
    throw new Error(`Missing canonical LAN device fixture for ${canonicalDeviceId}`);
  }
  return device;
}

export function lanAddDeviceReadModel() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-01T15:01:00Z',
    discoverySource: 'local-service',
    addDeviceState: 'paired',
    localServiceDiscoveryState: 'paired',
    physicalHouseholdLanState: 'manual-required',
    cloudRelayState: 'unavailable',
    scanSummary: {
      schemaVersion: 1,
      sourceLabels: ['local-service'],
      scannedDeviceCount: 2,
      agentDeviceCount: 1,
      passiveDeviceCount: 0,
      infrastructureDeviceCount: 0,
      unsupportedDeviceCount: 1,
    },
    discoveredDevices: [connectedLanDiscoveryDevice(), manualLanDiscoveryDevice()],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    householdDeviceDecisions: [],
    trustedDeviceIds: ['child-android-1'],
    revokedDeviceIds: [],
    selectedDeviceReadiness: connectedLanSelectedDeviceReadiness(),
    controllerAuthority: 'observer',
    observerAuthority: 'observer',
    routeRequirementLabels: ['Local service route only'],
    auditCheckLabels: ['No physical device-owner proof'],
    honestNonClaims: ['physical-device-owner-unavailable'],
  } as const;
}

function connectedLanDiscoveryDevice() {
  return {
    schemaVersion: 1,
    discoveredAt: '2026-06-01T15:00:00Z',
    childDevice: {
      deviceId: 'child-android-1',
      childProfileId: 'child-profile-1',
      label: 'Pixel child',
      platform: 'android',
      ipAddress: '192.168.2.42',
      macAddress: '54-27-1e-97-c3-31',
      hostname: 'pixel-child',
      networkInterface: 'Ethernet 2',
      agentStatus: 'ocentra-child-agent',
      hardwareProfile: connectedLanHardwareProfile(),
    },
    agentPeerId: 'child-peer-1',
    routeId: 'lan-route-local-1',
    networkMode: 'local-network',
    reachability: 'online',
    addressRef: 'lan-address-ref-1',
    discoveryStatus: 'websocket-direct',
    discoveryState: 'paired',
  } as const;
}

function connectedLanHardwareProfile() {
  return {
    manufacturer: 'Google',
    model: 'Pixel test',
    cpuModel: 'Tensor test',
    cpuCores: '8 cores',
    memoryTotal: '8 GiB',
    gpuModel: 'Mali test',
    gpuDriver: 'driver-test',
    gpuMemory: 'shared',
    nvidiaSmi: null,
  } as const;
}

export function runtimeLanAddDeviceReadModel() {
  return {
    ...lanAddDeviceReadModel(),
    discoverySource: 'physical-household-lan',
    physicalHouseholdLanState: 'discovered',
    scanSummary: {
      schemaVersion: 1,
      sourceLabels: ['local-service', 'windows-neighbor-table'],
      scannedDeviceCount: 3,
      agentDeviceCount: 1,
      passiveDeviceCount: 1,
      infrastructureDeviceCount: 1,
      unsupportedDeviceCount: 2,
    },
    discoveredDevices: [
      localAgentRuntimeDiscoveryDevice(),
      networkNeighborRuntimeDiscoveryDevice(),
      routerRuntimeDiscoveryDevice(),
    ],
    selectedDeviceReadiness: {
      schemaVersion: 1,
      selectedChildDeviceId: 'local-dev-agent',
      routeId: 'lan-route-local-network',
      pairingId: null,
      trustState: 'unpaired',
      reachability: 'online',
      readyForControl: false,
      staleAt: null,
      offlineAt: null,
    },
  } as const;
}

export function canonicalRuntimeLanAddDeviceReadModel() {
  return readLanAddDeviceFixture('confirmed_windows_child.json');
}

export function lanNeighborHouseholdDecision() {
  return {
    schemaVersion: 1,
    actionId: 'lan-action-rename-hpsujan',
    actionKind: 'rename',
    canonicalDeviceId: 'lan-physical-mac-54271e97c331',
    childProfileId: null,
    displayName: 'Kitchen laptop',
    deviceKind: 'laptop',
    parentActorId: 'parent-actor-1',
    decidedAt: '2026-06-01T15:22:05Z',
    revokedAt: null,
  } as const;
}

export function localAgentCanonicalHouseholdDevice() {
  return requiredCanonicalHouseholdDevice('lan-physical-mac-b42e993e72b9');
}

export function lanNeighborCanonicalHouseholdDevice() {
  return requiredCanonicalHouseholdDevice('lan-physical-mac-54271e97c331');
}

export function routerCanonicalHouseholdDevice() {
  return requiredCanonicalHouseholdDevice('lan-physical-mac-001122334455');
}

function localAgentRuntimeDiscoveryDevice() {
  return {
    schemaVersion: 1,
    discoveredAt: '2026-06-01T15:20:00Z',
    childDevice: {
      deviceId: 'local-dev-agent',
      childProfileId: null,
      label: 'local-dev-agent',
      platform: 'windows',
      ipAddress: '192.168.2.10',
      macAddress: 'b4-2e-99-3e-72-b9',
      hostname: 'GAMEDEV',
      networkInterface: 'Ethernet 2',
      agentStatus: 'ocentra-local-service',
      hardwareProfile: localAgentRuntimeHardwareProfile(),
    },
    agentPeerId: 'portal-dev',
    routeId: 'lan-route-local-network',
    networkMode: 'local-network',
    reachability: 'online',
    addressRef: 'lan-address-ref-direct-websocket',
    discoveryStatus: 'websocket-direct',
    discoveryState: 'discovered',
  } as const;
}

function localAgentRuntimeHardwareProfile() {
  return {
    manufacturer: 'Gigabyte Technology Co., Ltd.',
    model: 'X570 AORUS MASTER',
    cpuModel: 'AMD Ryzen 9 3900X 12-Core Processor',
    cpuCores: '12 cores / 24 logical',
    memoryTotal: '63 GiB',
    gpuModel: 'GeForce RTX 2070 SUPER',
    gpuDriver: '456.71',
    gpuMemory: '8192 MiB',
    nvidiaSmi: 'GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM',
  } as const;
}

function networkNeighborRuntimeDiscoveryDevice() {
  return {
    schemaVersion: 1,
    discoveredAt: '2026-06-01T15:20:02Z',
    childDevice: {
      deviceId: 'lan-device-54271e97c331',
      childProfileId: null,
      label: 'LAN 192.168.2.42',
      platform: 'unknown',
      ipAddress: '192.168.2.42',
      macAddress: '54-27-1e-97-c3-31',
      hostname: 'unknown-host',
      networkInterface: 'Ethernet 2',
      agentStatus: null,
      hardwareProfile: null,
    },
    agentPeerId: 'portal-dev',
    routeId: 'lan-route-local-network',
    networkMode: 'local-network',
    reachability: 'online',
    addressRef: 'lan-address-ref-network-neighbor',
    discoveryStatus: 'network-neighbor',
    discoveryState: 'discovered',
  } as const;
}

function routerRuntimeDiscoveryDevice() {
  return {
    schemaVersion: 1,
    discoveredAt: '2026-06-01T15:20:03Z',
    childDevice: {
      deviceId: 'lan-device-001122334455',
      childProfileId: null,
      label: 'LAN 192.168.2.1',
      platform: 'router',
      ipAddress: '192.168.2.1',
      macAddress: '00-11-22-33-44-55',
      hostname: 'unknown-host',
      networkInterface: 'Gateway',
      agentStatus: null,
      hardwareProfile: null,
    },
    agentPeerId: 'portal-dev',
    routeId: 'lan-route-local-network',
    networkMode: 'local-network',
    reachability: 'online',
    addressRef: 'lan-address-ref-network-neighbor',
    discoveryStatus: 'network-neighbor',
    discoveryState: 'discovered',
  } as const;
}

function manualLanDiscoveryDevice() {
  return {
    schemaVersion: 1,
    discoveredAt: '2026-06-01T15:00:02Z',
    childDevice: {
      deviceId: 'child-android-2',
      childProfileId: 'child-profile-2',
      label: 'Android manual',
      platform: 'android',
    },
    agentPeerId: 'child-peer-2',
    routeId: 'lan-route-manual-1',
    networkMode: 'local-network',
    reachability: 'stale',
    addressRef: 'lan-address-ref-2',
    discoveryStatus: 'planned-unsupported',
    discoveryState: 'manual-required',
  } as const;
}

function connectedLanSelectedDeviceReadiness() {
  return {
    schemaVersion: 1,
    selectedChildDeviceId: 'child-android-1',
    routeId: 'lan-route-local-1',
    pairingId: 'pairing-child-android-1',
    trustState: 'paired',
    reachability: 'online',
    readyForControl: true,
    staleAt: null,
    offlineAt: null,
  } as const;
}

export function emptyLanAddDeviceReadModel(addDeviceState: string) {
  return {
    ...readLanAddDeviceFixture('empty_devices.json'),
    addDeviceState,
    localServiceDiscoveryState: addDeviceState,
  } as const;
}

export function confirmedWindowsChildLanAddDeviceReadModel() {
  return readLanAddDeviceFixture('confirmed_windows_child.json');
}

export function unknownAppleDeviceLanAddDeviceReadModel() {
  return readLanAddDeviceFixture('unknown_apple_device.json');
}

export function sameIpDifferentMacLanAddDeviceReadModel() {
  return readLanAddDeviceFixture('same_ip_different_mac.json');
}

export function sameDeviceNewIpLanAddDeviceReadModel() {
  return readLanAddDeviceFixture('same_device_new_ip.json');
}

export function longHostnameLanAddDeviceReadModel() {
  return readLanAddDeviceFixture('long_hostname_device.json');
}

export function htmlInHostnameLanAddDeviceReadModel() {
  return readLanAddDeviceFixture('html_in_hostname_device.json');
}

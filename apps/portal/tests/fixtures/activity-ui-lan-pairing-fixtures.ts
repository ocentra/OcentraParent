import { readFileSync } from 'node:fs';
import type { ParentLanAddDeviceReadModelSnapshot } from '../../generated/parent-ui-bridge';
import { normalizeLanAddDeviceFixture } from './activity-ui-lan-pairing-fixtures-normalization';

const LanPlanFixtureDirectory = new URL('./lan-plan/', import.meta.url);

function readLanAddDeviceFixture(fileName: string): ParentLanAddDeviceReadModelSnapshot {
  const fixture = JSON.parse(readFileSync(new URL(fileName, LanPlanFixtureDirectory), 'utf8'));
  return requireLanAddDeviceFixture(normalizeLanAddDeviceFixture(fixture), fileName);
}

function requireLanAddDeviceFixture(value: unknown, fileName: string): ParentLanAddDeviceReadModelSnapshot {
  if (!isLanFixtureRecord(value) || !hasLanFixtureRequiredFields(value)) {
    throw new Error(`LAN add-device fixture ${fileName} does not match generated parent bridge shape`);
  }
  return value as unknown as ParentLanAddDeviceReadModelSnapshot;
}

function isLanFixtureRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

function hasLanFixtureRequiredFields(value: Record<string, unknown>): boolean {
  return (
    typeof value['schemaVersion'] === 'number' &&
    typeof value['generatedAt'] === 'string' &&
    typeof value['addDeviceState'] === 'string' &&
    typeof value['discoverySource'] === 'string' &&
    ['scanSummary', 'discoveryEventHistory', 'selectedDeviceReadiness'].every((field) =>
      isLanFixtureRecord(value[field])
    ) &&
    [
      'discoveredDevices',
      'canonicalHouseholdDevices',
      'pairingRequests',
      'trustedDeviceRegistry',
      'householdDeviceDecisions',
      'trustedDeviceIds',
      'revokedDeviceIds',
    ].every((field) => Array.isArray(value[field]))
  );
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
      childProfileId: null,
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

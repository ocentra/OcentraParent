import { describe, expect, it } from 'vitest';
import { LanBrowserAddDeviceReadModelSchema, LanPairingProductionDiscoveryStates } from '../src/lan-pairing';

const generatedAt = '2026-06-01T15:20:00.000Z';

describe('browser-first LAN add-device read model', () => {
  it('parses service-backed local discovery with honest physical LAN boundaries', () => {
    const parsed = LanBrowserAddDeviceReadModelSchema.parse(readModelFixture());

    expect(parsed.discoverySource).toBe('local-service');
    expect(parsed.physicalHouseholdLanState).toBe('manual-required');
    expect(parsed.cloudRelayState).toBe('unavailable');
    expect(parsed.trustedDeviceRegistry[0]?.childDevice.deviceId).toBe('child-device-1');
    expect(parsed.canonicalHouseholdDevices).toHaveLength(2);
    expect(parsed.canonicalHouseholdDevices[0]?.displayName).toBe('GAMEDEV');
    expect(parsed.canonicalHouseholdDevices[0]?.roleBadges).toEqual([
      'child-agent',
      'portal',
      'parent-controller',
    ]);
    expect(parsed.trustedDeviceIds).toEqual(['child-device-1']);
    expect(parsed.selectedDeviceReadiness.readyForControl).toBe(false);
    expect(parsed.auditCheckLabels).toEqual(['wrong-origin', 'wrong-device', 'replayed', 'stale', 'revoked']);
    expect(LanPairingProductionDiscoveryStates.Rejected).toBe('rejected');
    expect(LanPairingProductionDiscoveryStates.Expired).toBe('expired');
    expect(LanPairingProductionDiscoveryStates.ManualRequired).toBe('manual-required');
  });
});

function readModelFixture() {
  return {
    schemaVersion: 'v0.9',
    generatedAt,
    discoverySource: 'local-service',
    addDeviceState: 'pending',
    localServiceDiscoveryState: 'pending',
    physicalHouseholdLanState: 'manual-required',
    cloudRelayState: 'unavailable',
    discoveredDevices: [discoveredDevice()],
    canonicalHouseholdDevices: [canonicalHouseholdDeviceRef(), routerDevice()],
    pairingRequests: [pairingRequest()],
    trustedDeviceRegistry: [trustedRegistryEntry()],
    trustedDeviceIds: ['child-device-1'],
    revokedDeviceIds: [],
    selectedDeviceReadiness: selectedReadiness(),
    controllerAuthority: 'active-controller',
    observerAuthority: 'observer',
    routeRequirementLabels: ['allowed-origin', 'target-device-match', 'non-replayed-intent'],
    auditCheckLabels: ['wrong-origin', 'wrong-device', 'replayed', 'stale', 'revoked'],
    honestNonClaims: ['physical-household-lan-manual-required', 'cloud-relay-not-implemented'],
  };
}

function discoveredDevice() {
  return {
    schemaVersion: 'v0.9',
    discoveredAt: generatedAt,
    childProfile: { childProfileId: 'child-profile-1', displayName: 'Mia' },
    childDevice: childDeviceRef(),
    agentPeerId: 'child-agent-1',
    routeId: 'lan-route-child-1',
    networkMode: 'local-network',
    reachability: 'stale',
    addressRef: 'local-service:child-device-1',
    discoveryStatus: 'websocket-direct',
    discoveryState: 'stale',
  };
}

function pairingRequest() {
  return {
    schemaVersion: 'v0.9',
    challengeId: 'challenge-child-device-1-parent-peer-1',
    childDeviceId: 'child-device-1',
    parentDeviceId: 'parent-device-1',
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    pairingState: 'pending',
    rejectionReason: null,
    issuedAt: generatedAt,
    expiresAt: '2026-06-01T15:25:00.000Z',
  };
}

function selectedReadiness() {
  return {
    schemaVersion: 'v0.9',
    selectedChildDeviceId: null,
    routeId: null,
    pairingId: null,
    trustState: 'unpaired',
    reachability: 'offline',
    readyForControl: false,
    staleAt: null,
    offlineAt: null,
  };
}

function childDeviceRef() {
  return {
    deviceId: 'child-device-1',
    childProfileId: 'child-profile-1',
    label: 'Mia Windows PC',
    platform: 'windows',
    ipAddress: '192.168.2.42',
    macAddress: '54-27-1e-97-c3-31',
    hostname: 'GAMEDEV',
    networkInterface: 'Ethernet 2',
    agentStatus: 'ocentra-local-service',
    hardwareProfile: {
      manufacturer: 'Gigabyte Technology Co., Ltd.',
      model: 'X570 AORUS MASTER',
      cpuModel: 'AMD Ryzen 9 3900X 12-Core Processor',
      cpuCores: '12 cores / 24 logical',
      memoryTotal: '63 GiB',
      gpuModel: 'GeForce RTX 2070 SUPER',
      gpuDriver: '456.71',
      gpuMemory: '8192 MiB',
      nvidiaSmi: 'GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM',
    },
  };
}

function parentDeviceRef() {
  return {
    deviceId: 'parent-device-1',
    childProfileId: null,
    label: 'Parent Windows PC',
    platform: 'windows',
  };
}

function canonicalHouseholdDeviceRef() {
  return {
    schemaVersion: 'v0.9',
    canonicalDeviceId: 'child-device-1',
    displayName: 'GAMEDEV',
    classification: 'child-agent',
    roleBadges: ['child-agent', 'portal', 'parent-controller'],
    enrollable: true,
    discoveryState: 'paired',
    trustState: 'paired',
    routeId: 'lan-route-local-network',
    routeState: 'local-network',
    networkMode: 'local-network',
    sourceLabels: ['local-service', 'network-neighbor', 'trusted-registry'],
    networkIdentity: {
      hostname: 'GAMEDEV',
      ipAddresses: ['192.168.2.42'],
      macAddress: '54-27-1e-97-c3-31',
      macVendor: null,
      networkInterfaces: ['Ethernet 2'],
      reachability: 'online',
      confidence: 'mac-ip-match',
      staleAt: null,
      offlineAt: null,
    },
    childAgentInventory: {
      deviceName: 'GAMEDEV',
      platform: 'windows',
      os: 'windows',
      cpuModel: 'AMD Ryzen 9 3900X 12-Core Processor',
      cpuCores: '12 cores / 24 logical',
      memoryTotal: '63 GiB',
      gpuModel: 'GeForce RTX 2070 SUPER',
      gpuDriver: '456.71',
      gpuMemory: '8192 MiB',
      nvidiaSmi: 'GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM',
      networkInterfaces: ['Ethernet 2'],
      capabilities: ['direct-websocket', 'device-inventory', 'pairing-route'],
      roleState: 'implemented',
      routeState: 'local-network',
      pairingTrustState: 'paired',
    },
    policyTargetSurfaces: ['devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking', 'ai'],
  };
}

function trustedRegistryEntry() {
  return {
    schemaVersion: 'v0.9',
    pairingId: 'pairing-child-device-1',
    childDevice: {
      deviceId: 'child-device-1',
      childProfileId: null,
      label: 'Mia Windows PC',
      platform: 'windows',
    },
    parentDevice: parentDeviceRef(),
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    proofDigest: 'sha256:lan-proof',
    trustState: 'paired',
    trustedAt: generatedAt,
    expiresAt: '2026-06-01T16:20:00.000Z',
    revokedAt: null,
  };
}

function routerDevice() {
  return {
    schemaVersion: 'v0.9',
    canonicalDeviceId: 'lan-physical-mac-001122334455',
    displayName: 'LAN 192.168.2.1',
    classification: 'network-infrastructure',
    roleBadges: [],
    enrollable: false,
    discoveryState: 'discovered',
    trustState: 'unpaired',
    routeId: null,
    routeState: 'unavailable',
    networkMode: 'local-network',
    sourceLabels: ['network-neighbor'],
    networkIdentity: {
      hostname: null,
      ipAddresses: ['192.168.2.1'],
      macAddress: '00-11-22-33-44-55',
      macVendor: null,
      networkInterfaces: ['Ethernet 2'],
      reachability: 'online',
      confidence: 'network-neighbor',
      staleAt: null,
      offlineAt: null,
    },
    childAgentInventory: null,
    policyTargetSurfaces: ['devices', 'network'],
  } as const;
}

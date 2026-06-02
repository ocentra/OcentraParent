import { describe, expect, it } from 'vitest';
import { AgentLanBrowserAddDeviceReadModelSchema, AgentProtocolDefaults } from '../src/contracts';

describe('agent protocol browser-first LAN add-device state', () => {
  it('parses the service event read model D can consume without portal fixtures', () => {
    const parsed = AgentLanBrowserAddDeviceReadModelSchema.parse(lanAddDeviceReadModelFixture());

    expect(parsed.discoverySource).toBe('local-service');
    expect(parsed.addDeviceState).toBe('pending');
    expect(parsed.physicalHouseholdLanState).toBe('discovered');
    expect(parsed.cloudRelayState).toBe('unavailable');
    expect(parsed.discoveredDevices[0]?.childDevice.hardwareProfile?.cpuModel).toContain('Ryzen');
    expect(parsed.discoveredDevices[1]?.childDevice.ipAddress).toBe('192.168.2.42');
    expect(parsed.discoveredDevices[1]?.discoveryStatus).toBe('network-neighbor');
    expect(parsed.canonicalHouseholdDevices).toHaveLength(2);
    expect(parsed.canonicalHouseholdDevices[0]?.canonicalDeviceId).toBe('child-device-1');
    expect(parsed.canonicalHouseholdDevices[0]?.displayName).toBe('GAMEDEV');
    expect(parsed.canonicalHouseholdDevices[1]?.classification).toBe('network-infrastructure');
    expect(parsed.trustedDeviceRegistry[0]?.childDevice.deviceId).toBe('child-device-1');
    expect(parsed.selectedDeviceReadiness.readyForControl).toBe(false);
    expect(AgentProtocolDefaults.Field.LanAddDeviceReadModel).toBe('addDeviceReadModel');
    expect(AgentProtocolDefaults.Field.LanSelectedDeviceReady).toBe('selectedDeviceReady');
  });
});

function lanAddDeviceReadModelFixture() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    generatedAt: '2026-06-01T15:20:00.000Z',
    discoverySource: 'local-service',
    addDeviceState: AgentProtocolDefaults.LanProductionDiscoveryState.Pending,
    localServiceDiscoveryState: AgentProtocolDefaults.LanProductionDiscoveryState.Pending,
    physicalHouseholdLanState: AgentProtocolDefaults.LanProductionDiscoveryState.Discovered,
    cloudRelayState: AgentProtocolDefaults.LanProductionDiscoveryState.Unavailable,
    discoveredDevices: [localAgentDiscoveryDevice(), networkNeighborDiscoveryDevice()],
    canonicalHouseholdDevices: [canonicalChildAgentDevice(), canonicalRouterDevice()],
    pairingRequests: [pendingPairingRequest()],
    trustedDeviceRegistry: [trustedDeviceRegistryEntry()],
    trustedDeviceIds: ['child-device-1'],
    revokedDeviceIds: [],
    selectedDeviceReadiness: selectedDeviceReadiness(),
    controllerAuthority: 'active-controller',
    observerAuthority: 'observer',
    routeRequirementLabels: ['allowed-origin', 'target-device-match', 'non-replayed-intent'],
    auditCheckLabels: ['wrong-origin', 'wrong-device', 'replayed', 'stale', 'revoked'],
    honestNonClaims: ['physical-household-lan-manual-required', 'cloud-relay-not-implemented'],
  } as const;
}

function localAgentDiscoveryDevice() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    discoveredAt: '2026-06-01T15:20:00.000Z',
    childDevice: {
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
    },
    agentPeerId: 'portal-peer-1',
    routeId: 'lan-route-local-network',
    networkMode: 'local-network',
    reachability: 'online',
    addressRef: 'lan-address-ref-direct-websocket',
    discoveryStatus: 'websocket-direct',
    discoveryState: 'discovered',
  } as const;
}

function networkNeighborDiscoveryDevice() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    discoveredAt: '2026-06-01T15:20:00.000Z',
    childDevice: {
      deviceId: 'lan-device-54271e97c331',
      childProfileId: null,
      label: 'LAN 192.168.2.42',
      platform: 'unknown',
      ipAddress: '192.168.2.42',
      macAddress: '54-27-1e-97-c3-31',
      hostname: null,
      networkInterface: 'Ethernet 2',
      agentStatus: null,
      hardwareProfile: null,
    },
    agentPeerId: 'portal-peer-1',
    routeId: 'lan-route-local-network',
    networkMode: 'local-network',
    reachability: 'online',
    addressRef: 'lan-address-ref-network-neighbor',
    discoveryStatus: 'network-neighbor',
    discoveryState: 'discovered',
  } as const;
}

function pendingPairingRequest() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    challengeId: 'challenge-child-device-1-parent-peer-1',
    childDeviceId: 'child-device-1',
    parentDeviceId: 'parent-device-1',
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    pairingState: 'pending',
    rejectionReason: null,
    issuedAt: '2026-06-01T15:20:00.000Z',
    expiresAt: '2026-06-01T15:25:00.000Z',
  } as const;
}

function trustedDeviceRegistryEntry() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    pairingId: 'pairing-child-device-1',
    childDevice: trustedChildDevice(),
    parentDevice: trustedParentDevice(),
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    proofDigest: 'sha256:lan-proof',
    trustState: 'paired',
    trustedAt: '2026-06-01T15:20:00.000Z',
    expiresAt: '2026-06-01T16:20:00.000Z',
    revokedAt: null,
  } as const;
}

function canonicalChildAgentDevice() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
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
  } as const;
}

function canonicalRouterDevice() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
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

function trustedChildDevice() {
  return {
    deviceId: 'child-device-1',
    childProfileId: null,
    label: 'Mia Windows PC',
    platform: 'windows',
  } as const;
}

function trustedParentDevice() {
  return {
    deviceId: 'parent-device-1',
    childProfileId: null,
    label: 'Parent Windows PC',
    platform: 'windows',
  } as const;
}

function selectedDeviceReadiness() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    selectedChildDeviceId: null,
    routeId: null,
    pairingId: null,
    trustState: 'unpaired',
    reachability: 'offline',
    readyForControl: false,
    staleAt: null,
    offlineAt: null,
  } as const;
}

import { describe, expect, it } from 'vitest';
import { AgentLanBrowserAddDeviceReadModelSchema, AgentProtocolDefaults } from '../src/contracts';

describe('agent protocol browser-first LAN add-device state', () => {
  it('parses the service event read model D can consume without portal fixtures', () => {
    const parsed = parseLanAddDeviceReadModelFixture();

    expectDiscoveryState(parsed);
    expectDiscoveredDeviceState(parsed);
    expectScanSummaryState(parsed);
    expectCanonicalHouseholdDeviceState(parsed);
    expectRegistryAndDecisionState(parsed);
    expectProductionHouseholdProofState(parsed);
    expectSelectedDeviceAndDefaultFields(parsed);
  });

  it('rejects duplicate canonical household rows in the protocol add-device read model', () => {
    expect(() =>
      AgentLanBrowserAddDeviceReadModelSchema.parse({
        ...lanAddDeviceReadModelFixture(),
        canonicalHouseholdDevices: [canonicalChildAgentDevice(), canonicalChildAgentDevice()],
      })
    ).toThrow(/one canonical row/u);
  });

  it('rejects canonical household rows without evidence records', () => {
    expect(() =>
      AgentLanBrowserAddDeviceReadModelSchema.parse({
        ...lanAddDeviceReadModelFixture(),
        canonicalHouseholdDevices: [
          {
            ...canonicalRouterDevice(),
            networkIdentity: {
              ...canonicalRouterDevice().networkIdentity,
              evidenceRecords: [],
            },
          },
        ],
      })
    ).toThrow(/evidence record/u);
  });

  it('rejects unknown household decision device kinds', () => {
    expect(() =>
      AgentLanBrowserAddDeviceReadModelSchema.parse({
        ...lanAddDeviceReadModelFixture(),
        householdDeviceDecisions: [{ ...householdDecision(), deviceKind: 'television' }],
      })
    ).toThrow();
  });
});

function parseLanAddDeviceReadModelFixture() {
  return AgentLanBrowserAddDeviceReadModelSchema.parse(lanAddDeviceReadModelFixture());
}

type ParsedLanAddDeviceReadModel = ReturnType<typeof parseLanAddDeviceReadModelFixture>;

function expectDiscoveryState(parsed: ParsedLanAddDeviceReadModel) {
  expect(parsed.discoverySource).toBe('local-service');
  expect(parsed.addDeviceState).toBe('pending');
  expect(parsed.physicalHouseholdLanState).toBe('discovered');
  expect(parsed.cloudRelayState).toBe('unavailable');
}

function expectDiscoveredDeviceState(parsed: ParsedLanAddDeviceReadModel) {
  expect(parsed.discoveredDevices[0]?.childDevice.hardwareProfile?.cpuModel).toContain('Ryzen');
  expect(parsed.discoveredDevices[1]?.childDevice.ipAddress).toBe('192.168.2.42');
  expect(parsed.discoveredDevices[1]?.discoveryStatus).toBe('network-neighbor');
}

function expectScanSummaryState(parsed: ParsedLanAddDeviceReadModel) {
  expect(parsed.scanSummary.sourceLabels).toEqual(['local-service', 'windows-neighbor-table']);
  expect(parsed.scanSummary.scannedDeviceCount).toBe(2);
  expect(parsed.scanSummary.agentDeviceCount).toBe(1);
  expect(parsed.scanSummary.passiveDeviceCount).toBe(1);
  expect(parsed.scanSummary.infrastructureDeviceCount).toBe(0);
  expect(parsed.scanSummary.unsupportedDeviceCount).toBe(1);
}

function expectCanonicalHouseholdDeviceState(parsed: ParsedLanAddDeviceReadModel) {
  expect(parsed.canonicalHouseholdDevices).toHaveLength(2);
  expect(parsed.canonicalHouseholdDevices[0]?.canonicalDeviceId).toBe('lan-physical-mac-54271e97c331');
  expect(parsed.canonicalHouseholdDevices[0]?.roleBadges).toEqual(['child-agent', 'portal', 'parent-controller']);
  expect(parsed.canonicalHouseholdDevices[0]?.networkIdentity.evidenceRecords).toHaveLength(5);
  expect(parsed.canonicalHouseholdDevices[1]?.classification).toBe('network-infrastructure');
  expect(parsed.canonicalHouseholdDevices[1]?.enrollable).toBe(false);
}

function expectRegistryAndDecisionState(parsed: ParsedLanAddDeviceReadModel) {
  expect(parsed.trustedDeviceRegistry[0]?.childDevice.deviceId).toBe('child-device-1');
  expect(parsed.householdDeviceDecisions[0]?.actionKind).toBe('rename');
  expect(parsed.householdDeviceDecisions[0]?.deviceKind).toBe('desktop');
}

function expectProductionHouseholdProofState(parsed: ParsedLanAddDeviceReadModel) {
  expect(parsed.productionHouseholdProof?.manualProofRequired).toContain('signed-lan-hello');
  expect(parsed.productionHouseholdProof?.notImplemented).toEqual(['relay-route', 'cache-route']);
}

function expectSelectedDeviceAndDefaultFields(parsed: ParsedLanAddDeviceReadModel) {
  expect(parsed.selectedDeviceReadiness.readyForControl).toBe(false);
  expect(AgentProtocolDefaults.Field.LanAddDeviceReadModel).toBe('addDeviceReadModel');
  expect(AgentProtocolDefaults.Field.LanSelectedDeviceReady).toBe('selectedDeviceReady');
}

function lanAddDeviceReadModelFixture() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    generatedAt: '2026-06-01T15:20:00.000Z',
    discoverySource: 'local-service',
    addDeviceState: AgentProtocolDefaults.LanProductionDiscoveryState.Pending,
    localServiceDiscoveryState: AgentProtocolDefaults.LanProductionDiscoveryState.Pending,
    physicalHouseholdLanState: AgentProtocolDefaults.LanProductionDiscoveryState.Discovered,
    cloudRelayState: AgentProtocolDefaults.LanProductionDiscoveryState.Unavailable,
    scanSummary: {
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      sourceLabels: ['local-service', 'windows-neighbor-table'],
      scannedDeviceCount: 2,
      agentDeviceCount: 1,
      passiveDeviceCount: 1,
      infrastructureDeviceCount: 0,
      unsupportedDeviceCount: 1,
    },
    discoveredDevices: [localAgentDiscoveryDevice(), networkNeighborDiscoveryDevice()],
    canonicalHouseholdDevices: [canonicalChildAgentDevice(), canonicalRouterDevice()],
    pairingRequests: [pendingPairingRequest()],
    trustedDeviceRegistry: [trustedDeviceRegistryEntry()],
    householdDeviceDecisions: [householdDecision()],
    productionHouseholdProof: productionHouseholdProof(),
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

function productionHouseholdProof() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    generatedAt: '2026-06-01T15:20:00.000Z',
    statusRows: [
      productionStatus('signed-lan-hello', 'manual-required', 'manual-required', 'manual-proof'),
      productionStatus('signed-lan-heartbeat', 'manual-required', 'manual-required', 'manual-proof'),
      productionStatus('passive-neighbor-discovery', 'discovered', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('router-neighbor-discovery', 'discovered', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('mdns-name-discovery', 'manual-required', 'manual-required', 'manual-proof'),
      productionStatus('ssdp-name-discovery', 'manual-required', 'manual-required', 'manual-proof'),
      productionStatus('router-dhcp-name-discovery', 'manual-required', 'manual-required', 'manual-proof'),
      productionStatus('trusted-registry', 'paired', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('parent-assignment', 'manual-required', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('parent-rename', 'discovered', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('parent-ignore', 'manual-required', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('parent-revocation', 'manual-required', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('route-custody', 'paired', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('stale-selected-device', 'manual-required', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('offline-selected-device', 'offline', 'ci-mechanical-proof', 'rust-service-read-model'),
      productionStatus('relay-route', 'unavailable', 'not-implemented', 'manual-proof'),
      productionStatus('cache-route', 'unavailable', 'not-implemented', 'manual-proof'),
      productionStatus('second-physical-child-agent', 'manual-required', 'manual-required', 'manual-proof'),
      productionStatus('android-child-agent-parity', 'manual-required', 'manual-required', 'manual-proof'),
      productionStatus('ios-child-agent-parity', 'manual-required', 'manual-required', 'manual-proof'),
      productionStatus('store-signing', 'manual-required', 'manual-required', 'manual-proof'),
    ],
    manualProofRequired: [
      'signed-lan-hello',
      'signed-lan-heartbeat',
      'mdns-name-discovery',
      'ssdp-name-discovery',
      'router-dhcp-name-discovery',
      'second-physical-child-agent',
      'android-child-agent-parity',
      'ios-child-agent-parity',
      'store-signing',
    ],
    notImplemented: ['relay-route', 'cache-route'],
    claimsProved: [
      'passive Windows neighbor evidence is represented in typed LAN read-model state',
      'trusted registry, route custody, stale/offline, and parent decisions are represented in typed LAN read-model state',
    ],
    claimsNotProved: [
      'physical household LAN readiness remains manual-required until two physical child-agent hosts and router/firewall artifacts are attached',
      'signed LAN hello and heartbeat remain manual-required until a second installed child agent signs them',
      'cloud relay routing storage and authentication are not implemented in this LAN proof',
      'Android child-agent parity remains manual-required until real device permission and transport artifacts are attached',
      'iOS child-agent parity remains manual-required until entitlement device and transport artifacts are attached',
      'store signing remains manual-required until signing store and release artifacts are attached',
    ],
  } as const;
}

function productionStatus(capability: string, discoveryState: string, proofState: string, runtimeOwner: string) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    capability,
    discoveryState,
    proofState,
    runtimeOwner,
    evidenceLabel: `${capability} proof state`,
    requiredArtifactSummary: proofState === 'manual-required' ? `${capability} artifact required` : null,
  } as const;
}

function householdDecision() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    actionId: 'lan-action-rename-1',
    actionKind: 'rename',
    canonicalDeviceId: 'lan-physical-mac-54271e97c331',
    childProfileId: null,
    displayName: 'GAMEDEV Study PC',
    deviceKind: 'desktop',
    parentActorId: 'parent-actor-1',
    decidedAt: '2026-06-01T15:20:00.000Z',
    revokedAt: null,
  } as const;
}

function canonicalChildAgentDevice() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    canonicalDeviceId: 'lan-physical-mac-54271e97c331',
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
      confidence: 'agent-confirmed',
      staleAt: null,
      offlineAt: null,
      evidenceRecords: [
        evidenceRecord('local-service', 'ip-address', '192.168.2.42', 'ip:192.168.2.42', 'confirmed'),
        evidenceRecord('local-service', 'mac-address', '54-27-1e-97-c3-31', 'mac:54271e97c331', 'confirmed'),
        evidenceRecord('local-service', 'hostname', 'GAMEDEV', 'hostname:gamedev', 'confirmed'),
        evidenceRecord('local-service', 'interface', 'Ethernet 2', 'interface:ethernet2', 'confirmed'),
        evidenceRecord(
          'local-service',
          'child-agent-presence',
          'ocentra-local-service',
          'agent:lan-physical-mac-54271e97c331',
          'confirmed'
        ),
      ],
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
      evidenceRecords: [
        evidenceRecord('windows-neighbor-table', 'ip-address', '192.168.2.1', 'ip:192.168.2.1', 'strong'),
        evidenceRecord('windows-neighbor-table', 'mac-address', '00-11-22-33-44-55', 'mac:001122334455', 'strong'),
        evidenceRecord('windows-neighbor-table', 'router-classification', 'router', 'router:192.168.2.1', 'strong'),
      ],
    },
    childAgentInventory: null,
    policyTargetSurfaces: ['devices', 'network'],
  } as const;
}

function evidenceRecord(source: string, evidenceKind: string, value: string, mergeKey: string, confidence: string) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    evidenceId: `lan-evidence-${source}-${evidenceKind}-${value.toLowerCase().replace(/[^a-z0-9]/gu, '')}`,
    source,
    evidenceKind,
    deviceId: 'lan-physical-mac-54271e97c331',
    value,
    normalizedValue: value.toLowerCase(),
    firstSeenAt: '2026-06-01T15:20:00.000Z',
    lastSeenAt: '2026-06-01T15:20:00.000Z',
    expiresAt: null,
    confidence,
    mergeKey,
    note: null,
  } as const;
}

function localAgentDiscoveryDevice() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    discoveredAt: '2026-06-01T15:20:00.000Z',
    childDevice: {
      deviceId: 'local-dev-agent',
      childProfileId: null,
      label: 'local-dev-agent',
      platform: 'windows',
      ipAddress: null,
      macAddress: null,
      hostname: 'GAMEDEV',
      networkInterface: null,
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
      hostname: 'unknown-host',
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

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
  return {
    ...runtimeLanAddDeviceReadModel(),
    signedDiscoveryRelaySpine: signedDiscoveryRelaySpine(),
    trustedDeviceRegistry: [localAgentTrustedRegistryEntry()],
    trustedDeviceIds: ['local-dev-agent'],
    householdDeviceDecisions: [localAgentHouseholdDecision()],
    routeRequirementLabels: ['Only trusted signed child-agent routes become controllable'],
    auditCheckLabels: ['Signed hello manual proof required'],
    honestNonClaims: ['physical household signed child hello requires second device'],
    canonicalHouseholdDevices: [
      localAgentCanonicalHouseholdDevice(),
      lanNeighborCanonicalHouseholdDevice(),
      routerCanonicalHouseholdDevice(),
    ],
  } as const;
}

function signedDiscoveryRelaySpine() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-01T15:21:00Z',
    adapterRows: [
      {
        schemaVersion: 1,
        adapter: 'signed-child-agent-hello',
        discoveryState: 'manual-required',
        proofState: 'manual-required',
        sourceConfidence: 'manual-required',
        custodyLabel: 'signed-child-agent-artifact',
        runtimeOwner: 'manual-proof',
        evidenceLabel: 'Second physical child-agent signed hello is manual proof.',
        requiredArtifactSummary: 'Install child agent on another household device and capture signed hello.',
      },
    ],
    signedProofRows: [
      {
        schemaVersion: 1,
        check: 'signed-hello-manual-required',
        discoveryState: 'manual-required',
        responseState: 'degraded',
        rejectionReason: null,
        proofState: 'manual-required',
        runtimeOwner: 'manual-proof',
        evidenceLabel: 'Signed hello needs a second physical child-agent device.',
      },
    ],
    routeSafetyRows: [
      {
        schemaVersion: 1,
        check: 'selected-route-custody',
        routeId: 'lan-route-local-network',
        discoveryState: 'paired',
        responseState: 'accepted',
        rejectionReason: null,
        proofState: 'ci-mechanical-proof',
        runtimeOwner: 'rust-service-read-model',
        custodyLabel: 'parent-local-service',
        evidenceLabel: 'Selected route remains parent-local custody',
      },
      {
        schemaVersion: 1,
        check: 'parent-assign-decision-audited',
        routeId: 'lan-route-local-network',
        discoveryState: 'paired',
        responseState: 'accepted',
        rejectionReason: null,
        proofState: 'ci-mechanical-proof',
        runtimeOwner: 'rust-service-read-model',
        custodyLabel: 'parent-local-service',
        evidenceLabel: 'Parent assign decision is retained in the read model.',
      },
    ],
    relayCacheRows: [
      {
        schemaVersion: 1,
        check: 'relay-route-unavailable',
        decisionState: 'unavailable',
        discoveryState: 'unavailable',
        proofState: 'not-implemented',
        runtimeOwner: 'manual-proof',
        custodyLabel: 'no-ocentra-child-data-custody',
        evidenceLabel: 'Cloud relay is unavailable and not child data custody.',
      },
    ],
    manualProofRequired: ['signed-child-agent-hello', 'signed-child-agent-heartbeat'],
    notImplemented: ['relay-route-unavailable', 'cache-route-unavailable', 'parent-owned-storage-unavailable'],
    claimsProved: ['route custody rejects wrong target'],
    claimsNotProved: ['physical household signed child hello requires second device'],
  } as const;
}

function localAgentHouseholdDecision() {
  return {
    schemaVersion: 1,
    actionId: 'lan-action-assign-local-agent',
    actionKind: 'assign',
    canonicalDeviceId: 'lan-physical-mac-b42e993e72b9',
    childProfileId: 'child-profile-1',
    displayName: 'GAMEDEV',
    deviceKind: 'desktop',
    parentActorId: 'parent-actor-1',
    decidedAt: '2026-06-01T15:21:05Z',
    revokedAt: null,
  } as const;
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

function localAgentTrustedRegistryEntry() {
  return {
    schemaVersion: 1,
    pairingId: 'pairing-local-agent-1',
    childDevice: {
      deviceId: 'local-dev-agent',
      childProfileId: 'child-profile-1',
      label: 'GAMEDEV',
      platform: 'windows',
    },
    parentDevice: {
      deviceId: 'portal-dev',
      childProfileId: 'parent-profile-1',
      label: 'Parent portal',
      platform: 'windows',
    },
    routeId: 'lan-route-local-network',
    origin: 'http://127.0.0.1:4678',
    proofDigest: 'sha256:local-agent-proof',
    trustState: 'paired',
    trustedAt: '2026-06-01T15:20:00Z',
    expiresAt: '2026-06-01T16:20:00Z',
    revokedAt: null,
  } as const;
}

export function localAgentCanonicalHouseholdDevice() {
  return {
    schemaVersion: 1,
    canonicalDeviceId: 'lan-physical-mac-b42e993e72b9',
    displayName: 'GAMEDEV',
    classification: 'child-agent',
    roleBadges: ['child-agent', 'parent-controller'],
    enrollable: true,
    discoveryState: 'paired',
    trustState: 'paired',
    routeId: 'lan-route-local-network',
    routeState: 'local-network',
    networkMode: 'local-network',
    sourceLabels: ['local-service', 'network-neighbor'],
    networkIdentity: {
      hostname: 'GAMEDEV',
      ipAddresses: ['192.168.2.10'],
      macAddress: 'b4-2e-99-3e-72-b9',
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
      os: 'Windows',
      cpuModel: 'AMD Ryzen 9 3900X 12-Core Processor',
      cpuCores: '12 cores / 24 logical',
      memoryTotal: '63 GiB',
      gpuModel: 'GeForce RTX 2070 SUPER',
      gpuDriver: '456.71',
      gpuMemory: '8192 MiB',
      nvidiaSmi: 'GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM',
      networkInterfaces: ['Ethernet 2'],
      capabilities: ['direct-websocket', 'device-inventory'],
      roleState: 'implemented',
      routeState: 'local-network',
      pairingTrustState: 'paired',
    },
    policyTargetSurfaces: ['devices', 'policy', 'browser', 'activity', 'tracking'],
  } as const;
}

export function lanNeighborCanonicalHouseholdDevice() {
  return {
    schemaVersion: 1,
    canonicalDeviceId: 'lan-physical-mac-54271e97c331',
    displayName: 'HPSUJAN',
    classification: 'unknown-lan-device',
    roleBadges: [],
    enrollable: false,
    discoveryState: 'discovered',
    trustState: 'unpaired',
    routeId: null,
    routeState: 'unavailable',
    networkMode: 'local-network',
    sourceLabels: ['network-neighbor'],
    networkIdentity: {
      hostname: 'HPSUJAN',
      ipAddresses: ['192.168.2.42'],
      macAddress: '54-27-1e-97-c3-31',
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

export function routerCanonicalHouseholdDevice() {
  return {
    schemaVersion: 1,
    canonicalDeviceId: 'lan-physical-mac-001122334455',
    displayName: 'LAN 192.168.2.1',
    classification: 'network-infrastructure',
    roleBadges: ['router'],
    enrollable: false,
    discoveryState: 'discovered',
    trustState: 'unpaired',
    routeId: null,
    routeState: 'unavailable',
    networkMode: 'local-network',
    sourceLabels: ['gateway'],
    networkIdentity: {
      hostname: null,
      ipAddresses: ['192.168.2.1'],
      macAddress: '00-11-22-33-44-55',
      macVendor: null,
      networkInterfaces: ['Gateway'],
      reachability: 'online',
      confidence: 'network-neighbor',
      staleAt: null,
      offlineAt: null,
    },
    childAgentInventory: null,
    policyTargetSurfaces: ['devices', 'network'],
  } as const;
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
    ...lanAddDeviceReadModel(),
    addDeviceState,
    localServiceDiscoveryState: addDeviceState,
    discoveredDevices: [],
    trustedDeviceRegistry: [],
    trustedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: 1,
      selectedChildDeviceId: null,
      routeId: null,
      pairingId: null,
      trustState: 'unpaired',
      reachability: 'stale',
      readyForControl: false,
      staleAt: '2026-06-01T15:01:00Z',
      offlineAt: null,
    },
  } as const;
}

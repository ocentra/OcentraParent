import { describe, expect, it } from 'vitest';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
import {
  createParentPortalActivityUiIntent,
  createParentPortalCanonicalDeviceSlots,
  createParentPortalLanPairingPortalIds,
  createParentPortalLanPairingUiSlots,
  parentPortalActivityAdapterRecord,
} from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent';

const ActivityRequest = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  scope: {
    scopeKind: 'device',
    familyId: null,
    deviceId: 'child-device-1',
  },
  requestedAt: '2026-06-01T15:00:00Z',
  rangeStart: '2026-06-01T00:00:00Z',
  rangeEnd: '2026-06-01T15:00:00Z',
} as const;

describe('parent portal Activity UI intent', () => {
  parentPortalActivityIntentTests();
  parentPortalLanPairingIntentTests();
});

function parentPortalActivityIntentTests(): void {
  it('renders service-backed device slots and report files from adapter results', () => {
    const intent = serviceBackedActivityIntent();

    expect(intent.hasServiceBackedDeviceRows).toBe(true);
    expect(intent.deviceSlots.map((slot) => [slot.value, slot.status, slot.badge])).toEqual([
      ['child-device-1', 'connected', 'ready'],
      ['child-device-2', 'unsupported', 'permission-required'],
      ['activity-empty-seat-3', 'empty', undefined],
    ]);
    expect(intent.reportFiles.map((file) => file.id)).toEqual(['activity-report-1', 'saved-report-1']);
  });

  it('keeps absent or failed service adapter data unavailable without creating devices', () => {
    const intent = createParentPortalActivityUiIntent(
      {
        activityScreenReadModel: {
          ok: false,
          reason: 'invalid-json',
          state: 'unavailable',
        },
      },
      2
    );

    expect(parentPortalActivityAdapterRecord({ ok: false, reason: 'invalid-json', state: 'unavailable' })).toBeNull();
    expect(intent.hasServiceBackedDeviceRows).toBe(false);
    expect(intent.deviceSlots.map((slot) => slot.status)).toEqual(['empty', 'empty']);
    expect(intent.reportFiles).toEqual([]);
  });
}

function parentPortalLanPairingIntentTests(): void {
  parentPortalLanPairingStatusTests();
  parentPortalLanPairingReadModelTests();
  parentPortalRuntimeLanPairingIntentTests();
}

function parentPortalLanPairingStatusTests(): void {
  it('maps LAN pairing service rows into an honest status slot without discovered devices', () => {
    expect(
      createParentPortalLanPairingUiSlots([
        {
          label: 'Device pairing',
          primaryArea: 'Current device',
          readyCount: 0,
          trend: 'offline',
        },
      ])
    ).toEqual([
      {
        value: 'lan-pairing-service-state',
        label: 'LAN',
        status: 'offline',
        slotIndex: 0,
        badge: 'offline',
      },
    ]);

    expect(createParentPortalLanPairingUiSlots([])).toEqual([]);
  });
}

function parentPortalLanPairingReadModelTests(): void {
  it('renders real LAN add-device read-model devices without synthetic fallback devices', () => {
    const slots = createParentPortalLanPairingUiSlots(
      [
        {
          label: 'LAN discovery',
          primaryArea: 'LAN',
          readyCount: 2,
          trend: 'paired',
        },
      ],
      lanAddDeviceReadModel()
    );

    expect(slots.map((slot) => [slot.value, slot.label, slot.status, slot.badge])).toEqual([
      ['child-android-1', 'Pixel child', 'connected', 'ready'],
      ['child-android-2', 'Android manual', 'unsupported', 'manual-required'],
    ]);
    expect(slots[0]?.device).toMatchObject({
      ip: '192.168.2.42',
      mac: '54-27-1e-97-c3-31',
      hostname: 'pixel-child',
      networkInterface: 'Ethernet 2',
      agentStatus: 'ocentra-child-agent',
      manufacturer: 'Google',
      model: 'Pixel test',
      cpuModel: 'Tensor test',
      gpuModel: 'Mali test',
    });
    expect(slots.every((slot) => slot.value !== 'lan-pairing-service-state')).toBe(true);
    expect(createParentPortalLanPairingPortalIds(slots)).toEqual(['child-android-1']);
  });

  it('shows read-model manual-required or unavailable states as service status when no device evidence exists', () => {
    expect(createParentPortalLanPairingUiSlots([], emptyLanAddDeviceReadModel('manual-required'))).toEqual([
      {
        value: 'lan-pairing-service-state',
        label: 'LAN',
        status: 'unsupported',
        slotIndex: 0,
        badge: 'manual-required',
      },
    ]);
  });

  it('shows connected LAN service rows as scanning until the first LAN read model arrives', () => {
    expect(
      createParentPortalLanPairingUiSlots([
        {
          label: 'LAN discovery',
          primaryArea: 'LAN',
          readyCount: 0,
          signalScore: 0,
          trend: 'manual-required',
        },
      ])
    ).toEqual([
      {
        value: 'lan-pairing-service-state',
        label: 'Scanning LAN',
        status: 'available',
        slotIndex: 0,
        badge: 'scanning',
      },
    ]);
  });
}

function parentPortalRuntimeLanPairingIntentTests(): void {
  parentPortalRuntimeNeighborTests();
  parentPortalRuntimeCanonicalTargetTests();
}

function parentPortalRuntimeNeighborTests(): void {
  it('keeps local-agent hardware separate from observed LAN neighbor network fields', () => {
    const slots = createParentPortalLanPairingUiSlots([], runtimeLanAddDeviceReadModel());

    expect(slots.map((slot) => [slot.value, slot.label, slot.status, slot.badge])).toEqual([
      ['local-dev-agent', 'GAMEDEV', 'connected', 'online'],
      ['lan-device-54271e97c331', 'LAN 192.168.2.42', 'available', 'discovered'],
      ['lan-device-001122334455', 'LAN 192.168.2.1', 'unsupported', 'infrastructure'],
    ]);
    expect(slots.find((slot) => slot.value === 'lan-device-b42e993e72b9')).toBeUndefined();
    expect(createParentPortalLanPairingPortalIds(slots)).toEqual(['local-dev-agent']);

    expectLocalAgentRuntimeSlot(slots);
    expectLanNeighborRuntimeSlot(slots);
    expectRouterInfrastructureSlot(slots);
  });
}

function parentPortalRuntimeCanonicalTargetTests(): void {
  it('uses the canonical household spine to keep LAN neighbors out of controlled-device scopes', () => {
    const slots = createParentPortalLanPairingUiSlots([], canonicalRuntimeLanAddDeviceReadModel());

    expect(slots.map((slot) => [slot.value, slot.label, slot.status, slot.badge])).toEqual([
      ['lan-physical-mac-b42e993e72b9', 'GAMEDEV', 'connected', 'online'],
      ['lan-physical-mac-54271e97c331', 'HPSUJAN', 'available', 'discovered'],
      ['lan-physical-mac-001122334455', 'LAN 192.168.2.1', 'unsupported', 'infrastructure'],
    ]);
    expect(createParentPortalLanPairingPortalIds(slots)).toEqual(['lan-physical-mac-b42e993e72b9']);
    expect(createParentPortalCanonicalDeviceSlots([], slots).map((slot) => [slot.value, slot.label])).toEqual([
      ['lan-physical-mac-b42e993e72b9', 'GAMEDEV'],
    ]);

    const localAgent = slots.find((slot) => slot.value === 'lan-physical-mac-b42e993e72b9');
    expect(localAgent?.device).toMatchObject({
      portalEligible: true,
      agentStatus: 'ocentra-child-agent',
      cpuModel: 'AMD Ryzen 9 3900X 12-Core Processor',
      memoryTotal: '63 GiB',
      gpuModel: 'GeForce RTX 2070 SUPER',
    });

    const lanNeighbor = slots.find((slot) => slot.value === 'lan-physical-mac-54271e97c331');
    expect(lanNeighbor?.device).toMatchObject({
      portalEligible: false,
      ip: '192.168.2.42',
      mac: '54-27-1e-97-c3-31',
      hostname: 'HPSUJAN',
    });
    expectNoAgentHardware(lanNeighbor?.device);

    const router = slots.find((slot) => slot.value === 'lan-physical-mac-001122334455');
    expect(router?.device).toMatchObject({
      portalEligible: false,
      platform: 'router',
      type: 'router',
      status: 'unsupported',
    });
  });

  it('feeds canonical policy target slots from the same service-backed device spine', () => {
    const lanSlots = createParentPortalLanPairingUiSlots([], runtimeLanAddDeviceReadModel());
    const activitySlots = createParentPortalActivityUiIntent(
      {
        activityBrowserReadModel: adapterResult(runtimeBrowserTargetReadModel()),
      },
      3
    ).deviceSlots;
    const canonicalSlots = createParentPortalCanonicalDeviceSlots(activitySlots, lanSlots);

    expect(canonicalSlots.find((slot) => slot.value === 'local-dev-agent')).toMatchObject({
      label: 'GAMEDEV',
      status: 'connected',
      badge: 'online',
    });
    expect(canonicalSlots.find((slot) => slot.value === 'child-device-2')).toMatchObject({
      label: 'CE2',
      status: 'unsupported',
      badge: 'permission-required',
    });
    expect(canonicalSlots.find((slot) => slot.value === 'lan-device-54271e97c331')).toBeUndefined();
    expect(canonicalSlots.find((slot) => slot.value === 'lan-device-001122334455')).toBeUndefined();
  });
}

function expectLocalAgentRuntimeSlot(slots: ReturnType<typeof createParentPortalLanPairingUiSlots>): void {
  const localAgent = slots.find((slot) => slot.value === 'local-dev-agent');
  expect(localAgent?.device).toMatchObject({
    name: 'GAMEDEV',
    ip: '192.168.2.10',
    mac: 'b4-2e-99-3e-72-b9',
    hostname: 'GAMEDEV',
    networkInterface: 'Ethernet 2',
    agentStatus: 'ocentra-local-service',
    cpuModel: 'AMD Ryzen 9 3900X 12-Core Processor',
    memoryTotal: '63 GiB',
    gpuModel: 'GeForce RTX 2070 SUPER',
  });
}

function expectLanNeighborRuntimeSlot(slots: ReturnType<typeof createParentPortalLanPairingUiSlots>): void {
  const lanNeighbor = slots.find((slot) => slot.value === 'lan-device-54271e97c331');
  expect(lanNeighbor?.device).toMatchObject({
    ip: '192.168.2.42',
    mac: '54-27-1e-97-c3-31',
    hostname: 'unknown-host',
    networkInterface: 'Ethernet 2',
  });
  expectNoAgentHardware(lanNeighbor?.device);
}

function expectRouterInfrastructureSlot(slots: ReturnType<typeof createParentPortalLanPairingUiSlots>): void {
  const router = slots.find((slot) => slot.value === 'lan-device-001122334455');
  expect(router?.device).toMatchObject({
    ip: '192.168.2.1',
    mac: '00-11-22-33-44-55',
    hostname: 'unknown-host',
    networkInterface: 'Gateway',
    type: 'router',
    platform: 'router',
    status: 'unsupported',
  });
  expectNoAgentHardware(router?.device);
}

function expectNoAgentHardware(device: unknown): void {
  const typedDevice = device as { agentStatus?: string; cpuModel?: string; memoryTotal?: string; gpuModel?: string };
  expect(typedDevice?.agentStatus).toBeUndefined();
  expect(typedDevice?.cpuModel).toBeUndefined();
  expect(typedDevice?.memoryTotal).toBeUndefined();
  expect(typedDevice?.gpuModel).toBeUndefined();
}

function serviceBackedActivityIntent() {
  return createParentPortalActivityUiIntent(
    {
      activityReport: adapterResult(activityReportDocument('activity-report-1')),
      activityReportHistory: adapterResult(activityReportHistory()),
      activityBrowserReadModel: adapterResult(browserPermissionRequiredReadModel()),
    },
    3
  );
}

function activityReportDocument(reportId: string) {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    reportId,
    frequency: 'daily',
    scope: ActivityRequest.scope,
    requestedAt: ActivityRequest.requestedAt,
    rangeStart: ActivityRequest.rangeStart,
    rangeEnd: ActivityRequest.rangeEnd,
    generatedAt: '2026-06-01T15:00:01Z',
    savedMetadata: null,
    sourceStates: [
      {
        deviceId: 'child-device-1',
        reachabilityState: 'reachable',
        state: 'ready',
        reason: null,
        lastUpdatedAt: '2026-06-01T14:59:00Z',
      },
    ],
    sections: [],
  } as const;
}

function activityReportHistory() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'ready',
    storageState: 'saved',
    storageReason: null,
    reports: [savedActivityReport()],
  } as const;
}

function savedActivityReport() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    reportId: 'saved-report-1',
    fileName: 'saved-report-1.json',
    reportDate: '2026-06-01T15:00:00Z',
    rangeStart: ActivityRequest.rangeStart,
    rangeEnd: ActivityRequest.rangeEnd,
    summary: 'Saved activity report from service storage',
    savedState: 'saved',
    savedAt: '2026-06-01T15:00:02Z',
    sourceStateSummary: sourceStateSummary(),
    parsedReport: activityReportDocument('saved-report-1'),
  } as const;
}

function sourceStateSummary() {
  return {
    totalSources: 1,
    readySources: 1,
    offlineSources: 0,
    staleSources: 0,
    unavailableSources: 0,
    unreachableSources: 0,
    errorSources: 0,
  } as const;
}

function browserPermissionRequiredReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'permission-required',
    generatedAt: '2026-06-01T15:00:01Z',
    summary: 'Browser adapter requires child permission',
    rows: [
      {
        rowId: 'browser-row-1',
        domainLabel: 'school.example',
        deviceId: 'child-device-2',
        state: 'permission-required',
        visitCount: 1,
        totalMs: 120000,
        evidenceDigest: null,
      },
    ],
  } as const;
}

function runtimeBrowserTargetReadModel() {
  return {
    ...browserPermissionRequiredReadModel(),
    state: 'ready',
    rows: [
      {
        rowId: 'browser-row-local-agent',
        domainLabel: 'local.example',
        deviceId: 'local-dev-agent',
        state: 'ready',
        visitCount: 1,
        totalMs: 60000,
        evidenceDigest: null,
      },
      {
        rowId: 'browser-row-permission-required',
        domainLabel: 'school.example',
        deviceId: 'child-device-2',
        state: 'permission-required',
        visitCount: 1,
        totalMs: 120000,
        evidenceDigest: null,
      },
    ],
  } as const;
}

function adapterResult(value: Record<string, unknown>) {
  return {
    ok: true,
    state: value['state'] ?? 'ready',
    value,
  } as const;
}

function lanAddDeviceReadModel() {
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

function runtimeLanAddDeviceReadModel() {
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

function canonicalRuntimeLanAddDeviceReadModel() {
  return {
    ...runtimeLanAddDeviceReadModel(),
    canonicalHouseholdDevices: [
      localAgentCanonicalHouseholdDevice(),
      lanNeighborCanonicalHouseholdDevice(),
      routerCanonicalHouseholdDevice(),
    ],
  } as const;
}

function localAgentCanonicalHouseholdDevice() {
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

function lanNeighborCanonicalHouseholdDevice() {
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

function routerCanonicalHouseholdDevice() {
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

function emptyLanAddDeviceReadModel(addDeviceState: string) {
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

import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import {
  ActivityScreenReadModelSchema,
  ActivitySurfaceSchemaVersion,
} from '@ocentra-parent/schema-domain/activity-surface';
import { DeviceChoiceGrid } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/DeviceChoiceGrid/DeviceChoiceGrid';
import type { DeviceSlot } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/DeviceChoiceGrid/DeviceChoiceGridTypes';
import {
  createParentPortalActivityUiIntent,
  createParentPortalCanonicalDeviceSlots,
  createParentPortalLanPairingPortalIds,
  createParentPortalLanPairingUiSlots,
  parentPortalActivityAdapterRecord,
} from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent';
import {
  canonicalRuntimeLanAddDeviceReadModel,
  emptyLanAddDeviceReadModel,
  lanAddDeviceReadModel,
  lanNeighborCanonicalHouseholdDevice,
  lanNeighborHouseholdDecision,
  localAgentCanonicalHouseholdDevice,
  routerCanonicalHouseholdDevice,
  runtimeLanAddDeviceReadModel,
} from './activity-ui-lan-pairing-fixtures';
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

const LongLanSelectedLabel = 'Media Room Tablet With An Intentionally Long Hostname That Should Still Render As Text';

describe('parent portal Activity UI intent', () => {
  parentPortalActivityIntentTests();
  parentPortalLanPairingIntentTests();
});
function parentPortalActivityIntentTests(): void {
  it('renders service-backed device slots and report files from adapter results', () => {
    const intent = serviceBackedActivityIntent();
    const screen = ActivityScreenReadModelSchema.parse(
      parentPortalActivityAdapterRecord(adapterResult(screenReadModel()))
    );

    expect(intent.hasServiceBackedDeviceRows).toBe(true);
    expect(intent.deviceSlots.map((slot) => [slot.value, slot.status, slot.badge])).toEqual([
      ['child-device-1', 'connected', 'ready'],
      ['child-device-2', 'unsupported', 'permission-required'],
      ['activity-empty-seat-3', 'empty', undefined],
    ]);
    expect(intent.reportFiles.map((file) => file.id)).toEqual(['activity-report-1', 'saved-report-1']);
    expect(screen?.rows[0]).toMatchObject({
      captureReason: 'nativeAppForegroundStart',
      providerKind: 'localVision',
      imageDeletionState: 'deleted',
      policyEligible: true,
    });
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
  parentPortalRuntimeCanonicalSpineTests();
  parentPortalRuntimeCanonicalVisibilityTests();
  parentPortalRuntimeCanonicalPassiveNeighborTests();
  parentPortalRuntimeCanonicalRendererTests();
}

function parentPortalRuntimeCanonicalSpineTests(): void {
  it('uses the canonical household spine to keep LAN neighbors out of controlled-device scopes', () => {
    expectCanonicalHouseholdSpineTargetSlots();
  });
  it('feeds canonical policy target slots from the same service-backed device spine', () => {
    expectCanonicalPolicyTargetSlots();
  });

  it('surfaces signed discovery, custody, relay, and parent decision evidence from the LAN read model', () => {
    expectLanSignedDiscoveryRelaySlotEvidence();
  });

  it('applies parent LAN household name and device type overrides without losing detected names', () => {
    const baseModel = canonicalRuntimeLanAddDeviceReadModel();
    const slots = createParentPortalLanPairingUiSlots([], {
      ...baseModel,
      householdDeviceDecisions: [...baseModel.householdDeviceDecisions, lanNeighborHouseholdDecision()],
      canonicalHouseholdDevices: [
        localAgentCanonicalHouseholdDevice(),
        {
          ...lanNeighborCanonicalHouseholdDevice(),
          displayName: 'Kitchen laptop',
        },
        routerCanonicalHouseholdDevice(),
      ],
    });
    const neighbor = slots.find((slot) => slot.value === 'lan-physical-mac-54271e97c331');

    expect(neighbor).toMatchObject({
      label: 'Kitchen laptop',
      device: {
        name: 'Kitchen laptop',
        householdName: 'Kitchen laptop',
        detectedName: 'HPSUJAN',
        hostname: 'HPSUJAN',
        parentDeviceKind: 'laptop',
        type: 'laptop',
      },
    });
  });

  it('keeps discovered LAN neighbors visible when the canonical spine only covers the child-agent device', () => {
    const baseModel = canonicalRuntimeLanAddDeviceReadModel();
    const slots = createParentPortalLanPairingUiSlots([], {
      ...baseModel,
      canonicalHouseholdDevices: [localAgentCanonicalHouseholdDevice()],
    });

    expect(slots.map((slot) => [slot.value, slot.label, slot.status, slot.badge])).toEqual([
      ['lan-physical-mac-b42e993e72b9', 'GAMEDEV', 'connected', 'online'],
      ['lan-device-54271e97c331', 'LAN 192.168.2.42', 'available', 'discovered'],
      ['lan-device-001122334455', 'LAN 192.168.2.1', 'unsupported', 'infrastructure'],
    ]);
  });
}

function parentPortalRuntimeCanonicalVisibilityTests(): void {
  it('keeps ignored passive LAN neighbors visible but outside canonical policy-target scopes', () => {
    const baseModel = canonicalRuntimeLanAddDeviceReadModel();
    const slots = createParentPortalLanPairingUiSlots([], {
      ...baseModel,
      householdDeviceDecisions: [
        ...baseModel.householdDeviceDecisions,
        {
          ...lanNeighborHouseholdDecision(),
          actionId: 'lan-action-ignore-hpsujan',
          actionKind: 'ignore',
          displayName: 'HPSUJAN',
        },
      ],
    });
    const lanNeighbor = slots.find((slot) => slot.value === 'lan-physical-mac-54271e97c331');

    expect(lanNeighbor).toMatchObject({
      label: 'HPSUJAN',
      status: 'unsupported',
      badge: 'ignored',
      device: {
        sourceState: 'ignored',
        parentDecision: 'ignore: HPSUJAN',
        detectedName: 'HPSUJAN',
        portalEligible: false,
      },
    });
    expect(createParentPortalCanonicalDeviceSlots([], slots)).not.toContainEqual(
      expect.objectContaining({ value: 'lan-physical-mac-54271e97c331' })
    );
  });

  it('keeps revoked LAN devices visible without promoting them into policy-target scopes', () => {
    const baseModel = canonicalRuntimeLanAddDeviceReadModel();
    const slots = createParentPortalLanPairingUiSlots([], {
      ...baseModel,
      householdDeviceDecisions: [
        ...baseModel.householdDeviceDecisions,
        {
          ...lanNeighborHouseholdDecision(),
          actionId: 'lan-action-revoke-hpsujan',
          actionKind: 'revoke',
          displayName: 'HPSUJAN',
          revokedAt: '2026-06-01T15:25:00Z',
        },
      ],
    });
    const lanNeighbor = slots.find((slot) => slot.value === 'lan-physical-mac-54271e97c331');

    expect(lanNeighbor).toMatchObject({
      label: 'HPSUJAN',
      status: 'unsupported',
      badge: 'revoked',
      device: {
        sourceState: 'revoked',
        parentDecision: 'revoke revoked',
        detectedName: 'HPSUJAN',
        portalEligible: false,
      },
    });
    expect(createParentPortalCanonicalDeviceSlots([], slots)).not.toContainEqual(
      expect.objectContaining({ value: 'lan-physical-mac-54271e97c331' })
    );
  });
}

function parentPortalRuntimeCanonicalPassiveNeighborTests(): void {
  it('keeps stale and offline passive LAN neighbor reachability distinct', () => {
    const staleSlots = createPassiveNeighborStateSlots('stale');
    const offlineSlots = createPassiveNeighborStateSlots('offline');

    expect(staleSlots.find((slot) => slot.value === 'lan-physical-mac-54271e97c331')).toMatchObject({
      status: 'available',
      badge: 'stale',
      device: {
        sourceState: 'stale',
      },
    });
    expect(offlineSlots.find((slot) => slot.value === 'lan-physical-mac-54271e97c331')).toMatchObject({
      status: 'offline',
      badge: 'offline',
      device: {
        sourceState: 'offline',
      },
    });
  });

  it('renders dense LAN grids with long selected labels without inventing hardware details', () => {
    const markup = renderDenseLanGridMarkup();

    expect(markup).toContain(LongLanSelectedLabel);
    expect(markup).toContain('Garage desktop');
    expect(markup).toContain('Family iPad');
    expect(markup).toContain('textLength=');
    expect(markup).not.toContain('AMD Ryzen 9 3900X 12-Core Processor');
    expect(markup).not.toContain('GeForce RTX 2070 SUPER');
  });
}

function parentPortalRuntimeCanonicalRendererTests(): void {
  it('keeps missing LAN hardware rows on the Not reported fallback instead of inventing values', () => {
    const rendererSource = readFileSync(
      new URL(
        '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx',
        import.meta.url
      ),
      'utf8'
    );

    expect(rendererSource).toContain('function lanPairingMissingDeviceValue(value?: string): string {');
    expect(rendererSource).toContain("return trimmed ? trimmed : 'Not reported';");
    expect(rendererSource).toContain(
      "{ label: 'CPU', value: lanPairingMissingDeviceValue(selectedDevice.device?.cpuModel), tone: 'purple' }"
    );
    expect(rendererSource).toContain(
      "{ label: 'Memory', value: lanPairingMissingDeviceValue(selectedDevice.device?.memoryTotal), tone: 'gold' }"
    );
    expect(rendererSource).toContain(
      "{ label: 'GPU', value: lanPairingMissingDeviceValue(selectedDevice.device?.gpuModel), tone: 'purple' }"
    );
  });

  it('routes service-backed source and custody labels into the selected-device LAN surface helpers', () => {
    const rendererSource = readFileSync(
      new URL(
        '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx',
        import.meta.url
      ),
      'utf8'
    );

    expect(rendererSource).toContain(
      "{ label: 'Source', value: lanPairingDeviceSource(selectedDevice), tone: 'purple' }"
    );
    expect(rendererSource).toContain(
      "{ label: 'Custody', value: lanPairingDeviceCustody(selectedDevice), tone: 'cyan' }"
    );
    expect(rendererSource).toContain(
      "{ label: 'Custody', value: lanPairingDeviceCustody(selectedDevice), tone: 'purple' }"
    );
  });

  it('keeps dedicated LAN control-state labels for ignored revoked stale and offline surfaces', () => {
    const rendererSource = readFileSync(
      new URL(
        '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx',
        import.meta.url
      ),
      'utf8'
    );

    expect(rendererSource).toContain("if (state === 'ignored') return 'Ignored';");
    expect(rendererSource).toContain("if (state === 'revoked') return 'Revoked';");
    expect(rendererSource).toContain("if (state === 'stale') return 'Stale';");
    expect(rendererSource).toContain("if (slot.status === 'offline' || state === 'offline') return 'Offline';");
  });
}

function expectCanonicalHouseholdSpineTargetSlots(): void {
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

  expectCanonicalLocalAgentSlot(slots);
  expectCanonicalLanNeighborSlot(slots);
  expectCanonicalRouterSlot(slots);
}

function expectCanonicalLocalAgentSlot(slots: ReturnType<typeof createParentPortalLanPairingUiSlots>): void {
  const localAgent = slots.find((slot) => slot.value === 'lan-physical-mac-b42e993e72b9');
  expect(localAgent?.device).toMatchObject({
    portalEligible: true,
    agentStatus: 'ocentra-child-agent',
    cpuModel: 'AMD Ryzen 9 3900X 12-Core Processor',
    memoryTotal: '63 GiB',
    gpuModel: 'GeForce RTX 2070 SUPER',
  });
}

function expectCanonicalLanNeighborSlot(slots: ReturnType<typeof createParentPortalLanPairingUiSlots>): void {
  const lanNeighbor = slots.find((slot) => slot.value === 'lan-physical-mac-54271e97c331');
  expect(lanNeighbor?.device).toMatchObject({
    portalEligible: false,
    ip: '192.168.2.42',
    mac: '54-27-1e-97-c3-31',
    hostname: 'HPSUJAN',
    detectedName: 'HPSUJAN',
  });
  expectNoAgentHardware(lanNeighbor?.device);
}

function expectCanonicalRouterSlot(slots: ReturnType<typeof createParentPortalLanPairingUiSlots>): void {
  const router = slots.find((slot) => slot.value === 'lan-physical-mac-001122334455');
  expect(router?.device).toMatchObject({
    portalEligible: false,
    platform: 'router',
    type: 'router',
    status: 'unsupported',
  });
}

function expectCanonicalPolicyTargetSlots(): void {
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
}

function expectLanSignedDiscoveryRelaySlotEvidence(): void {
  const slots = createParentPortalLanPairingUiSlots([], canonicalRuntimeLanAddDeviceReadModel());
  const localAgent = slots.find((slot) => slot.value === 'lan-physical-mac-b42e993e72b9');

  expect(localAgent?.device).toMatchObject({
    pairingId: 'pairing-local-agent-1',
    proofDigest: 'sha256:local-agent-proof',
    origin: 'http://127.0.0.1:4678',
    parentDeviceId: 'portal-dev',
    childProfileId: 'child-profile-1',
    custodyLabel: 'parent-local-service',
    signedProofCheck: 'signed-hello-manual-required',
    signedProofState: 'manual-required',
    routeSafety: 'selected-route-custody',
    routeSafetyState: 'accepted',
    relayCacheCheck: 'relay-route-unavailable',
    relayCacheState: 'unavailable',
    manualProof: 'signed-child-agent-hello, signed-child-agent-heartbeat',
    claimsNotProved: 'physical household signed child hello requires second device',
    parentDecision: 'assign: GAMEDEV',
    householdName: 'GAMEDEV',
    parentDeviceKind: 'desktop',
    auditLabel: 'Signed hello manual proof required',
    requirementLabel: 'Only trusted signed child-agent routes become controllable',
    evidenceLabel: 'Selected route remains parent-local custody',
  });
}

function createPassiveNeighborStateSlots(reachability: 'stale' | 'offline') {
  const baseModel = canonicalRuntimeLanAddDeviceReadModel();
  const lanNeighbor = lanNeighborCanonicalHouseholdDevice();

  return createParentPortalLanPairingUiSlots([], {
    ...baseModel,
    canonicalHouseholdDevices: [
      localAgentCanonicalHouseholdDevice(),
      {
        ...lanNeighbor,
        networkIdentity: {
          ...lanNeighbor.networkIdentity,
          reachability,
        },
      },
      routerCanonicalHouseholdDevice(),
    ],
  });
}

function renderDenseLanGridMarkup(): string {
  const baseSlots = createParentPortalLanPairingUiSlots([], canonicalRuntimeLanAddDeviceReadModel());
  const localAgent = baseSlots.find((slot) => slot.value === 'lan-physical-mac-b42e993e72b9');
  const lanNeighbor = baseSlots.find((slot) => slot.value === 'lan-physical-mac-54271e97c331');
  const router = baseSlots.find((slot) => slot.value === 'lan-physical-mac-001122334455');
  if (!localAgent || !lanNeighbor || !router || !lanNeighbor.device) {
    throw new Error('Expected canonical LAN slots for dense-grid proof');
  }

  const denseSlots: DeviceSlot[] = [
    localAgent,
    {
      ...lanNeighbor,
      value: 'lan-physical-mac-aa11bb22cc33',
      label: LongLanSelectedLabel,
      slotIndex: 1,
      device: {
        ...lanNeighbor.device,
        id: 'lan-physical-mac-aa11bb22cc33',
        name: LongLanSelectedLabel,
        hostname: LongLanSelectedLabel,
        detectedName: LongLanSelectedLabel,
        ip: '192.168.2.66',
        mac: 'aa-11-bb-22-cc-33',
        status: 'available',
      },
    },
    {
      ...lanNeighbor,
      value: 'lan-physical-mac-dd44ee55ff66',
      label: 'Garage desktop',
      slotIndex: 2,
      device: {
        ...lanNeighbor.device,
        id: 'lan-physical-mac-dd44ee55ff66',
        name: 'Garage desktop',
        hostname: 'GARAGE-DESKTOP',
        detectedName: 'GARAGE-DESKTOP',
        ip: '192.168.2.67',
        mac: 'dd-44-ee-55-ff-66',
        status: 'available',
      },
    },
    {
      ...lanNeighbor,
      value: 'lan-physical-mac-112233445566',
      label: 'Family iPad',
      slotIndex: 3,
      device: {
        ...lanNeighbor.device,
        id: 'lan-physical-mac-112233445566',
        name: 'Family iPad',
        hostname: 'FAMILY-IPAD',
        detectedName: 'FAMILY-IPAD',
        ip: '192.168.2.68',
        mac: '11-22-33-44-55-66',
        platform: 'ios',
        type: 'tablet',
        parentDeviceKind: 'tablet',
        status: 'available',
      },
    },
    router,
  ];

  return renderToStaticMarkup(
    createElement(DeviceChoiceGrid, {
      slots: denseSlots,
      defaultValue: 'lan-physical-mac-aa11bb22cc33',
      showScopeSelector: false,
      showAddControls: false,
      scopeValues: ['lan'],
    })
  );
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

function screenReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'ready',
    generatedAt: '2026-06-01T15:00:01Z',
    summary: 'Screen summary is available from the local capture journal.',
    rows: [
      {
        rowId: 'screen-row-1',
        label: 'Visible activity summary',
        deviceId: 'child-device-1',
        state: 'ready',
        totalMs: 60000,
        foregroundMs: 60000,
        backgroundMs: 0,
        captureReason: 'nativeAppForegroundStart',
        captureScope: 'activeWindow',
        capabilityStatus: 'ready',
        queueJobId: 'screen-queue-job-1',
        modelRuntimeRef: 'local-vision-runtime-1',
        modelId: 'local-vision-model-1',
        providerKind: 'localVision',
        promptOrTemplateVersion: 'screen-template-v1',
        primaryCategory: 'productivity',
        confidence: 0.91,
        imageDeletionState: 'deleted',
        rawImageRetained: false,
        policyEligible: true,
        imageDigest: 'sha256:screen-image-digest',
        custodyState: 'child-device-journal',
        evidence: [],
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

import { describe, expect, it } from 'vitest';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import type { ActivitySurfaceAdapterResult } from '@ocentra-parent/agent-protocol-domain/activity-surface-adapter';
import { resolveLiveActivityState } from '../src/live-activity-state';
import { canonicalRuntimeLanAddDeviceReadModel } from './activity-ui-lan-pairing-fixtures';

type AdapterResult = ActivitySurfaceAdapterResult<unknown> | null;

const FamilyScope = {
  scopeKind: 'family',
  familyId: 'family-local-1',
  deviceId: null,
} as const;

const DeviceScope = {
  scopeKind: 'device',
  familyId: null,
  deviceId: 'child-device-1',
} as const;

const ActivityRequest = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  scope: DeviceScope,
  requestedAt: '2026-05-30T14:00:00Z',
  rangeStart: '2026-05-30T00:00:00Z',
  rangeEnd: '2026-05-30T14:00:00Z',
} as const;

const ReportDocument = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  reportId: 'activity-report-daily-1',
  frequency: 'daily',
  scope: FamilyScope,
  requestedAt: '2026-05-30T14:00:00Z',
  rangeStart: '2026-05-30T00:00:00Z',
  rangeEnd: '2026-05-30T14:00:00Z',
  generatedAt: '2026-05-30T14:00:01Z',
  savedMetadata: null,
  sourceStates: [
    {
      deviceId: 'child-device-1',
      reachabilityState: 'reachable',
      state: 'ready',
      reason: null,
      lastUpdatedAt: '2026-05-30T13:59:00Z',
    },
  ],
  sections: [
    {
      sectionKind: 'summary',
      title: 'Summary',
      state: 'ready',
      summary: 'Activity adapter returned a local daily summary',
      itemCount: 1,
      evidence: [],
    },
  ],
} as const;

describe('portal Activity surface adapter state', () => {
  activityAdapterReadModelTests();
  activityAdapterLanPairingTests();
});

function activityAdapterReadModelTests(): void {
  it('parses report, history, and every tab read model from real adapter events', () => {
    const state = resolveLiveActivityState(activitySurfaceEvents());

    expectAdapterState(state.activityReport, 'ready');
    expect(state.activityReportEvent?.event).toBe(AgentEvent.ActivityReportGenerated);

    expectAdapterState(state.activityReportHistory, 'unavailable');
    if (state.activityReportHistory?.ok) {
      expect(state.activityReportHistory.value.storageState).toBe('storage-unavailable');
    }

    expectAdapterState(state.activityScreenReadModel, 'ready');
    if (state.activityScreenReadModel?.ok) {
      expect(state.activityScreenReadModel.value.rows).toHaveLength(1);
      expect(state.activityScreenReadModel.value.rows[0]).toMatchObject({
        captureReason: 'nativeAppForegroundStart',
        providerKind: 'localVision',
        imageDeletionState: 'deleted',
        policyEligible: true,
      });
    }

    expectAdapterState(state.activityAppUseReadModel, 'empty');
    expectAdapterState(state.activityBrowserReadModel, 'permission-required');
    expectAdapterState(state.activityGamesReadModel, 'scaffold-only');
    expectAdapterState(state.activityNetworkReadModel, 'unavailable');
    expect(state.activityServiceUiSpine.dataOwner).toBe('rust-service-read-model');
    expect(state.activityServiceUiSpine.uiConsumer).toBe('c-owned-activity-ui');
    expect(state.activityServiceUiSpine.viteDataOwner).toBe(false);
    expect(state.activityServiceUiSpine.screen?.ok).toBe(true);
    expect(state.activityServiceUiSpine.appUse?.state).toBe('empty');
    expect(state.activityServiceUiSpine.browser?.state).toBe('permission-required');
    expect(state.activityServiceUiSpine.network?.state).toBe('unavailable');
  });

  it('keeps malformed adapter payloads visible as typed parser failures', () => {
    const state = resolveLiveActivityState([
      surfaceEventWithRawPayload(
        AgentEvent.ActivityNetworkReadModelReported,
        AgentProtocolDefaults.Field.ActivityReadModel,
        'not-json',
        'unavailable'
      ),
    ]);

    expect(state.activityNetworkReadModel?.ok).toBe(false);
    if (state.activityNetworkReadModel?.ok === false) {
      expect(state.activityNetworkReadModel.reason).toBe('invalid-json');
    }
    expect(state.activityNetworkReadModelEvent?.event).toBe(AgentEvent.ActivityNetworkReadModelReported);
  });
}

function activityAdapterLanPairingTests(): void {
  it('parses LAN add-device readiness from the real status event payload', () => {
    const state = resolveLiveActivityState([
      payloadEvent(AgentEvent.LanPairingStatusReported, {
        [AgentProtocolDefaults.Field.LanAddDeviceReadModel]: JSON.stringify(canonicalRuntimeLanAddDeviceReadModel()),
      }),
    ]);

    expect(state.lanPairingStatusEvent?.event).toBe(AgentEvent.LanPairingStatusReported);
    expectLanReadinessState(state);
    expectLanCanonicalDeviceState(state);
    expectLanSignedDiscoveryRelayState(state);
  });

  it('uses explicit LAN scan reports as the current add-device read model', () => {
    const state = resolveLiveActivityState([
      payloadEvent(
        AgentEvent.LanPairingStatusReported,
        {
          [AgentProtocolDefaults.Field.LanAddDeviceReadModel]: JSON.stringify({
            ...canonicalRuntimeLanAddDeviceReadModel(),
            addDeviceState: 'manual-required',
          }),
        },
        '2026-06-01T15:00:00Z'
      ),
      payloadEvent(
        AgentEvent.LanPairingBrowserDiscoveryReported,
        {
          [AgentProtocolDefaults.Field.LanAddDeviceReadModel]: JSON.stringify(canonicalRuntimeLanAddDeviceReadModel()),
        },
        '2026-06-01T15:00:04Z'
      ),
    ]);

    expect(state.lanPairingStatusEvent?.event).toBe(AgentEvent.LanPairingBrowserDiscoveryReported);
    expect(state.lanAddDeviceReadModel?.addDeviceState).toBe('paired');
  });
}

type LiveLanActivityState = ReturnType<typeof resolveLiveActivityState>;

function expectLanReadinessState(state: LiveLanActivityState): void {
  expect(state.lanAddDeviceReadModel?.selectedDeviceReadiness).toMatchObject({
    selectedChildDeviceId: 'local-dev-agent',
    reachability: 'online',
    readyForControl: false,
  });
  expect(state.lanAddDeviceReadModel?.scanSummary).toMatchObject({
    scannedDeviceCount: 3,
    agentDeviceCount: 1,
  });
}

function expectLanCanonicalDeviceState(state: LiveLanActivityState): void {
  expect(state.lanAddDeviceReadModel?.addDeviceState).toBe('paired');
  expect(state.lanAddDeviceReadModel?.canonicalHouseholdDevices[0]?.displayName).toBe('GAMEDEV');
  expect(state.lanAddDeviceReadModel?.canonicalHouseholdDevices[0]?.roleBadges).toEqual([
    'child-agent',
    'parent-controller',
  ]);
  expect(state.lanAddDeviceReadModel?.canonicalHouseholdDevices[0]?.networkIdentity.ipAddresses).toEqual([
    '192.168.2.10',
  ]);
}

function expectLanSignedDiscoveryRelayState(state: LiveLanActivityState): void {
  expect(state.lanAddDeviceReadModel?.signedDiscoveryRelaySpine?.signedProofRows[0]).toMatchObject({
    check: 'signed-hello-manual-required',
    proofState: 'manual-required',
  });
  expect(state.lanAddDeviceReadModel?.signedDiscoveryRelaySpine?.routeSafetyRows[0]).toMatchObject({
    check: 'selected-route-custody',
    custodyLabel: 'parent-local-service',
  });
  expect(state.lanAddDeviceReadModel?.signedDiscoveryRelaySpine?.relayCacheRows[0]).toMatchObject({
    check: 'relay-route-unavailable',
    decisionState: 'unavailable',
  });
}

function activitySurfaceEvents() {
  return [
    surfaceEvent(
      AgentEvent.ActivityReportGenerated,
      AgentProtocolDefaults.Field.ActivityReportDocument,
      ReportDocument,
      'ready'
    ),
    surfaceEvent(
      AgentEvent.ActivityReportHistoryReported,
      AgentProtocolDefaults.Field.ActivityReports,
      reportHistory(),
      'unavailable'
    ),
    surfaceEvent(
      AgentEvent.ActivityScreenReadModelReported,
      AgentProtocolDefaults.Field.ActivityReadModel,
      screenReadModel(),
      'ready'
    ),
    surfaceEvent(
      AgentEvent.ActivityAppUseReadModelReported,
      AgentProtocolDefaults.Field.ActivityReadModel,
      emptyReadModel('empty'),
      'empty'
    ),
    surfaceEvent(
      AgentEvent.ActivityBrowserReadModelReported,
      AgentProtocolDefaults.Field.ActivityReadModel,
      emptyReadModel('permission-required'),
      'permission-required'
    ),
    surfaceEvent(
      AgentEvent.ActivityGamesReadModelReported,
      AgentProtocolDefaults.Field.ActivityReadModel,
      emptyReadModel('scaffold-only'),
      'scaffold-only'
    ),
    surfaceEvent(
      AgentEvent.ActivityNetworkReadModelReported,
      AgentProtocolDefaults.Field.ActivityReadModel,
      emptyReadModel('unavailable'),
      'unavailable'
    ),
  ];
}

function reportHistory() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'unavailable',
    storageState: 'storage-unavailable',
    storageReason: 'Local report storage is not wired.',
    reports: [],
  } as const;
}

function expectAdapterState(result: AdapterResult, state: string): void {
  expect(result?.ok).toBe(true);
  if (result === null || !result.ok) {
    throw new Error(`Expected successful Activity adapter result for state ${state}`);
  }
  expect(result.state).toBe(state);
}

function screenReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'ready',
    generatedAt: '2026-05-30T14:00:01Z',
    summary: 'Screen read model returned local rows',
    rows: [
      {
        rowId: 'screen-row-1',
        label: 'Foreground use',
        deviceId: 'child-device-1',
        state: 'ready',
        totalMs: 3600000,
        foregroundMs: 2400000,
        backgroundMs: 1200000,
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

function emptyReadModel(state: 'empty' | 'permission-required' | 'scaffold-only' | 'unavailable') {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state,
    generatedAt: '2026-05-30T14:00:01Z',
    summary: 'Activity adapter returned a typed fallback state',
    rows: [],
  } as const;
}

function surfaceEvent(
  event: (typeof AgentEvent)[keyof typeof AgentEvent],
  jsonField: string,
  value: object,
  state: string
) {
  return surfaceEventWithRawPayload(event, jsonField, JSON.stringify(value), state);
}

function surfaceEventWithRawPayload(
  event: (typeof AgentEvent)[keyof typeof AgentEvent],
  jsonField: string,
  value: string,
  state: string
) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: `evt-${event}`,
    correlationId: `cmd-${event}`,
    sentAt: '2026-05-30T14:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event,
    severity: state === 'ready' || state === 'empty' ? 'info' : 'warn',
    payload: {
      [AgentProtocolDefaults.Field.ActivitySurfaceState]: state,
      [jsonField]: value,
    },
    snapshot: null,
  });
}

function payloadEvent(
  event: (typeof AgentEvent)[keyof typeof AgentEvent],
  payload: Record<string, unknown>,
  sentAt = '2026-05-30T14:00:01Z'
) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: `evt-${event}`,
    correlationId: `cmd-${event}`,
    sentAt,
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event,
    severity: 'info',
    payload,
    snapshot: null,
  });
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
      scannedDeviceCount: 1,
      agentDeviceCount: 1,
      passiveDeviceCount: 0,
      infrastructureDeviceCount: 0,
      unsupportedDeviceCount: 0,
    },
    discoveredDevices: [lanDiscoveredDevice()],
    canonicalHouseholdDevices: [lanCanonicalHouseholdDevice()],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    householdDeviceDecisions: [],
    signedDiscoveryRelaySpine: signedDiscoveryRelaySpine(),
    trustedDeviceIds: ['child-android-1'],
    revokedDeviceIds: [],
    selectedDeviceReadiness: lanSelectedDeviceReadiness(),
    controllerAuthority: 'observer',
    observerAuthority: 'observer',
    routeRequirementLabels: ['Local service route only'],
    auditCheckLabels: ['No physical device-owner proof'],
    honestNonClaims: ['physical-device-owner-unavailable'],
  };
}

function lanDiscoveredDevice() {
  return {
    schemaVersion: 1,
    discoveredAt: '2026-06-01T15:00:00Z',
    childDevice: {
      deviceId: 'child-android-1',
      childProfileId: 'child-profile-1',
      label: 'Pixel child',
      platform: 'android',
    },
    agentPeerId: 'child-peer-1',
    routeId: 'lan-route-local-1',
    networkMode: 'local-network',
    reachability: 'online',
    addressRef: 'lan-address-ref-1',
    discoveryStatus: 'websocket-direct',
    discoveryState: 'paired',
  };
}

function signedDiscoveryRelaySpine() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-01T15:01:05Z',
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
        routeId: 'lan-route-local-1',
        discoveryState: 'paired',
        responseState: 'accepted',
        rejectionReason: null,
        proofState: 'ci-mechanical-proof',
        runtimeOwner: 'rust-service-read-model',
        custodyLabel: 'parent-local-service',
        evidenceLabel: 'Selected route remains parent-local custody.',
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
  };
}

function lanCanonicalHouseholdDevice() {
  return {
    schemaVersion: 1,
    canonicalDeviceId: 'child-android-1',
    displayName: 'Pixel child',
    classification: 'child-agent',
    roleBadges: ['child-agent', 'portal', 'parent-controller'],
    enrollable: true,
    discoveryState: 'paired',
    trustState: 'paired',
    routeId: 'lan-route-local-1',
    routeState: 'local-network',
    networkMode: 'local-network',
    sourceLabels: ['local-service'],
    networkIdentity: lanCanonicalNetworkIdentity(),
    childAgentInventory: lanChildAgentInventory(),
    policyTargetSurfaces: ['devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking', 'ai'],
  };
}

function lanCanonicalNetworkIdentity() {
  return {
    hostname: 'pixel-child',
    ipAddresses: ['192.168.2.44'],
    macAddress: 'AA-BB-CC-DD-EE-FF',
    macVendor: null,
    networkInterfaces: ['Wi-Fi'],
    reachability: 'online',
    confidence: 'agent-confirmed',
    staleAt: null,
    offlineAt: null,
    evidenceRecords: [lanDiscoveryEvidenceRecord()],
  };
}

function lanDiscoveryEvidenceRecord() {
  return {
    schemaVersion: 1,
    evidenceId: 'lan-evidence-local-service-child-android-1',
    source: 'local-service',
    evidenceKind: 'child-agent-presence',
    deviceId: 'child-android-1',
    value: 'child-peer-1',
    normalizedValue: 'child-peer-1',
    firstSeenAt: '2026-06-01T15:00:00Z',
    lastSeenAt: '2026-06-01T15:00:00Z',
    expiresAt: null,
    confidence: 'confirmed',
    mergeKey: 'agent:child-peer-1',
    note: 'Local service reported the child agent.',
  };
}

function lanChildAgentInventory() {
  return {
    deviceName: 'Pixel child',
    platform: 'android',
    os: 'android',
    cpuModel: null,
    cpuCores: null,
    memoryTotal: null,
    gpuModel: null,
    gpuDriver: null,
    gpuMemory: null,
    nvidiaSmi: null,
    networkInterfaces: ['Wi-Fi'],
    capabilities: ['direct-websocket', 'device-inventory', 'pairing-route'],
    roleState: 'implemented',
    routeState: 'local-network',
    pairingTrustState: 'paired',
  };
}

function lanSelectedDeviceReadiness() {
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
  };
}

import { describe, expect, it } from 'vitest';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
import {
  createParentPortalActivityUiIntent,
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
    discoveredDevices: [
      {
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
      },
      {
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
      },
    ],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    trustedDeviceIds: ['child-android-1'],
    revokedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: 1,
      selectedChildDeviceId: 'child-android-1',
      routeId: 'lan-route-local-1',
      pairingId: 'pairing-child-android-1',
      trustState: 'paired',
      reachability: 'online',
      readyForControl: true,
      staleAt: null,
      offlineAt: null,
    },
    controllerAuthority: 'observer',
    observerAuthority: 'observer',
    routeRequirementLabels: ['Local service route only'],
    auditCheckLabels: ['No physical device-owner proof'],
    honestNonClaims: ['physical-device-owner-unavailable'],
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

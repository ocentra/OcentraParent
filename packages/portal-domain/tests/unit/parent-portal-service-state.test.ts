import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PARENT_PORTAL_CONTENT,
  PARENT_PORTAL_ROUTE_CONTEXT,
  PARENT_PORTAL_SERVICE_STATE,
  PortalBrowserInventoryFields,
  PortalRoute,
  resolveParentPortalServiceState,
} from '../../src/contracts';
import { resolveLiveActivityState } from '../../src/live-activity-state';
import { parentPortalManageLaneForRoute, parentPortalRouteContext } from '../../src/parent-portal-data';
import { portalRouteFromHashPath } from '../../src/routes';

describe('portal service-backed parent portal state', () => {
  parentPortalServiceRowTests();
  parentPortalLanAddDeviceRowTests();
  parentPortalLanParserOwnershipTests();
  parentPortalActivityNetworkRowTests();
  parentPortalProductShellRowTests();
  parentPortalRouteOwnershipNormalizationTests();
  parentPortalBrowserInventoryRowTests();
});

function parentPortalServiceRowTests(): void {
  it('uses real service events as the overview and manage row source', () => {
    const state = serviceBackedState();
    expect(state.content.modes.parentOverview.rowSource).toBe('api');
    expect(state.content.modes.parentManage.rowSource).toBe('api');
    expect(state.userEntry?.label).toBe('Local agent');
    expect(state.parentPortalRows.map((row) => row.label)).toEqual(expectedServiceRowLabels());
  });

  it('maps LAN and device service events into visible row readiness', () => {
    const state = serviceBackedState();

    expect(state.parentPortalRows[1]).toMatchObject({
      readyCount: 2,
      gapCount: 0,
      trend: 'paired',
    });
    expect(state.parentPortalRows[2]).toMatchObject({
      readyCount: 3,
      gapCount: 0,
      trend: 'online',
    });
  });

  it('keeps unavailable local-service states visible instead of falling back to sample rows', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [],
    });

    expect(state.content.modes.parentOverview.rowSource).toBe('api');
    expect(state.userEntry?.trend).toBe('LOCAL');
    expect(state.parentPortalRows[1]).toMatchObject({
      label: 'LAN discovery',
      readyCount: 0,
      gapCount: 1,
      primaryArea: 'LAN',
      trend: 'manual-required',
    });
    expect(state.parentPortalRows[3]).toMatchObject({
      label: 'Browser activity',
      readyCount: 0,
      gapCount: 1,
      primaryArea: 'Browser',
      trend: 'unavailable',
    });
  });
}

function parentPortalProductShellRowTests(): void {
  it('adds route-scoped product shell rows for manage surfaces without sample fallback claims', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [],
    });
    const rowAreas = new Set(state.parentPortalRows.map((row) => normalizedPortalTarget(row.primaryArea ?? '')));
    const selectableTargets = selectableParentPortalTargets();
    const missingTargets: string[] = [];

    for (const route of productShellRoutes()) {
      const routeContext = PARENT_PORTAL_ROUTE_CONTEXT[route];
      const control = selectableTargets.get(routeContext?.selectedControlId ?? '');
      if (control === undefined) {
        throw new Error(`Missing selectable control for route ${route} and id ${routeContext?.selectedControlId ?? ''}`);
      }
      if (!rowAreas.has(normalizedPortalTarget(control?.name ?? ''))) {
        missingTargets.push(`${route}:${routeContext?.selectedControlId ?? ''}:${control?.name ?? ''}`);
      }
    }
    expect(missingTargets).toEqual([]);
    expect(rowByPrimaryArea(state.parentPortalRows, 'AI SETUP')).toMatchObject({
      label: 'Assistant entry',
      trend: 'backend-not-connected',
    });
    expect(rowByPrimaryArea(state.parentPortalRows, 'REMOTE SCREEN POLICY')).toMatchObject({
      label: 'Remote screen policy',
      trend: 'backend-not-connected',
    });
  });

  it('uses policy and assistant events when those product shell backends report', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(AgentEvent.PolicyPreviewReadModelReported, {
          [AgentProtocolDefaults.Field.PolicyHandoffState]: 'observer-only',
        }),
        payloadEvent(AgentEvent.ParentAssistantProviderDegraded, {
          [AgentProtocolDefaults.Field.ParentAssistantBackendState]: 'degraded',
        }),
      ],
    });

    expect(rowByPrimaryArea(state.parentPortalRows, 'APP POLICY')).toMatchObject({
      label: 'App policy',
      readyCount: 1,
      gapCount: 0,
      trend: 'observer-only',
    });
    expect(rowByPrimaryArea(state.parentPortalRows, 'AI SETUP')).toMatchObject({
      label: 'Assistant entry',
      readyCount: 1,
      gapCount: 0,
      trend: 'degraded',
    });
  });
}

function parentPortalRouteOwnershipNormalizationTests(): void {
  it('keeps manage-lane ownership in portal-domain route contracts', () => {
    expect(parentPortalManageLaneForRoute(PortalRoute.Browser)).toBe('childPolicy');
    expect(parentPortalManageLaneForRoute(PortalRoute.Notifications)).toBe('portal');
    expect(parentPortalManageLaneForRoute(PortalRoute.Subscription)).toBe('portal');
    expect(parentPortalManageLaneForRoute(PortalRoute.Devices)).toBe('deviceOps');
    expect(parentPortalManageLaneForRoute(PortalRoute.PolicyTracking)).toBe('childPolicy');
    expect(parentPortalManageLaneForRoute(PortalRoute.Overview)).toBeNull();
  });

  it('parses portal hash routes and hydrates normalized current-route lane context', () => {
    expect(portalRouteFromHashPath('#/browser?panel=inventory')).toBe(PortalRoute.Browser);
    expect(portalRouteFromHashPath('#/notification-channels?panel=alerts')).toBe(PortalRoute.NotificationChannels);
    expect(portalRouteFromHashPath('#/subscription')).toBe(PortalRoute.Subscription);
    expect(portalRouteFromHashPath('#/unknown')).toBeNull();
    expect(parentPortalRouteContext(PortalRoute.Browser)).toMatchObject({
      selectedControlId: 'managed-web',
      manageLane: 'childPolicy',
    });
    expect(parentPortalRouteContext(PortalRoute.Notifications)).toMatchObject({
      selectedControlId: 'notifications',
      manageLane: 'portal',
    });
    expect(parentPortalRouteContext(PortalRoute.Subscription)).toMatchObject({
      selectedControlId: 'subscription-plans',
      manageLane: 'portal',
    });
    expect(parentPortalRouteContext(PortalRoute.Devices)).toMatchObject({
      selectedControlId: 'lan-pairing',
      manageLane: 'deviceOps',
    });
    expect(parentPortalRouteContext(PortalRoute.PolicyTracking)).toMatchObject({
      selectedControlId: 'policy-tracking',
      manageLane: 'childPolicy',
    });
    expect(parentPortalRouteContext(PortalRoute.Overview).manageLane).toBeNull();
  });
}

function parentPortalBrowserInventoryRowTests(): void {
  it('surfaces browser inventory capability rows without exact URL or active-tab overclaims', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(AgentEvent.BrowserInventoryReadModelReported, {
          [AgentProtocolDefaults.Field.Returned]: 1,
          [AgentProtocolDefaults.Field.CapabilityStatus]: 'unmanaged-browser',
          [PortalBrowserInventoryFields.RunningState]: 'running-unmanaged',
          [PortalBrowserInventoryFields.ExactUrlCapability]: 'not-claimed',
          [PortalBrowserInventoryFields.ActiveTabCapability]: 'not-claimed',
          [PortalBrowserInventoryFields.UnmanagedFallbackCapability]: 'report-only',
        }),
      ],
    });

    expect(rowByLabel(state.parentPortalRows, 'Browser inventory')).toMatchObject({
      label: 'Browser inventory',
      primaryArea: 'Managed Web',
      readyCount: 1,
      gapCount: 0,
      trend: 'running-unmanaged',
    });
    expect(rowByLabel(state.parentPortalRows, 'Exact URL capability')).toMatchObject({
      label: 'Exact URL capability',
      primaryArea: 'Managed Web',
      readyCount: 1,
      gapCount: 1,
      trend: 'not-claimed',
    });
    expect(rowByLabel(state.parentPortalRows, 'Active tab proof')).toMatchObject({
      label: 'Active tab proof',
      primaryArea: 'Managed Web',
      readyCount: 1,
      gapCount: 1,
      trend: 'not-claimed',
    });
    expect(rowByLabel(state.parentPortalRows, 'Unmanaged fallback')).toMatchObject({
      label: 'Unmanaged fallback',
      primaryArea: 'Managed Web',
      readyCount: 1,
      gapCount: 0,
      trend: 'report-only',
    });
  });
}

function parentPortalLanAddDeviceRowTests(): void {
  parentPortalLanSelectedDeviceRowTests();
  parentPortalLanDiscoveryScanRowTests();
}

function parentPortalLanParserOwnershipTests(): void {
  it('recovers legacy LAN add-device payloads inside portal-domain without an app-owned parser', () => {
    const state = resolveLiveActivityState([
      payloadEvent(AgentEvent.LanPairingStatusReported, {
        [AgentProtocolDefaults.Field.LanAddDeviceReadModel]: JSON.stringify(legacyLanAddDeviceReadModel()),
      }),
    ]);

    expect(state.lanPairingStatusEvent?.event).toBe(AgentEvent.LanPairingStatusReported);
    expect(state.lanAddDeviceReadModel?.addDeviceState).toBe('paired');
    expect(state.lanAddDeviceReadModel?.scanSummary.scannedDeviceCount).toBe(3);
    expect(state.lanAddDeviceReadModel?.selectedDeviceReadiness.routeId).toBe('lan-route-local-network');
    expect(state.lanAddDeviceReadModel?.canonicalHouseholdDevices[0]?.displayName).toBe('GAMEDEV');
    expect(state.lanAddDeviceReadModel?.signedDiscoveryRelaySpine?.signedProofRows[0]?.proofState).toBe(
      'manual-required'
    );
  });

  it('keeps incomplete legacy LAN payloads unclaimed inside portal-domain', () => {
    const state = resolveLiveActivityState([
      payloadEvent(AgentEvent.LanPairingStatusReported, {
        [AgentProtocolDefaults.Field.LanAddDeviceReadModel]: JSON.stringify({
          ...legacyLanAddDeviceReadModel(),
          scanSummary: null,
        }),
      }),
    ]);

    expect(state.lanPairingStatusEvent?.event).toBe(AgentEvent.LanPairingStatusReported);
    expect(state.lanAddDeviceReadModel).toBeNull();
  });
}

function parentPortalLanSelectedDeviceRowTests(): void {
  it('prefers service add-device readiness and selected-device state for LAN rows', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(AgentEvent.LanPairingStatusReported, {
          [AgentProtocolDefaults.Field.LanAddDeviceState]: 'manual-required',
          [AgentProtocolDefaults.Field.LanLocalServiceDiscoveryState]: 'manual-required',
          [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 0,
          [AgentProtocolDefaults.Field.LanPendingPairingCount]: 1,
          [AgentProtocolDefaults.Field.LanSelectedChildDeviceId]: 'child-android-1',
          [AgentProtocolDefaults.Field.LanSelectedDeviceReady]: false,
          [AgentProtocolDefaults.Field.LanSelectedDeviceReachability]: 'stale',
        }),
      ],
    });

    expect(state.parentPortalRows[1]).toMatchObject({
      label: 'LAN discovery',
      readyCount: 2,
      gapCount: 1,
      trend: 'manual-required',
    });
    expect(state.parentPortalRows[2]).toMatchObject({
      label: 'Device pairing',
      readyCount: 1,
      gapCount: 0,
      trend: 'stale',
    });
  });

  it('surfaces selected ready-for-control as a visible ready current-device state', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(AgentEvent.LanPairingStatusReported, {
          [AgentProtocolDefaults.Field.LanAddDeviceState]: 'paired',
          [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 1,
          [AgentProtocolDefaults.Field.LanSelectedChildDeviceId]: 'child-android-1',
          [AgentProtocolDefaults.Field.LanSelectedDeviceReady]: true,
          [AgentProtocolDefaults.Field.LanSelectedDeviceReachability]: 'online',
        }),
      ],
    });

    expect(state.parentPortalRows[2]).toMatchObject({
      label: 'Device pairing',
      readyCount: 2,
      trend: 'ready',
    });
  });
}

function parentPortalLanDiscoveryScanRowTests(): void {
  it('uses explicit LAN discovery scan reports for visible device rows', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(
          AgentEvent.LanPairingStatusReported,
          {
            [AgentProtocolDefaults.Field.LanAddDeviceState]: 'manual-required',
            [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 0,
          },
          '2026-06-01T13:55:00Z'
        ),
        payloadEvent(
          AgentEvent.LanPairingBrowserDiscoveryReported,
          {
            [AgentProtocolDefaults.Field.LanAddDeviceState]: 'paired',
            [AgentProtocolDefaults.Field.LanAddDeviceReadModel]: lanReadModelWithScanSummary(4),
            [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 1,
            [AgentProtocolDefaults.Field.LanSelectedChildDeviceId]: 'local-dev-agent',
            [AgentProtocolDefaults.Field.LanSelectedDeviceReachability]: 'online',
          },
          '2026-06-01T13:55:04Z'
        ),
      ],
    });

    expect(state.parentPortalRows[1]).toMatchObject({
      label: 'LAN discovery',
      readyCount: 4,
      trend: 'paired',
    });
    expect(state.parentPortalRows[2]).toMatchObject({
      label: 'Device pairing',
      readyCount: 2,
      trend: 'online',
    });
  });
}

function lanReadModelWithScanSummary(scannedDeviceCount: number): string {
  return JSON.stringify({
    schemaVersion: 1,
    generatedAt: '2026-06-01T13:55:04Z',
    discoverySource: 'physical-household-lan',
    addDeviceState: 'paired',
    localServiceDiscoveryState: 'paired',
    physicalHouseholdLanState: 'discovered',
    cloudRelayState: 'unavailable',
    scanSummary: {
      schemaVersion: 1,
      sourceLabels: ['local-service', 'windows-neighbor-table'],
      scannedDeviceCount,
      agentDeviceCount: 1,
      passiveDeviceCount: 2,
      infrastructureDeviceCount: 1,
      unsupportedDeviceCount: 3,
    },
    discoveredDevices: [],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    trustedDeviceIds: [],
    revokedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: 1,
      selectedChildDeviceId: null,
      routeId: null,
      pairingId: null,
      trustState: 'unpaired',
      reachability: 'offline',
      readyForControl: false,
      staleAt: null,
      offlineAt: null,
    },
    controllerAuthority: 'active-controller',
    observerAuthority: 'observer',
    routeRequirementLabels: [],
    auditCheckLabels: [],
    honestNonClaims: [],
  });
}

function legacyLanAddDeviceReadModel() {
  return {
    addDeviceState: 'paired',
    discoverySource: 'physical-household-lan',
    localServiceDiscoveryState: 'paired',
    physicalHouseholdLanState: 'discovered',
    cloudRelayState: 'unavailable',
    scanSummary: {
      schemaVersion: 1,
      sourceLabels: ['local-service', 'windows-neighbor-table'],
      scannedDeviceCount: 3,
      agentDeviceCount: 1,
      passiveDeviceCount: 1,
      infrastructureDeviceCount: 1,
      unsupportedDeviceCount: 0,
    },
    discoveredDevices: [
      {
        deviceId: 'local-dev-agent',
        displayName: 'GAMEDEV',
        reachability: 'online',
      },
    ],
    canonicalHouseholdDevices: [
      {
        displayName: 'GAMEDEV',
        roleBadges: ['child-agent'],
        sourceLabels: ['local-service'],
        networkIdentity: {
          ipAddresses: ['192.168.2.45'],
          reachability: 'online',
          evidenceRecords: [],
        },
      },
    ],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    householdDeviceDecisions: [],
    signedDiscoveryRelaySpine: {
      signedProofRows: [
        {
          check: 'signed-hello-manual-required',
          proofState: 'manual-required',
        },
      ],
      routeSafetyRows: [
        {
          check: 'route-custody-manual-required',
          custodyLabel: 'manual-proof',
        },
      ],
      relayCacheRows: [
        {
          check: 'relay-cache-unavailable',
          decisionState: 'manual-required',
        },
      ],
    },
    trustedDeviceIds: ['local-dev-agent'],
    revokedDeviceIds: [],
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
    controllerAuthority: 'parent-approved',
    observerAuthority: 'family-observer',
    routeRequirementLabels: ['Only trusted signed child-agent routes become controllable'],
    auditCheckLabels: ['Signed hello manual proof required'],
    honestNonClaims: ['Signed child hello still requires a second device for proof'],
  } as const;
}

function parentPortalActivityNetworkRowTests(): void {
  it('surfaces degraded Activity and network adapter states from service events', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(AgentEvent.ActivityBrowserReadModelReported, {
          [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'permission-required',
        }),
        payloadEvent(AgentEvent.ActivityNetworkReadModelReported, {
          [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'unavailable',
        }),
      ],
    });

    expect(state.parentPortalRows[4]).toMatchObject({
      label: 'Activity reports',
      primaryArea: 'Activity',
      trend: 'permission-required',
    });
    expect(state.parentPortalRows[5]).toMatchObject({
      label: 'Network tracking',
      primaryArea: 'Network',
      trend: 'unavailable',
    });
  });
}

function expectedServiceRowLabels(): string[] {
  return [
    'Local agent',
    'LAN discovery',
    'Device pairing',
    'Browser activity',
    'Activity reports',
    'Network tracking',
    'Household setup',
    'Household setup',
    'Browser inventory',
    'Exact URL capability',
    'Active tab proof',
    'Unmanaged fallback',
    'Managed web path',
    'Browser setup',
    'Activity store',
    'App and game sessions',
    'Reports surface',
    'App policy',
    'Game policy',
    'Screen analysis',
    'Network activity',
    'Tracking policy',
    'Remote screen policy',
    'Schedule plan',
    'Approval queue',
    'Enforcement readiness',
    'Assistant entry',
    'API providers',
    'API providers',
    'Memory setup',
    'Data custody',
    'Data custody',
    'Export retention',
    'Alerts',
    'Notification channels',
    'Remote access',
    'Audit history',
    'Support',
    'Subscription',
    'Entitlements',
    'Device pairing',
    'LAN discovery',
    'Household setup',
  ];
}

function productShellRoutes(): PortalRoute[] {
  return [
    PortalRoute.Overview,
    PortalRoute.Activity,
    PortalRoute.Browser,
    PortalRoute.BrowserSettings,
    PortalRoute.PolicyApps,
    PortalRoute.PolicyGames,
    PortalRoute.PolicyScreen,
    PortalRoute.PolicyNetwork,
    PortalRoute.PolicyTracking,
    PortalRoute.PolicyRemoteScreen,
    PortalRoute.Devices,
    PortalRoute.LanPairing,
    PortalRoute.CapabilityStatus,
    PortalRoute.Notifications,
    PortalRoute.NotificationChannels,
    PortalRoute.DriveConnections,
    PortalRoute.ExportRetention,
    PortalRoute.RemoteAccess,
    PortalRoute.AuditHistory,
    PortalRoute.Subscription,
    PortalRoute.Entitlements,
    PortalRoute.Diagnostics,
    PortalRoute.SettingsRules,
    PortalRoute.AiRuntime,
    PortalRoute.ApiProviders,
    PortalRoute.MemorySettings,
  ];
}

function selectableParentPortalTargets(): ReadonlyMap<string, { readonly name: string }> {
  const controls = [...PARENT_PORTAL_CONTENT.controlAreas, ...PARENT_PORTAL_CONTENT.quickControls];
  return new Map(controls.map((control) => [control.id, { name: control.name }]));
}

function rowByPrimaryArea(rows: readonly { readonly primaryArea?: string }[], primaryArea: string) {
  const normalized = normalizedPortalTarget(primaryArea);
  const row = rows.find((entry) => normalizedPortalTarget(entry.primaryArea ?? '') === normalized);
  if (row === undefined) {
    throw new Error(`Missing parent portal row for primary area ${primaryArea}`);
  }
  return row;
}

function rowByLabel(rows: readonly { readonly label?: string }[], label: string) {
  const normalized = normalizedPortalTarget(label);
  const row = rows.find((entry) => normalizedPortalTarget(entry.label ?? '') === normalized);
  if (row === undefined) {
    throw new Error(`Missing parent portal row for label ${label}`);
  }
  return row;
}

function normalizedPortalTarget(value: string): string {
  return value.toLowerCase().replace(/[^a-z0-9]+/g, '');
}

function serviceBackedState() {
  return resolveParentPortalServiceState({
    connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
    events: [
      lanStatusEvent(),
      payloadEvent(AgentEvent.BrowserManagedStatusReported, {
        [AgentProtocolDefaults.Field.ManagedState]: 'managed',
      }),
      payloadEvent(AgentEvent.ActivityRecentSummaryReported, {
        [AgentProtocolDefaults.Field.Returned]: 2,
      }),
      payloadEvent(AgentEvent.NetworkFlowReadModelReported, {
        [AgentProtocolDefaults.Field.Returned]: 3,
        [AgentProtocolDefaults.Field.CapabilityStatus]: 'ready',
      }),
    ],
  });
}

function lanStatusEvent(): AgentEventEnvelope {
  return payloadEvent(AgentEvent.LanPairingStatusReported, {
    [AgentProtocolDefaults.Field.LanDiscoveryState]: 'paired',
    [AgentProtocolDefaults.Field.LanPairingState]: 'paired',
    [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 2,
    [AgentProtocolDefaults.Field.LanSelectedChildDeviceId]: 'child-device-1',
    [AgentProtocolDefaults.Field.LanSelectedDeviceReachability]: 'online',
  });
}

function payloadEvent(
  event: AgentEventName,
  payload: AgentProtocolLogFields,
  sentAt = '2026-06-01T13:55:00Z'
): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: `evt-${event}`,
    correlationId: `cmd-${event}`,
    sentAt,
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event,
    severity: 'info',
    payload,
    snapshot: null,
  });
}

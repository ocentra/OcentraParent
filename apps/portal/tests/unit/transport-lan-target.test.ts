import { afterEach, expect, it, vi } from 'vitest';
import {
  ParentAgentCommand,
  ParentAgentProtocolField,
  ParentAgentTargetDefaults,
  ParentHostBridgeRuntime,
  ParentPortalParentAccessState,
  ParentRoute,
  ParentUiActionKind,
  presentationOnlyDevWebHostBridgeMessage,
  type ParentLanAddDeviceReadModelSnapshot,
  type ParentRouteSnapshot,
  type ParentUiActionResult,
} from '../../generated/parent-ui-bridge';
import { createDevWebHostBridge } from '../../src/host-bridge';

afterEach(() => {
  vi.unstubAllGlobals();
  vi.useRealTimers();
});

it('HostBridge dev adapter: rejects route loads when no dev bridge URL is configured', async () => {
  const bridge = createDevWebHostBridge(null);

  await expect(bridge.loadRoute(ParentRoute.Devices)).rejects.toThrow(presentationOnlyDevWebHostBridgeMessage());
});

it('HostBridge dev adapter: rejects action dispatch when no dev bridge URL is configured', async () => {
  const bridge = createDevWebHostBridge(null);

  await expect(
    bridge.dispatch({
      action: ParentUiActionKind.RefreshRoute,
      route: ParentRoute.Devices,
      payload: {},
    })
  ).rejects.toThrow(presentationOnlyDevWebHostBridgeMessage());
});

it('HostBridge dev adapter: rejects subscriptions when no dev bridge URL is configured', async () => {
  const bridge = createDevWebHostBridge(null);

  await expect(bridge.subscribe(ParentRoute.Devices, {}, () => undefined)).rejects.toThrow(
    presentationOnlyDevWebHostBridgeMessage()
  );
});

it('HostBridge dev adapter: loads Rust-owned route snapshots through the local dev bridge when configured', async () => {
  const bridge = createDevWebHostBridge('http://127.0.0.1:4779/api/parent-ui');
  vi.stubGlobal('WebSocket', ThrowingWebSocket);
  vi.stubGlobal('fetch', fetchResponder(jsonResponse(devBridgeSnapshot('connected', 'host-bridge', 'lan-device-1'))));

  const snapshot = await bridge.loadRoute('devices');

  expect(snapshot.connectionState).toBe('connected');
  expect(snapshot.dataSource).toBe('host-bridge');
  expect(snapshot.liveActivity?.lanAddDeviceReadModel).toMatchObject({
    addDeviceState: 'discovered',
  });
});

it('HostBridge dev adapter: dispatches through the Rust-owned local dev bridge when configured', async () => {
  const bridge = createDevWebHostBridge('http://127.0.0.1:4779/api/parent-ui');
  vi.stubGlobal('WebSocket', ThrowingWebSocket);
  vi.stubGlobal(
    'fetch',
    fetchResponder(
      jsonResponse({
        schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
        accepted: true,
        connectionState: 'connected',
        message: 'scan-requested',
        snapshot: devBridgeSnapshot('connected', 'rust-read-model', 'lan-device-1'),
        events: [],
      } satisfies ParentUiActionResult)
    )
  );

  const result = await bridge.dispatch({
    action: 'agent-command-requested',
    route: 'devices',
    command: ParentAgentCommand.LanPairingBrowserDiscoveryScan,
    payload: {
      [ParentAgentProtocolField.LanRouteId]: ParentAgentTargetDefaults.LocalNetworkWindowsAgent.route,
    },
  });

  expect(result).toMatchObject({
    accepted: true,
    connectionState: 'connected',
    message: 'scan-requested',
    snapshot: {
      route: 'devices',
      dataSource: 'rust-read-model',
    },
  });
});

it('HostBridge dev adapter: polls the local dev bridge and emits changed snapshots only', async () => {
  vi.useFakeTimers();
  const bridge = createDevWebHostBridge('http://127.0.0.1:4779/api/parent-ui');
  const fetchResponses = [
    jsonResponse(devBridgeSnapshot('connected', 'host-bridge', 'lan-device-1')),
    jsonResponse(devBridgeSnapshot('connected', 'host-bridge', 'lan-device-1')),
    jsonResponse(devBridgeSnapshot('connected', 'rust-read-model', 'lan-device-2')),
  ];
  vi.stubGlobal('fetch', fetchSequence(fetchResponses));

  const events: unknown[] = [];
  const unsubscribe = await bridge.subscribe('devices', {}, (event) => {
    events.push(event);
  });

  await Promise.resolve();
  await Promise.resolve();
  await vi.advanceTimersByTimeAsync(1000);
  await vi.advanceTimersByTimeAsync(1000);
  unsubscribe();

  expect(events).toMatchObject([
    {
      schemaVersion: 1,
      route: 'devices',
      snapshot: {
        dataSource: 'host-bridge',
        liveActivity: {
          lanAddDeviceReadModel: {
            discoveredDevices: [{ childDevice: { deviceId: 'lan-device-1' } }],
          },
        },
      },
    },
    {
      schemaVersion: 1,
      route: 'devices',
      snapshot: {
        dataSource: 'rust-read-model',
        liveActivity: {
          lanAddDeviceReadModel: {
            discoveredDevices: [{ childDevice: { deviceId: 'lan-device-2' } }],
          },
        },
      },
    },
  ]);
});

function devBridgeSnapshot(
  connectionState: ParentRouteSnapshot['connectionState'],
  dataSource: ParentRouteSnapshot['dataSource'],
  deviceId: string
): ParentRouteSnapshot {
  return {
    schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
    route: ParentRoute.Devices,
    generatedAt: '2026-06-24T12:00:00.000Z',
    seasonLabel: 'LOCAL',
    lastUpdated: '2026-06-24T12:00:00.000Z',
    connectionState,
    commandEnabled: true,
    agentEndpoint: 'host-bridge://tauri-parent',
    dataSource,
    summary: {
      title: 'Devices',
      routeCapability: 'available',
      parentAccess: 'paired',
      household: 'available',
      childDevice: 'available',
    },
    diagnosticPanelsEnabled: false,
    parentPortalRows: [],
    parentPortalShellStatus: {
      routeLabel: 'Devices',
      parentAccessState: ParentPortalParentAccessState.ObserverOnly,
      globalConnectionState: connectionState,
      routeCapabilityState: 'available',
      dataSourceLabel: dataSource,
      cards: [],
    },
    serviceHealth: null,
    parentDesktopDistribution: null,
    liveActivity: { lanAddDeviceReadModel: lanAddDeviceReadModel(deviceId) },
    browserPanels: null,
    setupFirstRunPanel: null,
    screenSettingsServiceResponse: null,
  };
}

function lanAddDeviceReadModel(deviceId: string): ParentLanAddDeviceReadModelSnapshot {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-24T12:00:00.000Z',
    discoverySource: 'local-dev-bridge',
    addDeviceState: 'discovered',
    localServiceDiscoveryState: 'discovered',
    physicalHouseholdLanState: 'unavailable',
    cloudRelayState: 'unavailable',
    scanSummary: {
      schemaVersion: 1,
      sourceLabels: ['local-dev-bridge'],
      scannedDeviceCount: 1,
      agentDeviceCount: 0,
      passiveDeviceCount: 1,
      infrastructureDeviceCount: 0,
      unsupportedDeviceCount: 0,
    },
    discoveredDevices: [
      {
        schemaVersion: 1,
        discoveredAt: '2026-06-24T12:00:00.000Z',
        childDevice: {
          deviceId,
          childProfileId: null,
          label: deviceId,
          platform: 'unknown',
          ipAddress: null,
          macAddress: null,
          hostname: null,
          networkInterface: null,
          agentStatus: null,
        },
        agentPeerId: 'parent-dev-bridge',
        routeId: 'local-network',
        networkMode: 'local-network',
        reachability: 'observed',
        addressRef: 'redacted-local-address',
        discoveryStatus: 'network-neighbor',
        discoveryState: 'discovered',
        evidenceSources: ['local-dev-bridge'],
        serviceIdentityProbeEvidence: [],
        hintSources: [],
      },
    ],
    discoveryEventHistory: {
      schemaVersion: 1,
      generatedAt: '2026-06-24T12:00:00.000Z',
      state: 'current',
      latestEventId: null,
      latestObservedAt: '2026-06-24T12:00:00.000Z',
      rows: [],
    },
    canonicalHouseholdDevices: [],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    householdDeviceDecisions: [],
    signedDiscoveryRelaySpine: null,
    lanDiscoverySourceMatrix: null,
    trustedDeviceIds: [],
    revokedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: 1,
      selectedChildDeviceId: deviceId,
      routeId: 'local-network',
      pairingId: null,
      trustState: 'unpaired',
      reachability: 'observed',
      readyForControl: false,
      staleAt: null,
      offlineAt: null,
    },
    controllerAuthority: 'unavailable',
    observerAuthority: 'local-dev-bridge',
    routeRequirementLabels: ['Authenticated parent-local route required'],
    auditCheckLabels: ['No control authority claimed'],
    honestNonClaims: ['Discovery observation is not pairing or control authority'],
  };
}

function jsonResponse(payload: unknown): Response {
  return {
    ok: true,
    status: 200,
    json: async () => payload,
  } as Response;
}

function fetchResponder(response: Response): typeof fetch {
  return async () => response;
}

function fetchSequence(responses: Response[]): typeof fetch {
  let nextResponseIndex = 0;
  if (responses.length === 0) {
    throw new Error('Expected at least one stubbed fetch response.');
  }
  return async () => {
    const response = responses[nextResponseIndex] ?? responses[responses.length - 1];
    if (response === undefined) {
      throw new Error('Expected a stubbed fetch response.');
    }
    nextResponseIndex += 1;
    return response;
  };
}

class ThrowingWebSocket {
  constructor() {
    throw new Error('WebSocket transport should not be used for LAN dev bridge snapshots.');
  }
}

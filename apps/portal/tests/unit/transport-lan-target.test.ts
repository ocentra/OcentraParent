import { afterEach, expect, it, vi } from 'vitest';
import {
  ParentAgentCommand,
  ParentAgentProtocolField,
  ParentAgentTargetDefaults,
  ParentRoute,
  ParentUiActionKind,
  presentationOnlyDevWebHostBridgeMessage,
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
  vi.stubGlobal(
    'fetch',
    fetchResponder(
      jsonResponse({
        schemaVersion: 1,
        route: 'devices',
        generatedAt: '2026-06-24T12:00:00.000Z',
        seasonLabel: 'LOCAL',
        lastUpdated: '2026-06-24T12:00:00.000Z',
        connectionState: 'connected',
        commandEnabled: true,
        agentEndpoint: 'host-bridge://tauri-parent',
        dataSource: 'host-bridge',
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
          parentAccessState: 'paired',
          globalConnectionState: 'connected',
          routeCapabilityState: 'available',
          dataSourceLabel: 'host-bridge',
          cards: [],
        },
        liveActivity: {
          lanAddDeviceReadModel: {
            addDeviceState: 'discovered',
            discoveredDevices: [{ childDevice: { deviceId: 'lan-device-1' } }],
          },
        },
        browserPanels: null,
        screenSettingsServiceResponse: null,
      })
    )
  );

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
        schemaVersion: 1,
        accepted: true,
        connectionState: 'connected',
        message: 'scan-requested',
        snapshot: {
          schemaVersion: 1,
          route: 'devices',
          generatedAt: '2026-06-24T12:00:00.000Z',
          seasonLabel: 'LOCAL',
          lastUpdated: '2026-06-24T12:00:00.000Z',
          connectionState: 'connected',
          commandEnabled: true,
          agentEndpoint: 'host-bridge://tauri-parent',
          dataSource: 'rust-read-model',
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
            parentAccessState: 'paired',
            globalConnectionState: 'connected',
            routeCapabilityState: 'available',
            dataSourceLabel: 'rust-read-model',
            cards: [],
          },
          liveActivity: null,
          browserPanels: null,
          screenSettingsServiceResponse: null,
        },
      })
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

function devBridgeSnapshot(connectionState: string, dataSource: string, deviceId: string) {
  return {
    schemaVersion: 1,
    route: 'devices',
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
      parentAccessState: 'paired',
      globalConnectionState: connectionState,
      routeCapabilityState: 'available',
      dataSourceLabel: dataSource,
      cards: [],
    },
    liveActivity: {
      lanAddDeviceReadModel: {
        addDeviceState: 'discovered',
        discoveredDevices: [{ childDevice: { deviceId } }],
      },
    },
    browserPanels: null,
    screenSettingsServiceResponse: null,
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

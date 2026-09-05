import { createServer, type IncomingMessage, type ServerResponse } from 'node:http';
import { describe, expect, it } from 'vitest';
import {
  ParentBridgeConnectionState,
  ParentRoute,
  ParentRouteDataSource,
  ParentHostBridgeRuntime,
  ParentServiceHealthAuthenticationState,
  ParentServiceHealthReason,
  ParentServiceHealthState,
  ParentUiActionKind,
  parentDevBridgeDispatchUnavailableMessage,
  presentationOnlyDevWebHostBridgeMessage,
} from '../../generated/parent-ui-bridge';
import { createDevWebHostBridge } from '../../src/host-bridge';

describe('host bridge', () => {
  it('rejects when the dev web bridge URL is missing instead of fabricating a product snapshot', async () => {
    const bridge = createDevWebHostBridge(null);

    await expect(bridge.loadRoute(ParentRoute.Overview)).rejects.toThrow(presentationOnlyDevWebHostBridgeMessage());
    await expect(
      bridge.dispatch({
        action: ParentUiActionKind.RefreshRoute,
        route: ParentRoute.Overview,
        payload: {},
      })
    ).rejects.toThrow(presentationOnlyDevWebHostBridgeMessage());
    await expect(bridge.subscribe(ParentRoute.Overview, {}, () => undefined)).rejects.toThrow(
      presentationOnlyDevWebHostBridgeMessage()
    );
  });

  it('returns an unavailable snapshot when the configured dev web bridge is unreachable and rejects dispatch', async () => {
    const parentDevBridgeUrl = 'http://127.0.0.1:1';
    const bridge = createDevWebHostBridge(parentDevBridgeUrl);

    const snapshot = await bridge.loadRoute(ParentRoute.Overview);
    expect(snapshot.connectionState).toBe(ParentBridgeConnectionState.Error);
    expect(snapshot.dataSource).toBe(ParentRouteDataSource.Unavailable);
    expect(snapshot.commandEnabled).toBe(false);
    expect(snapshot.serviceHealth?.state).toBe(ParentServiceHealthState.Unavailable);
    expect(snapshot.serviceHealth?.authenticationState).toBe(ParentServiceHealthAuthenticationState.Unavailable);
    expect(snapshot.serviceHealth?.reason).toBe(ParentServiceHealthReason.TransportUnavailable);
    expect(snapshot.summary.routeCapability).toBe('unavailable');
    expect(snapshot.summary.parentAccess).toBe('unavailable');
    expect(snapshot.summary.household).toBe('unavailable');
    expect(snapshot.summary.childDevice).toBe('unavailable');
    expect(snapshot.summary.title).toContain(parentDevBridgeUrl);
    await expect(
      bridge.dispatch({
        action: ParentUiActionKind.RefreshRoute,
        route: ParentRoute.Overview,
        payload: {},
      })
    ).rejects.toThrow(parentDevBridgeDispatchUnavailableMessage(parentDevBridgeUrl));
  });

  it('fails closed on wrong-route load and malformed dispatch payloads from a real local HTTP bridge', async () => {
    await withParentDevBridge(mismatchedBridgeResponse, async (parentDevBridgeUrl) => {
      const bridge = createDevWebHostBridge(parentDevBridgeUrl);

      const snapshot = await bridge.loadRoute(ParentRoute.Overview);
      expect(snapshot.route).toBe(ParentRoute.Overview);
      expect(snapshot.connectionState).toBe(ParentBridgeConnectionState.Error);
      expect(snapshot.dataSource).toBe(ParentRouteDataSource.Unavailable);
      expect(snapshot.commandEnabled).toBe(false);
      expect(snapshot.serviceHealth?.reason).toBe(ParentServiceHealthReason.ResponseIdentityMismatch);
      expect(snapshot.summary.title).toContain('did not match the requested Rust-owned route');
      await expect(
        bridge.dispatch({
          action: ParentUiActionKind.RefreshRoute,
          route: ParentRoute.Overview,
          payload: {},
        })
      ).rejects.toThrow('parent UI action result does not match the Rust-owned contract');
    });
  });

  it('reports invalid JSON as a schema mismatch instead of transport state', async () => {
    await withParentDevBridge(invalidJsonBridgeResponse, async (parentDevBridgeUrl) => {
      const snapshot = await createDevWebHostBridge(parentDevBridgeUrl).loadRoute(ParentRoute.Overview);

      expect(snapshot.serviceHealth?.reason).toBe(ParentServiceHealthReason.ResponseSchemaMismatch);
      expect(snapshot.summary.title).toContain('failed Rust-owned schema decoding');
    });
  });
});

function mismatchedBridgeResponse(request: IncomingMessage, response: ServerResponse): void {
  response.statusCode = 200;
  response.setHeader('content-type', 'application/json');
  if (request.url?.endsWith('/load-route') === true) {
    response.end(JSON.stringify(validRouteSnapshot(ParentRoute.Devices)));
    return;
  }
  response.end(
    JSON.stringify({
      schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
      accepted: 'yes',
      connectionState: ParentBridgeConnectionState.Connected,
      message: 'malformed',
      snapshot: null,
      events: [],
    })
  );
}

function invalidJsonBridgeResponse(_request: IncomingMessage, response: ServerResponse): void {
  response.statusCode = 200;
  response.setHeader('content-type', 'application/json');
  response.end('{');
}

async function withParentDevBridge(
  handler: (request: IncomingMessage, response: ServerResponse) => void,
  run: (parentDevBridgeUrl: string) => Promise<void>
): Promise<void> {
  const server = createServer(handler);
  await new Promise<void>((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', resolve);
  });
  const address = server.address();
  if (address === null || typeof address === 'string') {
    await closeServer(server);
    throw new Error('local parent bridge did not publish a TCP address');
  }
  try {
    await run(`http://127.0.0.1:${address.port}`);
  } finally {
    await closeServer(server);
  }
}

async function closeServer(server: ReturnType<typeof createServer>): Promise<void> {
  await new Promise<void>((resolve, reject) => {
    server.close((error) => {
      if (error) {
        reject(error);
        return;
      }
      resolve();
    });
  });
}

function validRouteSnapshot(route: string): Readonly<Record<string, unknown>> {
  return {
    schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
    route,
    generatedAt: '2026-08-30T10:00:00Z',
    seasonLabel: 'LOCAL',
    lastUpdated: '2026-08-30T10:00:00Z',
    connectionState: ParentBridgeConnectionState.Connected,
    commandEnabled: false,
    agentEndpoint: 'parent-local-bridge',
    dataSource: ParentRouteDataSource.HostBridge,
    summary: {
      title: 'Parent route',
      routeCapability: 'read-only',
      parentAccess: 'available',
      household: 'not reported',
      childDevice: 'not reported',
    },
    serviceHealth: null,
    parentDesktopDistribution: null,
    diagnosticPanelsEnabled: false,
    parentPortalRows: null,
    parentPortalShellStatus: null,
    liveActivity: null,
    browserPanels: null,
    setupFirstRunPanel: null,
    screenSettingsServiceResponse: null,
  };
}

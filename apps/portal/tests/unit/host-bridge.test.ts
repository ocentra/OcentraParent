import { describe, expect, it } from 'vitest';
import {
  ParentBridgeConnectionState,
  ParentRoute,
  ParentRouteDataSource,
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
    expect(snapshot.serviceHealth?.state).toBe('unavailable');
    expect(snapshot.serviceHealth?.authenticationState).toBe('unavailable');
    expect(snapshot.serviceHealth?.reason).toBe('transport-unavailable');
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
});

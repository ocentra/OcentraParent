import { describe, expect, it } from 'vitest';
import {
  ParentRoute,
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

  it('rejects when the configured dev web bridge is unreachable instead of returning an unavailable snapshot', async () => {
    const parentDevBridgeUrl = 'http://127.0.0.1:1';
    const bridge = createDevWebHostBridge(parentDevBridgeUrl);

    await expect(bridge.loadRoute(ParentRoute.Overview)).rejects.toThrow(
      parentDevBridgeDispatchUnavailableMessage(parentDevBridgeUrl)
    );
    await expect(
      bridge.dispatch({
        action: ParentUiActionKind.RefreshRoute,
        route: ParentRoute.Overview,
        payload: {},
      })
    ).rejects.toThrow(parentDevBridgeDispatchUnavailableMessage(parentDevBridgeUrl));
  });
});

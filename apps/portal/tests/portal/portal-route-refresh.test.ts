import { describe, expect, it } from 'vitest';
import { ParentBridgeConnectionState, ParentRoute } from '../../generated/parent-ui-bridge';
import { shouldRequestNetworkFlowReadModelForRoute } from '../../src/portal-route-refresh';

describe('portal route refresh', () => {
  it('requests network flow read model once when a network drawer route is connected', () => {
    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: ParentBridgeConnectionState.Connected,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: false,
        route: ParentRoute.Activity,
      })
    ).toBe(true);

    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: ParentBridgeConnectionState.Connected,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: true,
        route: ParentRoute.Activity,
      })
    ).toBe(false);

    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: ParentBridgeConnectionState.Connected,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: false,
        route: ParentRoute.NetworkActivity,
      })
    ).toBe(true);
  });

  it('does not request network flow while disconnected or outside network drawer routes', () => {
    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: ParentBridgeConnectionState.Connecting,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: false,
        route: ParentRoute.Activity,
      })
    ).toBe(false);
    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: ParentBridgeConnectionState.Connected,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: false,
        route: ParentRoute.Overview,
      })
    ).toBe(false);
  });

  it('does not overwrite an existing network flow read model event on route entry', () => {
    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: ParentBridgeConnectionState.Connected,
        hasNetworkFlowReadModelEvent: true,
        requestedForRoute: false,
        route: ParentRoute.Activity,
      })
    ).toBe(false);
  });
});

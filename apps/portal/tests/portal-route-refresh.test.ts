import { describe, expect, it } from 'vitest';
import { PortalConnectionState, PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { shouldRequestNetworkFlowReadModelForRoute } from '../src/portal-route-refresh';

describe('portal route refresh', () => {
  it('requests network flow read model once when Activity route is connected', () => {
    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: PortalConnectionState.Connected,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: false,
        route: PortalRoute.Activity,
      })
    ).toBe(true);

    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: PortalConnectionState.Connected,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: true,
        route: PortalRoute.Activity,
      })
    ).toBe(false);
  });

  it('does not request network flow while disconnected or outside Activity route', () => {
    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: PortalConnectionState.Connecting,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: false,
        route: PortalRoute.Activity,
      })
    ).toBe(false);
    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: PortalConnectionState.Connected,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: false,
        route: PortalRoute.Overview,
      })
    ).toBe(false);
  });

  it('does not overwrite an existing network flow read model event on route entry', () => {
    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: PortalConnectionState.Connected,
        hasNetworkFlowReadModelEvent: true,
        requestedForRoute: false,
        route: PortalRoute.Activity,
      })
    ).toBe(false);
  });
});

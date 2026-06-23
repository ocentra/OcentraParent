import { describe, expect, it } from 'vitest';
import { PortalConnectionState } from '@ocentra-parent/schema-domain/portal-contracts';
import { PortalRoute } from '@ocentra-parent/schema-domain/portal-contracts';
import { shouldRequestNetworkFlowReadModelForRoute } from '../../src/portal-route-refresh';

describe('portal route refresh', () => {
  it('requests network flow read model once when a network drawer route is connected', () => {
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

    expect(
      shouldRequestNetworkFlowReadModelForRoute({
        connectionState: PortalConnectionState.Connected,
        hasNetworkFlowReadModelEvent: false,
        requestedForRoute: false,
        route: PortalRoute.NetworkActivity,
      })
    ).toBe(true);
  });

  it('does not request network flow while disconnected or outside network drawer routes', () => {
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

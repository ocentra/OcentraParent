import {
  PortalConnectionState,
  PortalRoute,
  type PortalConnectionState as PortalConnectionStateValue,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';

export type PortalRouteNetworkRefreshState = {
  readonly connectionState: PortalConnectionStateValue;
  readonly requestedForRoute: boolean;
  readonly route: PortalRouteValue;
};

export function shouldRequestNetworkFlowReadModelForRoute({
  connectionState,
  requestedForRoute,
  route,
}: PortalRouteNetworkRefreshState): boolean {
  return route === PortalRoute.Activity && connectionState === PortalConnectionState.Connected && !requestedForRoute;
}

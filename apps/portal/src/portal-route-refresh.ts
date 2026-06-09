import {
  PortalConnectionState,
  isPortalNetworkEvidenceDrawerRoute,
  type PortalConnectionState as PortalConnectionStateValue,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';

export type PortalRouteNetworkRefreshState = {
  readonly connectionState: PortalConnectionStateValue;
  readonly hasNetworkFlowReadModelEvent: boolean;
  readonly requestedForRoute: boolean;
  readonly route: PortalRouteValue;
};

export function shouldRequestNetworkFlowReadModelForRoute({
  connectionState,
  hasNetworkFlowReadModelEvent,
  requestedForRoute,
  route,
}: PortalRouteNetworkRefreshState): boolean {
  return (
    isPortalNetworkEvidenceDrawerRoute(route) &&
    connectionState === PortalConnectionState.Connected &&
    !requestedForRoute &&
    !hasNetworkFlowReadModelEvent
  );
}

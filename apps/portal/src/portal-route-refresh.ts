import { isPortalNetworkEvidenceDrawerRoute } from '@ocentra-parent/portal-domain/routes';
import {
  PortalConnectionState,
  type PortalRoute as PortalRouteValue,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/schema-domain/portal-contracts';
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

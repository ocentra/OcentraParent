import {
  ParentBridgeConnectionState,
  ParentRoute,
  type ParentBridgeConnectionState as ParentBridgeConnectionStateValue,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';

export type PortalRouteNetworkRefreshState = {
  readonly connectionState: ParentBridgeConnectionStateValue;
  readonly hasNetworkFlowReadModelEvent: boolean;
  readonly requestedForRoute: boolean;
  readonly route: ParentRouteId;
};

export function isNetworkEvidenceDrawerRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.Activity || route === ParentRoute.NetworkActivity;
}

export function isInlineNetworkEvidenceDrawerRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.Activity;
}

export function shouldRequestNetworkFlowReadModelForRoute({
  connectionState,
  hasNetworkFlowReadModelEvent,
  requestedForRoute,
  route,
}: PortalRouteNetworkRefreshState): boolean {
  return (
    isNetworkEvidenceDrawerRoute(route) &&
    connectionState === ParentBridgeConnectionState.Connected &&
    !requestedForRoute &&
    !hasNetworkFlowReadModelEvent
  );
}

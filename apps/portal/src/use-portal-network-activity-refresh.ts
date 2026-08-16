import { useEffect, type MutableRefObject } from 'react';
import type { ParentBridgeConnectionState, ParentRouteId } from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import { isNetworkEvidenceDrawerRoute, shouldRequestNetworkFlowReadModelForRoute } from './portal-route-refresh';

type PortalNetworkActivityRefreshHook = {
  readonly actions: PortalRenderActions;
  readonly connectionState: ParentBridgeConnectionState;
  readonly hasNetworkFlowReadModelEvent: boolean;
  readonly networkActivityRefreshRequestedForRouteRef: MutableRefObject<boolean>;
  readonly route: ParentRouteId;
};

export function usePortalNetworkActivityRefresh({
  actions,
  connectionState,
  hasNetworkFlowReadModelEvent,
  networkActivityRefreshRequestedForRouteRef,
  route,
}: PortalNetworkActivityRefreshHook): void {
  useEffect(() => {
    if (!isNetworkEvidenceDrawerRoute(route)) {
      networkActivityRefreshRequestedForRouteRef.current = false;
      return;
    }
    if (
      !shouldRequestNetworkFlowReadModelForRoute({
        connectionState,
        hasNetworkFlowReadModelEvent,
        requestedForRoute: networkActivityRefreshRequestedForRouteRef.current,
        route,
      })
    ) {
      return;
    }
    networkActivityRefreshRequestedForRouteRef.current = true;
    void actions.requestNetworkFlowReadModelRefresh?.();
  }, [actions, connectionState, hasNetworkFlowReadModelEvent, networkActivityRefreshRequestedForRouteRef, route]);
}

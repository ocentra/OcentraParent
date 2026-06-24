import { useEffect, type MutableRefObject } from 'react';
import { type PortalConnectionState as PortalConnectionStateValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { isPortalNetworkEvidenceDrawerRoute } from '@ocentra-parent/portal-domain/routes';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import type { PortalRenderActions } from './portal-actions';
import { shouldRequestNetworkFlowReadModelForRoute } from './portal-route-refresh';

type PortalNetworkActivityRefreshHook = {
  readonly actions: PortalRenderActions;
  readonly connectionState: PortalConnectionStateValue;
  readonly hasNetworkFlowReadModelEvent: boolean;
  readonly networkActivityRefreshRequestedForRouteRef: MutableRefObject<boolean>;
  readonly route: PortalRouteValue;
};

export function usePortalNetworkActivityRefresh({
  actions,
  connectionState,
  hasNetworkFlowReadModelEvent,
  networkActivityRefreshRequestedForRouteRef,
  route,
}: PortalNetworkActivityRefreshHook): void {
  useEffect(() => {
    if (!isPortalNetworkEvidenceDrawerRoute(route)) {
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

import type { ReactElement } from 'react';
import {
  PARENT_PORTAL_CONTENT,
  PARENT_PORTAL_ROUTE,
  PARENT_PORTAL_ROWS,
  PortalDom,
  PortalRoute,
  PortalConnectionState,
  parentPortalRouteContext,
  type PortalRoute as PortalRouteValue,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/portal-domain/contracts';
import { ParentPortalSvgSurface } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface';
import type { ParentPortalSvgControls } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';
import './styles/parent-portal-route.css';

type ParentPortalRouteProps = {
  readonly actions: PortalRenderActions;
  readonly controls: ParentPortalSvgControls;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
};

export function ParentPortalRoute({ actions, controls, route, state }: ParentPortalRouteProps): ReactElement {
  const routeContext = parentPortalRouteContext(route);
  return (
    <div className={PARENT_PORTAL_ROUTE.ClassName}>
      <ParentPortalSvgSurface
        pageMode={routeContext.pageMode}
        controlCode={1}
        seasonId={seasonLabelForConnection(state.connectionState)}
        lastUpdated={latestReportedAt(state)}
        parentPortalRows={PARENT_PORTAL_ROWS}
        userEntry={PARENT_PORTAL_ROWS[0] ?? null}
        nearbyAbove={[]}
        nearbyBelow={[]}
        content={PARENT_PORTAL_CONTENT}
        controls={controls}
        initialNavLabel={routeContext.navLabel}
        initialSelectedControlId={routeContext.selectedControlId}
        assistantRouteActive={route === PortalRoute.Assistant}
        assistantRoutePath={PARENT_PORTAL_ROUTE.HashRoutes.Assistant}
        assistantReturnRoutePath={PARENT_PORTAL_ROUTE.HashRoutes.Overview}
        onRefreshParentPortal={actions.reconnect}
        onMatchmaking={actions.reconnect}
        onNavigate={(routePath) => {
          if (!routePath.startsWith(PortalDom.HashPrefix)) {
            return;
          }
          window.location.hash = routePath;
        }}
        onAssistantCommand={actions.sendCommand}
      />
    </div>
  );
}

function latestReportedAt(state: PortalRuntimeState): string {
  return (
    state.events.at(-1)?.sentAt ?? state.latestSnapshot?.entries.at(-1)?.timestamp ?? PARENT_PORTAL_ROUTE.EmptyTimestamp
  );
}

function seasonLabelForConnection(connectionState: PortalConnectionStateValue): string {
  if (connectionState === PortalConnectionState.Connected) {
    return PARENT_PORTAL_ROUTE.StatusText.Local;
  }
  if (connectionState === PortalConnectionState.Connecting) {
    return PARENT_PORTAL_ROUTE.StatusText.Connecting;
  }
  if (connectionState === PortalConnectionState.Error) {
    return PARENT_PORTAL_ROUTE.StatusText.CheckService;
  }
  return PARENT_PORTAL_ROUTE.StatusText.Offline;
}

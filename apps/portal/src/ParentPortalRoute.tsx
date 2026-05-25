import type { ReactElement } from 'react';
import {
  PARENT_PORTAL_CONTENT,
  PARENT_PORTAL_ROUTE,
  PARENT_PORTAL_ROWS,
  PortalDom,
  PortalConnectionState,
  parentPortalRouteContext,
  type PortalRoute as PortalRouteValue,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/portal-domain/contracts';
import { ParentPortalSvgSurface } from '../../../vendor/ocentra-games-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface';
import {
  DEFAULT_PARENT_PORTAL_SVG_CONTROLS,
  type ParentPortalSvgControls,
} from '../../../vendor/ocentra-games-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';
import './styles/parent-portal-route.css';

type ParentPortalRouteProps = {
  readonly actions: PortalRenderActions;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
};

const parentPortalSurfaceControls: Partial<ParentPortalSvgControls> = {
  layout: {
    ...DEFAULT_PARENT_PORTAL_SVG_CONTROLS.layout,
    topY: 15,
  },
};

export function ParentPortalRoute({ actions, route, state }: ParentPortalRouteProps): ReactElement {
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
        controls={parentPortalSurfaceControls}
        initialNavLabel={routeContext.navLabel}
        initialSelectedControlId={routeContext.selectedControlId}
        onRefreshParentPortal={actions.reconnect}
        onMatchmaking={actions.reconnect}
        onNavigate={(routePath) => {
          if (!routePath.startsWith(PortalDom.HashPrefix)) {
            return;
          }
          window.location.hash = routePath;
        }}
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

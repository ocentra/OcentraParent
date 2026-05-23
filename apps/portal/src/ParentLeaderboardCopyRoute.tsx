import type { ReactElement } from 'react';
import {
  PARENT_LEADERBOARD_COPY_CONTENT,
  PARENT_LEADERBOARD_COPY_ROUTE,
  PARENT_LEADERBOARD_COPY_ROWS,
  PortalConnectionState,
  parentLeaderboardCopyRouteContext,
  type PortalRoute as PortalRouteValue,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/portal-domain/contracts';
import { LeaderboardPageSvgSurface } from '../../../vendor/ocentra-games-core-ui/AppPages/Leaderboard/LeaderboardPageSvgSurface';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';
import './styles/parent-leaderboard-copy-route.css';

type ParentLeaderboardCopyRouteProps = {
  readonly actions: PortalRenderActions;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
};

export function ParentLeaderboardCopyRoute({ actions, route, state }: ParentLeaderboardCopyRouteProps): ReactElement {
  const routeContext = parentLeaderboardCopyRouteContext(route);
  return (
    <div className={PARENT_LEADERBOARD_COPY_ROUTE.ClassName}>
      <LeaderboardPageSvgSurface
        pageMode={routeContext.pageMode}
        gameType={1}
        seasonId={seasonLabelForConnection(state.connectionState)}
        lastUpdated={latestReportedAt(state)}
        leaderboardEntries={PARENT_LEADERBOARD_COPY_ROWS}
        userEntry={PARENT_LEADERBOARD_COPY_ROWS[0] ?? null}
        nearbyAbove={[]}
        nearbyBelow={[]}
        content={PARENT_LEADERBOARD_COPY_CONTENT}
        initialNavLabel={routeContext.navLabel}
        initialSelectedGameId={routeContext.selectedControlId}
        onRefreshLeaderboard={actions.reconnect}
        onMatchmaking={actions.reconnect}
      />
    </div>
  );
}

function latestReportedAt(state: PortalRuntimeState): string {
  return (
    state.events.at(-1)?.sentAt ??
    state.latestSnapshot?.entries.at(-1)?.timestamp ??
    PARENT_LEADERBOARD_COPY_ROUTE.EmptyTimestamp
  );
}

function seasonLabelForConnection(connectionState: PortalConnectionStateValue): string {
  if (connectionState === PortalConnectionState.Connected) {
    return PARENT_LEADERBOARD_COPY_ROUTE.StatusText.Local;
  }
  if (connectionState === PortalConnectionState.Connecting) {
    return PARENT_LEADERBOARD_COPY_ROUTE.StatusText.Connecting;
  }
  if (connectionState === PortalConnectionState.Error) {
    return PARENT_LEADERBOARD_COPY_ROUTE.StatusText.CheckService;
  }
  return PARENT_LEADERBOARD_COPY_ROUTE.StatusText.Offline;
}

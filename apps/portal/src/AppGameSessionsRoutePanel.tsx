import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  decodeDisplayText,
  PortalDevTextToken,
  resolvePortalDevText,
} from '@ocentra-parent/portal-domain/display-text';
import {
  isParentAppGameParentSurfaceRoute,
  type ParentAppGameNotificationParentSurfacePanelSnapshot,
  type ParentAppGamePanelSnapshot,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';
import { AppGameNotificationParentSurfaceRoutePanel } from './AppGameNotificationParentSurfaceRoutePanel';
import { AppGamePolicyReadinessRoutePanel } from './AppGamePolicyReadinessRoutePanel';
import type { PortalRenderActions } from './portal-actions';

const APP_GAME_SESSIONS_TEXT = {
  statusTitle: decodeDisplayText('App activity status'),
  unavailable: decodeDisplayText('Retry status to load app use, game, notification, and policy status.'),
  reported: decodeDisplayText('The latest app use, game, notification, and policy status is shown on this route.'),
  refresh: decodeDisplayText('Refresh app activity'),
} as const;

export function shouldRenderAppGameSessionsRoutePanel(route: ParentRouteId): boolean {
  return isParentAppGameParentSurfaceRoute(route);
}

export function AppGameSessionsRoutePanel({
  actions,
  commandEnabled,
  notificationPanel,
  policyPanel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly notificationPanel: ParentAppGameNotificationParentSurfacePanelSnapshot | null;
  readonly policyPanel: ParentAppGamePanelSnapshot | null;
}): ReactElement {
  const notificationState = notificationPanel?.state ?? 'unavailable';
  const policyState = policyPanel?.loadState ?? 'unavailable';
  const reported = notificationPanel !== null || policyPanel !== null;
  const routeAction = appGameSessionsRouteAction(actions, commandEnabled);
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.AppGameSessions)}
      className={PortalDom.Classes.AppGameSessionsRoutePanel}
      data-ocentra-app-game-status-panel="open-on-unavailable"
      data-ocentra-app-game-route-state={reported ? 'reported' : 'unavailable'}
    >
      <details className={PortalDom.Classes.AppGameSessionsRoutePanelShell} open={!reported}>
        <summary className={PortalDom.Classes.AppGameSessionsRoutePanelToolbar}>
          <div>
            <strong>{APP_GAME_SESSIONS_TEXT.statusTitle}</strong>
            <span>{reported ? APP_GAME_SESSIONS_TEXT.reported : APP_GAME_SESSIONS_TEXT.unavailable}</span>
          </div>
          <span className={PortalDom.Classes.AppGameSessionsRoutePanelState}>
            {reported ? 'reported' : 'unavailable'}
          </span>
        </summary>
        <div className={PortalDom.Classes.AppGameSessionsRoutePanelBody}>
          <button
            className={PortalDom.Classes.CommandResultTab}
            onClick={routeAction.run}
            type={PortalDom.ButtonType.Button}
          >
            {routeAction.label}
          </button>
          <div className={PortalDom.Classes.AppGameSessionsRoutePanelDrawers}>
            <details
              className={PortalDom.Classes.AppGameSessionsRoutePanelDisclosure}
              data-ocentra-app-game-notification-state={notificationState}
            >
              <summary>
                <span>{notificationPanel?.title ?? resolvePortalDevText(PortalDevTextToken.AppGameSessions)}</span>
                <span className={PortalDom.Classes.AppGameSessionsRoutePanelState}>{notificationState}</span>
              </summary>
              <AppGameNotificationParentSurfaceRoutePanel panel={notificationPanel} />
            </details>
            <details
              className={PortalDom.Classes.AppGameSessionsRoutePanelDisclosure}
              data-ocentra-app-game-policy-state={policyState}
            >
              <summary>
                <span>{resolvePortalDevText(PortalDevTextToken.AppGamePolicyReadiness)}</span>
                <span className={PortalDom.Classes.AppGameSessionsRoutePanelState}>{policyState}</span>
              </summary>
              <AppGamePolicyReadinessRoutePanel panel={policyPanel} />
            </details>
          </div>
        </div>
      </details>
    </section>
  );
}

function appGameSessionsRouteAction(
  actions: PortalRenderActions,
  commandEnabled: boolean
): { readonly label: string; readonly run: () => void } {
  if (!commandEnabled || actions.refreshRouteSnapshot === undefined) {
    return { label: resolvePortalDevText(PortalDevTextToken.RetryStatus), run: actions.reconnect };
  }
  return {
    label: APP_GAME_SESSIONS_TEXT.refresh,
    run: () => {
      void actions.refreshRouteSnapshot?.();
    },
  };
}

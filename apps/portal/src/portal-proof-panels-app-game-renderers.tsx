import type { ReactElement } from 'react';
import { AppGameAdapterDispatchRoutePanel } from './AppGameAdapterDispatchRoutePanel';
import { AppGameChildRuntimeTransportReceiptRoutePanel } from './AppGameChildRuntimeTransportReceiptRoutePanel';
import { AppGameNotificationParentSurfaceRoutePanel } from './AppGameNotificationParentSurfaceRoutePanel';
import { AppGamePlatformProofStatusRoutePanel } from './AppGamePlatformProofStatusRoutePanel';
import { AppGamePolicyReadinessRoutePanel } from './AppGamePolicyReadinessRoutePanel';
import { AppGameTimerParentSurfaceRoutePanel } from './AppGameTimerParentSurfaceRoutePanel';
import type { PortalProofPanelsRoutePanelProps } from './portal-proof-panels-renderers';

type AppGameProofPanelId = Extract<
  PortalProofPanelsRoutePanelProps['activePanel'],
  | 'app-game-notification-parent-surface'
  | 'app-game-policy-readiness'
  | 'app-game-platform-proof-status'
  | 'app-game-child-runtime-transport-receipt'
  | 'app-game-adapter-dispatch'
  | 'app-game-timer-parent-surface'
>;

type AppGameProofPanelProps = Pick<
  PortalProofPanelsRoutePanelProps,
  | 'actions'
  | 'activePanel'
  | 'commandEnabled'
  | 'appGameNotificationParentSurfacePanel'
  | 'appGamePolicyReadinessPanel'
  | 'appGamePlatformProofStatusPanel'
  | 'appGameChildRuntimeTransportReceiptPanel'
  | 'appGameAdapterDispatchPanel'
  | 'appGameTimerParentSurfacePanel'
>;

type AppGameProofPanelRenderer = (props: AppGameProofPanelProps) => ReactElement;

const APP_GAME_PROOF_PANEL_RENDERERS: Record<AppGameProofPanelId, AppGameProofPanelRenderer> = {
  'app-game-notification-parent-surface': ({ appGameNotificationParentSurfacePanel }) => (
    <AppGameNotificationParentSurfaceRoutePanel panel={appGameNotificationParentSurfacePanel} />
  ),
  'app-game-policy-readiness': ({ appGamePolicyReadinessPanel }) => (
    <AppGamePolicyReadinessRoutePanel panel={appGamePolicyReadinessPanel} />
  ),
  'app-game-platform-proof-status': ({ actions, commandEnabled, appGamePlatformProofStatusPanel }) => (
    <AppGamePlatformProofStatusRoutePanel
      actions={actions}
      commandEnabled={commandEnabled}
      panel={appGamePlatformProofStatusPanel}
    />
  ),
  'app-game-child-runtime-transport-receipt': ({
    actions,
    commandEnabled,
    appGameChildRuntimeTransportReceiptPanel,
  }) => (
    <AppGameChildRuntimeTransportReceiptRoutePanel
      actions={actions}
      commandEnabled={commandEnabled}
      panel={appGameChildRuntimeTransportReceiptPanel}
    />
  ),
  'app-game-adapter-dispatch': ({ actions, commandEnabled, appGameAdapterDispatchPanel }) => (
    <AppGameAdapterDispatchRoutePanel
      actions={actions}
      commandEnabled={commandEnabled}
      panel={appGameAdapterDispatchPanel}
    />
  ),
  'app-game-timer-parent-surface': ({ actions, commandEnabled, appGameTimerParentSurfacePanel }) => (
    <AppGameTimerParentSurfaceRoutePanel
      actions={actions}
      commandEnabled={commandEnabled}
      panel={appGameTimerParentSurfacePanel}
    />
  ),
};

export function renderAppGameProofPanel(props: AppGameProofPanelProps): ReactElement {
  return APP_GAME_PROOF_PANEL_RENDERERS[props.activePanel as AppGameProofPanelId](props);
}

import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PARENT_PORTAL_ROUTE, parentPortalRouteContext } from '@ocentra-parent/portal-domain/parent-portal-data';
import { PortalRoute } from '@ocentra-parent/schema-domain/portal-contracts';
import {
  PortalConnectionState,
  type PortalRoute as PortalRouteValue,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/schema-domain/portal-contracts';
import type { ParentPortalParentAccessState } from './generated/parent-ui-bridge';
import { resolveParentPortalServiceState } from '@ocentra-parent/portal-domain/parent-portal-service-state';
import { ParentPortalSvgSurface } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface';
import type { ParentPortalSvgControls } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';
import { resolveSnapshotLiveActivityState } from './live-activity-state';
import { PortalDeveloperRoutePanel, shouldRenderPortalDeveloperRoute } from './PortalDeveloperRoutePanel';
import { openPortalFrameTunerWindow } from './portal-dev-tool-window';
import { PortalDiagnosticsRoutePanel } from './PortalDiagnosticsRoutePanel';
import { PortalProofPanelsRoutePanel } from './PortalProofPanelsRoutePanel';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';
import { SetupFirstRunRoutePanel, shouldRenderSetupFirstRunRoute } from './SetupFirstRunRoutePanel';

type ParentPortalRouteProps = {
  readonly actions: PortalRenderActions;
  readonly controls: ParentPortalSvgControls;
  readonly lanPairingAutoScanSequence: number;
  readonly onProductSurfaceReady: () => void;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
};

export function ParentPortalRoute({
  actions,
  controls,
  lanPairingAutoScanSequence,
  onProductSurfaceReady,
  route,
  state,
}: ParentPortalRouteProps): ReactElement {
  const routeContext = parentPortalRouteContext(route);
  const activityState = resolveSnapshotLiveActivityState(state.routeSnapshot?.liveActivity ?? null);
  const shellStatus = state.routeSnapshot?.parentPortalShellStatus ?? null;
  const serviceState = resolveParentPortalServiceState({
    connectionState: state.connectionState,
    events: [],
    snapshotRows: state.routeSnapshot?.parentPortalRows ?? null,
  });
  const commandEnabled = state.commandEnabled;
  const parentAccessState = resolveParentAccessState(shellStatus?.parentAccessState);
  return (
    <div className={PARENT_PORTAL_ROUTE.ClassName}>
      <ParentPortalSvgSurface
        pageMode={routeContext.pageMode}
        controlCode={1}
        seasonId={state.routeSnapshot?.seasonLabel ?? seasonLabelForConnection(state.connectionState)}
        lastUpdated={state.routeSnapshot?.lastUpdated ?? latestReportedAt(state)}
        parentPortalRows={serviceState.parentPortalRows}
        userEntry={serviceState.userEntry}
        nearbyAbove={[]}
        nearbyBelow={[]}
        content={serviceState.content}
        controls={controls}
        initialNavLabel={routeContext.navLabel}
        initialSelectedControlId={routeContext.selectedControlId}
        assistantRouteActive={route === PortalRoute.Assistant}
        assistantRoutePath={PARENT_PORTAL_ROUTE.HashRoutes.Assistant}
        assistantReturnRoutePath={PARENT_PORTAL_ROUTE.HashRoutes.Overview}
        activityState={activityState}
        lanPairingAutoScanSequence={lanPairingAutoScanSequence}
        onInitialLayoutReady={onProductSurfaceReady}
        onRefreshParentPortal={actions.reconnect}
        onMatchmaking={actions.reconnect}
        onNavigate={(routePath) => {
          if (!routePath.startsWith(PortalDom.HashPrefix)) {
            return;
          }
          if (routePath === `${PortalDom.HashPrefix}${PortalRoute.FrameTuner}`) {
            void openPortalFrameTunerWindow();
            return;
          }
          window.location.hash = routePath;
        }}
        onAssistantCommand={actions.sendCommand}
      />
      {route === PortalRoute.Diagnostics ? <PortalDiagnosticsRoutePanel state={state} /> : null}
      {route === PortalRoute.ProofPanels ? (
        <PortalProofPanelsRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          liveActivity={activityState}
          parentAccessState={parentAccessState}
        />
      ) : null}
      {shouldRenderPortalDeveloperRoute(route) ? (
        <PortalDeveloperRoutePanel actions={actions} route={route} state={state} />
      ) : null}
      {shouldRenderSetupFirstRunRoute(route) ? <SetupFirstRunRoutePanel /> : null}
    </div>
  );
}

function resolveParentAccessState(snapshotState: ParentPortalParentAccessState | undefined): ParentPortalParentAccessState {
  return snapshotState ?? 'proof-missing';
}

function latestReportedAt(state: PortalRuntimeState): string {
  return state.latestSnapshot?.entries.at(-1)?.timestamp ?? PARENT_PORTAL_ROUTE.EmptyTimestamp;
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

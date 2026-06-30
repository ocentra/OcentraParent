import type { ComponentProps, ReactElement } from 'react';
import { PARENT_PORTAL_ROUTE, parentPortalRouteContext } from '@ocentra-parent/portal-domain/parent-portal-data';
import {
  type ParentBrowserPanelSnapshot,
  ParentBridgeConnectionState,
  ParentHostBridgeRuntime,
  ParentRoute,
  type ParentSetupFirstRunPanelSnapshot,
  parentRouteHashPath,
  type ParentBridgeConnectionState as ParentBridgeConnectionStateValue,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';
import { resolveParentPortalServiceState } from '@ocentra-parent/portal-domain/parent-portal-service-state';
import { ParentPortalSvgSurface } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface';
import type { ParentPortalSvgControls } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';
import { resolveSnapshotLiveActivityState } from './route-live-activity-state';
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
  readonly route: ParentRouteId;
  readonly state: PortalRuntimeState;
};

type ParentPortalSurfaceActivityState = NonNullable<ComponentProps<typeof ParentPortalSvgSurface>['activityState']> | null;

export function ParentPortalRoute({
  actions,
  controls,
  lanPairingAutoScanSequence,
  onProductSurfaceReady,
  route,
  state,
}: ParentPortalRouteProps): ReactElement {
  const routeContext = parentPortalRouteContext(route);
  const routeLiveActivity = state.routeSnapshot?.liveActivity ?? null;
  const activityState = resolveSnapshotLiveActivityState(routeLiveActivity);
  const surfaceActivityState = activityState as unknown as ParentPortalSurfaceActivityState;
  const serviceState = resolveParentPortalServiceState({
    connectionState: state.connectionState,
    events: [],
    snapshotRows: state.routeSnapshot?.parentPortalRows ?? null,
  });
  const commandEnabled = state.commandEnabled;
  const networkEvidenceSummary = activityState.networkEvidenceSummary ?? null;
  const policyPreviewPanel = activityState.policyPreviewPanel ?? null;
  const appGameNotificationParentSurfacePanel = activityState.appGameNotificationParentSurfacePanel ?? null;
  const appGamePolicyReadinessPanel = activityState.appGamePolicyReadinessPanel ?? null;
  const appGamePlatformProofStatusPanel = activityState.appGamePlatformProofStatusPanel ?? null;
  const appGameChildRuntimeTransportReceiptPanel = activityState.appGameChildRuntimeTransportReceiptPanel ?? null;
  const appGameAdapterDispatchPanel = activityState.appGameAdapterDispatchPanel ?? null;
  const appGameTimerParentSurfacePanel = activityState.appGameTimerParentSurfacePanel ?? null;
  const browserParentExplanationPanel =
    (state.routeSnapshot?.browserPanels?.browserParentExplanation as ParentBrowserPanelSnapshot | null | undefined) ??
    null;
  const socialAuditExplanationPanel =
    (state.routeSnapshot?.browserPanels?.socialAuditExplanation as ParentBrowserPanelSnapshot | null | undefined) ??
    null;
  const socialDashboardPanel =
    (state.routeSnapshot?.browserPanels?.socialDashboard as ParentBrowserPanelSnapshot | null | undefined) ?? null;
  const socialAlertReportPanel =
    (state.routeSnapshot?.browserPanels?.socialAlertReport as ParentBrowserPanelSnapshot | null | undefined) ?? null;
  const socialAlertReportParentSurfacePanel =
    (state.routeSnapshot?.browserPanels?.socialAlertReportParentSurface as
      | ParentBrowserPanelSnapshot
      | null
      | undefined) ?? null;
  const socialParentNotificationDeliveryPanel =
    (state.routeSnapshot?.browserPanels?.socialParentNotificationDelivery as
      | ParentBrowserPanelSnapshot
      | null
      | undefined) ?? null;
  const browserActionIntentStreamStatusPanel =
    (state.routeSnapshot?.browserPanels?.browserActionIntentStreamStatus as
      | ParentBrowserPanelSnapshot
      | null
      | undefined) ?? null;
  const browserSocialProviderReceiptStreamStatusPanel =
    (state.routeSnapshot?.browserPanels?.browserSocialProviderReceiptStreamStatus as
      | ParentBrowserPanelSnapshot
      | null
      | undefined) ?? null;
  const browserSocialProviderReceiptIngestionReadinessStatusPanel =
    (state.routeSnapshot?.browserPanels?.browserSocialProviderReceiptIngestionReadinessStatus as
      | ParentBrowserPanelSnapshot
      | null
      | undefined) ?? null;
  const setupFirstRunPanel =
    (state.routeSnapshot?.setupFirstRunPanel as ParentSetupFirstRunPanelSnapshot | null | undefined) ?? null;
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
        assistantRouteActive={route === ParentRoute.Assistant}
        assistantRoutePath={parentRouteHashPath(ParentRoute.Assistant)}
        assistantReturnRoutePath={parentRouteHashPath(ParentRoute.Overview)}
        activityState={surfaceActivityState}
        lanPairingAutoScanSequence={lanPairingAutoScanSequence}
        onInitialLayoutReady={onProductSurfaceReady}
        onRefreshParentPortal={actions.reconnect}
        onMatchmaking={actions.reconnect}
        onNavigate={(routePath) => {
          if (!routePath.startsWith(ParentHostBridgeRuntime.RouteHashPrefix)) {
            return;
          }
          if (routePath === parentRouteHashPath(ParentRoute.FrameTuner)) {
            void openPortalFrameTunerWindow();
            return;
          }
          window.location.hash = routePath;
        }}
        onAssistantCommand={actions.sendCommand}
      />
      {route === ParentRoute.Diagnostics ? <PortalDiagnosticsRoutePanel state={state} /> : null}
      {route === ParentRoute.ProofPanels ? (
        <PortalProofPanelsRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          liveActivity={activityState}
          networkEvidenceSummary={networkEvidenceSummary}
          policyPreviewPanel={policyPreviewPanel}
          appGameNotificationParentSurfacePanel={appGameNotificationParentSurfacePanel}
          appGamePolicyReadinessPanel={appGamePolicyReadinessPanel}
          appGamePlatformProofStatusPanel={appGamePlatformProofStatusPanel}
          appGameChildRuntimeTransportReceiptPanel={appGameChildRuntimeTransportReceiptPanel}
          appGameAdapterDispatchPanel={appGameAdapterDispatchPanel}
          appGameTimerParentSurfacePanel={appGameTimerParentSurfacePanel}
          browserParentExplanationPanel={browserParentExplanationPanel}
          socialAuditExplanationPanel={socialAuditExplanationPanel}
          socialDashboardPanel={socialDashboardPanel}
          socialAlertReportPanel={socialAlertReportPanel}
          socialAlertReportParentSurfacePanel={socialAlertReportParentSurfacePanel}
          socialParentNotificationDeliveryPanel={socialParentNotificationDeliveryPanel}
          browserActionIntentStreamStatusPanel={browserActionIntentStreamStatusPanel}
          browserSocialProviderReceiptStreamStatusPanel={browserSocialProviderReceiptStreamStatusPanel}
          browserSocialProviderReceiptIngestionReadinessStatusPanel={
            browserSocialProviderReceiptIngestionReadinessStatusPanel
          }
        />
      ) : null}
      {shouldRenderPortalDeveloperRoute(route) ? (
        <PortalDeveloperRoutePanel actions={actions} route={route} state={state} />
      ) : null}
      {shouldRenderSetupFirstRunRoute(route) ? <SetupFirstRunRoutePanel panel={setupFirstRunPanel} /> : null}
    </div>
  );
}

function latestReportedAt(state: PortalRuntimeState): string {
  return state.latestSnapshot?.entries.at(-1)?.timestamp ?? PARENT_PORTAL_ROUTE.EmptyTimestamp;
}

function seasonLabelForConnection(connectionState: ParentBridgeConnectionStateValue): string {
  if (connectionState === ParentBridgeConnectionState.Connected) {
    return PARENT_PORTAL_ROUTE.StatusText.Local;
  }
  if (connectionState === ParentBridgeConnectionState.Connecting) {
    return PARENT_PORTAL_ROUTE.StatusText.Connecting;
  }
  if (connectionState === ParentBridgeConnectionState.Error) {
    return PARENT_PORTAL_ROUTE.StatusText.CheckService;
  }
  return PARENT_PORTAL_ROUTE.StatusText.Offline;
}

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
import { ParentPortalSvgSurface } from './vendor-parent-portal-surface.js';
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

type ParentPortalSurfaceActivityState = NonNullable<
  ComponentProps<typeof ParentPortalSvgSurface>['activityState']
> | null;
type ParentPortalRouteContext = ReturnType<typeof parentPortalRouteContext>;
type ParentPortalServiceState = ReturnType<typeof resolveParentPortalServiceState>;
type ParentPortalLiveActivity = ReturnType<typeof resolveSnapshotLiveActivityState>;
type ParentBrowserPanelKey =
  | 'browserParentExplanation'
  | 'socialAuditExplanation'
  | 'socialDashboard'
  | 'socialAlertReport'
  | 'socialAlertReportParentSurface'
  | 'socialParentNotificationDelivery'
  | 'browserActionIntentStreamStatus'
  | 'browserSocialProviderReceiptStreamStatus'
  | 'browserSocialProviderReceiptIngestionReadinessStatus';

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
  const panels = parentPortalRoutePanels(state, activityState);
  return (
    <div className={PARENT_PORTAL_ROUTE.ClassName}>
      <ParentPortalRouteSurface
        actions={actions}
        activityState={surfaceActivityState}
        controls={controls}
        lanPairingAutoScanSequence={lanPairingAutoScanSequence}
        onProductSurfaceReady={onProductSurfaceReady}
        route={route}
        routeContext={routeContext}
        serviceState={serviceState}
        state={state}
      />
      {route === ParentRoute.Diagnostics ? <PortalDiagnosticsRoutePanel state={state} /> : null}
      <ParentPortalProofPanels
        actions={actions}
        commandEnabled={commandEnabled}
        liveActivity={activityState}
        panels={panels}
        route={route}
      />
      {shouldRenderPortalDeveloperRoute(route) ? (
        <PortalDeveloperRoutePanel actions={actions} route={route} state={state} />
      ) : null}
      {shouldRenderSetupFirstRunRoute(route) ? <SetupFirstRunRoutePanel panel={panels.setupFirstRunPanel} /> : null}
    </div>
  );
}

function ParentPortalRouteSurface({
  actions,
  activityState,
  controls,
  lanPairingAutoScanSequence,
  onProductSurfaceReady,
  route,
  routeContext,
  serviceState,
  state,
}: {
  readonly actions: PortalRenderActions;
  readonly activityState: ParentPortalSurfaceActivityState;
  readonly controls: ParentPortalSvgControls;
  readonly lanPairingAutoScanSequence: number;
  readonly onProductSurfaceReady: () => void;
  readonly route: ParentRouteId;
  readonly routeContext: ParentPortalRouteContext;
  readonly serviceState: ParentPortalServiceState;
  readonly state: PortalRuntimeState;
}): ReactElement {
  return (
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
      activityState={activityState}
      lanPairingAutoScanSequence={lanPairingAutoScanSequence}
      onInitialLayoutReady={onProductSurfaceReady}
      onRefreshParentPortal={actions.reconnect}
      onMatchmaking={actions.reconnect}
      onNavigate={handleParentRouteNavigate}
      onAssistantCommand={actions.sendCommand}
    />
  );
}

function ParentPortalProofPanels({
  actions,
  commandEnabled,
  liveActivity,
  panels,
  route,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: ParentPortalLiveActivity;
  readonly panels: ReturnType<typeof parentPortalRoutePanels>;
  readonly route: ParentRouteId;
}): ReactElement | null {
  if (route !== ParentRoute.ProofPanels) {
    return null;
  }
  return (
    <PortalProofPanelsRoutePanel
      actions={actions}
      commandEnabled={commandEnabled}
      liveActivity={liveActivity}
      networkEvidenceSummary={panels.networkEvidenceSummary}
      policyPreviewPanel={panels.policyPreviewPanel}
      appGameNotificationParentSurfacePanel={panels.appGameNotificationParentSurfacePanel}
      appGamePolicyReadinessPanel={panels.appGamePolicyReadinessPanel}
      appGamePlatformProofStatusPanel={panels.appGamePlatformProofStatusPanel}
      appGameChildRuntimeTransportReceiptPanel={panels.appGameChildRuntimeTransportReceiptPanel}
      appGameAdapterDispatchPanel={panels.appGameAdapterDispatchPanel}
      appGameTimerParentSurfacePanel={panels.appGameTimerParentSurfacePanel}
      browserParentExplanationPanel={panels.browserParentExplanationPanel}
      socialAuditExplanationPanel={panels.socialAuditExplanationPanel}
      socialDashboardPanel={panels.socialDashboardPanel}
      socialAlertReportPanel={panels.socialAlertReportPanel}
      socialAlertReportParentSurfacePanel={panels.socialAlertReportParentSurfacePanel}
      socialParentNotificationDeliveryPanel={panels.socialParentNotificationDeliveryPanel}
      browserActionIntentStreamStatusPanel={panels.browserActionIntentStreamStatusPanel}
      browserSocialProviderReceiptStreamStatusPanel={panels.browserSocialProviderReceiptStreamStatusPanel}
      browserSocialProviderReceiptIngestionReadinessStatusPanel={
        panels.browserSocialProviderReceiptIngestionReadinessStatusPanel
      }
    />
  );
}

function parentPortalRoutePanels(state: PortalRuntimeState, activityState: ParentPortalLiveActivity) {
  return {
    networkEvidenceSummary: activityState.networkEvidenceSummary ?? null,
    policyPreviewPanel: activityState.policyPreviewPanel ?? null,
    appGameNotificationParentSurfacePanel: activityState.appGameNotificationParentSurfacePanel ?? null,
    appGamePolicyReadinessPanel: activityState.appGamePolicyReadinessPanel ?? null,
    appGamePlatformProofStatusPanel: activityState.appGamePlatformProofStatusPanel ?? null,
    appGameChildRuntimeTransportReceiptPanel: activityState.appGameChildRuntimeTransportReceiptPanel ?? null,
    appGameAdapterDispatchPanel: activityState.appGameAdapterDispatchPanel ?? null,
    appGameTimerParentSurfacePanel: activityState.appGameTimerParentSurfacePanel ?? null,
    browserParentExplanationPanel: browserPanelSnapshot(state, 'browserParentExplanation'),
    socialAuditExplanationPanel: browserPanelSnapshot(state, 'socialAuditExplanation'),
    socialDashboardPanel: browserPanelSnapshot(state, 'socialDashboard'),
    socialAlertReportPanel: browserPanelSnapshot(state, 'socialAlertReport'),
    socialAlertReportParentSurfacePanel: browserPanelSnapshot(state, 'socialAlertReportParentSurface'),
    socialParentNotificationDeliveryPanel: browserPanelSnapshot(state, 'socialParentNotificationDelivery'),
    browserActionIntentStreamStatusPanel: browserPanelSnapshot(state, 'browserActionIntentStreamStatus'),
    browserSocialProviderReceiptStreamStatusPanel: browserPanelSnapshot(
      state,
      'browserSocialProviderReceiptStreamStatus'
    ),
    browserSocialProviderReceiptIngestionReadinessStatusPanel: browserPanelSnapshot(
      state,
      'browserSocialProviderReceiptIngestionReadinessStatus'
    ),
    setupFirstRunPanel:
      (state.routeSnapshot?.setupFirstRunPanel as ParentSetupFirstRunPanelSnapshot | null | undefined) ?? null,
  };
}

function browserPanelSnapshot(
  state: PortalRuntimeState,
  key: ParentBrowserPanelKey
): ParentBrowserPanelSnapshot | null {
  const browserPanels = state.routeSnapshot?.browserPanels as
    | Record<ParentBrowserPanelKey, ParentBrowserPanelSnapshot | null | undefined>
    | null
    | undefined;
  return browserPanels?.[key] ?? null;
}

function handleParentRouteNavigate(routePath: string): void {
  if (!routePath.startsWith(ParentHostBridgeRuntime.RouteHashPrefix)) {
    return;
  }
  if (routePath === parentRouteHashPath(ParentRoute.FrameTuner)) {
    void openPortalFrameTunerWindow();
    return;
  }
  window.location.hash = routePath;
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

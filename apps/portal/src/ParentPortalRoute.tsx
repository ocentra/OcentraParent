import { type ReactElement, useEffect, useRef } from 'react';
import {
  PARENT_PORTAL_ROUTE,
  latestParentAssistantResponse,
  parentPortalRouteContext,
} from '@ocentra-parent/portal-domain/parent-portal-data';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  type ParentBrowserPanelSnapshot,
  type ParentPolicyPreviewPanelSnapshot,
  ParentAgentCommand,
  ParentAgentEvent,
  ParentBridgeConnectionState,
  ParentHostBridgeRuntime,
  ParentRoute,
  type ParentScreenSummaryPanelSnapshot,
  parentRouteFromHashPath,
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
import { PolicyPreviewRoutePanel, shouldRenderPolicyPreviewRoute } from './PolicyPreviewRoutePanel';
import { PortalProofPanelsRoutePanel } from './PortalProofPanelsRoutePanel';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';
import { SetupFirstRunRoutePanel, shouldRenderSetupFirstRunRoute } from './SetupFirstRunRoutePanel';
import { ScreenSummaryRoutePanelMount } from './ScreenSummaryRoutePanel';
import { ScreenSettingsRoutePanel, shouldRenderScreenSettingsRoute } from './ScreenSettingsRoutePanel';
import { AppGameSessionsRoutePanel, shouldRenderAppGameSessionsRoutePanel } from './AppGameSessionsRoutePanel';
import { ScheduleRouteUnavailablePanel } from './ScheduleRouteUnavailablePanel';
import { AiRuntimeRoutePanel, shouldRenderAiRuntimeRoute } from './AiRuntimeRoutePanel';
import { TrackingStatusRoutePanel, shouldRenderTrackingStatusRoute } from './TrackingStatusRoutePanel';
import {
  ParentDesktopDistributionRoutePanel,
  resolveParentDesktopDistributionPanelState,
  shouldRenderParentDesktopDistributionRoute,
} from './ParentDesktopDistributionRoutePanel';
import { BrowserActivityRoutePanel, shouldRenderBrowserActivityRoute } from './BrowserActivityRoutePanel';
import { BrowserReviewRoutePanel, shouldRenderBrowserReviewRoute } from './BrowserReviewRoutePanel';
import {
  NetworkEvidenceDrawerRoutePanel,
  shouldRenderNetworkEvidenceDrawerRoute,
} from './NetworkEvidenceDrawerRoutePanel';
import { CapabilityStatusRoutePanel, shouldRenderCapabilityStatusRoute } from './CapabilityStatusRoutePanel';
import { parentPortalWorkspaceIsVisible } from './parent-portal-workspace-visibility';
import { PolicyCategoryRoutePanel, shouldRenderPolicyCategoryRoute } from './PolicyCategoryRoutePanel';
import { RemoteAccessRoutePanel, shouldRenderRemoteAccessRoute } from './RemoteAccessRoutePanel';

type ParentPortalRouteProps = {
  readonly actions: PortalRenderActions;
  readonly controls: ParentPortalSvgControls;
  readonly lanPairingAutoScanSequence: number;
  readonly onProductSurfaceReady: () => void;
  readonly route: ParentRouteId;
  readonly screenSummaryPanel: ParentScreenSummaryPanelSnapshot | null;
  readonly state: PortalRuntimeState;
};

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

const BROWSER_PRODUCT_ROUTE_STACK_CLASS = 'browser-product-route-stack';

export function ParentPortalRoute({
  actions,
  controls,
  lanPairingAutoScanSequence,
  onProductSurfaceReady,
  route,
  screenSummaryPanel,
  state,
}: ParentPortalRouteProps): ReactElement {
  const routeSurfaceRef = useRef<HTMLDivElement>(null);
  const routeContext = parentPortalRouteContext(route);
  const desktopDistributionRoute = shouldRenderParentDesktopDistributionRoute(route);
  const appGameSessionsRoute = shouldRenderAppGameSessionsRoutePanel(route);
  const trackingStatusRoute = shouldRenderTrackingStatusRoute(route);
  const browserActivityRoute = shouldRenderBrowserActivityRoute(route);
  const browserReviewRoute = shouldRenderBrowserReviewRoute(route);
  const networkEvidenceRoute = shouldRenderNetworkEvidenceDrawerRoute(route);
  const capabilityStatusRoute = shouldRenderCapabilityStatusRoute(route);
  const screenSettingsRoute = shouldRenderScreenSettingsRoute(route);
  const policyCategoryRoute = shouldRenderPolicyCategoryRoute(route);
  const remoteAccessRoute = shouldRenderRemoteAccessRoute(route);
  const routeLiveActivity = state.routeSnapshot?.liveActivity ?? null;
  const activityState = resolveSnapshotLiveActivityState(routeLiveActivity, state.events);
  const serviceState = resolveParentPortalServiceState({
    connectionState: state.connectionState,
    events: [],
    snapshotRows: state.routeSnapshot?.parentPortalRows ?? null,
  });
  const assistantResponse = latestParentAssistantResponse(state.events);
  const commandEnabled = state.commandEnabled;
  const panels = parentPortalRoutePanels(state, activityState);
  const scheduleRouteUnavailable = shouldRenderScheduleUnavailablePanel(route, panels.policyPreviewPanel);
  const workspaceVisible = parentPortalWorkspaceIsVisible(route);
  const activityReportHistoryRequestRef = useRef<string | null>(null);
  const latestSavedActivityReportEventId = state.events.find(
    (event) => event.event === ParentAgentEvent.ActivityReportSaved
  )?.eventId;
  useEffect(() => {
    routeSurfaceRef.current?.scrollTo({ behavior: 'auto', left: 0, top: 0 });
    window.scrollTo({ behavior: 'auto', left: 0, top: 0 });
  }, [route]);
  useEffect(() => {
    const reportRoute = route === ParentRoute.Activity || route === ParentRoute.ReportCompiler;
    if (!reportRoute || !state.commandEnabled) {
      activityReportHistoryRequestRef.current = null;
      return;
    }
    const requestKey = `${route}:${latestSavedActivityReportEventId ?? 'initial'}`;
    if (activityReportHistoryRequestRef.current === requestKey) return;
    activityReportHistoryRequestRef.current = requestKey;
    void actions.sendCommand(ParentAgentCommand.ActivityReportHistoryList, {});
  }, [actions, latestSavedActivityReportEventId, route, state.commandEnabled]);
  return (
    <div ref={routeSurfaceRef} className={PARENT_PORTAL_ROUTE.ClassName}>
      {appGameSessionsRoute ? (
        <AppGameSessionsRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          notificationPanel={panels.appGameNotificationParentSurfacePanel}
          policyPanel={panels.appGamePolicyReadinessPanel}
        />
      ) : null}
      <div className={PortalDom.Classes.ParentPortalRouteSurface}>
        <ParentPortalRouteSurface
          actions={actions}
          activityState={activityState}
          controls={controls}
          lanPairingAutoScanSequence={lanPairingAutoScanSequence}
          onProductSurfaceReady={onProductSurfaceReady}
          route={route}
          routeContext={routeContext}
          serviceState={serviceState}
          state={state}
          assistantResponse={assistantResponse}
          workspaceVisible={workspaceVisible}
        />
      </div>
      <ScreenSummaryRoutePanelMount
        actions={actions}
        commandEnabled={commandEnabled}
        panel={screenSummaryPanel}
        route={route}
      />
      {screenSettingsRoute ? (
        <ScreenSettingsRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          serviceResponseSnapshot={state.routeSnapshot?.screenSettingsServiceResponse ?? null}
        />
      ) : null}
      {shouldRenderPolicyPreviewRoute(route) ? (
        <PolicyPreviewRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          panel={panels.policyPreviewPanel}
          route={route}
        />
      ) : null}
      {scheduleRouteUnavailable ? (
        <ScheduleRouteUnavailablePanel onNavigate={handleParentRouteNavigate} />
      ) : desktopDistributionRoute ? (
        <ParentDesktopDistributionRoutePanel
          onNavigate={handleParentRouteNavigate}
          route={route}
          state={resolveParentDesktopDistributionPanelState(state.routeSnapshot)}
        />
      ) : (
        <>
          {policyCategoryRoute ? (
            <PolicyCategoryRoutePanel onNavigate={handleParentRouteNavigate} route={route} />
          ) : null}
          {remoteAccessRoute ? <RemoteAccessRoutePanel onNavigate={handleParentRouteNavigate} /> : null}
          {shouldRenderAiRuntimeRoute(route) ? (
            <AiRuntimeRoutePanel actions={actions} commandEnabled={commandEnabled} liveActivity={activityState} />
          ) : null}
          {networkEvidenceRoute ? (
            <NetworkEvidenceDrawerRoutePanel
              actions={actions}
              commandEnabled={commandEnabled}
              liveActivity={activityState}
              networkEvidenceSummary={panels.networkEvidenceSummary}
              route={route}
            />
          ) : null}
          {browserActivityRoute || browserReviewRoute ? (
            <div className={BROWSER_PRODUCT_ROUTE_STACK_CLASS}>
              {browserActivityRoute ? (
                <BrowserActivityRoutePanel
                  actions={actions}
                  commandEnabled={commandEnabled}
                  liveActivity={activityState}
                />
              ) : null}
              {browserReviewRoute ? (
                <BrowserReviewRoutePanel
                  actions={actions}
                  commandEnabled={commandEnabled}
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
              ) : null}
            </div>
          ) : null}
          {trackingStatusRoute ? (
            <TrackingStatusRoutePanel
              actions={actions}
              commandEnabled={commandEnabled}
              liveActivity={activityState}
              surface="product"
            />
          ) : null}
          {capabilityStatusRoute ? (
            <CapabilityStatusRoutePanel
              actions={actions}
              commandEnabled={commandEnabled}
              liveActivity={activityState}
              shellStatus={state.routeSnapshot?.parentPortalShellStatus ?? null}
            />
          ) : null}
        </>
      )}
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
      {shouldRenderSetupFirstRunRoute(route) ? (
        <SetupFirstRunRoutePanel actions={actions} panel={panels.setupFirstRunPanel} />
      ) : null}
    </div>
  );
}

export function shouldRenderScheduleUnavailablePanel(
  route: ParentRouteId,
  policyPanel: ParentPolicyPreviewPanelSnapshot | null
): boolean {
  return route === ParentRoute.Schedules && policyPanel === null;
}

function ParentPortalRouteSurface({
  actions,
  activityState,
  assistantResponse,
  controls,
  lanPairingAutoScanSequence,
  onProductSurfaceReady,
  route,
  routeContext,
  serviceState,
  state,
  workspaceVisible,
}: {
  readonly actions: PortalRenderActions;
  readonly activityState: ParentPortalLiveActivity;
  readonly assistantResponse: ReturnType<typeof latestParentAssistantResponse>;
  readonly controls: ParentPortalSvgControls;
  readonly lanPairingAutoScanSequence: number;
  readonly onProductSurfaceReady: () => void;
  readonly route: ParentRouteId;
  readonly routeContext: ParentPortalRouteContext;
  readonly serviceState: ParentPortalServiceState;
  readonly state: PortalRuntimeState;
  readonly workspaceVisible: boolean;
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
      error={null}
      statusMessage={state.lastHostMessage}
      content={serviceState.content}
      controls={controls}
      initialNavLabel={routeContext.navLabel}
      initialSelectedControlId={routeContext.selectedControlId}
      assistantRouteActive={route === ParentRoute.Assistant}
      assistantRoutePath={parentRouteHashPath(ParentRoute.Assistant)}
      assistantReturnRoutePath={parentRouteHashPath(ParentRoute.Overview)}
      assistantCommandAvailable={route === ParentRoute.Assistant && state.commandEnabled}
      assistantResponse={assistantResponse}
      activityState={activityState}
      lanPairingAutoScanSequence={lanPairingAutoScanSequence}
      workspaceVisible={workspaceVisible}
      onInitialLayoutReady={onProductSurfaceReady}
      onRefreshParentPortal={actions.reconnect}
      onMatchmaking={actions.reconnect}
      onNavigate={handleParentRouteNavigate}
      {...(state.commandEnabled ? { onAssistantCommand: actions.sendCommand } : {})}
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
    setupFirstRunPanel: state.routeSnapshot?.setupFirstRunPanel ?? null,
  };
}

function browserPanelSnapshot(
  state: PortalRuntimeState,
  key: ParentBrowserPanelKey
): ParentBrowserPanelSnapshot | null {
  return state.routeSnapshot?.browserPanels?.[key] ?? null;
}

function handleParentRouteNavigate(routePath: string): boolean {
  if (!routePath.startsWith(ParentHostBridgeRuntime.RouteHashPrefix)) {
    return false;
  }
  if (parentRouteFromHashPath(routePath) === ParentRoute.FrameTuner) {
    void openPortalFrameTunerWindow(routePath);
    return false;
  }
  window.location.hash = routePath;
  return true;
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

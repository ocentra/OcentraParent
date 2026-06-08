import type { ReactElement } from 'react';
import { AgentEvent, type AgentEventName } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PARENT_PORTAL_ROUTE,
  PortalDom,
  PortalRoute,
  PortalConnectionState,
  parentPortalRouteContext,
  resolveParentPortalServiceState,
  type PortalRoute as PortalRouteValue,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/portal-domain/contracts';
import { ParentPortalSvgSurface } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface';
import type { ParentPortalSvgControls } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';
import { resolveLiveActivityState } from './live-activity-state';
import {
  NetworkEvidenceDrawerRoutePanel,
  shouldRenderNetworkEvidenceDrawerRoute,
} from './NetworkEvidenceDrawerRoutePanel';
import { openPortalFrameTunerWindow } from './portal-dev-tool-window';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';
import {
  AppGameNotificationParentSurfaceRoutePanel,
  shouldRenderAppGameNotificationParentSurfaceRoute,
} from './AppGameNotificationParentSurfaceRoutePanel';
import {
  AppGamePolicyReadinessRoutePanel,
  shouldRenderAppGamePolicyReadinessRoute,
} from './AppGamePolicyReadinessRoutePanel';
import {
  BrowserParentExplanationRoutePanel,
  shouldRenderBrowserParentExplanationRoute,
} from './BrowserParentExplanationRoutePanel';
import {
  shouldRenderSocialAuditExplanationRoute,
  SocialAuditExplanationRoutePanel,
} from './SocialAuditExplanationRoutePanel';
import { shouldRenderSocialAlertReportRoute, SocialAlertReportRoutePanel } from './SocialAlertReportRoutePanel';
import { shouldRenderSocialDashboardRoute, SocialDashboardRoutePanel } from './SocialDashboardRoutePanel';
import { ScreenSettingsRoutePanel, shouldRenderScreenSettingsRoute } from './ScreenSettingsRoutePanel';
import {
  shouldRenderTrackingParentPortalSummary,
  TrackingParentPortalSummaryCard,
} from './TrackingParentPortalSummaryCard';
import { shouldRenderTrackingStatusRoute, TrackingStatusRoutePanel } from './TrackingStatusRoutePanel';

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
  const activityState = resolveLiveActivityState(state.events);
  const browserPanelEvent = resolveBrowserPanelEvent(state.selectedCommandResultEvent);
  const serviceState = resolveParentPortalServiceState({
    connectionState: state.connectionState,
    events: state.events,
  });
  return (
    <div className={PARENT_PORTAL_ROUTE.ClassName}>
      <ParentPortalSvgSurface
        pageMode={routeContext.pageMode}
        controlCode={1}
        seasonId={seasonLabelForConnection(state.connectionState)}
        lastUpdated={latestReportedAt(state)}
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
      {shouldRenderTrackingStatusRoute(route) ? (
        <TrackingStatusRoutePanel
          actions={actions}
          commandEnabled={state.socket?.readyState === WebSocket.OPEN}
          liveActivity={activityState}
        />
      ) : null}
      {shouldRenderTrackingParentPortalSummary(route) ? (
        <TrackingParentPortalSummaryCard liveActivity={activityState} />
      ) : null}
      {shouldRenderNetworkEvidenceDrawerRoute(route) ? (
        <NetworkEvidenceDrawerRoutePanel liveActivity={activityState} />
      ) : null}
      {shouldRenderAppGameNotificationParentSurfaceRoute(route) ? (
        <AppGameNotificationParentSurfaceRoutePanel
          readModel={activityState.appGameNotificationParentSurfaceIntentReadModel}
        />
      ) : null}
      {shouldRenderAppGamePolicyReadinessRoute(route) ? (
        <AppGamePolicyReadinessRoutePanel
          actions={actions}
          commandEnabled={state.socket?.readyState === WebSocket.OPEN}
          readModelResult={activityState.appGamePolicyReadinessReadModel}
        />
      ) : null}
      {shouldRenderBrowserParentExplanationRoute(route) ? <BrowserParentExplanationRoutePanel /> : null}
      {shouldRenderSocialAuditExplanationRoute(route) &&
      browserPanelEvent === AgentEvent.BrowserSocialAuditExplanationReadModelReported ? (
        <SocialAuditExplanationRoutePanel
          actions={actions}
          commandEnabled={state.socket?.readyState === WebSocket.OPEN}
          events={state.events}
        />
      ) : null}
      {shouldRenderSocialAlertReportRoute(route) &&
      browserPanelEvent === AgentEvent.BrowserSocialAlertReportReadModelReported ? (
        <SocialAlertReportRoutePanel
          actions={actions}
          commandEnabled={state.socket?.readyState === WebSocket.OPEN}
          events={state.events}
        />
      ) : null}
      {shouldRenderSocialDashboardRoute(route) &&
      browserPanelEvent === AgentEvent.BrowserSocialDashboardReadModelReported ? (
        <SocialDashboardRoutePanel
          actions={actions}
          commandEnabled={state.socket?.readyState === WebSocket.OPEN}
          events={state.events}
        />
      ) : null}
      {shouldRenderScreenSettingsRoute(route) ? <ScreenSettingsRoutePanel /> : null}
    </div>
  );
}

function resolveBrowserPanelEvent(selectedEvent: AgentEventName): AgentEventName {
  if (browserHashIncludes(AgentEvent.BrowserSocialAlertReportReadModelReported)) {
    return AgentEvent.BrowserSocialAlertReportReadModelReported;
  }
  if (browserHashIncludes(AgentEvent.BrowserSocialAuditExplanationReadModelReported)) {
    return AgentEvent.BrowserSocialAuditExplanationReadModelReported;
  }
  if (browserHashIncludes(AgentEvent.BrowserSocialDashboardReadModelReported)) {
    return AgentEvent.BrowserSocialDashboardReadModelReported;
  }
  if (selectedEvent === AgentEvent.BrowserSocialAlertReportReadModelReported) {
    return selectedEvent;
  }
  if (selectedEvent === AgentEvent.BrowserSocialAuditExplanationReadModelReported) {
    return selectedEvent;
  }
  if (selectedEvent === AgentEvent.BrowserSocialDashboardReadModelReported) {
    return selectedEvent;
  }
  return AgentEvent.BrowserSocialDashboardReadModelReported;
}

function browserHashIncludes(event: AgentEventName): boolean {
  return window.location.hash
    .split(PortalDom.HashQuerySeparator)
    .slice(1)
    .some((part) => part.includes(event));
}

function latestReportedAt(state: PortalRuntimeState): string {
  return (
    state.events[0]?.sentAt ?? state.latestSnapshot?.entries.at(-1)?.timestamp ?? PARENT_PORTAL_ROUTE.EmptyTimestamp
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

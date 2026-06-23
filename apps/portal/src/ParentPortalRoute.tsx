import type { ReactElement } from 'react';
import { AgentEvent, type AgentEventName } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PARENT_PORTAL_ROUTE, parentPortalRouteContext } from '@ocentra-parent/portal-domain/parent-portal-data';
import { PortalRoute } from '@ocentra-parent/schema-domain/portal-contracts';
import {
  PortalConnectionState,
  type PortalRoute as PortalRouteValue,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/schema-domain/portal-contracts';
import { resolveParentPortalServiceState } from '@ocentra-parent/portal-domain/parent-portal-service-state';
import { resolveParentPortalShellStatus } from '@ocentra-parent/portal-domain/parent-portal-shell-status';
import { ParentPortalSvgSurface } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface';
import type { ParentPortalSvgControls } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';
import { resolveLiveActivityState } from './live-activity-state';
import { PortalDeveloperRoutePanel, shouldRenderPortalDeveloperRoute } from './PortalDeveloperRoutePanel';
import { openPortalFrameTunerWindow } from './portal-dev-tool-window';
import { PortalDiagnosticsRoutePanel } from './PortalDiagnosticsRoutePanel';
import { PortalProofPanelsRoutePanel } from './PortalProofPanelsRoutePanel';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';
import {
  AppGameAdapterDispatchRoutePanel,
  shouldRenderAppGameAdapterDispatchRoute,
} from './AppGameAdapterDispatchRoutePanel';
import {
  AppGameNotificationParentSurfaceRoutePanel,
  shouldRenderAppGameNotificationParentSurfaceRoute,
} from './AppGameNotificationParentSurfaceRoutePanel';
import {
  AppGamePolicyReadinessRoutePanel,
  shouldRenderAppGamePolicyReadinessRoute,
} from './AppGamePolicyReadinessRoutePanel';
import { AiRuntimeRoutePanel, shouldRenderAiRuntimeRoute } from './AiRuntimeRoutePanel';
import {
  AppGamePlatformProofStatusRoutePanel,
  shouldRenderAppGamePlatformProofStatusRoute,
} from './AppGamePlatformProofStatusRoutePanel';
import {
  AppGameChildRuntimeTransportReceiptRoutePanel,
  shouldRenderAppGameChildRuntimeTransportReceiptRoute,
} from './AppGameChildRuntimeTransportReceiptRoutePanel';
import {
  AppGameTimerParentSurfaceRoutePanel,
  shouldRenderAppGameTimerParentSurfaceRoute,
} from './AppGameTimerParentSurfaceRoutePanel';
import {
  BrowserParentExplanationRoutePanel,
  shouldRenderBrowserParentExplanationRoute,
} from './BrowserParentExplanationRoutePanel';
import { SetupFirstRunRoutePanel, shouldRenderSetupFirstRunRoute } from './SetupFirstRunRoutePanel';
import {
  shouldRenderSocialAuditExplanationRoute,
  SocialAuditExplanationRoutePanel,
} from './SocialAuditExplanationRoutePanel';
import { shouldRenderSocialAlertReportRoute, SocialAlertReportRoutePanel } from './SocialAlertReportRoutePanel';
import { shouldRenderSocialDashboardRoute, SocialDashboardRoutePanel } from './SocialDashboardRoutePanel';
import { ScreenSettingsRoutePanel, shouldRenderScreenSettingsRoute } from './ScreenSettingsRoutePanel';
import { ScreenSummaryRoutePanel, shouldRenderScreenSummaryRoute } from './ScreenSummaryRoutePanel';

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
  const shellStatus = resolveParentPortalShellStatus({
    route,
    connectionState: state.connectionState,
    events: state.events,
  });
  const browserPanelEvent = resolveBrowserPanelEvent(state.selectedCommandResultEvent);
  const serviceState = resolveParentPortalServiceState({
    connectionState: state.connectionState,
    events: state.events,
  });
  const commandEnabled = state.socket?.readyState === WebSocket.OPEN;
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
      {route === PortalRoute.Diagnostics ? <PortalDiagnosticsRoutePanel state={state} /> : null}
      {route === PortalRoute.ProofPanels ? (
        <PortalProofPanelsRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          liveActivity={activityState}
          parentAccessState={shellStatus.parentAccessState}
        />
      ) : null}
      {shouldRenderPortalDeveloperRoute(route) ? (
        <PortalDeveloperRoutePanel actions={actions} route={route} state={state} />
      ) : null}
      {shouldRenderSetupFirstRunRoute(route) ? <SetupFirstRunRoutePanel /> : null}
      {shouldRenderAppGameNotificationParentSurfaceRoute(route) ? (
        <AppGameNotificationParentSurfaceRoutePanel
          readModel={activityState.appGameNotificationParentSurfaceIntentReadModel}
        />
      ) : null}
      {shouldRenderAppGamePolicyReadinessRoute(route) ? (
        <AppGamePolicyReadinessRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          readModelResult={activityState.appGamePolicyReadinessReadModel}
        />
      ) : null}
      {shouldRenderAppGamePlatformProofStatusRoute(route) ? (
        <AppGamePlatformProofStatusRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          readModelResult={activityState.appGamePlatformProofStatusReadModel}
        />
      ) : null}
      {shouldRenderAppGameChildRuntimeTransportReceiptRoute(route) ? (
        <AppGameChildRuntimeTransportReceiptRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          readModelResult={activityState.appGameChildRuntimeTransportReceiptReadModel}
        />
      ) : null}
      {shouldRenderAppGameAdapterDispatchRoute(route) ? (
        <AppGameAdapterDispatchRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          executeResult={activityState.appGameAdapterDispatchExecutedResult}
          preflightResult={activityState.appGameAdapterDispatchPreflightReadModel}
          resultReadModel={activityState.appGameAdapterDispatchResultReadModel}
        />
      ) : null}
      {shouldRenderAppGameTimerParentSurfaceRoute(route) ? (
        <AppGameTimerParentSurfaceRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          readModelResult={activityState.appGameTimerParentSurfaceReadModel}
        />
      ) : null}
      {shouldRenderAiRuntimeRoute(route) ? (
        <AiRuntimeRoutePanel actions={actions} commandEnabled={commandEnabled} liveActivity={activityState} />
      ) : null}
      {shouldRenderBrowserParentExplanationRoute(route) ? <BrowserParentExplanationRoutePanel /> : null}
      {shouldRenderSocialAuditExplanationRoute(route) &&
      browserPanelEvent === AgentEvent.BrowserSocialAuditExplanationReadModelReported ? (
        <SocialAuditExplanationRoutePanel actions={actions} commandEnabled={commandEnabled} events={state.events} />
      ) : null}
      {shouldRenderSocialAlertReportRoute(route) &&
      browserPanelEvent === AgentEvent.BrowserSocialAlertReportReadModelReported ? (
        <SocialAlertReportRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          events={state.events}
          liveActivity={activityState}
        />
      ) : null}
      {shouldRenderSocialDashboardRoute(route) &&
      browserPanelEvent === AgentEvent.BrowserSocialDashboardReadModelReported ? (
        <SocialDashboardRoutePanel actions={actions} commandEnabled={commandEnabled} events={state.events} />
      ) : null}
      {shouldRenderScreenSettingsRoute(route) ? (
        <ScreenSettingsRoutePanel actions={actions} commandEnabled={commandEnabled} events={state.events} />
      ) : null}
      {shouldRenderScreenSummaryRoute(route) ? <ScreenSummaryRoutePanel liveActivity={activityState} /> : null}
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

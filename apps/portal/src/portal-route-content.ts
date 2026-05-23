import { AgentEvent } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDom,
  PortalRoute,
  PortalText,
  PortalTextToken,
  type PortalRoute as PortalRouteValue,
  type PortalThemeValue,
} from '@ocentra-parent/portal-domain/contracts';
import { renderActivityMemoryGraph } from './activity-memory-graph-panel';
import { renderActivityTimeline } from './activity-timeline';
import { renderAgentSnapshotPanel } from './agent-snapshot-panel';
import { renderDevLogPanel } from './dev-log-panel';
import { renderDiagnosticsPanel } from './diagnostics-panel';
import { renderEvents } from './event-list';
import { latestCommandResult } from './event-results';
import { renderEvidenceStore, renderRecentActivity } from './live-activity-panel';
import { resolveLiveActivityState } from './live-activity-state';
import { renderNetworkFlow } from './live-network-flow-panel';
import { appendRuntimeDetails } from './local-ai-runtime-details';
import { renderCommands } from './portal-command-controls';
import { renderDashboard } from './portal-dashboard';
import {
  renderBrowserEvidenceSummary,
  renderBrowserProtectionSummary,
  renderBrowserStatusSummary,
} from './portal-browser-route-panels';
import {
  renderActivityGuidance,
  renderAiGuidance,
  renderDeviceGuidance,
  renderDriveConnectionsGuidance,
  renderMemoryGuidance,
  renderNotificationsGuidance,
  renderPrivacyDesignGuidance,
} from './portal-capability-guidance';
import type { PortalRenderActions } from './portal-actions';
import { renderDeviceRuleScope } from './portal-device-rule-scope';
import { renderOverviewRoute } from './portal-overview-route';
import { renderBrowserGuidance, renderPolicyGuidance } from './portal-product-guidance';
import type { PortalRuntimeState } from './portal-state';
import { renderPolicyPreview } from './policy-preview-panel';
import { renderSettingsRulesRoute } from './portal-settings-route-panels';

type RouteRenderContext = {
  readonly container: HTMLElement;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
  readonly actions: PortalRenderActions;
  readonly theme: PortalThemeValue;
  readonly rerender: () => void;
};

export function renderRouteContent(
  container: HTMLElement,
  route: PortalRouteValue,
  state: PortalRuntimeState,
  actions: PortalRenderActions,
  theme: PortalThemeValue,
  rerender: () => void
): void {
  const context: RouteRenderContext = {
    container,
    route,
    state,
    actions,
    theme,
    rerender,
  };
  if (renderMonitorRouteContent(context)) {
    return;
  }
  if (renderGuideRouteContent(context)) {
    return;
  }
  if (renderManageRouteContent(context)) {
    return;
  }
  renderOverviewRoute(container, state);
}

function renderMonitorRouteContent(context: RouteRenderContext): boolean {
  const { container, route, state } = context;
  if (route === PortalRoute.Overview) {
    renderOverviewRoute(container, state);
    return true;
  }
  if (route === PortalRoute.Activity) {
    renderActivityRoute(container, state);
    return true;
  }
  if (route === PortalRoute.Browser) {
    renderBrowserRoute(container, state);
    return true;
  }
  return false;
}

function renderGuideRouteContent(context: RouteRenderContext): boolean {
  const { container, route, state } = context;
  if (route === PortalRoute.Policy) {
    renderPolicyRoute(container, state);
    return true;
  }
  if (route === PortalRoute.PrivacyDesign) {
    renderPrivacyDesignGuidance(container);
    return true;
  }
  if (route === PortalRoute.Memory) {
    renderMemoryRoute(container, state);
    return true;
  }
  if (route === PortalRoute.AiRuntime) {
    renderAiRuntimeRoute(container, state);
    return true;
  }
  return false;
}

function renderManageRouteContent(context: RouteRenderContext): boolean {
  const { actions, container, rerender, route, state, theme } = context;
  if (route === PortalRoute.Devices) {
    renderDevicesRoute(container, state);
    return true;
  }
  if (route === PortalRoute.Notifications) {
    renderNotificationsGuidance(container);
    return true;
  }
  if (route === PortalRoute.DriveConnections) {
    renderDriveConnectionsGuidance(container);
    return true;
  }
  if (route === PortalRoute.Diagnostics) {
    renderDiagnosticsRoute(container, state, actions);
    return true;
  }
  if (route === PortalRoute.SettingsRules) {
    renderSettingsRulesRoute(container, theme, rerender);
    return true;
  }
  if (route === PortalRoute.Commands) {
    renderCommands(container, state, actions);
    return true;
  }
  if (route === PortalRoute.Events) {
    renderEvents(container, state.events);
    return true;
  }
  return false;
}

function renderActivityRoute(container: HTMLElement, state: PortalRuntimeState): void {
  const liveActivity = resolveLiveActivityState(state.events);
  renderActivityGuidance(container);
  renderDashboard(container, (dashboard) => {
    renderEvidenceStore(dashboard, liveActivity);
    renderRecentActivity(dashboard, liveActivity);
    renderNetworkFlow(dashboard, liveActivity);
    renderActivityTimeline(dashboard, state.events);
  });
}

function renderBrowserRoute(container: HTMLElement, state: PortalRuntimeState): void {
  const liveActivity = resolveLiveActivityState(state.events);
  renderBrowserGuidance(container);
  renderDashboard(container, (dashboard) => {
    renderBrowserStatusSummary(dashboard, liveActivity);
    renderBrowserEvidenceSummary(dashboard, liveActivity);
    renderBrowserProtectionSummary(dashboard, liveActivity);
  });
}

function renderPolicyRoute(container: HTMLElement, state: PortalRuntimeState): void {
  const liveActivity = resolveLiveActivityState(state.events);
  renderPolicyGuidance(container);
  renderDashboard(container, (dashboard) => {
    renderDeviceRuleScope(dashboard, state);
    renderPolicyPreview(dashboard, state, liveActivity);
  });
}

function renderMemoryRoute(container: HTMLElement, state: PortalRuntimeState): void {
  const liveActivity = resolveLiveActivityState(state.events);
  renderMemoryGuidance(container);
  renderDashboard(container, (dashboard) => {
    renderActivityMemoryGraph(dashboard, liveActivity);
  });
}

function renderAiRuntimeRoute(container: HTMLElement, state: PortalRuntimeState): void {
  renderAiGuidance(container);
  renderDashboard(container, (dashboard) => {
    const panel = document.createElement(PortalDom.Tags.Section);
    panel.className = PortalDom.Classes.Summary;

    const title = document.createElement(PortalDom.Tags.HeadingTwo);
    title.textContent = PortalText.Resolve(PortalTextToken.AiRuntime);

    const metadata = document.createElement(PortalDom.Tags.DefinitionList);
    appendRuntimeDetails(metadata, latestCommandResult(state.events, AgentEvent.LocalAiRuntimeStatusReported));
    panel.append(title, metadata);
    dashboard.append(panel);
  });
}

function renderDevicesRoute(container: HTMLElement, state: PortalRuntimeState): void {
  renderDeviceGuidance(container);
  renderDashboard(container, (dashboard) => {
    renderAgentSnapshotPanel(dashboard, state.latestSnapshot);
    renderDiagnosticsPanel(dashboard, state);
  });
}

function renderDiagnosticsRoute(container: HTMLElement, state: PortalRuntimeState, actions: PortalRenderActions): void {
  renderDashboard(container, (dashboard) => {
    renderDiagnosticsPanel(dashboard, state);
    renderDevLogPanel(dashboard, state.latestSnapshot);
    renderCommands(dashboard, state, actions);
    renderEvents(dashboard, state.events);
  });
}

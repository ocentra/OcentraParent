import {
  PortalCommandButtons,
  PortalConnectionState,
  PortalDetails,
  PortalDom,
  PortalRoute,
  PortalRoutes,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import { renderActivityTimeline } from './activity-timeline';
import { renderAgentSnapshotPanel } from './agent-snapshot-panel';
import { renderCommandResultPanel } from './command-result-panel';
import { appendDetail } from './detail-list';
import { renderDevLogPanel } from './dev-log-panel';
import { renderEvents } from './event-list';
import { renderLiveActivityOverview } from './live-activity-panel';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';

export function renderShell(app: HTMLDivElement, state: PortalRuntimeState, actions: PortalRenderActions): void {
  clear(app);

  const shell = document.createElement(PortalDom.Tags.Section);
  shell.className = PortalDom.Classes.Shell;

  shell.append(renderHeader(actions), renderNavigation(), renderState(state, actions));
  app.append(shell);
}

function renderHeader(actions: PortalRenderActions): HTMLElement {
  const header = document.createElement(PortalDom.Tags.Header);
  header.className = PortalDom.Classes.Header;

  const heading = document.createElement(PortalDom.Tags.HeadingOne);
  heading.textContent = PortalText.Resolve(PortalTextToken.AppTitle);

  const subheading = document.createElement(PortalDom.Tags.Paragraph);
  subheading.textContent = PortalText.Resolve(PortalTextToken.Subtitle);

  const reconnectButton = document.createElement(PortalDom.Tags.Button);
  reconnectButton.type = PortalDom.ButtonType.Button;
  reconnectButton.textContent = PortalText.Resolve(PortalTextToken.Reconnect);
  reconnectButton.addEventListener(PortalDom.Events.Click, actions.reconnect);

  header.append(heading, subheading, reconnectButton);
  return header;
}

function renderNavigation(): HTMLElement {
  const nav = document.createElement(PortalDom.Tags.Navigation);
  nav.className = PortalDom.Classes.Routes;

  for (const route of PortalRoutes) {
    const link = document.createElement(PortalDom.Tags.Anchor);
    link.href = `${PortalDom.HashPrefix}${route}`;
    link.textContent = route;
    if (route === getRoute()) {
      link.setAttribute(PortalDom.Attributes.AriaCurrent, PortalDom.Attributes.Page);
    }
    nav.append(link);
  }

  return nav;
}

function renderState(state: PortalRuntimeState, actions: PortalRenderActions): HTMLElement {
  const container = document.createElement(PortalDom.Tags.Section);
  container.className = PortalDom.Classes.State;

  const route = getRoute();
  if (route === PortalRoute.Commands) {
    renderCommands(container, state, actions);
    return container;
  }
  if (route === PortalRoute.Events) {
    renderEvents(container, state.events);
    return container;
  }

  renderOverview(container, state);
  return container;
}

function renderOverview(container: HTMLElement, state: PortalRuntimeState): void {
  const status = document.createElement(PortalDom.Tags.Section);
  status.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.LiveActivity);

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(
    metadata,
    PortalDetails.Status,
    decodePortalDetailValue(
      state.connectionState === PortalConnectionState.Connected
        ? PortalText.Resolve(PortalTextToken.Connected)
        : PortalText.Resolve(PortalTextToken.Unavailable)
    )
  );
  appendDetail(metadata, PortalDetails.Transport, decodePortalDetailValue(state.agentWsUrl));
  appendDetail(metadata, PortalDetails.State, decodePortalDetailValue(state.connectionState));
  appendDetail(metadata, PortalDetails.Events, decodePortalDetailValue(String(state.events.length)));

  status.append(title, metadata);
  container.append(status);
  renderLiveActivityOverview(container, state);
  renderActivityTimeline(container, state.events);
  renderDevLogPanel(container, state.latestSnapshot);
  renderAgentSnapshotPanel(container, state.latestSnapshot);
}

function renderCommands(container: HTMLElement, state: PortalRuntimeState, actions: PortalRenderActions): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.AgentCommands);

  const commandGrid = document.createElement(PortalDom.Tags.Division);
  commandGrid.className = PortalDom.Classes.CommandGrid;
  commandGrid.append(...PortalCommandButtons.map((command) => commandButton(command, state, actions)));

  panel.append(title, commandGrid);
  renderCommandResultPanel(panel, state);
  container.append(panel);
}

function commandButton(
  command: (typeof PortalCommandButtons)[number],
  state: PortalRuntimeState,
  actions: PortalRenderActions
): HTMLButtonElement {
  const button = document.createElement(PortalDom.Tags.Button);
  button.type = PortalDom.ButtonType.Button;
  button.textContent = command.label;
  button.className = activeCommandButtonClass(command.resultEvent === state.selectedCommandResultEvent);
  button.disabled = state.socket?.readyState !== WebSocket.OPEN;
  button.addEventListener(PortalDom.Events.Click, () => {
    actions.selectCommandResult(command.resultEvent);
    actions.sendCommand(command.command, command.payload);
  });
  return button;
}

function activeCommandButtonClass(active: boolean) {
  if (!active) {
    return PortalDom.Classes.CommandResultTab;
  }
  return [PortalDom.Classes.CommandResultTab, PortalDom.Classes.CommandResultTabActive].join(
    PortalDom.Classes.ClassNameSeparator
  );
}

function getRoute(): PortalRouteValue {
  const route = window.location.hash.replace(/^#\/?/u, PortalDom.EmptyHashRoute);
  if (route === PortalRoute.Commands || route === PortalRoute.Events) {
    return route;
  }
  return PortalRoute.Overview;
}

function clear(element: HTMLElement): void {
  while (element.firstChild !== null) {
    element.firstChild.remove();
  }
}

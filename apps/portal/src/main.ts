import { createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { AgentProtocolDefaults, decodeAgentWebSocketUrl } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDom,
  PortalEnvironment,
  PortalRoute,
  PortalRouteSchema,
  PortalRoutes,
  PortalText,
  PortalTextToken,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import { DevLogField, DevLogMessage, writePortalDevLog } from './dev-logger';
import { PortalApp } from './PortalApp';
import type { PortalRenderActions } from './portal-actions';
import { createPortalRuntimeState } from './portal-state';
import { applyTheme, resolveTheme } from './portal-theme';
import { connectWebSocket, sendCommand } from './transport';
import './styles.css';
import './portal-unified-chrome.css';
import './styles/deck-frame-fit.css';
import './styles/control-card-frame.css';
import './styles/frame-tuner.css';

const agentWsUrl = decodeAgentWebSocketUrl(
  import.meta.env[PortalEnvironment.AgentWebSocketUrl] ?? AgentProtocolDefaults.WebSocketUrl
);
const app = requirePortalRoot();
const root = createRoot(app);
const state = createPortalRuntimeState(agentWsUrl);
let revision = 0;

writePortalDevLog(DevLogMessage.PortalStarted, {
  [DevLogField.AgentWebSocketUrl]: agentWsUrl,
});

const actions: PortalRenderActions = {
  reconnect() {
    connectWebSocket(state, refresh);
  },
  selectCommandResult(resultEvent) {
    state.selectedCommandResultEvent = resultEvent;
    refresh();
  },
  sendCommand(command, payload) {
    sendCommand(state, refresh, command, payload);
  },
};

function refresh(): void {
  revision += 1;
  const theme = resolveTheme();
  applyTheme(theme);
  root.render(
    createElement(PortalApp, {
      actions,
      revision,
      route: getRoute(),
      state,
      theme,
      rerender: refresh,
    })
  );
}

function getRoute(): PortalRouteValue {
  const route = window.location.hash.replace(/^#\/?/u, PortalDom.EmptyHashRoute);
  const parsedRoute = PortalRouteSchema.safeParse(route);
  if (parsedRoute.success && PortalRoutes.includes(parsedRoute.data)) {
    return parsedRoute.data;
  }
  return PortalRoute.Overview;
}

function requirePortalRoot(): HTMLDivElement {
  const rootElement = document.querySelector<HTMLDivElement>(PortalDom.RootSelector);
  if (rootElement === null) {
    throw new Error(PortalText.Resolve(PortalTextToken.RootMissing));
  }
  return rootElement;
}

refresh();
connectWebSocket(state, refresh);
window.addEventListener(PortalDom.Events.HashChange, refresh);

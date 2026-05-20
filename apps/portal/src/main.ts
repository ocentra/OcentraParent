import { AgentProtocolDefaults, decodeAgentWebSocketUrl } from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalDom, PortalEnvironment, PortalText, PortalTextToken } from '@ocentra-parent/portal-domain/contracts';
import { DevLogField, DevLogMessage, writePortalDevLog } from './dev-logger';
import type { PortalRenderActions } from './portal-actions';
import { createPortalRuntimeState } from './portal-state';
import { renderShell } from './render';
import { connectWebSocket, sendCommand } from './transport';
import './styles.css';

const agentWsUrl = decodeAgentWebSocketUrl(
  import.meta.env[PortalEnvironment.AgentWebSocketUrl] ?? AgentProtocolDefaults.WebSocketUrl
);
const app = requirePortalRoot();
const state = createPortalRuntimeState(agentWsUrl);

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
  renderShell(app, state, actions);
}

function requirePortalRoot(): HTMLDivElement {
  const root = document.querySelector<HTMLDivElement>(PortalDom.RootSelector);
  if (root === null) {
    throw new Error(PortalText.Resolve(PortalTextToken.RootMissing));
  }
  return root;
}

refresh();
connectWebSocket(state, refresh);
window.addEventListener(PortalDom.Events.HashChange, refresh);

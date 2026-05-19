import { type AgentCommandName, type AgentProtocolLogFields } from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalConnectionState, PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { createAgentCommand, parseAgentEventMessage, serializeAgentCommand } from './agent-client';
import type { PortalRuntimeState } from './portal-state';

export type PortalRefresh = () => void;

export function connectWebSocket(state: PortalRuntimeState, refresh: PortalRefresh): void {
  state.socket?.close();
  state.connectionState = PortalConnectionState.Connecting;
  refresh();

  const nextSocket = new WebSocket(state.agentWsUrl);
  state.socket = nextSocket;

  nextSocket.addEventListener(PortalDom.Events.Open, () => {
    state.connectionState = PortalConnectionState.Connected;
    refresh();
  });

  nextSocket.addEventListener(PortalDom.Events.Message, (message) => {
    const event = parseAgentEventMessage(message.data);
    state.events.unshift(event);
    if (event.snapshot !== null) {
      state.latestSnapshot = event.snapshot;
    }
    refresh();
  });

  nextSocket.addEventListener(PortalDom.Events.Close, () => {
    if (state.socket === nextSocket) {
      state.connectionState = PortalConnectionState.Disconnected;
      refresh();
    }
  });

  nextSocket.addEventListener(PortalDom.Events.Error, () => {
    state.connectionState = PortalConnectionState.Error;
    refresh();
  });
}

export function sendCommand(
  state: PortalRuntimeState,
  refresh: PortalRefresh,
  command: AgentCommandName,
  payload: AgentProtocolLogFields
): void {
  if (state.socket?.readyState !== WebSocket.OPEN) {
    state.connectionState = PortalConnectionState.Disconnected;
    refresh();
    return;
  }

  state.socket.send(serializeAgentCommand(createAgentCommand(command, payload, state.target)));
}

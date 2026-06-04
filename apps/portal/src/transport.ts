import {
  AgentCommand,
  AgentLanPairingSupportedWebSocketCommand,
  AgentProtocolDefaults,
  decodeAgentDeviceId,
  isAgentProtocolLogText,
  type AgentCommandName,
  type AgentMessageTarget,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalConnectionState, PortalDom, PortalOverviewCommands } from '@ocentra-parent/portal-domain/contracts';
import { createAgentCommand, parseAgentEventMessage, serializeAgentCommand } from './agent-client';
import { DevLogField, DevLogMessage, writePortalDevLog } from './dev-logger';
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
    writePortalDevLog(DevLogMessage.PortalEventReceived, {
      [DevLogField.ConnectionState]: state.connectionState,
    });
    sendOverviewCommands(nextSocket, state);
    refresh();
  });

  nextSocket.addEventListener(PortalDom.Events.Message, (message) => {
    const event = parseAgentEventMessage(message.data);
    state.events.unshift(event);
    writePortalDevLog(DevLogMessage.PortalEventReceived, {
      [DevLogField.Event]: event.event,
      [DevLogField.EventsBuffered]: state.events.length,
    });
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
  if (isPortalDirectEnforcementActionCommand(command)) {
    return;
  }
  if (state.socket?.readyState !== WebSocket.OPEN) {
    state.connectionState = PortalConnectionState.Disconnected;
    refresh();
    return;
  }

  sendSocketCommand(state.socket, state, command, payload);
}

function sendOverviewCommands(socket: WebSocket, state: PortalRuntimeState): void {
  for (const overviewCommand of PortalOverviewCommands) {
    sendSocketCommand(socket, state, overviewCommand.command, overviewCommand.payload);
  }
}

function sendSocketCommand(
  socket: WebSocket,
  state: PortalRuntimeState,
  command: AgentCommandName,
  payload: AgentProtocolLogFields
): void {
  if (isPortalDirectEnforcementActionCommand(command)) {
    return;
  }
  const target = resolvePortalCommandTarget(state.target, command, payload);
  socket.send(serializeAgentCommand(createAgentCommand(command, payload, target)));
  writePortalDevLog(DevLogMessage.PortalCommandSent, {
    [DevLogField.Command]: command,
  });
}

export function isPortalDirectEnforcementActionCommand(command: AgentCommandName): boolean {
  return (
    command === AgentCommand.EnforcementExecute ||
    command === AgentCommand.EnforcementTimerRecover ||
    command === AgentCommand.EnforcementTimerExpire ||
    command === AgentCommand.EnforcementOverrideCancel
  );
}

export function resolvePortalCommandTarget(
  baseTarget: AgentMessageTarget,
  command: AgentCommandName,
  payload: AgentProtocolLogFields
): AgentMessageTarget {
  if (!Object.values(AgentLanPairingSupportedWebSocketCommand).includes(command)) {
    return baseTarget;
  }
  const childDeviceId = payload[AgentProtocolDefaults.Field.LanChildDeviceId];
  if (!isAgentProtocolLogText(childDeviceId) || !childDeviceId.trim()) {
    if (command === AgentLanPairingSupportedWebSocketCommand.StatusGet) {
      return {
        ...baseTarget,
        route: AgentProtocolDefaults.Target.LocalhostWindowsAgent.route,
      };
    }
    if (
      command === AgentLanPairingSupportedWebSocketCommand.AddDeviceRequest &&
      hasLanHouseholdDecisionTarget(payload)
    ) {
      return {
        ...baseTarget,
        route: AgentProtocolDefaults.Target.LocalNetworkWindowsAgent.route,
      };
    }
    if (command !== AgentLanPairingSupportedWebSocketCommand.BrowserDiscoveryScan) {
      return baseTarget;
    }
    return {
      ...baseTarget,
      route: AgentProtocolDefaults.Target.LocalNetworkWindowsAgent.route,
    };
  }
  return {
    ...baseTarget,
    deviceId: decodeAgentDeviceId(childDeviceId),
    route: AgentProtocolDefaults.Target.LocalNetworkWindowsAgent.route,
  };
}

function hasLanHouseholdDecisionTarget(payload: AgentProtocolLogFields): boolean {
  const canonicalDeviceId = payload[AgentProtocolDefaults.Field.LanCanonicalDeviceId];
  const actionKind = payload[AgentProtocolDefaults.Field.LanHouseholdActionKind];
  return (
    isAgentProtocolLogText(canonicalDeviceId) &&
    canonicalDeviceId.trim().length > 0 &&
    isAgentProtocolLogText(actionKind) &&
    actionKind.trim().length > 0
  );
}
